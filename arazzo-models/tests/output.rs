use maplit::btreemap;
use trim_margin::MarginTrimmable;
use pretty_assertions::assert_eq;
use serde_json::json;
use arazzo_models::either::Either;
use arazzo_models::v1_0::{ArazzoDescription, Criterion, Info, ParameterObject, SourceDescription, Step, Workflow};

#[test]
fn model_to_yaml_test() {
  let yaml = serde_yaml::to_string(&document()).unwrap();
  assert_eq!(yaml.as_str(),
    r#"|arazzo: 1.0.1
       |info:
       |  description: |2-
       |
       |    This Arazzo Description walks you through the workflow and steps of `searching` for, `selecting`, and `purchasing` an available pet.
       |  summary: This Arazzo Description showcases the workflow for how to purchase a pet through a sequence of API calls
       |  title: A pet purchasing workflow
       |  version: 1.0.0
       |sourceDescriptions:
       |- name: petStoreDescription
       |  type: openapi
       |  url: https://github.com/swagger-api/swagger-petstore/blob/master/src/main/resources/openapi.yaml
       |workflows:
       |- description: This workflow lays out the steps to login a user and then retrieve pets
       |  inputs:
       |    properties:
       |      password:
       |        type: string
       |      username:
       |        type: string
       |    type: object
       |  outputs:
       |    available: $steps.getPetStep.outputs.availablePets
       |  steps:
       |  - description: This step demonstrates the user login step
       |    operationId: loginUser
       |    outputs:
       |      rateLimit: $response.header.X-Rate-Limit
       |      sessionToken: $response.body
       |      tokenExpires: $response.header.X-Expires-After
       |    parameters:
       |    - in: query
       |      name: username
       |      value: $inputs.username
       |    - in: query
       |      name: password
       |      value: $inputs.password
       |    stepId: loginStep
       |    successCriteria:
       |    - condition: $statusCode == 200
       |  - description: retrieve a pet by status from the GET pets endpoint
       |    operationPath: '{$sourceDescriptions.petstoreDescription.url}#/paths/~1pet~1findByStatus/get'
       |    outputs:
       |      availablePets: $response.body
       |    parameters:
       |    - in: query
       |      name: status
       |      value: available
       |    - in: header
       |      name: Authorization
       |      value: $steps.loginUser.outputs.sessionToken
       |    stepId: getPetStep
       |    successCriteria:
       |    - condition: $statusCode == 200
       |  summary: Login User and then retrieve pets
       |  workflowId: loginUserAndRetrievePet
       |"#.trim_margin().as_ref().unwrap());
}

fn document() -> ArazzoDescription {
  ArazzoDescription {
    info: Info {
      title: "A pet purchasing workflow".to_string(),
      summary: Some("This Arazzo Description showcases the workflow for how to purchase a pet through a sequence of API calls".to_string()),
      description: Some("\nThis Arazzo Description walks you through the workflow and steps of `searching` for, `selecting`, and `purchasing` an available pet.".to_string()),
      version: "1.0.0".to_string(),
      .. Info::default()
    },
    source_descriptions: vec![
      SourceDescription {
        name: "petStoreDescription".to_string(),
        url: "https://github.com/swagger-api/swagger-petstore/blob/master/src/main/resources/openapi.yaml".to_string(),
        r#type: Some("openapi".to_string()),
        .. SourceDescription::default()
      }
    ],
    workflows: vec![
      Workflow {
        workflow_id: "loginUserAndRetrievePet".to_string(),
        summary: Some("Login User and then retrieve pets".to_string()),
        description: Some("This workflow lays out the steps to login a user and then retrieve pets".to_string()),
        inputs: json!({
          "properties": {
            "password": {
              "type": "string"
            },
            "username": {
              "type": "string"
            }
          },
          "type": "object"
        }),
        steps: vec![
          Step {
            step_id: "loginStep".to_string(),
            operation_id: Some("loginUser".to_string()),
            description: Some("This step demonstrates the user login step".to_string()),
            parameters: vec![
              Either::First(ParameterObject {
                name: "username".to_string(),
                r#in: Some("query".to_string()),
                value: Either::Second("$inputs.username".to_string()),
                .. ParameterObject::default()
              }),
              Either::First(ParameterObject {
                name: "password".to_string(),
                r#in: Some("query".to_string()),
                value: Either::Second("$inputs.password".to_string()),
                .. ParameterObject::default()
              })
            ],
            success_criteria: vec![
              Criterion {
                condition: "$statusCode == 200".to_string(),
                .. Criterion::default()
              }
            ],
            outputs: btreemap!{
              "tokenExpires".to_string() => "$response.header.X-Expires-After".to_string(),
              "rateLimit".to_string() => "$response.header.X-Rate-Limit".to_string(),
              "sessionToken".to_string() => "$response.body".to_string()
            },
            .. Step::default()
          },
          Step {
            step_id: "getPetStep".to_string(),
            operation_path: Some("{$sourceDescriptions.petstoreDescription.url}#/paths/~1pet~1findByStatus/get".to_string()),
            description: Some("retrieve a pet by status from the GET pets endpoint".to_string()),
            parameters: vec![
              Either::First(ParameterObject {
                name: "status".to_string(),
                r#in: Some("query".to_string()),
                value: Either::First("available".into()),
                .. ParameterObject::default()
              }),
              Either::First(ParameterObject {
                name: "Authorization".to_string(),
                r#in: Some("header".to_string()),
                value: Either::Second("$steps.loginUser.outputs.sessionToken".to_string()),
                .. ParameterObject::default()
              })
            ],
            success_criteria: vec![
              Criterion {
                condition: "$statusCode == 200".to_string(),
                .. Criterion::default()
              }
            ],
            outputs: btreemap!{
              "availablePets".to_string() => "$response.body".to_string()
            },
            .. Step::default()
          }
        ],
        outputs: btreemap!{
          "available".to_string() => "$steps.getPetStep.outputs.availablePets".to_string()
        },
        .. Workflow::default()
      }
    ],
    components: Default::default(),
    .. ArazzoDescription::default()
  }
}

#[test]
fn model_to_json_test() {
  let json = serde_json::to_string_pretty(&document()).unwrap();
  assert_eq!(json.as_str(),
    r#"|{
       |  "arazzo": "1.0.1",
       |  "info": {
       |    "description": "\nThis Arazzo Description walks you through the workflow and steps of `searching` for, `selecting`, and `purchasing` an available pet.",
       |    "summary": "This Arazzo Description showcases the workflow for how to purchase a pet through a sequence of API calls",
       |    "title": "A pet purchasing workflow",
       |    "version": "1.0.0"
       |  },
       |  "sourceDescriptions": [
       |    {
       |      "name": "petStoreDescription",
       |      "type": "openapi",
       |      "url": "https://github.com/swagger-api/swagger-petstore/blob/master/src/main/resources/openapi.yaml"
       |    }
       |  ],
       |  "workflows": [
       |    {
       |      "description": "This workflow lays out the steps to login a user and then retrieve pets",
       |      "inputs": {
       |        "properties": {
       |          "password": {
       |            "type": "string"
       |          },
       |          "username": {
       |            "type": "string"
       |          }
       |        },
       |        "type": "object"
       |      },
       |      "outputs": {
       |        "available": "$steps.getPetStep.outputs.availablePets"
       |      },
       |      "steps": [
       |        {
       |          "description": "This step demonstrates the user login step",
       |          "operationId": "loginUser",
       |          "outputs": {
       |            "rateLimit": "$response.header.X-Rate-Limit",
       |            "sessionToken": "$response.body",
       |            "tokenExpires": "$response.header.X-Expires-After"
       |          },
       |          "parameters": [
       |            {
       |              "in": "query",
       |              "name": "username",
       |              "value": "$inputs.username"
       |            },
       |            {
       |              "in": "query",
       |              "name": "password",
       |              "value": "$inputs.password"
       |            }
       |          ],
       |          "stepId": "loginStep",
       |          "successCriteria": [
       |            {
       |              "condition": "$statusCode == 200"
       |            }
       |          ]
       |        },
       |        {
       |          "description": "retrieve a pet by status from the GET pets endpoint",
       |          "operationPath": "{$sourceDescriptions.petstoreDescription.url}#/paths/~1pet~1findByStatus/get",
       |          "outputs": {
       |            "availablePets": "$response.body"
       |          },
       |          "parameters": [
       |            {
       |              "in": "query",
       |              "name": "status",
       |              "value": "available"
       |            },
       |            {
       |              "in": "header",
       |              "name": "Authorization",
       |              "value": "$steps.loginUser.outputs.sessionToken"
       |            }
       |          ],
       |          "stepId": "getPetStep",
       |          "successCriteria": [
       |            {
       |              "condition": "$statusCode == 200"
       |            }
       |          ]
       |        }
       |      ],
       |      "summary": "Login User and then retrieve pets",
       |      "workflowId": "loginUserAndRetrievePet"
       |    }
       |  ]
       |}"#.trim_margin().as_ref().unwrap());
}
