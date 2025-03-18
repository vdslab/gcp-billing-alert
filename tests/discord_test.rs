use anyhow::Result;
use gcp_billing_alert::bigquery::{BillingData, BillingSummary};
use gcp_billing_alert::config::Settings;
use gcp_billing_alert::discord::send_billing_summary;

#[tokio::test]
async fn test_discord_message_posting() -> Result<()> {
    // Load settings from environment variables (.env file)
    let settings = Settings::from_env()?;

    // Create a test billing summary
    let billing_summary = BillingSummary {
        total_cost: 100.0,
        currency: "USD".to_string(),
        services: vec![
            BillingData {
                service_description: "Test Service 1".to_string(),
                cost: 75.0,
                currency: "USD".to_string(),
            },
            BillingData {
                service_description: "Test Service 2".to_string(),
                cost: 25.0,
                currency: "USD".to_string(),
            },
        ],
        period_days: 7,
    };

    // Attempt to send the billing summary to Discord
    // Use ? to propagate the error and fail the test if there's an error
    send_billing_summary(&settings.discord, &billing_summary).await?;

    println!("Successfully sent message to Discord");
    Ok(())
}
