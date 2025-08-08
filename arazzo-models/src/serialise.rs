//! Implementations to support serialization of the models using serde

use itertools::Itertools;
use serde::{Serialize, Serializer};
use serde::ser::{SerializeMap, SerializeSeq};

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
        for (k, v) in o.iter()
          .sorted_by(|(a, _), (b, _)| Ord::cmp(a, b)) {
          map.serialize_entry(k, v)?;
        }
        map.end()
      }
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

  use std::collections::HashMap;
  use std::rc::Rc;
  use itertools::{Either, Itertools};
  use serde::{Serialize, Serializer};
  use serde::ser::{SerializeMap, SerializeStruct};
  use crate::extensions::AnyValue;
  use crate::payloads::Payload;
  use crate::v1_0::{Components, Criterion, PayloadReplacement, RequestBody, Step, Workflow};

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
      todo!()
    }
  }

  impl Serialize for Components {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer
    {
      todo!()
    }
  }

  impl Serialize for Criterion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer
    {
      todo!()
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

      for (k, v) in self.extensions.iter()
        .sorted_by(|(a, _), (b, _)| Ord::cmp(a, b)) {
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
        Either::Left(any) => map.serialize_entry("value", any)?,
        Either::Right(exp) => map.serialize_entry("value", exp)?
      }

      for (k, v) in self.extensions.iter()
        .sorted_by(|(a, _), (b, _)| Ord::cmp(a, b)) {
        map.serialize_entry(k, v)?;
      }

      map.end()
    }
  }

  #[cfg(test)]
  mod tests {
    use std::rc::Rc;

    use expectest::prelude::*;
    use itertools::Either;
    use maplit::hashmap;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use trim_margin::MarginTrimmable;

    use crate::extensions::AnyValue;
    use crate::payloads::StringPayload;
    use crate::v1_0::{PayloadReplacement, RequestBody};

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
    }

    #[test]
    fn payload_replacement() {
      let payload_replacement = PayloadReplacement {
        target: "/petId".to_string(),
        value: Either::Right("$inputs.pet_id".to_string()),
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
        value: Either::Left(AnyValue::Integer(10)),
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
        value: Either::Right("$inputs.pet_id".to_string()),
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
  }
}
