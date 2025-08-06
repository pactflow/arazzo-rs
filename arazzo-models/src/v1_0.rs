//! Version 1.0.x specification models (https://spec.openapis.org/arazzo/v1.0.1.html)

use std::collections::HashMap;
use std::rc::Rc;

#[cfg(feature = "yaml")] use anyhow::anyhow;
use itertools::Either;
use serde_json::Value;
#[cfg(feature = "yaml")] use yaml_rust2::yaml::Hash;
#[cfg(feature = "yaml")] use yaml_rust2::Yaml;

#[cfg(feature = "yaml")] use crate::extensions::yaml_extract_extensions;
use crate::extensions::AnyValue;
use crate::payloads::{EmptyPayload, JsonPayload, Payload, StringPayload};
#[cfg(feature = "yaml")] use crate::yaml::{
  yaml_hash_entry_to_json,
  yaml_hash_lookup,
  yaml_hash_lookup_integer,
  yaml_hash_lookup_number,
  yaml_hash_lookup_string,
  yaml_hash_lookup_string_list,
  yaml_hash_require_string,
  yaml_to_json,
  yaml_type_name
};

/// 4.6.1 Arazzo Description is the root object of the loaded specification.
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#arazzo-description)
#[derive(Debug, Clone, PartialEq)]
pub struct ArazzoDescription {
  /// Version number of the Arazzo Specification
  pub arazzo: String,
  /// Metadata about API workflows defined in the Arazzo document
  pub info: Info,
  /// List of source descriptions
  pub source_descriptions: Vec<SourceDescription>,
  /// List of workflows
  pub workflows: Vec<Workflow>,
  /// An element to hold shared schemas.
  pub components: Components,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>,
}

#[cfg(feature = "yaml")]
impl TryFrom<&Yaml> for ArazzoDescription {
  type Error = anyhow::Error;

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    if let Some(hash) = value.as_hash() {
      if let Ok(version) = yaml_hash_require_string(hash, "arazzo") {
        let info = Info::try_from(hash)?;
        let source_descriptions = yaml_load_source_descriptions(hash)?;
        let workflows = yaml_load_workflows(hash)?;
        let components = Components::try_from(hash)?;

        Ok(ArazzoDescription {
          arazzo: version,
          info,
          source_descriptions,
          workflows,
          components,
          extensions: yaml_extract_extensions(&hash)?
        })
      } else {
        Err(anyhow!("Arazzo version number is required [4.6.1.1 Fixed Fields]"))
      }
    } else {
      Err(anyhow!("YAML value must be a Hash, got {}", yaml_type_name(value)))
    }
  }
}

/// 4.6.2 Info Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#info-object)
#[derive(Debug, Clone, PartialEq)]
pub struct Info {
  /// A human-readable title of the Arazzo Description.
  pub title: String,
  /// A short summary of the Arazzo Description.
  pub summary: Option<String>,
  /// A description of the purpose of the workflows defined.
  pub description: Option<String>,
  /// Document version
  pub version: String,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
}

#[cfg(feature = "yaml")]
impl TryFrom<&Hash> for Info {
  type Error = anyhow::Error;

  fn try_from(value: &Hash) -> Result<Self, Self::Error> {
    if let Some(hash) = yaml_hash_lookup(value, "info", |v | v.as_hash().cloned()) {
      Ok(Info {
        title: yaml_hash_require_string(&hash, "title")?,
        summary: yaml_hash_lookup_string(&hash, "summary"),
        description: yaml_hash_lookup_string(&hash, "description"),
        version: yaml_hash_require_string(&hash, "version")?,
        extensions: yaml_extract_extensions(&hash)?
      })
    } else {
      Err(anyhow!("Info Object is required [4.6.1.1 Fixed Fields]"))
    }
  }
}

/// 4.6.3 Source Description Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#source-description-object)
#[derive(Debug, Clone, PartialEq)]
pub struct SourceDescription {
  /// Unique name for the source description.
  pub name: String,
  /// URL to a source description to be used by a workflow.
  pub url: String,
  /// The type of source description.
  pub r#type: Option<String>,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
}

#[cfg(feature = "yaml")]
fn yaml_load_source_descriptions(hash: &Hash) -> anyhow::Result<Vec<SourceDescription>> {
  if let Some(array) = yaml_hash_lookup(hash, "sourceDescriptions", |v | v.as_vec().cloned()) {
    if array.is_empty() {
      Err(anyhow!("Source Description list must have at least one entry [4.6.1.1 Fixed Fields]"))
    } else {
      let mut list = vec![];

      for item in &array {
        list.push(SourceDescription::try_from(item)?);
      }

      Ok(list)
    }
  } else {
    Err(anyhow!("Source Description Object is required [4.6.1.1 Fixed Fields]"))
  }
}

