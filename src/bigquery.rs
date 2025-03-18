use anyhow::{Context, Result};
use gcp_bigquery_client::Client;
use gcp_bigquery_client::model::query_request::QueryRequest;
use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::config::BigQuerySettings;

#[derive(Debug, Serialize, Deserialize)]
pub struct BillingData {
    pub service_description: String,
    pub cost: f64,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BillingSummary {
    pub total_cost: f64,
    pub currency: String,
    pub services: Vec<BillingData>,
    pub period_days: i64,
}

pub async fn get_billing_data(settings: &BigQuerySettings) -> Result<BillingSummary> {
    info!("Connecting to BigQuery");

    // Create a BigQuery client
    let client = Client::from_application_default_credentials()
        .await
        .context("Failed to create BigQuery client from application default credentials")?;

    // SQL query to get billing data grouped by service
    let query = format!(
        r#"
        WITH billing_data AS (
          SELECT
            service.description as service_description,
            SUM(cost) as cost,
            currency
          FROM `{}.{}.{}`
          WHERE DATE(usage_start_time) >= DATE_SUB(CURRENT_DATE(), INTERVAL {} DAY)
          GROUP BY service_description, currency
          HAVING cost > 0
          ORDER BY cost DESC
        ),
        total AS (
          SELECT SUM(cost) as total_cost, currency
          FROM billing_data
          GROUP BY currency
        )
        SELECT
          bd.service_description,
          bd.cost,
          bd.currency,
          t.total_cost
        FROM billing_data bd
        JOIN total t ON bd.currency = t.currency
        ORDER BY bd.cost DESC
        "#,
        settings.project_id, settings.dataset, settings.table, settings.days_to_report
    );

    debug!("Executing query: {}", query);

    // Create a query request
    let query_request = QueryRequest::new(format!("{}", query));

    // Execute the query
    let mut result = client
        .job()
        .query(&settings.project_id, query_request)
        .await
        .context("Failed to execute BigQuery query")?;

    // Process the results
    let mut services = Vec::new();
    let mut total_cost = 0.0;
    let mut currency = String::new();

    // Process each row
    while result.next_row() {
        // Extract values from the row
        let service_description = result
            .get_string_by_name("service_description")?
            .unwrap_or_default();
        let cost = result.get_f64_by_name("cost")?.unwrap_or_default();
        let row_currency = result.get_string_by_name("currency")?.unwrap_or_default();
        let row_total_cost = result.get_f64_by_name("total_cost")?.unwrap_or_default();

        // Set the total cost and currency from the first row
        if currency.is_empty() {
            total_cost = row_total_cost;
            currency = row_currency.clone();
        }

        services.push(BillingData {
            service_description,
            cost,
            currency: row_currency,
        });
    }

    Ok(BillingSummary {
        total_cost,
        currency,
        services,
        period_days: settings.days_to_report,
    })
}
