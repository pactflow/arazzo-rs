//! Structs and Traits for dealing with extensions (https://spec.openapis.org/arazzo/v1.0.1.html#specification-extensions).

use std::collections::HashMap;

#[cfg(feature = "yaml")] use anyhow::anyhow;
#[cfg(feature = "yaml")] use maplit::hashmap;
#[cfg(feature = "yaml")] use yaml_rust2::Yaml;
#[cfg(feature = "yaml")] use yaml_rust2::yaml::Hash;
#[cfg(feature = "yaml")] use crate::yaml::type_name;

/// Enum to store a value of additional data
#[derive(Clone, Debug, Default, PartialEq)]
pub enum ExtensionValue {
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
  Array(Vec<ExtensionValue>),

  /// An Object, which is stored as a Map with String keys
  Object(HashMap<String, ExtensionValue>)
}

#[cfg(feature = "yaml")]
impl TryFrom<&Yaml> for ExtensionValue {
  type Error = anyhow::Error;

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    match value {
      Yaml::Real(f) => f.parse::<f64>()
        .map(|f| ExtensionValue::Float(f))
        .map_err(|err| anyhow!(err)),
      Yaml::Integer(i) => Ok(ExtensionValue::Integer(*i)),
      Yaml::String(s) => Ok(ExtensionValue::String(s.clone())),
      Yaml::Boolean(b) => Ok(ExtensionValue::Boolean(*b)),
      Yaml::Array(a) => {
        let mut array = vec![];

        for value in a {
          array.push(value.try_into()?);
        }

        Ok(ExtensionValue::Array(array))
      }
      Yaml::Hash(h) => {
        let mut map = hashmap!{};

        for (k, value) in h {
          let key = k.as_str()
            .ok_or_else(|| {
              anyhow!("Only String values can be used for extension keys. Got '{}'", type_name(k))
            })?;
          map.insert(key.to_string(), value.try_into()?);
        }

        Ok(ExtensionValue::Object(map))
      }
      Yaml::Null => Ok(ExtensionValue::Null),
      _ => Err(anyhow!("Values of '{}' can not be used as an extension value", type_name(value)))
    }
  }
}

/// Extracts all the extension values from the Hash, stripping the `x-` suffix off.
#[cfg(feature = "yaml")]
pub fn extract_extensions_from_yaml(hash: &Hash) -> anyhow::Result<HashMap<String, ExtensionValue>> {
  let mut extensions = hashmap!{};

  for (k, v) in hash {
    if let Some(key) = k.as_str() && let Some(suffix) = key.strip_prefix("x-") {
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

  use crate::extensions::ExtensionValue;

  #[test]
  #[cfg(feature = "yaml")]
  fn create_extension_value_from_primitive_yaml() {
    expect!(ExtensionValue::try_from(&Yaml::Null))
      .to(be_ok().value(ExtensionValue::Null));
    expect!(ExtensionValue::try_from(&Yaml::Boolean(true)))
      .to(be_ok().value(ExtensionValue::Boolean(true)));
    expect!(ExtensionValue::try_from(&Yaml::String("test".to_string())))
      .to(be_ok().value(ExtensionValue::String("test".to_string())));
    expect!(ExtensionValue::try_from(&Yaml::Integer(1234)))
      .to(be_ok().value(ExtensionValue::Integer(1234)));
    expect!(ExtensionValue::try_from(&Yaml::Real("1234.56".to_string())))
      .to(be_ok().value(ExtensionValue::Float(1234.56)));
  }

  #[test]
  #[cfg(feature = "yaml")]
  fn create_extension_value_from_array() {
    let array = Yaml::Array(vec![]);
    expect!(ExtensionValue::try_from(&array))
      .to(be_ok().value(ExtensionValue::Array(vec![])));

    let array = Yaml::Array(vec![
      Yaml::Null,
      Yaml::Boolean(false),
      Yaml::Integer(100)
    ]);
    expect!(ExtensionValue::try_from(&array))
      .to(be_ok().value(ExtensionValue::Array(vec![
        ExtensionValue::Null,
        ExtensionValue::Boolean(false),
        ExtensionValue::Integer(100)
      ])));
  }

  #[test]
  #[cfg(feature = "yaml")]
  fn create_extension_value_from_object() {
    let hash = Hash::new();
    expect!(ExtensionValue::try_from(&Yaml::Hash(hash)))
      .to(be_ok().value(ExtensionValue::Object(hashmap!{})));

    let mut hash = Hash::new();
    hash.insert(Yaml::String("a".to_string()), Yaml::Null);
    hash.insert(Yaml::String("b".to_string()), Yaml::Real("123.4".to_string()));
    let array = Yaml::Array(vec![
      Yaml::Null,
      Yaml::Boolean(false),
      Yaml::Integer(100)
    ]);
    hash.insert(Yaml::String("c".to_string()), array);

    expect!(ExtensionValue::try_from(&Yaml::Hash(hash)))
      .to(be_ok().value(ExtensionValue::Object(hashmap!{
        "a".to_string() => ExtensionValue::Null,
        "b".to_string() => ExtensionValue::Float(123.4),
        "c".to_string() => ExtensionValue::Array(vec![
          ExtensionValue::Null,
          ExtensionValue::Boolean(false),
          ExtensionValue::Integer(100)
        ])
      })));
  }
}