#[cfg(feature = "yaml")]
impl TryFrom<&Yaml> for SourceDescription {
  type Error = anyhow::Error;

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    if let Some(hash) = value.as_hash() {
      Ok(SourceDescription {
        name: yaml_hash_require_string(&hash, "name")?,
        url: yaml_hash_require_string(&hash, "url")?,
        r#type: yaml_hash_lookup_string(&hash, "type"),
        extensions: yaml_extract_extensions(&hash)?
      })
    } else {
      Err(anyhow!("YAML value must be a Hash, got {}", yaml_type_name(value)))
    }
  }
}

/// 4.6.4 Workflow Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#workflow-object)
#[derive(Debug, Clone, PartialEq)]
pub struct Workflow {
  /// Unique string to represent the workflow.
  pub workflow_id: String,
  /// Summary of the purpose or objective of the workflow.
  pub summary: Option<String>,
  /// Description of the workflow.
  pub description: Option<String>,
  /// JSON Schema 2020-12 object representing the input parameters used by the workflow.
  pub inputs: Value,
  /// List of workflows that must be completed before this workflow can be processed.
  pub depends_on: Vec<String>,
  /// An ordered list of workflow steps
  pub steps: Vec<Step>,
  /// List of success actions that are applicable for all steps described under the workflow.
  pub success_actions: Vec<Either<SuccessObject, ReusableObject>>,
  /// List of success actions that are applicable for all steps described under the workflow.
  pub failure_actions: Vec<Either<FailureObject, ReusableObject>>,
  /// Defined outputs of the workflow.
  pub outputs: HashMap<String, String>,
  /// List of parameters that are applicable for all steps described under the workflow.
  pub parameters: Vec<Either<ParameterObject, ReusableObject>>,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
}

#[cfg(feature = "yaml")]
fn yaml_load_workflows(hash: &Hash) -> anyhow::Result<Vec<Workflow>> {
  if let Some(array) = yaml_hash_lookup(hash, "workflows", |v | v.as_vec().cloned()) {
    if array.is_empty() {
      Err(anyhow!("Workflows list must have at least one entry [4.6.1.1 Fixed Fields]"))
    } else {
      let mut list = vec![];

      for item in &array {
        list.push(Workflow::try_from(item)?);
      }

      Ok(list)
    }
  } else {
    Err(anyhow!("Workflow Object is required [4.6.1.1 Fixed Fields]"))
  }
}

#[cfg(feature = "yaml")]
impl TryFrom<&Yaml> for Workflow {
  type Error = anyhow::Error;

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    if let Some(hash) = value.as_hash() {
      Ok(Workflow {
        workflow_id: yaml_hash_require_string(hash, "workflowId")?,
        summary: yaml_hash_lookup_string(hash, "summary"),
        description: yaml_hash_lookup_string(hash, "description"),
        inputs: yaml_hash_entry_to_json(hash, "inputs")?,
        depends_on: yaml_hash_lookup_string_list(hash, "dependsOn").unwrap_or_default(),
        steps: yaml_load_steps(hash)?,
        success_actions: yaml_load_success_actions(hash)?,
        failure_actions: yaml_load_failure_actions(hash)?,
        outputs: yaml_load_outputs(hash),
        parameters: yaml_load_parameters(hash)?,
        extensions: yaml_extract_extensions(&hash)?
      })
    } else {
      Err(anyhow!("YAML value must be a Hash, got {}", yaml_type_name(value)))
    }
  }
}

/// 4.6.5 Step Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#step-object)
#[derive(Debug, Clone, PartialEq)]
pub struct Step {
  /// Unique string to represent the step.
  pub step_id: String,
  /// Name of an existing, resolvable operation, as defined with a unique operation Id and existing
  /// within one of the source descriptions.
  pub operation_id: Option<String>,
  /// Reference to a Source Description Object combined with a JSON Pointer to reference an operation.
  pub operation_path: Option<String>,
  /// The workflow Id referencing an existing workflow within the Arazzo Description.
  pub workflow_id: Option<String>,
  /// Description of the step.
  pub description: Option<String>,
  /// List of parameters that must be passed to an operation or workflow as referenced by
  /// operationId, operationPath, or workflowId.
  pub parameters: Vec<Either<ParameterObject, ReusableObject>>,
  /// Request body to pass to an operation as referenced by operationId or operationPath.
  pub request_body: Option<RequestBody>,
  /// Array of success action objects that specify what to do upon step success.
  pub on_success: Vec<Either<SuccessObject, ReusableObject>>,
  /// Array of failure action objects that specify what to do upon step failure.
  pub on_failure: Vec<Either<FailureObject, ReusableObject>>,
  /// Defined outputs of the step.
  pub outputs: HashMap<String, String>,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
}

#[cfg(feature = "yaml")]
fn yaml_load_steps(hash: &Hash) -> anyhow::Result<Vec<Step>> {
  if let Some(array) = yaml_hash_lookup(hash, "steps", |v | v.as_vec().cloned()) {
    if array.is_empty() {
      Err(anyhow!("At lest one Step is required [4.6.4.1 Fixed Fields]"))
    } else {
      let mut list = vec![];

      for item in &array {
        list.push(Step::try_from(item)?);
      }

      Ok(list)
    }
  } else {
    Err(anyhow!("At lest one Step is required [4.6.4.1 Fixed Fields]"))
  }
}

