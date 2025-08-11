//! Implementations to support serialization of the models using serde

use std::fmt::Debug;
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};

use crate::either::Either;
use crate::extensions::AnyValue;

impl Serialize for AnyValue {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer
  {
    match self {
      AnyValue::Null => serializer.serialize_unit(),
      AnyValue::Boolean(b) => serializer.serialize_bool(*b),
      AnyValue::Integer(i) => serializer.serialize_i64(*i),
      AnyValue::UInteger(u) => serializer.serialize_u64(*u),
      AnyValue::Float(f) => serializer.serialize_f64(*f),
      AnyValue::String(s) => serializer.serialize_str(s.as_str()),
      AnyValue::Array(a) => {
        let mut seq = serializer.serialize_seq(Some(a.len()))?;
        for e in a {
          seq.serialize_element(e)?;
        }
        seq.end()
      }
      AnyValue::Object(o) => {
        let mut map = serializer.serialize_map(Some(o.len()))?;
        let mut entries = o.iter().collect::<Vec<_>>();
        entries.sort_by(|(a, _), (b, _)| Ord::cmp(a, b));
        for (k, v) in entries {
          map.serialize_entry(k, v)?;
        }
        map.end()
      }
    }
  }
}

impl <A, B> Serialize for Either<A, B>
  where A: Debug + Clone + PartialEq + Serialize,
        B: Debug + Clone + PartialEq + Serialize {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer
  {
    match self {
      Either::First(a) => a.serialize(serializer),
      Either::Second(b) => b.serialize(serializer)
    }
  }
}

#[cfg(test)]
mod tests {
  use expectest::prelude::*;
  use maplit::hashmap;
  use pretty_assertions::assert_eq;
  use serde_json::json;
  use trim_margin::MarginTrimmable;

  use crate::extensions::AnyValue;

  #[test]
  fn serialize_any_to_json() {
    let json = serde_json::to_string(&AnyValue::Null).unwrap();
    expect!(json).to(be_equal_to("null"));

    let json = serde_json::to_string(&AnyValue::Boolean(true)).unwrap();
    expect!(json).to(be_equal_to("true"));

    let json = serde_json::to_string(&AnyValue::Integer(-100)).unwrap();
    expect!(json).to(be_equal_to("-100"));

    let json = serde_json::to_string(&AnyValue::UInteger(100)).unwrap();
    expect!(json).to(be_equal_to("100"));

    let json = serde_json::to_string(&AnyValue::Float(1.234)).unwrap();
    expect!(json).to(be_equal_to("1.234"));

    let json = serde_json::to_string(&AnyValue::String("I'm a String!".to_string())).unwrap();
    expect!(json).to(be_equal_to("\"I'm a String!\""));

    let value = AnyValue::Array(vec![
      AnyValue::Null,
      AnyValue::UInteger(100),
      AnyValue::Array(vec![
        AnyValue::Integer(-1),
        AnyValue::Integer(0),
        AnyValue::Integer(1),
      ])
    ]);
    let json = serde_json::to_string(&value).unwrap();
    expect!(json).to(be_equal_to("[null,100,[-1,0,1]]"));

    let value = AnyValue::Object(hashmap!{
      "a".to_string() => AnyValue::Null,
      "b".to_string() => AnyValue::UInteger(100),
      "c".to_string() => AnyValue::Object(hashmap!{
        "-1".to_string() => AnyValue::String("A".to_string()),
        "0".to_string() => AnyValue::String("B".to_string()),
        "1".to_string() => AnyValue::String("C".to_string())
      }),
    });
    let json = serde_json::to_string(&value).unwrap();
    expect!(json).to(be_equal_to(json!({
      "a": null,
      "b": 100,
      "c": {
        "-1": "A",
        "0": "B",
        "1": "C"
      }
    }).to_string()));
  }

