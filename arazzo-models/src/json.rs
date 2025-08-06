//! Functions and Traits for loading Arazzo objects from a JSON document

use anyhow::anyhow;
use itertools::Either;
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::rc::Rc;

use crate::extensions::{json_extract_extensions, yaml_extract_extensions, AnyValue};
use crate::payloads::{EmptyPayload, JsonPayload, Payload, StringPayload};
use crate::v1_0::{ArazzoDescription, Components, Criterion, CriterionExpressionType, FailureObject, Info, ParameterObject, RequestBody, ReusableObject, SourceDescription, Step, SuccessObject, Workflow};

// impl TryFrom<&Value> for ArazzoDescription {
//   type Error = anyhow::Error;
//
//   fn try_from(value: &Value) -> Result<Self, Self::Error> {
//     if let Some(hash) = value.as_object() {
//       if let Ok(version) = json_object_require_string(hash, "arazzo") {
//         let info = Info::try_from(hash)?;
//         let source_descriptions = json_load_source_descriptions(hash)?;
//         let workflows = json_load_workflows(hash)?;
//         let components = Components::try_from(hash)?;
//
//         Ok(ArazzoDescription {
//           arazzo: version,
//           info,
//           source_descriptions,
//           workflows,
//           components,
//           extensions: yaml_extract_extensions(&hash)?
//         })
//       } else {
//         Err(anyhow!("Arazzo version number is required [4.6.1.1 Fixed Fields]"))
//       }
//     } else {
//       Err(anyhow!("JSON value must be a Hash, got {}", json_type_name(value)))
//     }
//   }
// }

