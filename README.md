# rocket-webservice-test-rust
A basic web service template for Rust using the Rocket framework and tested with GCP Cloud Run.

#### To deploy to Cloud Run:

Export variables

```bash
export PROJECT_ID=<project-id>
export SERVICE_NAME=<service-name>
export REGION=<region>
```

Substitute values and run Cloud Build file

```bash
 cp ./cloudbuild.yaml ./cloudbuild_copy.yaml && \
 sed -i '' -e "s/<project-id>/${PROJECT_ID}/" \
           -e "s/<service-name>/${SERVICE_NAME}/" \
           -e "s/<region>/${REGION}/" ./cloudbuild_copy.yaml && \
gcloud builds submit --config ./cloudbuild_copy.yaml && \
rm ./cloudbuild_copy.yaml
```