#[cfg(feature = "yaml")]
impl TryFrom<&Yaml> for Step {
  type Error = anyhow::Error;

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    if let Some(hash) = value.as_hash() {
      Ok(Step {
        step_id: yaml_hash_require_string(&hash, "stepId")?,
        operation_id: yaml_hash_lookup_string(&hash, "operationId"),
        operation_path: yaml_hash_lookup_string(&hash, "operationPath"),
        workflow_id: yaml_hash_lookup_string(&hash, "workflowId"),
        description: yaml_hash_lookup_string(&hash, "description"),
        parameters: yaml_load_parameters(hash)?,
        request_body: yaml_hash_lookup(hash, "requestBody", |v| Some(RequestBody::try_from(v)))
          .transpose()?,
        on_success: yaml_load_success_actions(hash)?,
        on_failure: yaml_load_failure_actions(hash)?,
        outputs: yaml_load_outputs(hash),
        extensions: yaml_extract_extensions(&hash)?
      })
    } else {
      Err(anyhow!("YAML value must be a Hash, got {}", yaml_type_name(value)))
    }
  }
}

/// 4.6.6 Parameter Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#parameter-object)
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterObject {
  /// The name of the parameter.
  pub name: String,
  /// The location of the parameter.
  pub r#in: Option<String>,
  /// Value to pass in the parameter.
  pub value: Either<AnyValue, String>,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
}

#[cfg(feature = "yaml")]
impl TryFrom<&Hash> for ParameterObject {
  type Error = anyhow::Error;

  fn try_from(value: &Hash) -> Result<Self, Self::Error> {
    Ok(ParameterObject {
      name: yaml_hash_require_string(value, "name")?,
      r#in: yaml_hash_lookup_string(value, "in"),
      value: yaml_load_parameter_value(value, "value")?,
      extensions: yaml_extract_extensions(value)?
    })
  }
}

#[cfg(feature = "yaml")]
fn yaml_load_parameters(hash: &Hash) -> anyhow::Result<Vec<Either<ParameterObject, ReusableObject>>> {
  if let Some(array) = yaml_hash_lookup(hash, "parameters", |v | v.as_vec().cloned()) {
    let mut list = vec![];

    for item in &array {
      if let Some(hash) = item.as_hash() {
        if hash.contains_key(&Yaml::String("reference".to_string())) {
          list.push(Either::Right(ReusableObject::try_from(hash)?));
        } else {
          list.push(Either::Left(ParameterObject::try_from(hash)?));
        }
      }
    }

    Ok(list)
  } else {
    Ok(vec![])
  }
}

#[cfg(feature = "yaml")]
fn yaml_load_parameter_value(hash: &Hash, key: &str) -> anyhow::Result<Either<AnyValue, String>> {
  yaml_hash_lookup(hash, key, |v | {
    if let Some(s) = v.as_str() {
      if s.starts_with('$') {
        Some(Either::Right(s.to_string()))
      } else {
        Some(Either::Left(AnyValue::String(s.to_string())))
      }
    } else {
      AnyValue::try_from(v)
        .ok()
        .map(Either::Left)
    }
  }).ok_or_else(|| anyhow!("Parameter value is required [4.6.6.1 Fixed Fields]"))
}

#[cfg(feature = "yaml")]
fn yaml_load_success_actions(hash: &Hash) -> anyhow::Result<Vec<Either<SuccessObject, ReusableObject>>> {
  if let Some(array) = yaml_hash_lookup(hash, "successActions", |v | v.as_vec().cloned()) {
    let mut list = vec![];

    for item in &array {
      if let Some(hash) = item.as_hash() {
        if hash.contains_key(&Yaml::String("reference".to_string())) {
          list.push(Either::Right(ReusableObject::try_from(hash)?));
        } else {
          list.push(Either::Left(SuccessObject::try_from(hash)?));
        }
      }
    }

    Ok(list)
  } else {
    Ok(vec![])
  }
}

#[cfg(feature = "yaml")]
fn yaml_load_failure_actions(hash: &Hash) -> anyhow::Result<Vec<Either<FailureObject, ReusableObject>>> {
  if let Some(array) = yaml_hash_lookup(hash, "failureActions", |v | v.as_vec().cloned()) {
    let mut list = vec![];

    for item in &array {
      if let Some(hash) = item.as_hash() {
        if hash.contains_key(&Yaml::String("reference".to_string())) {
          list.push(Either::Right(ReusableObject::try_from(hash)?));
        } else {
          list.push(Either::Left(FailureObject::try_from(hash)?));
        }
      }
    }

    Ok(list)
  } else {
    Ok(vec![])
  }
}

#[cfg(feature = "yaml")]
fn yaml_load_outputs(hash: &Hash) -> HashMap<String, String> {
  yaml_hash_lookup(hash, "outputs", |v | {
    if let Some(outputs_hash) = v.as_hash() {
      Some(outputs_hash.iter()
        .filter_map(|(k, v)| {
          if let Some(key) = k.as_str() {
            v.as_str().map(|value| (key.to_string(), value.to_string()))
          } else {
            None
          }
        }).collect())
    } else {
      None
    }
  }).unwrap_or_default()
}

