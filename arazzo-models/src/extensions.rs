//! Structs and Traits for dealing with extensions (<https://spec.openapis.org/arazzo/v1.0.1.html#specification-extensions>).

use std::collections::HashMap;

#[cfg(feature = "yaml")] use anyhow::anyhow;
#[cfg(feature = "yaml")] use maplit::hashmap;
#[cfg(feature = "json")] use serde_json::{Map, Value};
#[cfg(feature = "yaml")] use yaml_rust2::Yaml;
#[cfg(feature = "yaml")] use yaml_rust2::yaml::Hash;

#[cfg(feature = "yaml")] use crate::yaml::yaml_type_name;

/// Enum to store a value of additional data
#[derive(Clone, Debug, Default, PartialEq)]
pub enum AnyValue {
  /// Empty value
  #[default]
  Null,

  /// Boolean value
  Boolean(bool),

  /// 64-bit signed Integer
  Integer(i64),

  /// 64-bit unsigned Integer
  UInteger(u64),

  /// 64-bit floating point number
  Float(f64),

  /// String
  String(String),

  /// An array of values
  Array(Vec<AnyValue>),

  /// An Object, which is stored as a Map with String keys
  Object(HashMap<String, AnyValue>)
}

#[cfg(feature = "yaml")]
impl TryFrom<&Yaml> for AnyValue {
  type Error = anyhow::Error;

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    match value {
      Yaml::Real(f) => f.parse::<f64>()
        .map(|f| AnyValue::Float(f))
        .map_err(|err| anyhow!(err)),
      Yaml::Integer(i) => Ok(AnyValue::Integer(*i)),
      Yaml::String(s) => Ok(AnyValue::String(s.clone())),
      Yaml::Boolean(b) => Ok(AnyValue::Boolean(*b)),
      Yaml::Array(a) => {
        let mut array = vec![];

        for value in a {
          array.push(value.try_into()?);
        }

        Ok(AnyValue::Array(array))
      }
      Yaml::Hash(h) => {
        let mut map = hashmap!{};

        for (k, value) in h {
          let key = k.as_str()
            .ok_or_else(|| {
              anyhow!("Only String values can be used for extension keys. Got '{}'", yaml_type_name(k))
            })?;
          map.insert(key.to_string(), value.try_into()?);
        }

        Ok(AnyValue::Object(map))
      }
      Yaml::Null => Ok(AnyValue::Null),
      _ => Err(anyhow!("Values of '{}' can not be used as an extension value", yaml_type_name(value)))
    }
  }
}

/// Extracts all the extension values from the Hash, stripping the `x-` suffix off.
#[cfg(feature = "yaml")]
pub fn yaml_extract_extensions(hash: &Hash) -> anyhow::Result<HashMap<String, AnyValue>> {
  let mut extensions = hashmap!{};

  for (k, v) in hash {
    if let Some(key) = k.as_str() && let Some(suffix) = key.strip_prefix("x-") {
      extensions.insert(suffix.to_string(), v.try_into()?);
    }
  }

  Ok(extensions)
}

#[cfg(feature = "json")]
impl TryFrom<&Value> for AnyValue {
  type Error = anyhow::Error;

  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    match value {
      Value::Null => Ok(AnyValue::Null),
      Value::Bool(b) => Ok(AnyValue::Boolean(*b)),
      Value::Number(n) => {
        if let Some(uint) = n.as_u64() {
          Ok(AnyValue::UInteger(uint))
        } else if let Some(int) = n.as_i64() {
          Ok(AnyValue::Integer(int))
        } else {
          Ok(AnyValue::Float(n.as_f64().unwrap_or_default()))
        }
      }
      Value::String(s) => Ok(AnyValue::String(s.clone())),
      Value::Array(a) => {
        let mut array = vec![];

        for value in a {
          array.push(value.try_into()?);
        }

        Ok(AnyValue::Array(array))
      }
      Value::Object(o) => {
        let mut map = hashmap!{};

        for (k, value) in o {
          map.insert(k.clone(), value.try_into()?);
        }

        Ok(AnyValue::Object(map))
      }
    }
  }
}

/// Extracts all the extension values from the Object, stripping the `x-` suffix off.
#[cfg(feature = "json")]
pub fn json_extract_extensions(map: &Map<String, Value>) -> anyhow::Result<HashMap<String, AnyValue>> {
  let mut extensions = hashmap!{};

  for (k, v) in map {
    if let Some(suffix) = k.strip_prefix("x-") {
      extensions.insert(suffix.to_string(), v.try_into()?);
    }
  }

  Ok(extensions)
}

#[cfg(test)]
mod tests {
  use expectest::prelude::*;
  use maplit::hashmap;
  #[cfg(feature = "yaml")] use yaml_rust2::Yaml;
  #[cfg(feature = "yaml")] use yaml_rust2::yaml::Hash;

  use crate::extensions::AnyValue;

  #[test]
  #[cfg(feature = "yaml")]
  fn create_extension_value_from_primitive_yaml() {
    expect!(AnyValue::try_from(&Yaml::Null))
      .to(be_ok().value(AnyValue::Null));
    expect!(AnyValue::try_from(&Yaml::Boolean(true)))
      .to(be_ok().value(AnyValue::Boolean(true)));
    expect!(AnyValue::try_from(&Yaml::String("test".to_string())))
      .to(be_ok().value(AnyValue::String("test".to_string())));
    expect!(AnyValue::try_from(&Yaml::Integer(1234)))
      .to(be_ok().value(AnyValue::Integer(1234)));
    expect!(AnyValue::try_from(&Yaml::Real("1234.56".to_string())))
      .to(be_ok().value(AnyValue::Float(1234.56)));
  }

  #[test]
  #[cfg(feature = "yaml")]
  fn create_extension_value_from_array() {
    let array = Yaml::Array(vec![]);
    expect!(AnyValue::try_from(&array))
      .to(be_ok().value(AnyValue::Array(vec![])));

    let array = Yaml::Array(vec![
      Yaml::Null,
      Yaml::Boolean(false),
      Yaml::Integer(100)
    ]);
    expect!(AnyValue::try_from(&array))
      .to(be_ok().value(AnyValue::Array(vec![
        AnyValue::Null,
        AnyValue::Boolean(false),
        AnyValue::Integer(100)
      ])));
  }

  #[test]
  #[cfg(feature = "yaml")]
  fn create_extension_value_from_object() {
    let hash = Hash::new();
    expect!(AnyValue::try_from(&Yaml::Hash(hash)))
      .to(be_ok().value(AnyValue::Object(hashmap!{})));

    let mut hash = Hash::new();
    hash.insert(Yaml::String("a".to_string()), Yaml::Null);
    hash.insert(Yaml::String("b".to_string()), Yaml::Real("123.4".to_string()));
    let array = Yaml::Array(vec![
      Yaml::Null,
      Yaml::Boolean(false),
      Yaml::Integer(100)
    ]);
    hash.insert(Yaml::String("c".to_string()), array);

    expect!(AnyValue::try_from(&Yaml::Hash(hash)))
      .to(be_ok().value(AnyValue::Object(hashmap!{
        "a".to_string() => AnyValue::Null,
        "b".to_string() => AnyValue::Float(123.4),
        "c".to_string() => AnyValue::Array(vec![
          AnyValue::Null,
          AnyValue::Boolean(false),
          AnyValue::Integer(100)
        ])
      })));
  }
}
