//! Functions and Traits for loading Arazzo objects from a YAML document

use anyhow::anyhow;
use yaml_rust2::Yaml;
use yaml_rust2::yaml::Hash;

use crate::ArazzoDescription;

impl TryFrom<&Yaml> for ArazzoDescription {
  type Error = anyhow::Error;

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    if let Some(hash) = value.as_hash() {
      if let Ok(version) = hash_require_string(hash, "arazzo") {
        Ok(ArazzoDescription {
          arazzo: version
        })
      } else {
        Err(anyhow!("Arazzo version number is required [4.6.1.1 Fixed Fields]"))
      }
    } else {
      Err(anyhow!("YAML document must be a Hash, got {}", type_name(value)))
    }
  }
}

/// Returns the type name of the YAML value
pub fn type_name(yaml: &Yaml) -> String {
  match yaml {
    Yaml::Real(_) => "Real",
    Yaml::Integer(_) => "Integer",
    Yaml::String(_) => "String",
    Yaml::Boolean(_) => "Boolean",
    Yaml::Array(_) => "Array",
    Yaml::Hash(_) => "Hash",
    Yaml::Alias(_) => "Alias",
    Yaml::Null => "Null",
    Yaml::BadValue => "BadValue"
  }.to_string()
}

/// Looks up a String value with the given String key in a YAML Hash. If the value is easily
/// convertable to a String (is a Number or Boolean), `to_string()` will be called on it.
pub fn hash_lookup_string(hash: &Hash, key: &str) -> Option<String> {
  if let Some(value) = hash.get(&Yaml::String(key.to_string())) {
    match value {
      Yaml::Real(s) => Some(s.clone()),
      Yaml::Integer(i) => Some(i.to_string()),
      Yaml::String(s) => Some(s.clone()),
      Yaml::Boolean(b) => Some(b.to_string()),
      _ => None
    }
  } else {
    None
  }
}

/// Looks up a required String value with the given String key in a YAML Hash. If the key does
/// not exist, or the resulting value is not a String, an Error is returned.
pub fn hash_require_string(hash: &Hash, key: &str) -> anyhow::Result<String> {
  if let Some(value) = hash.get(&Yaml::String(key.to_string())) {
    if let Some(value) = value.as_str() {
      Ok(value.to_string())
    } else {
      Err(anyhow!("Value for key '{}' in hash was not a string, was {}", key, type_name(value)))
    }
  } else {
    Err(anyhow!("Did not find key '{}' in hash", key))
  }
}