/// 4.6.7 Success Action Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#success-action-object)
#[derive(Debug, Clone, PartialEq)]
pub struct SuccessObject {
  /// The name of the success action.
  pub name: String,
  /// The type of action to take.
  pub r#type: String,
  /// The workflowId referencing an existing workflow within the Arazzo Description to transfer to
  /// upon success of the step.
  pub workflow_id: Option<String>,
  /// The stepId to transfer to upon success of the step.
  pub step_id: Option<String>,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
}

#[cfg(feature = "yaml")]
impl TryFrom<&Hash> for SuccessObject {
  type Error = anyhow::Error;

  fn try_from(value: &Hash) -> Result<Self, Self::Error> {
    Ok(SuccessObject {
      name: yaml_hash_require_string(value, "name")?,
      r#type: yaml_hash_require_string(value, "type")?,
      workflow_id: yaml_hash_lookup_string(value, "workflowId"),
      step_id: yaml_hash_lookup_string(value, "stepId"),
      extensions: yaml_extract_extensions(value)?
    })
  }
}

/// 4.6.8 Failure Action Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#failure-action-object)
#[derive(Debug, Clone, PartialEq)]
pub struct FailureObject {
  /// The name of the success action.
  pub name: String,
  /// The type of action to take.
  pub r#type: String,
  /// The workflowId referencing an existing workflow within the Arazzo Description to transfer to
  /// upon success of the step.
  pub workflow_id: Option<String>,
  /// The stepId to transfer to upon success of the step.
  pub step_id: Option<String>,
  /// A non-negative decimal indicating the seconds to delay after the step failure before another
  /// attempt shall be made.
  pub retry_after: Option<f64>,
  /// A non-negative integer indicating how many attempts to retry the step may be attempted before
  /// failing the overall step.
  pub retry_limit: Option<i64>,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
}

#[cfg(feature = "yaml")]
impl TryFrom<&Hash> for FailureObject {
  type Error = anyhow::Error;

  fn try_from(value: &Hash) -> Result<Self, Self::Error> {
    Ok(FailureObject {
      name: yaml_hash_require_string(value, "name")?,
      r#type: yaml_hash_require_string(value, "type")?,
      workflow_id: yaml_hash_lookup_string(value, "workflowId"),
      step_id: yaml_hash_lookup_string(value, "stepId"),
      retry_after: yaml_hash_lookup_number(value, "retryAfter"),
      retry_limit: yaml_hash_lookup_integer(value, "retryLimit"),
      extensions: yaml_extract_extensions(value)?
    })
  }
}

/// 4.6.9 Components Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#components-object)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Components {
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
}

#[cfg(feature = "yaml")]
impl TryFrom<&Hash> for Components {
  type Error = anyhow::Error;

  fn try_from(value: &Hash) -> Result<Self, Self::Error> {
    if let Some(hash) = yaml_hash_lookup(value, "components", |v | v.as_hash().cloned()) {
      Ok(Components {
        extensions: yaml_extract_extensions(&hash)?
      })
    } else {
      Ok(Components::default())
    }
  }
}

/// 4.6.10 Reusable Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#reusable-object)
#[derive(Debug, Clone, PartialEq)]
pub struct ReusableObject {
  /// Runtime Expression used to reference the desired object.
  pub reference: String,
  /// Sets a value of the referenced parameter.
  pub value: Option<String>
}

#[cfg(feature = "yaml")]
impl TryFrom<&Hash> for ReusableObject {
  type Error = anyhow::Error;

  fn try_from(value: &Hash) -> Result<Self, Self::Error> {
    if let Ok(reference) = yaml_hash_require_string(value, "reference") {
      Ok(ReusableObject {
        reference,
        value: yaml_hash_lookup_string(value, "value")
      })
    } else {
      Err(anyhow!("Reference is required [4.6.10.1 Fixed Fields]"))
    }
  }
}

/// 4.6.13 Request Body Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#fixed-fields-11)
#[derive(Debug, Clone)]
pub struct RequestBody {
  /// Content-Type for the request content.
  pub content_type: Option<String>,
  /// Value representing the request body payload.
  pub payload: Option<Rc<dyn Payload + Send + Sync>>,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
}

#[cfg(feature = "yaml")]
impl TryFrom<&Yaml> for RequestBody {
  type Error = anyhow::Error;

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    if let Some(hash) = value.as_hash() {
      let content_type = yaml_hash_lookup_string(hash, "contentType");
      let payload = yaml_load_payload(hash, "payload", content_type.as_ref())?;
      Ok(RequestBody {
        content_type,
        payload,
        extensions: yaml_extract_extensions(&hash)?
      })
    } else {
      Err(anyhow!("YAML value must be a Hash, got {}", yaml_type_name(value)))
    }
  }
}

