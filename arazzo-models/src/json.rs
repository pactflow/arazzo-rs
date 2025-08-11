//! Functions and Traits for loading Arazzo objects from a JSON document

use std::collections::HashMap;
use std::rc::Rc;

use anyhow::anyhow;
use maplit::hashmap;
use serde_json::{Map, Value};

use crate::either::Either;
use crate::extensions::{json_extract_extensions, AnyValue};
use crate::payloads::{EmptyPayload, JsonPayload, Payload, StringPayload};
use crate::v1_0::{
  ArazzoDescription,
  Components,
  Criterion,
  CriterionExpressionType,
  FailureObject,
  Info,
  ParameterObject,
  PayloadReplacement,
  RequestBody,
  ReusableObject,
  SourceDescription,
  Step,
  SuccessObject,
  Workflow
};

impl TryFrom<&Value> for ArazzoDescription {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(map) = value.as_object() {
      if let Ok(version) = json_object_require_string(map, "arazzo") {
        let info = if let Some(json) = map.get("info") {
          Info::try_from(json)
        } else {
          Err(anyhow!("Info Object is required [4.6.1.1 Fixed Fields]"))
        }?;
        let source_descriptions = json_load_source_descriptions(map)?;
        let workflows = json_load_workflows(map)?;
        let components = if let Some(value) = map.get("components") {
          Components::try_from(value)?
        } else {
          Components::default()
        };

        Ok(ArazzoDescription {
          arazzo: version,
          info,
          source_descriptions,
          workflows,
          components,
          extensions: json_extract_extensions(&map)?
        })
      } else {
        Err(anyhow!("Arazzo version number is required [4.6.1.1 Fixed Fields]"))
      }
    } else {
      Err(anyhow!("JSON value must be an Object, got {:?}", value))
    }
  }
}

impl TryFrom<&Value> for SourceDescription {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(map) = value.as_object() {
      Ok(SourceDescription {
        name: json_object_require_string(&map, "name")?,
        url: json_object_require_string(&map, "url")?,
        r#type: json_object_lookup_string(&map, "type"),
        extensions: json_extract_extensions(&map)?
      })
    } else {
      Err(anyhow!("JSON value must be an Object, got {:?}", value))
    }
  }
}

fn json_load_source_descriptions(map: &Map<String, Value>) -> anyhow::Result<Vec<SourceDescription>> {
  if let Some(descriptions) = map.get("sourceDescriptions") &&
    let Some(array) = descriptions.as_array() {
    if array.is_empty() {
      Err(anyhow!("Source Description list must have at least one entry [4.6.1.1 Fixed Fields]"))
    } else {
      let mut list = vec![];

      for item in array {
        list.push(SourceDescription::try_from(item)?);
      }

      Ok(list)
    }
  } else {
    Err(anyhow!("Source Description Object is required [4.6.1.1 Fixed Fields]"))
  }
}

impl TryFrom<&Value> for Info {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(map) = value.as_object() {
      Ok(Info {
        title: json_object_require_string(&map, "title")?,
        summary: json_object_lookup_string(&map, "summary"),
        description: json_object_lookup_string(&map, "description"),
        version: json_object_require_string(&map, "version")?,
        extensions: json_extract_extensions(&map)?
      })
    } else {
      Err(anyhow!("JSON value must be an Object, got {:?}", value))
    }
  }
}

fn json_load_workflows(map: &Map<String, Value>) -> anyhow::Result<Vec<Workflow>> {
  if let Some(array) = map.get("workflows") &&
     let Some(workflows) = array.as_array() {
    if workflows.is_empty() {
      Err(anyhow!("Workflows list must have at least one entry [4.6.1.1 Fixed Fields]"))
    } else {
      let mut list = vec![];

      for item in workflows {
        list.push(Workflow::try_from(item)?);
      }

      Ok(list)
    }
  } else {
    Err(anyhow!("Workflow Object is required [4.6.1.1 Fixed Fields]"))
  }
}

impl TryFrom<&Value> for Workflow {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(map) = value.as_object() {
      Ok(Workflow {
        workflow_id: json_object_require_string(map, "workflowId")?,
        summary: json_object_lookup_string(map, "summary"),
        description: json_object_lookup_string(map, "description"),
        inputs: map.get("inputs").cloned().unwrap_or_default(),
        depends_on: json_object_lookup_string_list(map, "dependsOn").unwrap_or_default(),
        steps: json_load_steps(map)?,
        success_actions: json_load_success_actions(map)?,
        failure_actions: json_load_failure_actions(map)?,
        outputs: json_load_outputs(map),
        parameters: json_load_parameters(map)?,
        extensions: json_extract_extensions(&map)?
      })
    } else {
      Err(anyhow!("JSON value must be an Object, got {:?}", value))
    }
  }
}

