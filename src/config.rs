use anyhow::Result;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub bigquery: BigQuerySettings,
    pub discord: DiscordSettings,
}

#[derive(Debug, Deserialize)]
pub struct BigQuerySettings {
    pub project_id: String,
    pub dataset: String,
    pub table: String,
    pub days_to_report: i64,
}

#[derive(Debug, Deserialize)]
pub struct DiscordSettings {
    pub webhook_url: String,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
}

impl Settings {
    pub fn from_env() -> Result<Self> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        Ok(Self {
            bigquery: BigQuerySettings {
                project_id: env::var("APP_BIGQUERY__PROJECT_ID")?,
                dataset: env::var("APP_BIGQUERY__DATASET")?,
                table: env::var("APP_BIGQUERY__TABLE")?,
                days_to_report: env::var("APP_BIGQUERY__DAYS_TO_REPORT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()?,
            },
            discord: DiscordSettings {
                webhook_url: env::var("APP_DISCORD__WEBHOOK_URL")?,
                username: env::var("APP_DISCORD__USERNAME").ok(),
                avatar_url: env::var("APP_DISCORD__AVATAR_URL").ok(),
            },
        })
    }
}