#[cfg(feature = "yaml")]
fn yaml_load_payload(
  hash: &Hash,
  key: &str,
  _content_type: Option<&String>
) -> anyhow::Result<Option<Rc<dyn Payload + Send + Sync>>> {
  yaml_hash_lookup(hash, key, |value| {
    match value {
      Yaml::String(s) => {
        let payload: Rc<dyn Payload + Send + Sync> = Rc::new(StringPayload(s.clone()));
        Some(Ok(payload))
      },
      Yaml::Null => Some(Ok(Rc::new(EmptyPayload))),
      _ => Some(yaml_to_json(value)
        .map(|json| {
          let payload: Rc<dyn Payload + Send + Sync> = Rc::new(JsonPayload(json));
          payload
        }))
    }
  }).transpose()
}

impl PartialEq for RequestBody {
  fn eq(&self, other: &Self) -> bool {
    if self.content_type == other.content_type && self.extensions == other.extensions {
      if self.payload.is_none() && other.payload.is_none() {
        true
      } else if let Some(payload) = &self.payload && let Some(other_payload) = &other.payload {
        payload.as_bytes() == other_payload.as_bytes()
      } else {
        false
      }
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use std::any::Any;
  use std::rc::Rc;

  use expectest::expect;
  use expectest::matchers::be_equal_to;
  use maplit::hashmap;

  use crate::extensions::AnyValue;
  use crate::payloads::StringPayload;
  use crate::v1_0::RequestBody;

  #[test]
  fn request_body_partial_equals() {
    let body1 = RequestBody {
      content_type: None,
      payload: None,
      extensions: Default::default()
    };
    let body2 = RequestBody {
      content_type: Some("text/plain".to_string()),
      payload: None,
      extensions: Default::default()
    };
    let body3 = RequestBody {
      content_type: None,
      payload: None,
      extensions: hashmap!{
        "a".to_string() => AnyValue::Integer(100)
      }
    };
    let body4 = RequestBody {
      content_type: None,
      payload: Some(Rc::new(StringPayload("some text".to_string()))),
      extensions: hashmap!{
        "a".to_string() => AnyValue::Integer(100)
      }
    };

    expect!(&body1).to(be_equal_to(&body1));
    expect!(&body1).to_not(be_equal_to(&body2));
    expect!(&body1).to_not(be_equal_to(&body3));
    expect!(&body1).to_not(be_equal_to(&body4));
    expect!(&body2).to(be_equal_to(&body2));
    expect!(&body2).to_not(be_equal_to(&body1));
    expect!(&body2).to_not(be_equal_to(&body3));
    expect!(&body2).to_not(be_equal_to(&body4));
    expect!(&body3).to(be_equal_to(&body3));
    expect!(&body3).to_not(be_equal_to(&body1));
    expect!(&body3).to_not(be_equal_to(&body2));
    expect!(&body3).to_not(be_equal_to(&body4));
    expect!(&body4).to(be_equal_to(&body4));
    expect!(&body4).to_not(be_equal_to(&body1));
    expect!(&body4).to_not(be_equal_to(&body2));
    expect!(&body4).to_not(be_equal_to(&body3));

    let payload: &dyn Any = body4.payload.as_ref().unwrap().as_ref();
    let p = payload.downcast_ref::<StringPayload>().unwrap();
    expect!(&p.0).to(be_equal_to("some text"));
  }
}

#[cfg(test)]
#[cfg(feature = "yaml")]
mod yaml_tests {
  use expectest::prelude::*;
  use maplit::hashmap;
  use pretty_assertions::assert_eq;
  use serde_json::json;
  use std::any::Any;
  use trim_margin::MarginTrimmable;
  use yaml_rust2::yaml::Hash;
  use yaml_rust2::{Yaml, YamlLoader};

  use crate::extensions::AnyValue;
  use crate::v1_0::*;

  #[test]
  fn fails_to_load_if_the_main_value_is_not_a_yaml_hash() {
    expect!(ArazzoDescription::try_from(&Yaml::String("test".to_string()))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_version_is_missing() {
    expect!(ArazzoDescription::try_from(&Yaml::Hash(Hash::new()))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_version_is_not_a_string() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::Hash(Hash::new()));
    expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_info_is_missing() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
    expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_source_descriptions_are_missing() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
    hash.insert(Yaml::String("info".to_string()), Yaml::Hash(info_fixture()));
    expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_source_descriptions_are_empty() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
    hash.insert(Yaml::String("info".to_string()), Yaml::Hash(info_fixture()));
    hash.insert(Yaml::String("sourceDescriptions".to_string()), Yaml::Array(vec![]));
    expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_workflows_are_missing() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
    hash.insert(Yaml::String("info".to_string()), Yaml::Hash(info_fixture()));
    hash.insert(Yaml::String("sourceDescriptions".to_string()), Yaml::Array(source_descriptions_fixture()));
    expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_workflows_are_empty() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
    hash.insert(Yaml::String("info".to_string()), Yaml::Hash(info_fixture()));
    hash.insert(Yaml::String("sourceDescriptions".to_string()), Yaml::Array(source_descriptions_fixture()));
    hash.insert(Yaml::String("workflows".to_string()), Yaml::Array(vec![]));
    expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  }

  #[test]
  fn arazzo_description_supports_extensions() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
    hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
    hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));

    hash.insert(Yaml::String("info".to_string()), Yaml::Hash(info_fixture()));
    hash.insert(Yaml::String("sourceDescriptions".to_string()), Yaml::Array(source_descriptions_fixture()));
    hash.insert(Yaml::String("workflows".to_string()), Yaml::Array(workflows_fixture()));

    let desc = ArazzoDescription::try_from(&Yaml::Hash(hash)).unwrap();
    expect!(desc.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::Integer(2)
    }));
  }

  fn info_fixture() -> Hash {
    let mut info = Hash::new();
    info.insert(Yaml::String("title".to_string()), Yaml::String("test".to_string()));
    info.insert(Yaml::String("version".to_string()), Yaml::String("1.0.0".to_string()));
    info
  }

  fn source_descriptions_fixture() -> Vec<Yaml> {
    let mut desc = Hash::new();
    desc.insert(Yaml::String("name".to_string()), Yaml::String("test".to_string()));
    desc.insert(Yaml::String("url".to_string()), Yaml::String("http://test".to_string()));
    vec![Yaml::Hash(desc)]
  }

  fn workflows_fixture() -> Vec<Yaml> {
    let mut wf = Hash::new();
    wf.insert(Yaml::String("workflowId".to_string()), Yaml::String("test".to_string()));
    wf.insert(Yaml::String("steps".to_string()), Yaml::Array(steps_fixture()));
    vec![Yaml::Hash(wf)]
  }

  fn steps_fixture() -> Vec<Yaml> {
    let mut desc = Hash::new();
    desc.insert(Yaml::String("stepId".to_string()), Yaml::String("test".to_string()));
    vec![Yaml::Hash(desc)]
  }

  #[test]
  fn info_supports_extensions() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("title".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("version".to_string()), Yaml::String("1.0.0".to_string()));
    hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
    hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));

    let mut outer = Hash::new();
    outer.insert(Yaml::String("info".to_string()), Yaml::Hash(hash));
    let info = Info::try_from(&outer).unwrap();
    expect!(info.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::Integer(2)
    }));
  }

  #[test]
  fn source_description_supports_extensions() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("name".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("url".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
    hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));

    let desc = SourceDescription::try_from(&Yaml::Hash(hash)).unwrap();
    expect!(desc.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::Integer(2)
    }));
  }

  #[test]
  fn workflow_fails_to_load_if_there_are_no_steps() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("workflowId".to_string()), Yaml::String("test".to_string()));

    expect!(Workflow::try_from(&Yaml::Hash(hash.clone()))).to(be_err());

    hash.insert(Yaml::String("steps".to_string()), Yaml::Array(vec![]));
    expect!(Workflow::try_from(&Yaml::Hash(hash))).to(be_err());
  }

  #[test]
  fn workflow_supports_extensions() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("workflowId".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("steps".to_string()), Yaml::Array(steps_fixture()));
    hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
    hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));

    let wf = Workflow::try_from(&Yaml::Hash(hash)).unwrap();
    expect!(wf.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::Integer(2)
    }));
  }

  #[test]
  fn steps_supports_extensions() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("stepId".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
    hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));

    let step = Step::try_from(&Yaml::Hash(hash)).unwrap();
    expect!(step.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::Integer(2)
    }));
  }

  #[test]
  fn components_supports_extensions() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("workflowId".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
    hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));

    let mut outer = Hash::new();
    outer.insert(Yaml::String("components".to_string()), Yaml::Hash(hash));

    let components = Components::try_from(&outer).unwrap();
    expect!(components.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::Integer(2)
    }));
  }

  #[test]
  fn load_success_object() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("name".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("type".to_string()), Yaml::String("end".to_string()));
    hash.insert(Yaml::String("workflowId".to_string()), Yaml::String("workflowId".to_string()));
    hash.insert(Yaml::String("stepId".to_string()), Yaml::String("stepId".to_string()));

    let success = SuccessObject::try_from(&hash).unwrap();
    expect!(&success.name).to(be_equal_to("test"));
    expect!(&success.r#type).to(be_equal_to("end"));
    expect!(success.workflow_id.clone()).to(be_some().value("workflowId"));
    expect!(success.step_id.clone()).to(be_some().value("stepId"));

    let mut hash = Hash::new();
    hash.insert(Yaml::String("name".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("type".to_string()), Yaml::String("end".to_string()));

    let success = SuccessObject::try_from(&hash).unwrap();
    expect!(&success.name).to(be_equal_to("test"));
    expect!(&success.r#type).to(be_equal_to("end"));
    expect!(success.workflow_id.clone()).to(be_none());
    expect!(success.step_id.clone()).to(be_none());
  }

  #[test]
  fn success_object_supports_extensions() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("name".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("type".to_string()), Yaml::String("end".to_string()));
    hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
    hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));

    let success = SuccessObject::try_from(&hash).unwrap();
    expect!(success.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::Integer(2)
    }));
  }

  #[test]
  fn load_failure_object() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("name".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("type".to_string()), Yaml::String("end".to_string()));
    hash.insert(Yaml::String("workflowId".to_string()), Yaml::String("workflowId".to_string()));
    hash.insert(Yaml::String("stepId".to_string()), Yaml::String("stepId".to_string()));
    hash.insert(Yaml::String("retryAfter".to_string()), Yaml::Real("10.5".to_string()));
    hash.insert(Yaml::String("retryLimit".to_string()), Yaml::Integer(10));

    let failure = FailureObject::try_from(&hash).unwrap();
    expect!(&failure.name).to(be_equal_to("test"));
    expect!(&failure.r#type).to(be_equal_to("end"));
    expect!(failure.workflow_id.clone()).to(be_some().value("workflowId"));
    expect!(failure.step_id.clone()).to(be_some().value("stepId"));
    expect!(failure.retry_after.clone()).to(be_some().value(10.5));
    expect!(failure.retry_limit.clone()).to(be_some().value(10));

    let mut hash = Hash::new();
    hash.insert(Yaml::String("name".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("type".to_string()), Yaml::String("end".to_string()));

    let failure = FailureObject::try_from(&hash).unwrap();
    expect!(&failure.name).to(be_equal_to("test"));
    expect!(&failure.r#type).to(be_equal_to("end"));
    expect!(failure.workflow_id.clone()).to(be_none());
    expect!(failure.step_id.clone()).to(be_none());
    expect!(failure.retry_after.clone()).to(be_none());
    expect!(failure.retry_limit.clone()).to(be_none());
  }

  #[test]
  fn failure_object_supports_extensions() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("name".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("type".to_string()), Yaml::String("end".to_string()));
    hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
    hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));

    let failure = FailureObject::try_from(&hash).unwrap();
    expect!(failure.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::Integer(2)
    }));
  }

  #[test]
  fn load_reusable_object() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("reference".to_string()), Yaml::String("$test".to_string()));
    hash.insert(Yaml::String("value".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("workflowId".to_string()), Yaml::String("workflowId".to_string()));

    let obj = ReusableObject::try_from(&hash).unwrap();
    expect!(&obj.reference).to(be_equal_to("$test"));
    expect!(obj.value.clone()).to(be_some().value("test"));

    let mut hash = Hash::new();
    hash.insert(Yaml::String("reference".to_string()), Yaml::String("$test".to_string()));

    let obj = ReusableObject::try_from(&hash).unwrap();
    expect!(&obj.reference).to(be_equal_to("$test"));
    expect!(obj.value.clone()).to(be_none());
  }

  #[test]
  fn load_workflow_outputs() {
    let mut outputs = Hash::new();
    outputs.insert(Yaml::String("tokenExpires".to_string()), Yaml::String("$response.header.X-Expires-After".to_string()));
    outputs.insert(Yaml::String("rateLimit".to_string()), Yaml::String("$response.header.X-Rate-Limit".to_string()));
    outputs.insert(Yaml::String("invalid".to_string()), Yaml::Array(vec![]));

    let mut hash = Hash::new();
    hash.insert(Yaml::String("workflowId".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("steps".to_string()), Yaml::Array(steps_fixture()));
    hash.insert(Yaml::String("outputs".to_string()), Yaml::Hash(outputs));

    let wf = Workflow::try_from(&Yaml::Hash(hash)).unwrap();
    expect!(wf.outputs).to(be_equal_to(hashmap!{
      "tokenExpires".to_string() => "$response.header.X-Expires-After".to_string(),
      "rateLimit".to_string() => "$response.header.X-Rate-Limit".to_string()
    }));
  }

  #[test]
  fn load_workflow_parameters() {
    let mut parameter = Hash::new();
    parameter.insert(Yaml::String("name".to_string()), Yaml::String("username".to_string()));
    parameter.insert(Yaml::String("in".to_string()), Yaml::String("query".to_string()));
    parameter.insert(Yaml::String("value".to_string()), Yaml::String("$inputs.username".to_string()));

    let mut hash = Hash::new();
    hash.insert(Yaml::String("workflowId".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("steps".to_string()), Yaml::Array(steps_fixture()));
    hash.insert(Yaml::String("parameters".to_string()), Yaml::Array(vec![Yaml::Hash(parameter)]));

    let wf = Workflow::try_from(&Yaml::Hash(hash)).unwrap();
    expect!(wf.parameters).to(be_equal_to(vec![
      Either::Left(ParameterObject {
        name: "username".to_string(),
        r#in: Some("query".to_string()),
        value: Either::Right("$inputs.username".to_string()),
        extensions: Default::default()
      })
    ]));

    let mut parameter_hash = Hash::new();
    parameter_hash.insert(Yaml::String("name".to_string()), Yaml::String("username".to_string()));
    parameter_hash.insert(Yaml::String("value".to_string()), Yaml::Integer(10));

    let parameter = ParameterObject::try_from(&parameter_hash).unwrap();
    expect!(parameter).to(be_equal_to(ParameterObject {
      name: "username".to_string(),
      r#in: None,
      value: Either::Left(AnyValue::Integer(10)),
      extensions: Default::default()
    }));
  }

  #[test]
  fn parameter_object_supports_extensions() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("name".to_string()), Yaml::String("username".to_string()));
    hash.insert(Yaml::String("value".to_string()), Yaml::Integer(10));
    hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
    hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));

    let parameter = ParameterObject::try_from(&hash).unwrap();
    expect!(parameter.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::Integer(2)
    }));
  }

  #[test]
  fn load_step_parameters() {
    let mut parameter = Hash::new();
    parameter.insert(Yaml::String("name".to_string()), Yaml::String("username".to_string()));
    parameter.insert(Yaml::String("in".to_string()), Yaml::String("query".to_string()));
    parameter.insert(Yaml::String("value".to_string()), Yaml::String("$inputs.username".to_string()));

    let mut hash = Hash::new();
    hash.insert(Yaml::String("stepId".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("parameters".to_string()), Yaml::Array(vec![Yaml::Hash(parameter)]));

    let step = Step::try_from(&Yaml::Hash(hash)).unwrap();
    expect!(step.parameters).to(be_equal_to(vec![
      Either::Left(ParameterObject {
        name: "username".to_string(),
        r#in: Some("query".to_string()),
        value: Either::Right("$inputs.username".to_string()),
        extensions: Default::default()
      })
    ]));
  }

  #[test]
  fn load_request_body() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("contentType".to_string()), Yaml::String("text/plain".to_string()));
    hash.insert(Yaml::String("payload".to_string()), Yaml::String("some text".to_string()));

    let body = RequestBody::try_from(&Yaml::Hash(hash)).unwrap();
    expect!(body.content_type).to(be_some().value("text/plain"));
  }

  #[test]
  fn request_body_supports_extensions() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("contentType".to_string()), Yaml::String("text/plain".to_string()));
    hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
    hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));

    let parameter = RequestBody::try_from(&Yaml::Hash(hash)).unwrap();
    expect!(parameter.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => AnyValue::String("1".to_string()),
      "two".to_string() => AnyValue::Integer(2)
    }));
  }

  #[test]
  fn load_payload() {
    let body = r#"
                    contentType: application/json
                    payload: |
                      {
                        "petOrder": {
                          "petId": "{$inputs.pet_id}",
                          "couponCode": "{$inputs.coupon_code}",
                          "quantity": "{$inputs.quantity}",
                          "status": "placed",
                          "complete": false
                        }
                      }
                    "#;
    let yaml = YamlLoader::load_from_str(body).unwrap();

    let body = RequestBody::try_from(&yaml[0]).unwrap();
    expect!(body.content_type).to(be_some().value("application/json"));
    let payload: &dyn Any = body.payload.as_ref().unwrap().as_ref();
    let p = payload.downcast_ref::<StringPayload>().unwrap();
    assert_eq!(
      r#" |{
          |  "petOrder": {
          |    "petId": "{$inputs.pet_id}",
          |    "couponCode": "{$inputs.coupon_code}",
          |    "quantity": "{$inputs.quantity}",
          |    "status": "placed",
          |    "complete": false
          |  }
          |}
          |"#.trim_margin().as_ref().unwrap(), &p.0);

    let body = r#"
                    contentType: application/json
                    payload:
                      petOrder:
                        petId: $inputs.pet_id
                        couponCode: $inputs.coupon_code
                        quantity: $inputs.quantity
                        status: placed
                        complete: false
                    "#;
    let yaml = YamlLoader::load_from_str(body).unwrap();

    let body = RequestBody::try_from(&yaml[0]).unwrap();
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
          "complete": false
        }
      }),
      &p.0
    );

    let body = r#"
                    contentType: application/x-www-form-urlencoded
                    payload:
                      client_id: $inputs.clientId
                      grant_type: $inputs.grantType
                      redirect_uri: $inputs.redirectUri
                      client_secret: $inputs.clientSecret
                      code: $steps.browser-authorize.outputs.code
                      scope: $inputs.scope
                    "#;
    let yaml = YamlLoader::load_from_str(body).unwrap();

    let body = RequestBody::try_from(&yaml[0]).unwrap();
    expect!(body.content_type).to(be_some().value("application/x-www-form-urlencoded"));
    let payload: &dyn Any = body.payload.as_ref().unwrap().as_ref();
    let p = payload.downcast_ref::<JsonPayload>().unwrap();
    assert_eq!(
      &json!({
        "client_id": "$inputs.clientId",
        "grant_type": "$inputs.grantType",
        "redirect_uri": "$inputs.redirectUri",
        "client_secret": "$inputs.clientSecret",
        "code": "$steps.browser-authorize.outputs.code",
        "scope": "$inputs.scope"
      }),
      &p.0
    );
  }
}
