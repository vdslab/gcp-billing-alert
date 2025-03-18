pub mod bigquery;
pub mod config;
pub mod discord;

use anyhow::{Context, Result};
use env_logger::Env;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting GCP Billing Alert application");

    // Load configuration from environment variables
    let settings = config::Settings::from_env().context("Failed to load configuration")?;

    // Get billing data from BigQuery
    let billing_data = bigquery::get_billing_data(&settings.bigquery)
        .await
        .context("Failed to get billing data from BigQuery")?;

    info!(
        "Retrieved billing data: Total cost {:.2} {} for the last {} days",
        billing_data.total_cost, billing_data.currency, billing_data.period_days
    );

    // Send billing data to Discord
    discord::send_billing_summary(&settings.discord, &billing_data)
        .await
        .context("Failed to send billing data to Discord")?;

    info!("GCP Billing Alert completed successfully");
    Ok(())
}
