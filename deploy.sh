#!/bin/bash
set -e

# Load environment variables from .env file
if [ -f .env ]; then
  echo "Loading environment variables from .env file"
  export $(grep -v '^#' .env | xargs)
else
  echo "Warning: .env file not found. Using default or provided values."
fi

# Configuration
PROJECT_ID=${1:-${APP_BIGQUERY__PROJECT_ID:-$(gcloud config get-value project)}}
REGION=${2:-"asia-northeast1"}
SERVICE_NAME="gcp-billing-alert"
SCHEDULE=${3:-"0 9 * * *"}  # Default: 9:00 AM every day
DISCORD_WEBHOOK_URL=${4:-${APP_DISCORD__WEBHOOK_URL:-""}}

if [ -z "$PROJECT_ID" ]; then
  echo "Error: PROJECT_ID is required"
  echo "Usage: $0 PROJECT_ID [REGION] [SCHEDULE] [DISCORD_WEBHOOK_URL]"
  exit 1
fi

if [ -z "$DISCORD_WEBHOOK_URL" ]; then
  echo "Warning: DISCORD_WEBHOOK_URL is not provided. You will need to set it manually later."
fi

echo "Deploying $SERVICE_NAME to $PROJECT_ID in $REGION with schedule: $SCHEDULE"

# Build and push the Docker image
IMAGE_NAME="gcr.io/$PROJECT_ID/$SERVICE_NAME"
echo "Building and pushing Docker image: $IMAGE_NAME"
gcloud builds submit --tag "$IMAGE_NAME" .

# Deploy to Cloud Run
echo "Deploying to Cloud Run"
gcloud run deploy "$SERVICE_NAME" \
  --image "$IMAGE_NAME" \
  --platform managed \
  --region "$REGION" \
  --no-allow-unauthenticated \
  --set-env-vars="APP_BIGQUERY__PROJECT_ID=$PROJECT_ID" \
  --set-env-vars="APP_BIGQUERY__DATASET=${APP_BIGQUERY__DATASET:-billing_export}" \
  --set-env-vars="APP_BIGQUERY__TABLE=${APP_BIGQUERY__TABLE:-gcp_billing_export_v1_XXXXXX_XXXXXX_XXXXXX}" \
  --set-env-vars="APP_BIGQUERY__DAYS_TO_REPORT=${APP_BIGQUERY__DAYS_TO_REPORT:-30}" \
  --set-env-vars="APP_DISCORD__WEBHOOK_URL=$DISCORD_WEBHOOK_URL" \
  --set-env-vars="APP_DISCORD__USERNAME=${APP_DISCORD__USERNAME:-GCP Billing Bot}" \
  --set-env-vars="APP_DISCORD__AVATAR_URL=${APP_DISCORD__AVATAR_URL:-https://cloud.google.com/images/social-icon-google-cloud-1200-630.png}"

# Get the Cloud Run service URL
SERVICE_URL=$(gcloud run services describe "$SERVICE_NAME" --region "$REGION" --format="value(status.url)")
echo "Cloud Run service deployed at: $SERVICE_URL"

# Create a service account for Cloud Scheduler
SA_NAME="$SERVICE_NAME-invoker"
SA_EMAIL="$SA_NAME@$PROJECT_ID.iam.gserviceaccount.com"

echo "Creating service account for Cloud Scheduler: $SA_EMAIL"
gcloud iam service-accounts create "$SA_NAME" \
  --display-name="$SERVICE_NAME Invoker"

# Grant the service account permission to invoke the Cloud Run service
echo "Granting Cloud Run invoker permission to $SA_EMAIL"
gcloud run services add-iam-policy-binding "$SERVICE_NAME" \
  --region="$REGION" \
  --member="serviceAccount:$SA_EMAIL" \
  --role="roles/run.invoker"

# Create the Cloud Scheduler job
JOB_NAME="$SERVICE_NAME-job"
echo "Creating Cloud Scheduler job: $JOB_NAME"
gcloud scheduler jobs create http "$JOB_NAME" \
  --schedule="$SCHEDULE" \
  --uri="$SERVICE_URL" \
  --http-method=GET \
  --oidc-service-account-email="$SA_EMAIL" \
  --oidc-token-audience="$SERVICE_URL"

echo "Deployment completed successfully!"
echo ""
# Only show the table update message if we're using the default placeholder table name
if [[ "${APP_BIGQUERY__TABLE:-}" == *"XXXXXX_XXXXXX_XXXXXX"* ]]; then
  echo "IMPORTANT: You need to update the BigQuery table name in the Cloud Run service configuration."
  echo "Run the following command to edit the configuration:"
  echo "gcloud run services update $SERVICE_NAME --region=$REGION --set-env-vars=APP_BIGQUERY__TABLE=your_actual_billing_table_name"
fi
echo ""
echo "To manually trigger the job, run:"
echo "gcloud scheduler jobs run $JOB_NAME"
