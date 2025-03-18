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
PROJECT_ID=${APP_BIGQUERY__PROJECT_ID:-$(gcloud config get-value project)}
REGION="asia-northeast1"
SERVICE_NAME="gcp-billing-alert"

if [ -z "$PROJECT_ID" ]; then
  echo "Error: PROJECT_ID is required. Please set APP_BIGQUERY__PROJECT_ID in .env file or configure gcloud default project."
  exit 1
fi

echo "Deploying $SERVICE_NAME to $PROJECT_ID in $REGION"

# Build and push the Docker image
IMAGE_NAME="gcr.io/$PROJECT_ID/$SERVICE_NAME"
echo "Building and pushing Docker image: $IMAGE_NAME"
gcloud builds submit --tag "$IMAGE_NAME" .

# Update Cloud Run job
echo "Updating Cloud Run job"
gcloud run jobs update "$SERVICE_NAME" \
  --image "$IMAGE_NAME" \
  --region "$REGION" \
  --set-env-vars="APP_BIGQUERY__PROJECT_ID=$PROJECT_ID" \
  --set-env-vars="APP_BIGQUERY__DATASET=${APP_BIGQUERY__DATASET:-billing_export}" \
  --set-env-vars="APP_BIGQUERY__TABLE=${APP_BIGQUERY__TABLE:-gcp_billing_export_v1_XXXXXX_XXXXXX_XXXXXX}" \
  --set-env-vars="APP_BIGQUERY__DAYS_TO_REPORT=${APP_BIGQUERY__DAYS_TO_REPORT:-30}" \
  --set-env-vars="APP_DISCORD__WEBHOOK_URL=${APP_DISCORD__WEBHOOK_URL:-}" \
  --set-env-vars="APP_DISCORD__USERNAME=${APP_DISCORD__USERNAME:-GCP Billing Bot}" \
  --set-env-vars="APP_DISCORD__AVATAR_URL=${APP_DISCORD__AVATAR_URL:-https://cloud.google.com/images/social-icon-google-cloud-1200-630.png}"

echo "Deployment completed successfully!"