fn json_load_steps(map: &Map<String, Value>) -> anyhow::Result<Vec<Step>> {
  if let Some(steps) = map.get("steps") &&
     let Some(array) = steps.as_array() {
    if array.is_empty() {
      Err(anyhow!("At lest one Step is required [4.6.4.1 Fixed Fields]"))
    } else {
      let mut list = vec![];

      for item in array {
        list.push(Step::try_from(item)?);
      }

      Ok(list)
    }
  } else {
    Err(anyhow!("At lest one Step is required [4.6.4.1 Fixed Fields]"))
  }
}

fn json_load_parameters(map: &Map<String, Value>) -> anyhow::Result<Vec<Either<ParameterObject, ReusableObject>>> {
  if let Some(parameters) = map.get("parameters") &&
     let Some(array) = parameters.as_array() {
    let mut list = vec![];

    for item in array {
      if let Some(map) = item.as_object() {
        if map.contains_key("reference") {
          list.push(Either::Second(ReusableObject::try_from(item)?));
        } else {
          list.push(Either::First(ParameterObject::try_from(item)?));
        }
      }
    }

    Ok(list)
  } else {
    Ok(vec![])
  }
}

fn json_load_success_actions(map: &Map<String, Value>) -> anyhow::Result<Vec<Either<SuccessObject, ReusableObject>>> {
  if let Some(array) = map.get("successActions") {
    let mut list = vec![];

    if let Some(array) = array.as_array() {
      for item in array {
        if let Some(map) = item.as_object() {
          if map.contains_key("reference") {
            list.push(Either::Second(ReusableObject::try_from(item)?));
          } else {
            list.push(Either::First(SuccessObject::try_from(item)?));
          }
        }
      }
    }

    Ok(list)
  } else {
    Ok(vec![])
  }
}

fn json_load_failure_actions(map: &Map<String, Value>) -> anyhow::Result<Vec<Either<FailureObject, ReusableObject>>> {
  if let Some(array) = map.get("failureActions") {
    let mut list = vec![];

    if let Some(array) = array.as_array() {
      for item in array {
        if let Some(map) = item.as_object() {
          if map.contains_key("reference") {
            list.push(Either::Second(ReusableObject::try_from(item)?));
          } else {
            list.push(Either::First(FailureObject::try_from(item)?));
          }
        }
      }
    }

    Ok(list)
  } else {
    Ok(vec![])
  }
}

fn json_load_outputs(map: &Map<String, Value>) -> HashMap<String, String> {
  map.get("outputs").map(|v | {
    if let Some(outputs) = v.as_object() {
      outputs.iter()
        .filter_map(|(k, v)| v.as_str().map(|v| (k.clone(), v.to_string())))
        .collect()
    } else {
      hashmap!{}
    }
  }).unwrap_or_default()
}

impl TryFrom<&Value> for Step {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(map) = value.as_object() {
      Ok(Step {
        step_id: json_object_require_string(&map, "stepId")?,
        operation_id: json_object_lookup_string(&map, "operationId"),
        operation_path: json_object_lookup_string(&map, "operationPath"),
        workflow_id: json_object_lookup_string(&map, "workflowId"),
        description: json_object_lookup_string(&map, "description"),
        parameters: json_load_parameters(map)?,
        request_body: map.get("requestBody")
          .map(|v| RequestBody::try_from(v))
          .transpose()?,
        on_success: json_load_success_actions(map)?,
        success_criteria: json_load_success_criteria(map)?,
        on_failure: json_load_failure_actions(map)?,
        outputs: json_load_outputs(map),
        extensions: json_extract_extensions(map)?
      })
    } else {
      Err(anyhow!("JSON value must be an Object, got {:?}", value))
    }
  }
}

fn json_load_success_criteria(map: &Map<String, Value>) -> anyhow::Result<Vec<Criterion>> {
  if let Some(criteria) = map.get("successCriteria") {
    let mut result = vec![];

    if let Some(array) = criteria.as_array() {
      for value in array {
        result.push(Criterion::try_from(value)?);
      }
    }

    Ok(result)
  } else {
    Ok(vec![])
  }
}

impl TryFrom<&Value> for ParameterObject {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(map) = value.as_object() {
      Ok(ParameterObject {
        name: json_object_require_string(map, "name")?,
        r#in: json_object_lookup_string(map, "in"),
        value: json_load_any_or_expression(map, "value")?,
        extensions: json_extract_extensions(map)?
      })
    } else {
      Err(anyhow!("JSON value must be an Object, got {:?}", value))
    }
  }
}