// impl TryFrom<&Yaml> for SourceDescription {
//   type Error = anyhow::Error;
//
//   fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
//     if let Some(hash) = value.as_hash() {
//       Ok(SourceDescription {
//         name: yaml_hash_require_string(&hash, "name")?,
//         url: yaml_hash_require_string(&hash, "url")?,
//         r#type: yaml_hash_lookup_string(&hash, "type"),
//         extensions: yaml_extract_extensions(&hash)?
//       })
//     } else {
//       Err(anyhow!("YAML value must be a Hash, got {}", yaml_type_name(value)))
//     }
//   }
// }
//
// fn yaml_load_source_descriptions(hash: &Hash) -> anyhow::Result<Vec<SourceDescription>> {
//   if let Some(array) = yaml_hash_lookup(hash, "sourceDescriptions", |v | v.as_vec().cloned()) {
//     if array.is_empty() {
//       Err(anyhow!("Source Description list must have at least one entry [4.6.1.1 Fixed Fields]"))
//     } else {
//       let mut list = vec![];
//
//       for item in &array {
//         list.push(SourceDescription::try_from(item)?);
//       }
//
//       Ok(list)
//     }
//   } else {
//     Err(anyhow!("Source Description Object is required [4.6.1.1 Fixed Fields]"))
//   }
// }
//
// impl TryFrom<&Hash> for Info {
//   type Error = anyhow::Error;
//
//   fn try_from(value: &Hash) -> Result<Self, Self::Error> {
//     if let Some(hash) = yaml_hash_lookup(value, "info", |v | v.as_hash().cloned()) {
//       Ok(Info {
//         title: yaml_hash_require_string(&hash, "title")?,
//         summary: yaml_hash_lookup_string(&hash, "summary"),
//         description: yaml_hash_lookup_string(&hash, "description"),
//         version: yaml_hash_require_string(&hash, "version")?,
//         extensions: yaml_extract_extensions(&hash)?
//       })
//     } else {
//       Err(anyhow!("Info Object is required [4.6.1.1 Fixed Fields]"))
//     }
//   }
// }
//
// fn yaml_load_workflows(hash: &Hash) -> anyhow::Result<Vec<Workflow>> {
//   if let Some(array) = yaml_hash_lookup(hash, "workflows", |v | v.as_vec().cloned()) {
//     if array.is_empty() {
//       Err(anyhow!("Workflows list must have at least one entry [4.6.1.1 Fixed Fields]"))
//     } else {
//       let mut list = vec![];
//
//       for item in &array {
//         list.push(Workflow::try_from(item)?);
//       }
//
//       Ok(list)
//     }
//   } else {
//     Err(anyhow!("Workflow Object is required [4.6.1.1 Fixed Fields]"))
//   }
// }
//
// impl TryFrom<&Yaml> for Workflow {
//   type Error = anyhow::Error;
//
//   fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
//     if let Some(hash) = value.as_hash() {
//       Ok(Workflow {
//         workflow_id: yaml_hash_require_string(hash, "workflowId")?,
//         summary: yaml_hash_lookup_string(hash, "summary"),
//         description: yaml_hash_lookup_string(hash, "description"),
//         inputs: yaml_hash_entry_to_json(hash, "inputs")?,
//         depends_on: yaml_hash_lookup_string_list(hash, "dependsOn").unwrap_or_default(),
//         steps: yaml_load_steps(hash)?,
//         success_actions: yaml_load_success_actions(hash)?,
//         failure_actions: yaml_load_failure_actions(hash)?,
//         outputs: yaml_load_outputs(hash),
//         parameters: yaml_load_parameters(hash)?,
//         extensions: yaml_extract_extensions(&hash)?
//       })
//     } else {
//       Err(anyhow!("YAML value must be a Hash, got {}", yaml_type_name(value)))
//     }
//   }
// }
//
// fn yaml_load_steps(hash: &Hash) -> anyhow::Result<Vec<Step>> {
//   if let Some(array) = yaml_hash_lookup(hash, "steps", |v | v.as_vec().cloned()) {
//     if array.is_empty() {
//       Err(anyhow!("At lest one Step is required [4.6.4.1 Fixed Fields]"))
//     } else {
//       let mut list = vec![];
//
//       for item in &array {
//         list.push(Step::try_from(item)?);
//       }
//
//       Ok(list)
//     }
//   } else {
//     Err(anyhow!("At lest one Step is required [4.6.4.1 Fixed Fields]"))
//   }
// }
//
// fn yaml_load_parameters(hash: &Hash) -> anyhow::Result<Vec<Either<ParameterObject, ReusableObject>>> {
//   if let Some(array) = yaml_hash_lookup(hash, "parameters", |v | v.as_vec().cloned()) {
//     let mut list = vec![];
//
//     for item in &array {
//       if let Some(hash) = item.as_hash() {
//         if hash.contains_key(&Yaml::String("reference".to_string())) {
//           list.push(Either::Right(ReusableObject::try_from(hash)?));
//         } else {
//           list.push(Either::Left(ParameterObject::try_from(hash)?));
//         }
//       }
//     }
//
//     Ok(list)
//   } else {
//     Ok(vec![])
//   }
// }
//
// fn yaml_load_success_actions(hash: &Hash) -> anyhow::Result<Vec<Either<SuccessObject, ReusableObject>>> {
//   if let Some(array) = yaml_hash_lookup(hash, "successActions", |v | v.as_vec().cloned()) {
//     let mut list = vec![];
//
//     for item in &array {
//       if let Some(hash) = item.as_hash() {
//         if hash.contains_key(&Yaml::String("reference".to_string())) {
//           list.push(Either::Right(ReusableObject::try_from(hash)?));
//         } else {
//           list.push(Either::Left(SuccessObject::try_from(hash)?));
//         }
//       }
//     }
//
//     Ok(list)
//   } else {
//     Ok(vec![])
//   }
// }
//
// fn yaml_load_failure_actions(hash: &Hash) -> anyhow::Result<Vec<Either<FailureObject, ReusableObject>>> {
//   if let Some(array) = yaml_hash_lookup(hash, "failureActions", |v | v.as_vec().cloned()) {
//     let mut list = vec![];
//
//     for item in &array {
//       if let Some(hash) = item.as_hash() {
//         if hash.contains_key(&Yaml::String("reference".to_string())) {
//           list.push(Either::Right(ReusableObject::try_from(hash)?));
//         } else {
//           list.push(Either::Left(FailureObject::try_from(hash)?));
//         }
//       }
//     }
//
//     Ok(list)
//   } else {
//     Ok(vec![])
//   }
// }
//
// fn yaml_load_outputs(hash: &Hash) -> HashMap<String, String> {
//   yaml_hash_lookup(hash, "outputs", |v | {
//     if let Some(outputs_hash) = v.as_hash() {
//       Some(outputs_hash.iter()
//         .filter_map(|(k, v)| {
//           if let Some(key) = k.as_str() {
//             v.as_str().map(|value| (key.to_string(), value.to_string()))
//           } else {
//             None
//           }
//         }).collect())
//     } else {
//       None
//     }
//   }).unwrap_or_default()
// }
//
// impl TryFrom<&Yaml> for Step {
//   type Error = anyhow::Error;
//
//   fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
//     if let Some(hash) = value.as_hash() {
//       Ok(Step {
//         step_id: yaml_hash_require_string(&hash, "stepId")?,
//         operation_id: yaml_hash_lookup_string(&hash, "operationId"),
//         operation_path: yaml_hash_lookup_string(&hash, "operationPath"),
//         workflow_id: yaml_hash_lookup_string(&hash, "workflowId"),
//         description: yaml_hash_lookup_string(&hash, "description"),
//         parameters: yaml_load_parameters(hash)?,
//         request_body: yaml_hash_lookup(hash, "requestBody", |v| Some(RequestBody::try_from(v)))
//           .transpose()?,
//         on_success: yaml_load_success_actions(hash)?,
//         success_criteria: yaml_load_success_criteria(hash)?,
//         on_failure: yaml_load_failure_actions(hash)?,
//         outputs: yaml_load_outputs(hash),
//         extensions: yaml_extract_extensions(&hash)?
//       })
//     } else {
//       Err(anyhow!("YAML value must be a Hash, got {}", yaml_type_name(value)))
//     }
//   }
// }
//
// fn yaml_load_success_criteria(hash: &Hash) -> anyhow::Result<Vec<Criterion>> {
//   if let Some(criteria) = yaml_hash_lookup(hash, "successCriteria", |v | v.as_vec().cloned()) {
//     let mut result = vec![];
//
//     for value in &criteria {
//       result.push(Criterion::try_from(value)?);
//     }
//
//     Ok(result)
//   } else {
//     Ok(vec![])
//   }
// }