  #[test]
  fn serialize_any_to_yaml() {
    let yaml = serde_yaml::to_string(&AnyValue::Null).unwrap();
    expect!(yaml).to(be_equal_to("null\n"));

    let yaml = serde_yaml::to_string(&AnyValue::Boolean(true)).unwrap();
    expect!(yaml).to(be_equal_to("true\n"));

    let yaml = serde_yaml::to_string(&AnyValue::Integer(-100)).unwrap();
    expect!(yaml).to(be_equal_to("-100\n"));

    let yaml = serde_yaml::to_string(&AnyValue::UInteger(100)).unwrap();
    expect!(yaml).to(be_equal_to("100\n"));

    let yaml = serde_yaml::to_string(&AnyValue::Float(1.234)).unwrap();
    expect!(yaml).to(be_equal_to("1.234\n"));

    let yaml = serde_yaml::to_string(&AnyValue::String("I'm a String!".to_string())).unwrap();
    expect!(yaml).to(be_equal_to("I'm a String!\n"));

    let value = AnyValue::Array(vec![
      AnyValue::Null,
      AnyValue::UInteger(100),
      AnyValue::Array(vec![
        AnyValue::Integer(-1),
        AnyValue::Integer(0),
        AnyValue::Integer(1),
      ])
    ]);
    let yaml = serde_yaml::to_string(&value).unwrap();
    assert_eq!(
      r#"|- null
         |- 100
         |- - -1
         |  - 0
         |  - 1
         |"#.trim_margin().as_ref().unwrap(), yaml.as_str());

    let value = AnyValue::Object(hashmap!{
      "a".to_string() => AnyValue::Null,
      "b".to_string() => AnyValue::UInteger(100),
      "c".to_string() => AnyValue::Object(hashmap!{
        "-1".to_string() => AnyValue::String("A".to_string()),
        "0".to_string() => AnyValue::String("B".to_string()),
        "1".to_string() => AnyValue::String("C".to_string())
      }),
    });
    let yaml = serde_yaml::to_string(&value).unwrap();
    assert_eq!(
      r#"|a: null
         |b: 100
         |c:
         |  '-1': A
         |  '0': B
         |  '1': C
         |"#.trim_margin().as_ref().unwrap(), yaml.as_str());
  }
}

pub mod v1_0 {
  //! Implementations to support serialization of the 1.0.x models using serde

  use serde::ser::SerializeMap;
  use serde::{Serialize, Serializer};

  use crate::either::Either;
  use crate::v1_0::{
    Criterion,
    ParameterObject,
    PayloadReplacement,
    RequestBody,
    Step,
    Workflow
  };

  impl Serialize for Workflow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer
    {
      todo!()
    }
  }

  impl Serialize for Step {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer
    {
      let extensions_len = self.extensions.len();
      let operation_id_len = if self.operation_id.is_some() { 1 } else { 0 };
      let operation_path_len = if self.operation_path.is_some() { 1 } else { 0 };
      let workflow_id_len = if self.workflow_id.is_some() { 1 } else { 0 };
      let description_len = if self.description.is_some() { 1 } else { 0 };
      let parameters_len = if self.parameters.is_empty() { 0 } else { 1 };
      let request_body_len = if self.request_body.is_some() { 1 } else { 0 };
      let success_criteria_len = if self.success_criteria.is_empty() { 0 } else { 1 };
      let on_success_len = if self.on_success.is_empty() { 0 } else { 1 };
      let on_failure_len = if self.on_failure.is_empty() { 0 } else { 1 };
      let outputs_len = if self.parameters.is_empty() { 0 } else { 1 };

      let mut map = serializer.serialize_map(Some(1 + extensions_len +
        operation_id_len + operation_path_len + workflow_id_len + description_len + parameters_len +
        request_body_len + success_criteria_len + on_success_len + on_failure_len + outputs_len))?;

      if let Some(value) = &self.description {
        map.serialize_entry("description", value)?;
      }

      if !self.on_failure.is_empty() {
        map.serialize_entry("onFailure", &self.on_failure)?;
      }

      if !self.on_success.is_empty() {
        map.serialize_entry("onSuccess", &self.on_success)?;
      }

      if let Some(value) = &self.operation_id {
        map.serialize_entry("operationId", value)?;
      }

      if let Some(value) = &self.operation_path {
        map.serialize_entry("operationPath", value)?;
      }

      if !self.outputs.is_empty() {
        map.serialize_entry("outputs", &self.outputs)?;
      }

      if !self.parameters.is_empty() {
        map.serialize_entry("parameters", &self.parameters)?;
      }

      if let Some(value) = &self.request_body {
        map.serialize_entry("requestBody", value)?;
      }

      map.serialize_entry("stepId", &self.step_id)?;

      if !self.success_criteria.is_empty() {
        map.serialize_entry("successCriteria", &self.success_criteria)?;
      }

      if let Some(value) = &self.workflow_id {
        map.serialize_entry("workflowId", value)?;
      }

      let mut extensions = self.extensions.iter().collect::<Vec<_>>();
      extensions.sort_by(|(a, _), (b, _)| Ord::cmp(a, b));
      for (k, v) in extensions {
        map.serialize_entry(k, v)?;
      }

      map.end()
    }
  }

  impl Serialize for ParameterObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer
    {
      let extensions_len = self.extensions.len();
      let in_len = if self.r#in.is_some() { 1 } else { 0 };

      let mut map = serializer.serialize_map(Some(2 + extensions_len +
        in_len))?;

      if let Some(value) = &self.r#in {
        map.serialize_entry("in", value)?;
      }
      map.serialize_entry("name", &self.name)?;
      match &self.value {
        Either::First(any) => map.serialize_entry("value", any)?,
        Either::Second(exp) => map.serialize_entry("value", exp)?
      }

      let mut extensions = self.extensions.iter().collect::<Vec<_>>();
      extensions.sort_by(|(a, _), (b, _)| Ord::cmp(a, b));
      for (k, v) in extensions {
        map.serialize_entry(k, v)?;
      }

      map.end()
    }
  }

  impl Serialize for Criterion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer
    {
      let extensions_len = self.extensions.len();
      let context_len = if self.context.is_some() { 1 } else { 0 };
      let type_len = if self.r#type.is_some() { 1 } else { 0 };

      let mut map = serializer.serialize_map(Some(1 + extensions_len +
        context_len + type_len))?;

      map.serialize_entry("condition", &self.condition)?;
      if let Some(context) = &self.context {
        map.serialize_entry("context", context)?;
      }
      if let Some(condition_type) = &self.r#type {
        match condition_type {
          Either::First(str) => map.serialize_entry("type", str)?,
          Either::Second(cet) => map.serialize_entry("type", cet)?
        }
      }

      let mut extensions = self.extensions.iter().collect::<Vec<_>>();
      extensions.sort_by(|(a, _), (b, _)| Ord::cmp(a, b));
      for (k, v) in extensions {
        map.serialize_entry(k, v)?;
      }

      map.end()
    }
  }

  impl Serialize for RequestBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer
    {
      let extensions_len = self.extensions.len();
      let content_type_len = if self.content_type.is_some() { 1 } else { 0 };
      let payload_len = if self.payload.is_some() { 1 } else { 0 };
      let replacements_len = if self.replacements.is_empty() { 0 } else { 1 };

      let mut map = serializer.serialize_map(Some(extensions_len +
        content_type_len + payload_len + replacements_len))?;

      if let Some(content_type) = &self.content_type {
        map.serialize_entry("contentType", content_type)?;
      }
      if let Some(payload) = &self.payload {
        map.serialize_entry("payload", payload.as_ref())?;
      }
      if !self.replacements.is_empty() {
        map.serialize_entry("replacements", &self.replacements)?;
      }

      let mut extensions = self.extensions.iter().collect::<Vec<_>>();
      extensions.sort_by(|(a, _), (b, _)| Ord::cmp(a, b));
      for (k, v) in extensions {
        map.serialize_entry(k, v)?;
      }

      map.end()
    }
  }

  impl Serialize for PayloadReplacement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer
    {
      let extensions_len = self.extensions.len();

      let mut map = serializer.serialize_map(Some(extensions_len + 2))?;

      map.serialize_entry("target", &self.target)?;
      match &self.value {
        Either::First(any) => map.serialize_entry("value", any)?,
        Either::Second(exp) => map.serialize_entry("value", exp)?
      }

      let mut extensions = self.extensions.iter().collect::<Vec<_>>();
      extensions.sort_by(|(a, _), (b, _)| Ord::cmp(a, b));
      for (k, v) in extensions {
        map.serialize_entry(k, v)?;
      }

      map.end()
    }
  }

  #[cfg(test)]
  mod tests {
    use std::rc::Rc;

    use expectest::prelude::*;
    use maplit::{btreemap, hashmap};
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use trim_margin::MarginTrimmable;

    use crate::either::Either;
    use crate::extensions::AnyValue;
    use crate::payloads::{JsonPayload, StringPayload};
    use crate::v1_0::{
      Criterion,
      CriterionExpressionType,
      ParameterObject,
      PayloadReplacement,
      RequestBody,
      Step
    };

    #[test]
    fn request_body() {
      let body = RequestBody {
        content_type: None,
        payload: None,
        replacements: vec![],
        extensions: Default::default()
      };
      let json = serde_json::to_string(&body).unwrap();
      expect!(json).to(be_equal_to(json!({}).to_string()));
      let yaml = serde_yaml::to_string(&body).unwrap();
      assert_eq!(
        r#"|{}
           |"#.trim_margin().as_ref().unwrap(), yaml.as_str());

      let body = RequestBody {
        content_type: Some("application/json".to_string()),
        payload: Some(Rc::new(StringPayload(r#"
        {
          "petOrder": {
            "petId": "{$inputs.pet_id}",
            "couponCode": "{$inputs.coupon_code}",
            "quantity": "{$inputs.quantity}",
            "status": "placed",
            "complete": false
          }
        }
        "#.to_string()))),
        replacements: vec![],
        extensions: hashmap!{
          "x-one".to_string() => AnyValue::String("one".to_string()),
          "x-two".to_string() => AnyValue::Integer(2),
        }
      };
      let json = serde_json::to_string(&body).unwrap();
      expect!(json).to(be_equal_to(json!({
        "contentType": "application/json",
        "payload": "\n        {\n          \"petOrder\": {\n            \"petId\": \"{$inputs.pet_id}\",\n            \"couponCode\": \"{$inputs.coupon_code}\",\n            \"quantity\": \"{$inputs.quantity}\",\n            \"status\": \"placed\",\n            \"complete\": false\n          }\n        }\n        ",
        "x-one": "one",
        "x-two": 2
      }).to_string()));
      let yaml = serde_yaml::to_string(&body).unwrap();
      assert_eq!(
        r#"|contentType: application/json
           |payload: "\n        {\n          \"petOrder\": {\n            \"petId\": \"{$inputs.pet_id}\",\n            \"couponCode\": \"{$inputs.coupon_code}\",\n            \"quantity\": \"{$inputs.quantity}\",\n            \"status\": \"placed\",\n            \"complete\": false\n          }\n        }\n        "
           |x-one: one
           |x-two: 2
           |"#.trim_margin().as_ref().unwrap(), yaml.as_str());

      let body = RequestBody {
        content_type: Some("application/json".to_string()),
        payload: Some(Rc::new(JsonPayload(json!({
          "petOrder": {
            "petId": "{$inputs.pet_id}",
            "couponCode": "{$inputs.coupon_code}",
            "quantity": "{$inputs.quantity}",
            "status": "placed",
            "complete": false
          }
        })))),
        replacements: vec![],
        extensions: hashmap!{}
      };
      let json = serde_json::to_string(&body).unwrap();
      expect!(json).to(be_equal_to(json!({
        "contentType": "application/json",
        "payload": {
          "petOrder": {
            "petId": "{$inputs.pet_id}",
            "couponCode": "{$inputs.coupon_code}",
            "quantity": "{$inputs.quantity}",
            "status": "placed",
            "complete": false
          }
        }
      }).to_string()));
      let yaml = serde_yaml::to_string(&body).unwrap();
      assert_eq!(
        r#"|contentType: application/json
           |payload:
           |  petOrder:
           |    complete: false
           |    couponCode: '{$inputs.coupon_code}'
           |    petId: '{$inputs.pet_id}'
           |    quantity: '{$inputs.quantity}'
           |    status: placed
           |"#.trim_margin().as_ref().unwrap(), yaml.as_str());
    }

    #[test]
    fn payload_replacement() {
      let payload_replacement = PayloadReplacement {
        target: "/petId".to_string(),
        value: Either::Second("$inputs.pet_id".to_string()),
        extensions: Default::default()
      };
      let json = serde_json::to_string(&payload_replacement).unwrap();
      expect!(json).to(be_equal_to(json!({
        "target": "/petId",
        "value": "$inputs.pet_id"
      }).to_string()));
      let yaml = serde_yaml::to_string(&payload_replacement).unwrap();
      assert_eq!(
        r#"|target: /petId
           |value: $inputs.pet_id
           |"#.trim_margin().as_ref().unwrap(), yaml.as_str());

      let payload_replacement = PayloadReplacement {
        target: "/quantity".to_string(),
        value: Either::First(AnyValue::Integer(10)),
        extensions: Default::default()
      };
      let json = serde_json::to_string(&payload_replacement).unwrap();
      expect!(json).to(be_equal_to(json!({
        "target": "/quantity",
        "value": 10
      }).to_string()));
      let yaml = serde_yaml::to_string(&payload_replacement).unwrap();
      assert_eq!(
        r#"|target: /quantity
           |value: 10
           |"#.trim_margin().as_ref().unwrap(), yaml.as_str());

      let payload_replacement = PayloadReplacement {
        target: "/petId".to_string(),
        value: Either::Second("$inputs.pet_id".to_string()),
        extensions: hashmap!{
          "x-one".to_string() => AnyValue::String("one".to_string()),
          "x-two".to_string() => AnyValue::Integer(2),
        }
      };
      let json = serde_json::to_string(&payload_replacement).unwrap();
      expect!(json).to(be_equal_to(json!({
        "target": "/petId",
        "value": "$inputs.pet_id",
        "x-one": "one",
        "x-two": 2
      }).to_string()));
      let yaml = serde_yaml::to_string(&payload_replacement).unwrap();
      assert_eq!(
        r#"|target: /petId
           |value: $inputs.pet_id
           |x-one: one
           |x-two: 2
           |"#.trim_margin().as_ref().unwrap(), yaml.as_str());
    }

    #[test]
    fn criterion() {
      let criterion = Criterion {
        context: None,
        condition: "$statusCode == 200".to_string(),
        r#type: None,
        extensions: Default::default()
      };
      let json = serde_json::to_string(&criterion).unwrap();
      expect!(json).to(be_equal_to(json!({
        "condition": "$statusCode == 200"
      }).to_string()));
      let yaml = serde_yaml::to_string(&criterion).unwrap();
      assert_eq!(
        r#"|condition: $statusCode == 200
           |"#.trim_margin().as_ref().unwrap(), yaml.as_str());

      let criterion = Criterion {
        context: Some("$statusCode".to_string()),
        condition: "^200$".to_string(),
        r#type: Some(Either::First("regex".to_string())),
        extensions: hashmap!{
          "x-one".to_string() => AnyValue::String("one".to_string()),
          "x-two".to_string() => AnyValue::Integer(2),
        }
      };
      let json = serde_json::to_string(&criterion).unwrap();
      expect!(json).to(be_equal_to(json!({
        "condition": "^200$",
        "context": "$statusCode",
        "type": "regex",
        "x-one": "one",
        "x-two": 2
      }).to_string()));
      let yaml = serde_yaml::to_string(&criterion).unwrap();
      assert_eq!(
        r#"|condition: ^200$
           |context: $statusCode
           |type: regex
           |x-one: one
           |x-two: 2
           |"#.trim_margin().as_ref().unwrap(), yaml.as_str());

      let criterion = Criterion {
        context: Some("$response.body".to_string()),
        condition: "$[?count(@.pets) > 0]".to_string(),
        r#type: Some(Either::Second(CriterionExpressionType {
          r#type: "jsonpath".to_string(),
          version: "draft-goessner-dispatch-jsonpath-00".to_string(),
          extensions: Default::default()
        })),
        extensions: Default::default()
      };
      let json = serde_json::to_string(&criterion).unwrap();
      expect!(json).to(be_equal_to(json!({
        "condition": "$[?count(@.pets) > 0]",
        "context": "$response.body",
        "type": {
          "type": "jsonpath",
          "version": "draft-goessner-dispatch-jsonpath-00"
        }
      }).to_string()));
      let yaml = serde_yaml::to_string(&criterion).unwrap();
      assert_eq!(
        r#"|condition: $[?count(@.pets) > 0]
           |context: $response.body
           |type:
           |  type: jsonpath
           |  version: draft-goessner-dispatch-jsonpath-00
           |"#.trim_margin().as_ref().unwrap(), yaml.as_str());
    }

    #[test]
    fn parameter_object() {
      let parameter = ParameterObject {
        name: "username".to_string(),
        r#in: Some("query".to_string()),
        value: Either::Second("$inputs.username".to_string()),
        extensions: Default::default()
      };
      let json = serde_json::to_string(&parameter).unwrap();
      expect!(json).to(be_equal_to(json!({
        "name": "username",
        "in": "query",
        "value": "$inputs.username"
      }).to_string()));
      let yaml = serde_yaml::to_string(&parameter).unwrap();
      assert_eq!(
        r#"|in: query
           |name: username
           |value: $inputs.username
           |"#.trim_margin().as_ref().unwrap(), yaml.as_str());

      let parameter = ParameterObject {
        name: "username".to_string(),
        r#in: None,
        value: Either::Second("$inputs.username".to_string()),
        extensions: hashmap!{
          "x-one".to_string() => AnyValue::String("one".to_string()),
          "x-two".to_string() => AnyValue::Integer(2),
        }
      };
      let json = serde_json::to_string(&parameter).unwrap();
      expect!(json).to(be_equal_to(json!({
        "name": "username",
        "value": "$inputs.username",
        "x-one": "one",
        "x-two": 2
      }).to_string()));
      let yaml = serde_yaml::to_string(&parameter).unwrap();
      assert_eq!(
        r#"|name: username
           |value: $inputs.username
           |x-one: one
           |x-two: 2
           |"#.trim_margin().as_ref().unwrap(), yaml.as_str());

      let parameter = ParameterObject {
        name: "username".to_string(),
        r#in: None,
        value: Either::First(AnyValue::Integer(1000)),
        extensions: hashmap!{}
      };
      let json = serde_json::to_string(&parameter).unwrap();
      expect!(json).to(be_equal_to(json!({
        "name": "username",
        "value": 1000
      }).to_string()));
      let yaml = serde_yaml::to_string(&parameter).unwrap();
      assert_eq!(
        r#"|name: username
           |value: 1000
           |"#.trim_margin().as_ref().unwrap(), yaml.as_str());
    }

    #[test]
    fn step_object() {
      let step = Step {
        step_id: "loginStep".to_string(),
        operation_id: Some("loginUser".to_string()),
        operation_path: None,
        workflow_id: None,
        description: Some("This step demonstrates the user login step".to_string()),
        parameters: vec![
          Either::First(ParameterObject {
            name: "username".to_string(),
            r#in: Some("query".to_string()),
            value: Either::Second("$inputs.username".to_string()),
            extensions: Default::default()
          }),
          Either::First(ParameterObject {
            name: "password".to_string(),
            r#in: Some("query".to_string()),
            value: Either::Second("$inputs.password".to_string()),
            extensions: Default::default()
          })
        ],
        request_body: None,
        success_criteria: vec![
          Criterion {
            context: None,
            condition: "$statusCode == 200".to_string(),
            r#type: None,
            extensions: Default::default(),
          }
        ],
        on_success: vec![],
        on_failure: vec![],
        outputs: btreemap!{
          "tokenExpires".to_string() => "$response.header.X-Expires-After".to_string(),
          "rateLimit".to_string() => "$response.header.X-Rate-Limit".to_string()
        },
        extensions: Default::default()
      };
      let json = serde_json::to_string(&step).unwrap();
      assert_eq!(json!({
        "stepId": "loginStep",
        "description": "This step demonstrates the user login step",
        "operationId": "loginUser",
        "parameters": [
          {
            "name": "username",
            "in": "query",
            "value": "$inputs.username"
          }, {
            "name": "password",
            "in": "query",
            "value": "$inputs.password"
          }
        ],
        "successCriteria": [
          {
            "condition": "$statusCode == 200"
          }
        ],
        "outputs": {
          "tokenExpires": "$response.header.X-Expires-After",
          "rateLimit": "$response.header.X-Rate-Limit"
        }
      }).to_string(), json);
      let yaml = serde_yaml::to_string(&step).unwrap();
      assert_eq!(
        r#"|description: This step demonstrates the user login step
           |operationId: loginUser
           |outputs:
           |  rateLimit: $response.header.X-Rate-Limit
           |  tokenExpires: $response.header.X-Expires-After
           |parameters:
           |- in: query
           |  name: username
           |  value: $inputs.username
           |- in: query
           |  name: password
           |  value: $inputs.password
           |stepId: loginStep
           |successCriteria:
           |- condition: $statusCode == 200
           |"#.trim_margin().as_ref().unwrap(), yaml.as_str());

      let step = Step {
        step_id: "test-extensions".to_string(),
        operation_id: None,
        operation_path: None,
        workflow_id: None,
        description: None,
        parameters: vec![],
        request_body: None,
        success_criteria: vec![],
        on_success: vec![],
        on_failure: vec![],
        outputs: Default::default(),
        extensions: hashmap!{
          "x-one".to_string() => AnyValue::String("one".to_string()),
          "x-two".to_string() => AnyValue::Integer(2),
        }
      };
      let json = serde_json::to_string(&step).unwrap();
      expect!(json).to(be_equal_to(json!({
        "stepId": "test-extensions",
        "x-one": "one",
        "x-two": 2
      }).to_string()));
      let yaml = serde_yaml::to_string(&step).unwrap();
      assert_eq!(
        r#"|stepId: test-extensions
           |x-one: one
           |x-two: 2
           |"#.trim_margin().as_ref().unwrap(), yaml.as_str());
    }
  }
}