fn json_load_any_or_expression(map: &Map<String, Value>, key: &str) -> anyhow::Result<Either<AnyValue, String>> {
  if let Some(value) = map.get(key) {
    if let Some(s) = value.as_str() {
      if s.starts_with('$') {
        Ok(Either::Second(s.to_string()))
      } else {
        Ok(Either::First(AnyValue::String(s.to_string())))
      }
    } else {
      AnyValue::try_from(value).map(Either::First)
    }
  } else {
    Err(anyhow!("Parameter value is required [4.6.6.1 Fixed Fields]"))
  }
}

impl TryFrom<&Value> for SuccessObject {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(map) = value.as_object() {
      Ok(SuccessObject {
        name: json_object_require_string(map, "name")?,
        r#type: json_object_require_string(map, "type")?,
        workflow_id: json_object_lookup_string(map, "workflowId"),
        step_id: json_object_lookup_string(map, "stepId"),
        criteria: json_load_criteria(map, "criteria")?,
        extensions: json_extract_extensions(map)?
      })
    } else {
      Err(anyhow!("JSON value must be an Object, got {:?}", value))
    }
  }
}

impl TryFrom<&Value> for FailureObject {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(map) = value.as_object() {
      Ok(FailureObject {
        name: json_object_require_string(map, "name")?,
        r#type: json_object_require_string(map, "type")?,
        workflow_id: json_object_lookup_string(map, "workflowId"),
        step_id: json_object_lookup_string(map, "stepId"),
        retry_after: json_object_lookup_number(map, "retryAfter"),
        retry_limit: json_object_lookup_integer(map, "retryLimit"),
        criteria: json_load_criteria(map, "criteria")?,
        extensions: json_extract_extensions(map)?
      })
    } else {
      Err(anyhow!("JSON value must be an Object, got {:?}", value))
    }
  }
}

impl TryFrom<&Value> for Components {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(map) = value.as_object() {
      let mut inputs = hashmap!{};
      if let Some(object) = map.get("inputs") &&
         let Some(map) = object.as_object() {
        for (key, value) in map {
          inputs.insert(key.clone(), value.clone());
        }
      }

      let mut parameters = hashmap!{};
      if let Some(object) = map.get("parameters") &&
         let Some(map) = object.as_object() {
        for (key, value) in map {
          parameters.insert(key.to_string(), ParameterObject::try_from(value)?);
        }
      }

      let mut success_actions = hashmap!{};
      if let Some(object) = map.get("successActions") &&
         let Some(map) = object.as_object() {
        for (key, value) in map {
          success_actions.insert(key.to_string(), SuccessObject::try_from(value)?);
        }
      }

      let mut failure_actions = hashmap!{};
      if let Some(object) = map.get("failureActions") &&
         let Some(map) = object.as_object() {
        for (key, value) in map {
          failure_actions.insert(key.to_string(), FailureObject::try_from(value)?);
        }
      }

      Ok(Components {
        inputs,
        parameters,
        success_actions,
        failure_actions,
        extensions: json_extract_extensions(map)?
      })
    } else {
      Ok(Components::default())
    }
  }
}

impl TryFrom<&Value> for ReusableObject {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(map) = value.as_object() {
      if let Ok(reference) = json_object_require_string(map, "reference") {
        Ok(ReusableObject {
          reference,
          value: json_object_lookup_string(map, "value")
        })
      } else {
        Err(anyhow!("Reference is required [4.6.10.1 Fixed Fields]"))
      }
    } else {
      Err(anyhow!("JSON value must be an Object, got {:?}", value))
    }
  }
}

impl TryFrom<&Value> for Criterion {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(map) = value.as_object() {
      Ok(Criterion {
        context: json_object_lookup_string(map, "context"),
        condition: json_object_require_string(map, "condition")?,
        r#type: json_load_criterion_expression_type(map)?,
        extensions: json_extract_extensions(map)?
      })
    } else {
      Err(anyhow!("JSON value must be an Object, got {:?}", value))
    }
  }
}

fn json_load_criteria(map: &Map<String, Value>, key: &str) -> anyhow::Result<Vec<Criterion>> {
  let mut criterion = vec![];

  if let Some(criteria) = map.get(key) && let Some(array) = criteria.as_array() {
    for item in array {
      criterion.push(Criterion::try_from(item)?);
    }
  }

  Ok(criterion)
}

impl TryFrom<&Value> for CriterionExpressionType {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(object) = value.as_object() {
      Ok(CriterionExpressionType {
        r#type: json_object_require_string(object, "type")?,
        version: json_object_require_string(object, "version")?,
        extensions: json_extract_extensions(object)?
      })
    } else {
      Err(anyhow!("JSON value must be an Object, got {:?}", value))
    }
  }
}

