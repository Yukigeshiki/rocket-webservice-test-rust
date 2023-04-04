# rust-rocket-test
Testing the Rocket web framework with GCP Cloud Run

To deploy to Cloud Run:

```bash
export PROJECT_ID=<project-id>
export SERVICE_NAME=<service-name>
export REGION=<region>

gcloud builds submit --tag gcr.io/$PROJECT_ID/$SERVICE_NAME
gcloud run deploy $SERVICE_NAME --image gcr.io/$PROJECT_ID/$SERVICE_NAME --platform managed --region $REGION --allow-unauthenticated
```
