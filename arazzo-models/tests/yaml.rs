use expectest::prelude::*;
use maplit::hashmap;
use serde_json::json;
use yaml_rust2::YamlLoader;

use arazzo_models::v1_0::ArazzoDescription;

const BASIC_SPEC_EXAMPLE: &str = r#"arazzo: 1.0.1
info:
  title: A pet purchasing workflow
  summary: This Arazzo Description showcases the workflow for how to purchase a pet through a sequence of API calls
  description: |
      This Arazzo Description walks you through the workflow and steps of `searching` for, `selecting`, and `purchasing` an available pet.
  version: 1.0.0
sourceDescriptions:
- name: petStoreDescription
  url: https://github.com/swagger-api/swagger-petstore/blob/master/src/main/resources/openapi.yaml
  type: openapi

workflows:
- workflowId: loginUserAndRetrievePet
  summary: Login User and then retrieve pets
  description: This workflow lays out the steps to login a user and then retrieve pets
  inputs:
      type: object
      properties:
          username:
              type: string
          password:
              type: string
  steps:
  - stepId: loginStep
    description: This step demonstrates the user login step
    operationId: loginUser
    parameters:
      # parameters to inject into the loginUser operation (parameter name must be resolvable at the referenced operation and the value is determined using {expression} syntax)
      - name: username
        in: query
        value: $inputs.username
      - name: password
        in: query
        value: $inputs.password
    successCriteria:
      # assertions to determine step was successful
      - condition: $statusCode == 200
    outputs:
      # outputs from this step
      tokenExpires: $response.header.X-Expires-After
      rateLimit: $response.header.X-Rate-Limit
      sessionToken: $response.body
  - stepId: getPetStep
    description: retrieve a pet by status from the GET pets endpoint
    operationPath: '{$sourceDescriptions.petstoreDescription.url}#/paths/~1pet~1findByStatus/get'
    parameters:
      - name: status
        in: query
        value: 'available'
      - name: Authorization
        in: header
        value: $steps.loginUser.outputs.sessionToken
    successCriteria:
      - condition: $statusCode == 200
    outputs:
      # outputs from this step
      availablePets: $response.body
  outputs:
      available: $steps.getPetStep.outputs.availablePets
"#;

#[test]
fn loads_the_main_spec_descriptors() {
  let yaml = YamlLoader::load_from_str(BASIC_SPEC_EXAMPLE).unwrap();
  let descriptor = ArazzoDescription::try_from(&yaml[0]).unwrap();

  expect!(descriptor.arazzo).to(be_equal_to("1.0.1"));

  let info = &descriptor.info;
  expect!(&info.title).to(be_equal_to("A pet purchasing workflow"));
  expect!(info.summary.clone()).to(be_some().value(
    "This Arazzo Description showcases the workflow for how to purchase a pet through a sequence of API calls"));
  expect!(info.description.clone()).to(be_some().value(
    "This Arazzo Description walks you through the workflow and steps of `searching` for, `selecting`, and `purchasing` an available pet.\n"));
  expect!(&info.version).to(be_equal_to("1.0.0"));

  let sources = &descriptor.source_descriptions;
  expect!(sources.is_empty()).to(be_false());
  let source = &sources[0];
  expect!(&source.name).to(be_equal_to("petStoreDescription"));
  expect!(&source.url).to(be_equal_to("https://github.com/swagger-api/swagger-petstore/blob/master/src/main/resources/openapi.yaml"));
  expect!(source.r#type.clone()).to(be_some().value("openapi"));

  let workflows = &descriptor.workflows;
  expect!(workflows.is_empty()).to(be_false());
  let workflow = &workflows[0];
  expect!(&workflow.workflow_id).to(be_equal_to("loginUserAndRetrievePet"));
  expect!(workflow.summary.clone()).to(be_some().value("Login User and then retrieve pets"));
  expect!(workflow.description.clone()).to(be_some().value("This workflow lays out the steps to login a user and then retrieve pets"));
  expect!(workflow.depends_on.is_empty()).to(be_true());
  expect!(&workflow.inputs).to(be_equal_to(&json!({
    "type": "object",
    "properties": {
      "username": {
        "type": "string"
      },
      "password": {
        "type": "string"
      }
    }
  })));
  expect!(workflow.outputs.clone()).to(be_equal_to(hashmap!{
    "available".to_string() => "$steps.getPetStep.outputs.availablePets".to_string()
  }));

  let steps = &workflow.steps;
  expect!(steps.len()).to(be_equal_to(2));

  let step1 = steps[0].clone();
  expect!(step1.step_id).to(be_equal_to("loginStep"));
  expect!(step1.description).to(be_some().value("This step demonstrates the user login step"));
  expect!(step1.operation_id).to(be_some().value("loginUser"));
  expect!(step1.operation_path).to(be_none());
  expect!(step1.workflow_id).to(be_none());

  let step2 = steps[1].clone();
  expect!(step2.step_id).to(be_equal_to("getPetStep"));
  expect!(step2.description).to(be_some().value("retrieve a pet by status from the GET pets endpoint"));
  expect!(step2.operation_id).to(be_none());
  expect!(step2.operation_path).to(be_some().value("{$sourceDescriptions.petstoreDescription.url}#/paths/~1pet~1findByStatus/get"));
  expect!(step2.workflow_id).to(be_none());
}