fn json_load_criterion_expression_type(json: &Map<String, Value>) -> anyhow::Result<Option<Either<String, CriterionExpressionType>>> {
  json.get("type").map(|value| {
    if let Some(s) = value.as_str() {
      Ok(Either::First(s.to_string()))
    } else {
      CriterionExpressionType::try_from(value).map(Either::Second)
    }
  }).transpose()
}

impl TryFrom<&Value> for RequestBody {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(map) = value.as_object() {
      let content_type = json_object_lookup_string(map, "contentType");
      let payload = json_load_payload(map, "payload", content_type.as_ref())?;
      let replacements = json_load_replacements(map, "replacements")?;
      Ok(RequestBody {
        content_type,
        payload,
        replacements,
        extensions: json_extract_extensions(&map)?
      })
    } else {
      Err(anyhow!("JSON value must be an Object, got {:?}", value))
    }
  }
}

fn json_load_payload(
  map: &Map<String, Value>,
  key: &str,
  _content_type: Option<&String>
) -> anyhow::Result<Option<Rc<dyn Payload + Send + Sync>>> {
  if let Some(value) = map.get(key) {
    match value {
      Value::Null => Ok(Some(Rc::new(EmptyPayload))),
      Value::String(s) => Ok(Some(Rc::new(StringPayload(s.clone())))),
      _ => Ok(Some(Rc::new(JsonPayload(value.clone()))))
    }
  } else {
    Ok(None)
  }
}

fn json_load_replacements(map: &Map<String, Value>, key: &str) -> anyhow::Result<Vec<PayloadReplacement>> {
  let mut replacements = vec![];

  if let Some(value) = map.get(key) && let Some(array) = value.as_array() {
    for item in array {
      replacements.push(PayloadReplacement::try_from(item)?);
    }
  }

  Ok(replacements)
}

impl TryFrom<&Value> for PayloadReplacement {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(map) = value.as_object() {
      Ok(PayloadReplacement {
        target: json_object_require_string(map, "target")?,
        value: json_load_any_or_expression(map, "value")?,
        extensions: json_extract_extensions(map)?
      })
    } else {
      Err(anyhow!("JSON value must be an Object, got {:?}", value))
    }
  }
}

/// Returns the type name of the JSON value
pub fn json_type_name(json: &Value) -> String {
  match json {
    Value::Null => "Null",
    Value::Bool(_) => "Boolean",
    Value::Number(_) => "Number",
    Value::String(_) => "String",
    Value::Array(_) => "Array",
    Value::Object(_) => "Object"
  }.to_string()
}

/// Looks up a value with the given key in a JSON Object. If the value is easily
/// convertable to a String (is a Number or Boolean), `to_string()` will be called on it.
pub fn json_object_lookup_string(map: &Map<String, Value>, key: &str) -> Option<String> {
  if let Some(value) = map.get(key) {
    match value {
      Value::Bool(b) => Some(b.to_string()),
      Value::Number(n) => Some(n.to_string()),
      Value::String(s) => Some(s.clone()),
      _ => None
    }
  } else {
    None
  }
}

/// Looks up a numeric value with the given key in an Object. If the value is an integer
/// it will be converted to a double.
pub fn json_object_lookup_number(map: &Map<String, Value>, key: &str) -> Option<f64> {
  if let Some(value) = map.get(key) {
    match value {
      Value::Number(n) => {
        if let Some(uint) = n.as_u64() {
          Some(uint as f64)
        } else if let Some(int) = n.as_i64() {
          Some(int as f64)
        } else {
          n.as_f64()
        }
      },
      _ => None
    }
  } else {
    None
  }
}

/// Looks up an integer value with the given key in an Object. If the value is a float
/// it will be converted to an integer.
pub fn json_object_lookup_integer(map: &Map<String, Value>, key: &str) -> Option<i64> {
  if let Some(value) = map.get(key) {
    match value {
      Value::Number(n) => {
        if let Some(uint) = n.as_u64() {
          Some(uint as i64)
        } else if let Some(int) = n.as_i64() {
          Some(int)
        } else {
          n.as_f64().map(|f| f as i64)
        }
      },
      _ => None
    }
  } else {
    None
  }
}

/// Looks up a required String value with the given key in a JSON Object. If the key does
/// not exist, or the resulting value is not a String, an Error is returned.
pub fn json_object_require_string(map: &Map<String, Value>, key: &str) -> anyhow::Result<String> {
  if let Some(value) = map.get(key) {
    if let Some(value) = value.as_str() {
      Ok(value.to_string())
    } else {
      Err(anyhow!("Value for key '{}' in Object was not a string, was {}", key, json_type_name(value)))
    }
  } else {
    Err(anyhow!("Did not find key '{}' in Object", key))
  }
}

