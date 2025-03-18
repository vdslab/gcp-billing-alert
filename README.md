# GCP Billing Alert

A Rust application that retrieves Google Cloud billing information from BigQuery and posts it to Discord. The application is designed to be deployed to Google Cloud Run Jobs and scheduled with Cloud Scheduler.

## Features

- Retrieves billing data from BigQuery export
- Groups costs by service
- Formats data into a Discord embed message
- Sends data to Discord via webhook
- Configurable reporting period (default: 30 days)
- Deployable to Google Cloud Run Jobs
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

### Deploying to Google Cloud Run Jobs

The included `deploy.sh` script automates the deployment process. The script loads environment variables from the `.env` file if it exists, and then uses default values for any missing variables:

```bash
./deploy.sh
```

The script performs two main operations:

1. Builds and pushes the Docker image to Google Container Registry
2. Updates the Cloud Run job with the new image and configuration

Before running the script, make sure your `.env` file contains the necessary configuration or that you have set the default Google Cloud project using `gcloud config set project your-project-id`.

### Manual Deployment

If you prefer to deploy manually:

1. Build and push the Docker image:

```bash
gcloud builds submit --tag gcr.io/your-project-id/gcp-billing-alert .
```

2. Create or update a Cloud Run job:

```bash
gcloud run jobs update gcp-billing-alert \
  --image gcr.io/your-project-id/gcp-billing-alert \
  --region asia-northeast1 \
  --set-env-vars="APP_BIGQUERY__PROJECT_ID=your-project-id" \
  --set-env-vars="APP_BIGQUERY__DATASET=billing_export" \
  --set-env-vars="APP_BIGQUERY__TABLE=your_billing_table" \
  --set-env-vars="APP_BIGQUERY__DAYS_TO_REPORT=30" \
  --set-env-vars="APP_DISCORD__WEBHOOK_URL=your_webhook_url" \
  --set-env-vars="APP_DISCORD__USERNAME=GCP Billing Bot" \
  --set-env-vars="APP_DISCORD__AVATAR_URL=https://cloud.google.com/images/social-icon-google-cloud-1200-630.png"
```

3. Set up Cloud Scheduler to execute the job:

```bash
gcloud scheduler jobs create http gcp-billing-alert-scheduler \
  --schedule="0 9 * * *" \
  --uri="https://REGION-run.googleapis.com/apis/run.googleapis.com/v1/namespaces/PROJECT_ID/jobs/gcp-billing-alert:run" \
  --http-method=POST \
  --oauth-service-account-email=YOUR_SERVICE_ACCOUNT@PROJECT_ID.iam.gserviceaccount.com
```

Replace `REGION`, `PROJECT_ID`, and `YOUR_SERVICE_ACCOUNT` with your specific values.

## License

See the [LICENSE](LICENSE) file for details.
