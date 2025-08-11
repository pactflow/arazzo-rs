//! Version 1.0.x specification models (<https://spec.openapis.org/arazzo/v1.0.1.html>)

use std::collections::HashMap;
use std::rc::Rc;

use serde_json::Value;

use crate::either::Either;
use crate::extensions::AnyValue;
use crate::payloads::Payload;


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
  /// List of assertions to determine the success of the step.
  pub success_criteria: Vec<Criterion>,
  /// Array of success action objects that specify what to do upon step success.
  pub on_success: Vec<Either<SuccessObject, ReusableObject>>,
  /// Array of failure action objects that specify what to do upon step failure.
  pub on_failure: Vec<Either<FailureObject, ReusableObject>>,
  /// Defined outputs of the step.
  pub outputs: HashMap<String, String>,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
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
  /// List of assertions to determine if this action shall be executed.
  pub criteria: Vec<Criterion>,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
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
  /// List of assertions to determine if this action shall be executed.
  pub criteria: Vec<Criterion>,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
}

/// 4.6.9 Components Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#components-object)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Components {
  /// Object to hold reusable JSON Schema objects to be referenced from workflow inputs.
  pub inputs: HashMap<String, Value>,
  /// Object to hold reusable Parameter Objects
  pub parameters: HashMap<String, ParameterObject>,
  /// Object to hold reusable Success Actions Objects.
  pub success_actions: HashMap<String, SuccessObject>,
  /// Object to hold reusable Failure Actions Objects.
  pub failure_actions: HashMap<String, FailureObject>,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
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

/// 4.6.11 Criterion Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#criterion-object)
#[derive(Debug, Clone, PartialEq)]
pub struct Criterion {
  /// Runtime Expression used to set the context for the condition to be applied on.
  pub context: Option<String>,
  /// The condition to apply.
  pub condition: String,
  /// The type of condition to be applied.
  pub r#type: Option<Either<String, CriterionExpressionType>>,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
}

/// 4.6.12 Criterion Expression Type Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#criterion-expression-type-object)
#[derive(Debug, Clone, PartialEq)]
pub struct CriterionExpressionType {
  /// The type of condition to be applied.
  pub r#type: String,
  /// A shorthand string representing the version of the expression type being used.
  pub version: String,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
}

/// 4.6.13 Request Body Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#request-body-object)
#[derive(Debug, Clone)]
pub struct RequestBody {
  /// Content-Type for the request content.
  pub content_type: Option<String>,
  /// Value representing the request body payload.
  pub payload: Option<Rc<dyn Payload + Send + Sync>>,
  /// List of locations and values to set within a payload
  pub replacements: Vec<PayloadReplacement>,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
}

impl PartialEq for RequestBody {
  fn eq(&self, other: &Self) -> bool {
    if self.content_type == other.content_type &&
       self.extensions == other.extensions &&
       self.replacements == other.replacements {
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

/// 4.6.14 Payload Replacement Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#payload-replacement-object)
#[derive(Debug, Clone, PartialEq)]
pub struct PayloadReplacement {
  /// A JSON Pointer or XPath Expression which must be resolved against the request body.
  pub target: String,
  /// The value set within the target location.
  pub  value: Either<AnyValue, String>,
  /// Extension values
  pub extensions: HashMap<String, AnyValue>
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
      replacements: vec![],
      extensions: Default::default()
    };
    let body2 = RequestBody {
      content_type: Some("text/plain".to_string()),
      payload: None,
      replacements: vec![],
      extensions: Default::default()
    };
    let body3 = RequestBody {
      content_type: None,
      payload: None,
      replacements: vec![],
      extensions: hashmap!{
        "a".to_string() => AnyValue::Integer(100)
      }
    };
    let body4 = RequestBody {
      content_type: None,
      payload: Some(Rc::new(StringPayload("some text".to_string()))),
      replacements: vec![],
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
