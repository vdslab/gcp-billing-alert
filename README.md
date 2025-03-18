# GCP Billing Alert

A Rust application that retrieves Google Cloud billing information from BigQuery and posts it to Discord. The application is designed to be deployed to Google Cloud Run and scheduled with Cloud Scheduler.

## Features

- Retrieves billing data from BigQuery export
- Groups costs by service
- Formats data into a Discord embed message
- Sends data to Discord via webhook
- Configurable reporting period (default: 30 days)
- Deployable to Google Cloud Run
- Schedulable with Google Cloud Scheduler

## Prerequisites

- Google Cloud project with billing enabled
- BigQuery billing export set up ([instructions](https://cloud.google.com/billing/docs/how-to/export-data-bigquery))
- Discord webhook URL ([instructions](https://support.discord.com/hc/en-us/articles/228383668-Intro-to-Webhooks))
- Rust development environment (for local development)
- Google Cloud SDK (for deployment)

## Configuration

The application is configured using environment variables. Create a `.env` file based on the provided sample:

```bash
cp .env.sample .env
```

Edit the `.env` file with your specific settings:

```
APP_BIGQUERY__PROJECT_ID=your-project-id
APP_BIGQUERY__DATASET=billing_export
APP_BIGQUERY__TABLE=gcp_billing_export_v1_XXXXXX_XXXXXX_XXXXXX
APP_BIGQUERY__DAYS_TO_REPORT=30

APP_DISCORD__WEBHOOK_URL=https://discord.com/api/webhooks/your-webhook-id/your-webhook-token
APP_DISCORD__USERNAME=GCP Billing Bot
APP_DISCORD__AVATAR_URL=https://cloud.google.com/images/social-icon-google-cloud-1200-630.png

# For local development, you may need to set the path to your Google Cloud credentials
GOOGLE_APPLICATION_CREDENTIALS=/path/to/your/credentials.json
```

## Local Development

### Building the Application

```bash
cargo build
```

### Running the Application Locally

```bash
cargo run
```

For local development, you'll need to authenticate with Google Cloud:

```bash
gcloud auth application-default login
```

## Deployment

### Deploying to Google Cloud Run

The included `deploy.sh` script automates the deployment process. The script will first try to load environment variables from the `.env` file if it exists, and then use command-line arguments or defaults:

```bash
./deploy.sh [PROJECT_ID] [REGION] [SCHEDULE] [DISCORD_WEBHOOK_URL]
```

Parameters (all optional if defined in `.env`):

- `PROJECT_ID`: Your Google Cloud project ID (defaults to value from `.env` or gcloud config)
- `REGION`: Google Cloud region (default: asia-northeast1)
- `SCHEDULE`: Cron schedule for Cloud Scheduler (default: "0 9 \* \* \*", which is 9:00 AM daily)
- `DISCORD_WEBHOOK_URL`: Your Discord webhook URL (defaults to value from `.env`)

Example:

```bash
# Using values from .env file
./deploy.sh

# Overriding specific values
./deploy.sh my-project-id asia-northeast1 "0 9 * * *" "https://discord.com/api/webhooks/your-webhook-id/your-webhook-token"
```

### Manual Deployment

If you prefer to deploy manually:

1. Build and push the Docker image:

```bash
gcloud builds submit --tag gcr.io/your-project-id/gcp-billing-alert .
```

2. Deploy to Cloud Run:

```bash
gcloud run deploy gcp-billing-alert \
  --image gcr.io/your-project-id/gcp-billing-alert \
  --platform managed \
  --region asia-northeast1 \
  --no-allow-unauthenticated \
  --set-env-vars="APP_BIGQUERY__PROJECT_ID=your-project-id" \
  --set-env-vars="APP_BIGQUERY__DATASET=billing_export" \
  --set-env-vars="APP_BIGQUERY__TABLE=your_billing_table" \
  --set-env-vars="APP_BIGQUERY__DAYS_TO_REPORT=30" \
  --set-env-vars="APP_DISCORD__WEBHOOK_URL=your_webhook_url" \
  --set-env-vars="APP_DISCORD__USERNAME=GCP Billing Bot" \
  --set-env-vars="APP_DISCORD__AVATAR_URL=https://cloud.google.com/images/social-icon-google-cloud-1200-630.png"
```

3. Set up Cloud Scheduler (see the `deploy.sh` script for details)

## License

See the [LICENSE](LICENSE) file for details.