/// Looks up an Array of String values with the given key in a JSON Object. If each value
/// is easily convertable to a String (is a Number or Boolean), `to_string()` will be called on it.
/// All other values are ignored.
pub fn json_object_lookup_string_list(map: &Map<String, Value>, key: &str) -> Option<Vec<String>> {
  if let Some(value) = map.get(key) && let Some(array) = value.as_array() {
    Some(array.iter().filter_map(|value| {
      match value {
        Value::Bool(b) => Some(b.to_string()),
        Value::Number(n) => Some(n.to_string()),
        Value::String(s) => Some(s.clone()),
        _ => None
      }
    }).collect())
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use std::any::Any;

  use expectest::prelude::*;
  use maplit::hashmap;
  use pretty_assertions::assert_eq;
  use serde_json::{json, Value};

  use crate::either::Either;
  use crate::extensions::AnyValue;
  use crate::payloads::{JsonPayload, StringPayload};
  use crate::v1_0::*;

  #[test]
  fn fails_to_load_if_the_main_value_is_not_a_json_object() {
    expect!(ArazzoDescription::try_from(&Value::String("test".to_string()))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_version_is_missing() {
    expect!(ArazzoDescription::try_from(&json!({}))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_version_is_not_a_string() {
    expect!(ArazzoDescription::try_from(&json!({ "arazzo": {} }))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_info_is_missing() {
    expect!(ArazzoDescription::try_from(&json!({ "arazzo": "1.0.0" }))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_source_descriptions_are_missing() {
    let json = json!({
      "arazzo": "1.0.0",
      "info": {
        "title": "test",
        "version": "1.2.3"
      }
    });
    expect!(ArazzoDescription::try_from(&json)).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_source_descriptions_are_empty() {
    let json = json!({
      "arazzo": "1.0.0",
      "info": {
        "title": "test",
        "version": "1.2.3"
      },
      "sourceDescriptions": []
    });
    expect!(ArazzoDescription::try_from(&json)).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_workflows_are_missing() {
    let json = json!({
      "arazzo": "1.0.0",
      "info": {
        "title": "test",
        "version": "1.2.3"
      },
      "sourceDescriptions": [
        {
          "name": "test",
          "url": "http://test"
        }
      ]
    });
    expect!(ArazzoDescription::try_from(&json)).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_workflows_are_empty() {
    let json = json!({
      "arazzo": "1.0.0",
      "info": {
        "title": "test",
        "version": "1.2.3"
      },
      "sourceDescriptions": [
        {
          "name": "test",
          "url": "http://test"
        }
      ],
      "workflows": []
    });
    expect!(ArazzoDescription::try_from(&json)).to(be_err());
  }

  #[test]
  fn arazzo_description_supports_extensions() {
    let json = json!({
      "arazzo": "1.0.0",
      "x-one": "1",
      "x-two": 2,
      "info": {
        "title": "test",
        "version": "1.2.3"
      },
      "sourceDescriptions": [
        {
          "name": "test",
          "url": "http://test"
        }
      ],
      "workflows": [
        {
          "workflowId": "test",
          "steps": [
            { "stepId": "test" }
          ]
        }
      ]
    });

    let desc = ArazzoDescription::try_from(&json).unwrap();
    expect!(desc.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::UInteger(2)
    }));
  }

  #[test]
  fn info_supports_extensions() {
    let json = json!({
      "title": "test",
      "version": "1.0.0",
      "x-one": "1",
      "x-two": 2
    });

    let info = Info::try_from(&json).unwrap();
    expect!(info.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::UInteger(2)
    }));
  }

  #[test]
  fn source_description_supports_extensions() {
    let json = json!({
      "name": "test",
      "url": "test",
      "x-one": "1",
      "x-two": 2
    });

    let desc = SourceDescription::try_from(&json).unwrap();
    expect!(desc.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::UInteger(2)
    }));
  }

  #[test]
  fn workflow_fails_to_load_if_there_are_no_steps() {
    let json = json!({
      "workflowId": "test"
    });

    expect!(Workflow::try_from(&json)).to(be_err());

    let json = json!({
      "workflowId": "test",
      "steps": []
    });
    expect!(Workflow::try_from(&json)).to(be_err());
  }

  #[test]
  fn workflow_supports_extensions() {
    let json = json!({
      "workflowId": "test",
      "steps": [
        { "stepId": "test" }
      ],
      "x-one": "1",
      "x-two": 2
    });

    let wf = Workflow::try_from(&json).unwrap();
    expect!(wf.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::UInteger(2)
    }));
  }

  #[test]
  fn steps_supports_extensions() {
    let json = json!({
      "stepId": "test",
      "x-one": "1",
      "x-two": 2
    });

    let step = Step::try_from(&json).unwrap();
    expect!(step.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::UInteger(2)
    }));
  }

  #[test]
  fn load_components() {
    let json_str = r#"{
      "parameters": {
        "storeId": {
          "name": "storeId",
          "in": "header",
          "value": "$inputs.x-store-id"
        }
      },
      "inputs": {
        "pagination": {
          "type": "object",
          "properties": {
            "page": {
              "type": "integer",
              "format": "int32"
            },
            "pageSize": {
              "type": "integer",
              "format": "int32"
            }
          }
        }
      },
      "failureActions": {
        "refreshToken": {
          "name": "refreshExpiredToken",
          "type": "retry",
          "retryAfter": 1,
          "retryLimit": 5,
          "workflowId": "refreshTokenWorkflowId",
          "criteria": [
            {
              "condition": "{$statusCode == 401}"
            }
          ]
        }
      }
    }"#;
    let json: Value = serde_json::from_str(json_str).unwrap();

    let components = Components::try_from(&json).unwrap();
    assert_eq!(components, Components {
      inputs: hashmap!{
        "pagination".to_string() => json!({
          "type": "object",
          "properties": {
            "page": {
              "type": "integer",
              "format": "int32"
            },
            "pageSize": {
              "type": "integer",
              "format": "int32"
            }
          }
        })
      },
      parameters: hashmap!{
        "storeId".to_string() => ParameterObject {
          name: "storeId".to_string(),
          r#in: Some("header".to_string()),
          value: Either::Second("$inputs.x-store-id".to_string()),
          extensions: Default::default()
        }
      },
      success_actions: hashmap!{},
      failure_actions: hashmap!{
        "refreshToken".to_string() => FailureObject {
          name: "refreshExpiredToken".to_string(),
          r#type: "retry".to_string(),
          workflow_id: Some("refreshTokenWorkflowId".to_string()),
          step_id: None,
          retry_after: Some(1f64),
          retry_limit: Some(5),
          criteria: vec![
            Criterion {
              context: None,
              condition: "{$statusCode == 401}".to_string(),
              r#type: None,
              extensions: Default::default()
            }
          ],
          extensions: Default::default()
        }
      },
      extensions: hashmap!{}
    });
  }

  #[test]
  fn components_supports_extensions() {
    let json = json!({
      "workflowId": "test",
      "x-one": "1",
      "x-two": 2
    });

    let components = Components::try_from(&json).unwrap();
    expect!(components.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::UInteger(2)
    }));
  }

  #[test]
  fn load_success_object() {
    let json = json!({
      "name": "test",
      "type": "end",
      "workflowId": "workflowId",
      "stepId": "stepId"
    });

    let success = SuccessObject::try_from(&json).unwrap();
    expect!(&success.name).to(be_equal_to("test"));
    expect!(&success.r#type).to(be_equal_to("end"));
    expect!(success.workflow_id.clone()).to(be_some().value("workflowId"));
    expect!(success.step_id.clone()).to(be_some().value("stepId"));

    let json = json!({
      "name": "test",
      "type": "end"
    });

    let success = SuccessObject::try_from(&json).unwrap();
    expect!(&success.name).to(be_equal_to("test"));
    expect!(&success.r#type).to(be_equal_to("end"));
    expect!(success.workflow_id.clone()).to(be_none());
    expect!(success.step_id.clone()).to(be_none());
  }

  #[test]
  fn success_object_supports_extensions() {
    let json = json!({
      "name": "test",
      "type": "end",
      "x-one": "1",
      "x-two": 2
    });

    let success = SuccessObject::try_from(&json).unwrap();
    expect!(success.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::UInteger(2)
    }));
  }

  #[test]
  fn load_failure_object() {
    let json = json!({
      "name": "test",
      "type": "end",
      "workflowId": "workflowId",
      "stepId": "stepId",
      "retryAfter": 10.5,
      "retryLimit": 10
    });

    let failure = FailureObject::try_from(&json).unwrap();
    expect!(&failure.name).to(be_equal_to("test"));
    expect!(&failure.r#type).to(be_equal_to("end"));
    expect!(failure.workflow_id.clone()).to(be_some().value("workflowId"));
    expect!(failure.step_id.clone()).to(be_some().value("stepId"));
    expect!(failure.retry_after.clone()).to(be_some().value(10.5));
    expect!(failure.retry_limit.clone()).to(be_some().value(10));

    let json = json!({
      "name": "test",
      "type": "end"
    });

    let failure = FailureObject::try_from(&json).unwrap();
    expect!(&failure.name).to(be_equal_to("test"));
    expect!(&failure.r#type).to(be_equal_to("end"));
    expect!(failure.workflow_id.clone()).to(be_none());
    expect!(failure.step_id.clone()).to(be_none());
    expect!(failure.retry_after.clone()).to(be_none());
    expect!(failure.retry_limit.clone()).to(be_none());
  }

  #[test]
  fn failure_object_supports_extensions() {
    let json = json!({
      "name": "test",
      "type": "end",
      "x-one": "1",
      "x-two": 2
    });

    let failure = FailureObject::try_from(&json).unwrap();
    expect!(failure.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::UInteger(2)
    }));
  }

  #[test]
  fn load_reusable_object() {
    let json = json!({
      "reference": "$test",
      "value": "test",
      "workflowId": "workflowId"
    });

    let obj = ReusableObject::try_from(&json).unwrap();
    expect!(&obj.reference).to(be_equal_to("$test"));
    expect!(obj.value.clone()).to(be_some().value("test"));

    let json = json!({
      "reference": "$test"
    });

    let obj = ReusableObject::try_from(&json).unwrap();
    expect!(&obj.reference).to(be_equal_to("$test"));
    expect!(obj.value.clone()).to(be_none());
  }

  #[test]
  fn load_workflow_outputs() {
    let json = json!({
      "workflowId": "test",
      "steps": [
        { "stepId": "test" }
      ],
      "outputs": {
        "tokenExpires": "$response.header.X-Expires-After",
        "rateLimit": "$response.header.X-Rate-Limit",
        "invalid": []
      }
    });

    let wf = Workflow::try_from(&json).unwrap();
    expect!(wf.outputs).to(be_equal_to(hashmap!{
      "tokenExpires".to_string() => "$response.header.X-Expires-After".to_string(),
      "rateLimit".to_string() => "$response.header.X-Rate-Limit".to_string()
    }));
  }

  #[test]
  fn load_workflow_parameters() {
    let json = json!({
      "workflowId": "test",
      "steps": [
        { "stepId": "test" }
      ],
      "parameters": [
        {
          "name": "username",
          "in": "query",
          "value": "$inputs.username"
        }
      ]
    });

    let wf = Workflow::try_from(&json).unwrap();
    expect!(wf.parameters).to(be_equal_to(vec![
      Either::First(ParameterObject {
        name: "username".to_string(),
        r#in: Some("query".to_string()),
        value: Either::Second("$inputs.username".to_string()),
        extensions: Default::default()
      })
    ]));

    let json = json!({
      "name": "username",
      "value": 10
    });

    let parameter = ParameterObject::try_from(&json).unwrap();
    expect!(parameter).to(be_equal_to(ParameterObject {
      name: "username".to_string(),
      r#in: None,
      value: Either::First(AnyValue::UInteger(10)),
      extensions: Default::default()
    }));
  }

  #[test]
  fn parameter_object_supports_extensions() {
    let json = json!({
      "name": "username",
      "value": 10,
      "x-one": "1",
      "x-two": 2
    });

    let parameter = ParameterObject::try_from(&json).unwrap();
    expect!(parameter.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::UInteger(2)
    }));
  }

  #[test]
  fn load_step_parameters() {
    let json = json!({
      "stepId": "test",
      "parameters": [
        {
          "name": "username",
          "in": "query",
          "value": "$inputs.username"
        }
      ]
    });

    let step = Step::try_from(&json).unwrap();
    expect!(step.parameters).to(be_equal_to(vec![
      Either::First(ParameterObject {
        name: "username".to_string(),
        r#in: Some("query".to_string()),
        value: Either::Second("$inputs.username".to_string()),
        extensions: Default::default()
      })
    ]));
  }

  #[test]
  fn load_request_body() {
    let json = json!({
      "contentType": "text/plain",
      "payload": "some text"
    });

    let body = RequestBody::try_from(&json).unwrap();
    expect!(body.content_type).to(be_some().value("text/plain"));
  }

  #[test]
  fn request_body_supports_extensions() {
    let json = json!({
      "contentType": "text/plain",
      "payload": "some text",
      "x-one": "1",
      "x-two": 2
    });

    let parameter = RequestBody::try_from(&json).unwrap();
    expect!(parameter.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::UInteger(2)
    }));
  }

  #[test]
  fn load_payload() {
    let body = json!({
      "contentType": "application/json",
      "payload": "{\"petOrder\":{\"petId\": \"{$inputs.pet_id}\",\"couponCode\"\
      :\"{$inputs.coupon_code}\",\"quantity\":\"{$inputs.quantity}\",\"status\":\
      \"placed\",\"complete\":false}}"
    });
    let body = RequestBody::try_from(&body).unwrap();
    expect!(body.content_type).to(be_some().value("application/json"));
    let payload: &dyn Any = body.payload.as_ref().unwrap().as_ref();
    let p = payload.downcast_ref::<StringPayload>().unwrap();
    assert_eq!(
      r#"{"petOrder":{"petId": "{$inputs.pet_id}","couponCode":"{$inputs.coupon_code}","quantity":"{$inputs.quantity}","status":"placed","complete":false}}"#,
      &p.0
    );

    let body = json!({
      "contentType": "application/json",
      "payload": {
        "petOrder": {
          "petId": "$inputs.pet_id",
          "couponCode": "$inputs.coupon_code",
          "quantity": "$inputs.quantity",
          "status": "placed",
          "complete": "false"
        }
      }
    });
    let body = RequestBody::try_from(&body).unwrap();
    expect!(body.content_type).to(be_some().value("application/json"));
    let payload: &dyn Any = body.payload.as_ref().unwrap().as_ref();
    let p = payload.downcast_ref::<JsonPayload>().unwrap();
    assert_eq!(
      &json!({
       "petOrder": {
          "petId": "$inputs.pet_id",
          "couponCode": "$inputs.coupon_code",
          "quantity": "$inputs.quantity",
          "status": "placed",
          "complete": "false"
        }
      }),
      &p.0
    );
  }

  #[test]
  fn load_criterion() {
    let json = json!({
      "condition": "$statusCode == 200"
    });

    let criterion = Criterion::try_from(&json).unwrap();
    expect!(criterion.condition).to(be_equal_to("$statusCode == 200"));
    expect!(criterion.context).to(be_none());
    expect!(criterion.r#type).to(be_none());

    let json = json!({
      "context": "$statusCode",
      "condition": "^200$",
      "type": "regex"
    });

    let criterion = Criterion::try_from(&json).unwrap();
    expect!(criterion.condition).to(be_equal_to("^200$"));
    expect!(criterion.context).to(be_some().value("$statusCode"));
    expect!(criterion.r#type).to(be_some().value(Either::First("regex".to_string())));
  }

  #[test]
  fn criterion_supports_extensions() {
    let json = json!({
      "condition": "$statusCode == 200",
      "x-one": "1",
      "x-two": 2
    });

    let criterion = Criterion::try_from(&json).unwrap();
    expect!(criterion.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::UInteger(2)
    }));
  }

  #[test]
  fn load_criterion_expression_type() {
    let json = json!({
      "type": "jsonpath",
      "version": "draft-goessner-dispatch-jsonpath-00"
    });

    let criterion = CriterionExpressionType::try_from(&json).unwrap();
    expect!(criterion.r#type).to(be_equal_to("jsonpath"));
    expect!(criterion.version).to(be_equal_to("draft-goessner-dispatch-jsonpath-00"));
  }

  #[test]
  fn criterion_expression_type_supports_extensions() {
    let json = json!({
      "type": "jsonpath",
      "version": "draft-goessner-dispatch-jsonpath-00",
      "x-one": "1",
      "x-two": 2
    });

    let criterion = CriterionExpressionType::try_from(&json).unwrap();
    expect!(criterion.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::UInteger(2)
    }));
  }

  #[test]
  fn load_payload_replacement() {
    let json = json!({
      "target": "/petId",
      "value": "$inputs.pet_id"
    });

    let payload_replacement = PayloadReplacement::try_from(&json).unwrap();
    expect!(payload_replacement.target).to(be_equal_to("/petId"));
    expect!(payload_replacement.value).to(be_equal_to(Either::Second("$inputs.pet_id".to_string())));

    let json = json!({
      "target": "/quantity",
      "value": 10
    });

    let payload_replacement = PayloadReplacement::try_from(&json).unwrap();
    expect!(payload_replacement.target).to(be_equal_to("/quantity"));
    expect!(payload_replacement.value).to(be_equal_to(Either::First(AnyValue::UInteger(10))));
  }

  #[test]
  fn payload_replacement_supports_extensions() {
    let json = json!({
      "target": "/petId",
      "value": "$inputs.pet_id",
      "x-one": "1",
      "x-two": 2
    });

    let payload_replacement = PayloadReplacement::try_from(&json).unwrap();
    expect!(payload_replacement.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::UInteger(2)
    }));
  }
}
