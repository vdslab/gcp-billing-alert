use anyhow::Result;
use gcp_billing_alert::bigquery::get_billing_data;
use gcp_billing_alert::config::Settings;

#[tokio::test]
async fn test_bigquery_data_extraction() -> Result<()> {
    // Load settings from environment variables (.env file)
    let settings = Settings::from_env()?;

    // Attempt to get billing data from BigQuery
    // Use ? to propagate the error and fail the test if there's an error
    let billing_summary = get_billing_data(&settings.bigquery).await?;

    // Print the result for debugging
    println!("Successfully retrieved billing data:");
    println!(
        "Total cost: {:.2} {}",
        billing_summary.total_cost, billing_summary.currency
    );
    println!("Period: {} days", billing_summary.period_days);
    println!("Services:");
    for service in &billing_summary.services {
        println!(
            "  {}: {:.2} {}",
            service.service_description, service.cost, service.currency
        );
    }

    Ok(())
}
