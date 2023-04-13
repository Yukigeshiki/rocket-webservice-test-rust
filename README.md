# rust-rocket-test
A basic web service template for Rust using the Rocket framework and GCP Cloud Run

To deploy to Cloud Run:

```bash
export PROJECT_ID=<project-id>
export SERVICE_NAME=<service-name>
export REGION=<region>

gcloud builds submit --tag gcr.io/$PROJECT_ID/$SERVICE_NAME
gcloud run deploy $SERVICE_NAME --image gcr.io/$PROJECT_ID/$SERVICE_NAME --platform managed --region $REGION --allow-unauthenticated
```