impl TryFrom<&Value> for ParameterObject {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(map) = value.as_object() {
      Ok(ParameterObject {
        name: json_object_require_string(map, "name")?,
        r#in: json_object_lookup_string(map, "in"),
        value: json_load_parameter_value(map, "value")?,
        extensions: json_extract_extensions(map)?
      })
    } else {
      Err(anyhow!("JSON value must be an Object, got {:?}", value))
    }
  }
}

fn json_load_parameter_value(map: &Map<String, Value>, key: &str) -> anyhow::Result<Either<AnyValue, String>> {
  if let Some(value) = map.get(key) {
    if let Some(s) = value.as_str() {
      if s.starts_with('$') {
        Ok(Either::Right(s.to_string()))
      } else {
        Ok(Either::Left(AnyValue::String(s.to_string())))
      }
    } else {
      AnyValue::try_from(value).map(Either::Left)
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
        extensions: json_extract_extensions(map)?
      })
    } else {
      Err(anyhow!("JSON value must be an Object, got {:?}", value))
    }
  }
}

// impl TryFrom<&Hash> for Components {
//   type Error = anyhow::Error;
//
//   fn try_from(value: &Hash) -> Result<Self, Self::Error> {
//     if let Some(hash) = yaml_hash_lookup(value, "components", |v | v.as_hash().cloned()) {
//       Ok(Components {
//         extensions: yaml_extract_extensions(&hash)?
//       })
//     } else {
//       Ok(Components::default())
//     }
//   }
// }

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
      Ok(Either::Left(s.to_string()))
    } else {
      CriterionExpressionType::try_from(value).map(Either::Right)
    }
  }).transpose()
}

