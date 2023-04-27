# rocket-webservice-test-rust
A basic web service template for Rust using the Rocket framework and tested with GCP Cloud Run.

For a GraphQL implementation, checkout the `feat/graphql` branch.

For a MongoDB integrations example, checkout the `feat/db-integrations` branch. I'll be adding more database integration
examples to this branch at a later stage as well.

#### To deploy to Cloud Run:

Export variables

```bash
export PROJECT_ID=<project-id>
export SERVICE_NAME=<service-name>
export SERVICE_ACC=<service-acc>
export REGION=<region>
```

Substitute values and run Cloud Build file

```bash
 cp ./cloudbuild.yaml ./cloudbuild_copy.yaml && \
 sed -i '' -e "s/<project-id>/${PROJECT_ID}/" \
           -e "s/<service-name>/${SERVICE_NAME}/" \
           -e "s/<service-acc>/${SERVICE_ACC}/" \
           -e "s/<region>/${REGION}/" ./cloudbuild_copy.yaml && \
gcloud builds submit --config ./cloudbuild_copy.yaml && \
rm ./cloudbuild_copy.yaml
```

The permissions needed for the Cloud Build and Cloud Run service accounts can be found in the README of [this](https://github.com/Yukigeshiki/gcp-cloud-run-deploy-rust)
repo.
