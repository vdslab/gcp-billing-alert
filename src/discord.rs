use anyhow::{Context as _, Result};
use log::{debug, info};
use reqwest::Client;
use serde::Serialize;

use crate::bigquery::BillingSummary;
use crate::config::DiscordSettings;

#[derive(Debug, Serialize)]
struct DiscordEmbed {
    title: String,
    description: Option<String>,
    color: u32,
    fields: Vec<DiscordEmbedField>,
    footer: Option<DiscordEmbedFooter>,
}

#[derive(Debug, Serialize)]
struct DiscordEmbedField {
    name: String,
    value: String,
    inline: bool,
}

#[derive(Debug, Serialize)]
struct DiscordEmbedFooter {
    text: String,
}

#[derive(Debug, Serialize)]
struct DiscordWebhookPayload {
    username: Option<String>,
    avatar_url: Option<String>,
    content: Option<String>,
    embeds: Vec<DiscordEmbed>,
}

pub async fn send_billing_summary(
    settings: &DiscordSettings,
    billing_summary: &BillingSummary,
) -> Result<()> {
    info!("Formatting billing data for Discord");

    // Format the billing data into a Discord embed
    let mut fields = Vec::new();

    // Add top services as fields (limit to top 10 to avoid Discord's limits)
    for (i, service) in billing_summary.services.iter().take(10).enumerate() {
        fields.push(DiscordEmbedField {
            name: format!("{}. {}", i + 1, service.service_description),
            value: format!("{:.2} {}", service.cost, service.currency),
            inline: true,
        });
    }

    // Create the embed
    let embed = DiscordEmbed {
        title: format!(
            "Google Cloud Billing Summary (Last {} Days)",
            billing_summary.period_days
        ),
        description: Some(format!(
            "Total cost: **{:.2} {}**",
            billing_summary.total_cost, billing_summary.currency
        )),
        color: 4886754, // Google Cloud blue
        fields,
        footer: Some(DiscordEmbedFooter {
            text: format!(
                "Generated on {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            ),
        }),
    };

    debug!("Sending payload to Discord webhook");

    // Check if we're in development mode with a placeholder webhook URL
    if settings.webhook_url.contains("your-webhook-id") {
        // In development mode, just log the payload
        info!("Development mode: Would send to Discord webhook:");
        info!("Title: {}", embed.title);
        info!(
            "Description: {}",
            embed.description.as_ref().unwrap_or(&"None".to_string())
        );
        info!("Services:");
        for field in &embed.fields {
            info!("  {}: {}", field.name, field.value);
        }

        info!("Successfully simulated sending billing data to Discord");
    } else {
        // In production mode, actually send the webhook
        // Create the webhook payload
        let payload = DiscordWebhookPayload {
            username: settings.username.clone(),
            avatar_url: settings.avatar_url.clone(),
            content: None,
            embeds: vec![embed],
        };

        // Validate webhook URL format
        if !settings
            .webhook_url
            .starts_with("https://discord.com/api/webhooks/")
        {
            anyhow::bail!("Invalid Discord webhook URL format");
        }

        let client = Client::new();
        let response = client
            .post(&settings.webhook_url)
            .json(&payload)
            .send()
            .await
            .context("Failed to send request to Discord webhook")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Discord webhook failed: {}", error_text);
        }

        info!("Successfully sent billing data to Discord");
    }
    Ok(())
}