impl TryFrom<&Value> for RequestBody {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    if let Some(map) = value.as_object() {
      let content_type = json_object_lookup_string(map, "contentType");
      let payload = json_load_payload(map, "payload", content_type.as_ref())?;
      Ok(RequestBody {
        content_type,
        payload,
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

// /// Looks up a String key in the given hash, calling the provided callback if it is found.
// pub fn yaml_hash_lookup<F, U>(
//   hash: &Hash,
//   key: &str,
//   callback: F
// ) -> Option<U> where F: FnOnce(&Yaml) -> Option<U> {
//   if let Some(value) = hash.get(&Yaml::String(key.to_string())) {
//     callback(value)
//   } else {
//     None
//   }
// }
//
// /// Looks up an Array of String values with the given String key in a YAML Hash. If each value
// /// is easily convertable to a String (is a Number or Boolean), `to_string()` will be called on it.
// /// All other values are ignored.
// pub fn yaml_hash_lookup_string_list(hash: &Hash, key: &str) -> Option<Vec<String>> {
//   if let Some(value) = hash.get(&Yaml::String(key.to_string())) {
//     if let Some(array) = value.as_vec() {
//       Some(array.iter().flat_map(|value| {
//         match value {
//           Yaml::Real(s) => Some(s.clone()),
//           Yaml::Integer(i) => Some(i.to_string()),
//           Yaml::String(s) => Some(s.clone()),
//           Yaml::Boolean(b) => Some(b.to_string()),
//           _ => None
//         }
//       }).collect())
//     } else {
//       None
//     }
//   } else {
//     None
//   }
// }
//
// /// Looks up the entry in the hash and converts it to JSON. If there is no entry with that key,
// /// JSON Null is returned.
// pub fn yaml_hash_entry_to_json(hash: &Hash, key: &str) -> anyhow::Result<Value> {
//   if let Some(value) = hash.get(&Yaml::String(key.to_string())) {
//     yaml_to_json(value)
//   } else {
//     Ok(Value::Null)
//   }
// }

#[cfg(test)]
mod tests {
  use expectest::prelude::*;
  use maplit::hashmap;
  use pretty_assertions::assert_eq;
  use serde_json::{json, Value};
  use std::any::Any;
  use itertools::Either;
  use trim_margin::MarginTrimmable;

  use crate::extensions::AnyValue;
  use crate::payloads::{JsonPayload, StringPayload};
  use crate::v1_0::*;

  // #[test]
  // fn fails_to_load_if_the_main_value_is_not_a_json_object() {
  //   expect!(ArazzoDescription::try_from(&Value::String("test".to_string()))).to(be_err());
  // }

  // #[test]
  // fn fails_to_load_if_the_version_is_missing() {
  //   expect!(ArazzoDescription::try_from(&Yaml::Hash(Hash::new()))).to(be_err());
  // }
  //
  // #[test]
  // fn fails_to_load_if_the_version_is_not_a_string() {
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("arazzo".to_string()), Yaml::Hash(Hash::new()));
  //   expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  // }
  //
  // #[test]
  // fn fails_to_load_if_the_info_is_missing() {
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
  //   expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  // }
  //
  // #[test]
  // fn fails_to_load_if_the_source_descriptions_are_missing() {
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
  //   hash.insert(Yaml::String("info".to_string()), Yaml::Hash(info_fixture()));
  //   expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  // }
  //
  // #[test]
  // fn fails_to_load_if_the_source_descriptions_are_empty() {
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
  //   hash.insert(Yaml::String("info".to_string()), Yaml::Hash(info_fixture()));
  //   hash.insert(Yaml::String("sourceDescriptions".to_string()), Yaml::Array(vec![]));
  //   expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  // }
  //
  // #[test]
  // fn fails_to_load_if_the_workflows_are_missing() {
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
  //   hash.insert(Yaml::String("info".to_string()), Yaml::Hash(info_fixture()));
  //   hash.insert(Yaml::String("sourceDescriptions".to_string()), Yaml::Array(source_descriptions_fixture()));
  //   expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  // }
  //
  // #[test]
  // fn fails_to_load_if_the_workflows_are_empty() {
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
  //   hash.insert(Yaml::String("info".to_string()), Yaml::Hash(info_fixture()));
  //   hash.insert(Yaml::String("sourceDescriptions".to_string()), Yaml::Array(source_descriptions_fixture()));
  //   hash.insert(Yaml::String("workflows".to_string()), Yaml::Array(vec![]));
  //   expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  // }
  //
  // #[test]
  // fn arazzo_description_supports_extensions() {
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
  //   hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
  //   hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));
  //
  //   hash.insert(Yaml::String("info".to_string()), Yaml::Hash(info_fixture()));
  //   hash.insert(Yaml::String("sourceDescriptions".to_string()), Yaml::Array(source_descriptions_fixture()));
  //   hash.insert(Yaml::String("workflows".to_string()), Yaml::Array(workflows_fixture()));
  //
  //   let desc = ArazzoDescription::try_from(&Yaml::Hash(hash)).unwrap();
  //   expect!(desc.extensions).to(be_equal_to(hashmap!{
  //     "one".to_string() => AnyValue::String("1".to_string()),
  //     "two".to_string() => AnyValue::Integer(2)
  //   }));
  // }
  //
  // fn info_fixture() -> Hash {
  //   let mut info = Hash::new();
  //   info.insert(Yaml::String("title".to_string()), Yaml::String("test".to_string()));
  //   info.insert(Yaml::String("version".to_string()), Yaml::String("1.0.0".to_string()));
  //   info
  // }
  //
  // fn source_descriptions_fixture() -> Vec<Yaml> {
  //   let mut desc = Hash::new();
  //   desc.insert(Yaml::String("name".to_string()), Yaml::String("test".to_string()));
  //   desc.insert(Yaml::String("url".to_string()), Yaml::String("http://test".to_string()));
  //   vec![Yaml::Hash(desc)]
  // }
  //
  // fn workflows_fixture() -> Vec<Yaml> {
  //   let mut wf = Hash::new();
  //   wf.insert(Yaml::String("workflowId".to_string()), Yaml::String("test".to_string()));
  //   wf.insert(Yaml::String("steps".to_string()), Yaml::Array(steps_fixture()));
  //   vec![Yaml::Hash(wf)]
  // }
  //
  // fn steps_fixture() -> Vec<Yaml> {
  //   let mut desc = Hash::new();
  //   desc.insert(Yaml::String("stepId".to_string()), Yaml::String("test".to_string()));
  //   vec![Yaml::Hash(desc)]
  // }
  //
  // #[test]
  // fn info_supports_extensions() {
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("title".to_string()), Yaml::String("test".to_string()));
  //   hash.insert(Yaml::String("version".to_string()), Yaml::String("1.0.0".to_string()));
  //   hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
  //   hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));
  //
  //   let mut outer = Hash::new();
  //   outer.insert(Yaml::String("info".to_string()), Yaml::Hash(hash));
  //   let info = Info::try_from(&outer).unwrap();
  //   expect!(info.extensions).to(be_equal_to(hashmap!{
  //     "one".to_string() => AnyValue::String("1".to_string()),
  //     "two".to_string() => AnyValue::Integer(2)
  //   }));
  // }
  //
  // #[test]
  // fn source_description_supports_extensions() {
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("name".to_string()), Yaml::String("test".to_string()));
  //   hash.insert(Yaml::String("url".to_string()), Yaml::String("test".to_string()));
  //   hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
  //   hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));
  //
  //   let desc = SourceDescription::try_from(&Yaml::Hash(hash)).unwrap();
  //   expect!(desc.extensions).to(be_equal_to(hashmap!{
  //     "one".to_string() => AnyValue::String("1".to_string()),
  //     "two".to_string() => AnyValue::Integer(2)
  //   }));
  // }
  //
  // #[test]
  // fn workflow_fails_to_load_if_there_are_no_steps() {
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("workflowId".to_string()), Yaml::String("test".to_string()));
  //
  //   expect!(Workflow::try_from(&Yaml::Hash(hash.clone()))).to(be_err());
  //
  //   hash.insert(Yaml::String("steps".to_string()), Yaml::Array(vec![]));
  //   expect!(Workflow::try_from(&Yaml::Hash(hash))).to(be_err());
  // }
  //
  // #[test]
  // fn workflow_supports_extensions() {
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("workflowId".to_string()), Yaml::String("test".to_string()));
  //   hash.insert(Yaml::String("steps".to_string()), Yaml::Array(steps_fixture()));
  //   hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
  //   hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));
  //
  //   let wf = Workflow::try_from(&Yaml::Hash(hash)).unwrap();
  //   expect!(wf.extensions).to(be_equal_to(hashmap!{
  //     "one".to_string() => AnyValue::String("1".to_string()),
  //     "two".to_string() => AnyValue::Integer(2)
  //   }));
  // }
  //
  // #[test]
  // fn steps_supports_extensions() {
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("stepId".to_string()), Yaml::String("test".to_string()));
  //   hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
  //   hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));
  //
  //   let step = Step::try_from(&Yaml::Hash(hash)).unwrap();
  //   expect!(step.extensions).to(be_equal_to(hashmap!{
  //     "one".to_string() => AnyValue::String("1".to_string()),
  //     "two".to_string() => AnyValue::Integer(2)
  //   }));
  // }
  //
  // #[test]
  // fn components_supports_extensions() {
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("workflowId".to_string()), Yaml::String("test".to_string()));
  //   hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
  //   hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));
  //
  //   let mut outer = Hash::new();
  //   outer.insert(Yaml::String("components".to_string()), Yaml::Hash(hash));
  //
  //   let components = Components::try_from(&outer).unwrap();
  //   expect!(components.extensions).to(be_equal_to(hashmap!{
  //     "one".to_string() => AnyValue::String("1".to_string()),
  //     "two".to_string() => AnyValue::Integer(2)
  //   }));
  // }

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

  // #[test]
  // fn load_workflow_outputs() {
  //   let mut outputs = Hash::new();
  //   outputs.insert(Yaml::String("tokenExpires".to_string()), Yaml::String("$response.header.X-Expires-After".to_string()));
  //   outputs.insert(Yaml::String("rateLimit".to_string()), Yaml::String("$response.header.X-Rate-Limit".to_string()));
  //   outputs.insert(Yaml::String("invalid".to_string()), Yaml::Array(vec![]));
  //
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("workflowId".to_string()), Yaml::String("test".to_string()));
  //   hash.insert(Yaml::String("steps".to_string()), Yaml::Array(steps_fixture()));
  //   hash.insert(Yaml::String("outputs".to_string()), Yaml::Hash(outputs));
  //
  //   let wf = Workflow::try_from(&Yaml::Hash(hash)).unwrap();
  //   expect!(wf.outputs).to(be_equal_to(hashmap!{
  //     "tokenExpires".to_string() => "$response.header.X-Expires-After".to_string(),
  //     "rateLimit".to_string() => "$response.header.X-Rate-Limit".to_string()
  //   }));
  // }
  //
  // #[test]
  // fn load_workflow_parameters() {
  //   let mut parameter = Hash::new();
  //   parameter.insert(Yaml::String("name".to_string()), Yaml::String("username".to_string()));
  //   parameter.insert(Yaml::String("in".to_string()), Yaml::String("query".to_string()));
  //   parameter.insert(Yaml::String("value".to_string()), Yaml::String("$inputs.username".to_string()));
  //
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("workflowId".to_string()), Yaml::String("test".to_string()));
  //   hash.insert(Yaml::String("steps".to_string()), Yaml::Array(steps_fixture()));
  //   hash.insert(Yaml::String("parameters".to_string()), Yaml::Array(vec![Yaml::Hash(parameter)]));
  //
  //   let wf = Workflow::try_from(&Yaml::Hash(hash)).unwrap();
  //   expect!(wf.parameters).to(be_equal_to(vec![
  //     Either::Left(ParameterObject {
  //       name: "username".to_string(),
  //       r#in: Some("query".to_string()),
  //       value: Either::Right("$inputs.username".to_string()),
  //       extensions: Default::default()
  //     })
  //   ]));
  //
  //   let mut parameter_hash = Hash::new();
  //   parameter_hash.insert(Yaml::String("name".to_string()), Yaml::String("username".to_string()));
  //   parameter_hash.insert(Yaml::String("value".to_string()), Yaml::Integer(10));
  //
  //   let parameter = ParameterObject::try_from(&parameter_hash).unwrap();
  //   expect!(parameter).to(be_equal_to(ParameterObject {
  //     name: "username".to_string(),
  //     r#in: None,
  //     value: Either::Left(AnyValue::Integer(10)),
  //     extensions: Default::default()
  //   }));
  // }

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

  // #[test]
  // fn load_step_parameters() {
  //   let mut parameter = Hash::new();
  //   parameter.insert(Yaml::String("name".to_string()), Yaml::String("username".to_string()));
  //   parameter.insert(Yaml::String("in".to_string()), Yaml::String("query".to_string()));
  //   parameter.insert(Yaml::String("value".to_string()), Yaml::String("$inputs.username".to_string()));
  //
  //   let mut hash = Hash::new();
  //   hash.insert(Yaml::String("stepId".to_string()), Yaml::String("test".to_string()));
  //   hash.insert(Yaml::String("parameters".to_string()), Yaml::Array(vec![Yaml::Hash(parameter)]));
  //
  //   let step = Step::try_from(&Yaml::Hash(hash)).unwrap();
  //   expect!(step.parameters).to(be_equal_to(vec![
  //     Either::Left(ParameterObject {
  //       name: "username".to_string(),
  //       r#in: Some("query".to_string()),
  //       value: Either::Right("$inputs.username".to_string()),
  //       extensions: Default::default()
  //     })
  //   ]));
  // }

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
    expect!(criterion.r#type).to(be_some().value(Either::Left("regex".to_string())));
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
}
