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
  use itertools::Itertools;
  use serde::{Serialize, Serializer};
  use serde::ser::SerializeStruct;
  use crate::extensions::AnyValue;
  use crate::payloads::Payload;
  use crate::v1_0::{Components, Criterion, RequestBody, Step, Workflow};

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
      let extensions = self.extensions.len();
      let mut state = serializer.serialize_struct("RequestBody", 2 + extensions)?;

      state.serialize_field("contentType", &self.content_type)?;

      if let Some(payload) = &self.payload {
        state.serialize_field("payload", &payload.as_string())?;
      } else {
        state.skip_field("payload")?;
      }

      for (k, v) in self.extensions.iter()
        .sorted_by(|(a, _), (b, _)| Ord::cmp(a, b)) {
        state.serialize_field(k.as_str(), v)?;
      }

      state.end()
    }
  }

  #[cfg(test)]
  mod tests {
    use expectest::prelude::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use trim_margin::MarginTrimmable;

    use crate::v1_0::RequestBody;

    #[test]
    fn request_body() {
      let body = RequestBody {
        content_type: None,
        payload: None,
        extensions: Default::default()
      };
      let json = serde_json::to_string(&body).unwrap();
      expect!(json).to(be_equal_to(json!({
        "a": null,
        "b": 100,
        "c": {
          "-1": "A",
          "0": "B",
          "1": "C"
        }
      }).to_string()));
      let yaml = serde_yaml::to_string(&body).unwrap();
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
}
