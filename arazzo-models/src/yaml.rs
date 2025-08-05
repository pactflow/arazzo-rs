//! Functions and Traits for loading Arazzo objects from a YAML document

use anyhow::anyhow;
use serde_json::{json, Map, Value};
use yaml_rust2::Yaml;
use yaml_rust2::yaml::Hash;

/// Returns the type name of the YAML value
pub fn yaml_type_name(yaml: &Yaml) -> String {
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
pub fn yaml_hash_lookup_string(hash: &Hash, key: &str) -> Option<String> {
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

/// Looks up a numeric value with the given String key in a YAML Hash. If the value is an integer
/// it will be converted to a double.
pub fn yaml_hash_lookup_number(hash: &Hash, key: &str) -> Option<f64> {
  if let Some(value) = hash.get(&Yaml::String(key.to_string())) {
    match value {
      Yaml::Real(f) => f.parse::<f64>().ok(),
      Yaml::Integer(i) => Some(*i as f64),
      _ => None
    }
  } else {
    None
  }
}

/// Looks up an integer value with the given String key in a YAML Hash. If the value is a float
/// it will be converted to an integer.
pub fn yaml_hash_lookup_integer(hash: &Hash, key: &str) -> Option<i64> {
  if let Some(value) = hash.get(&Yaml::String(key.to_string())) {
    match value {
      Yaml::Real(f) => f.parse::<f64>().ok().map(|f| f as i64),
      Yaml::Integer(i) => Some(*i),
      _ => None
    }
  } else {
    None
  }
}

/// Looks up a required String value with the given String key in a YAML Hash. If the key does
/// not exist, or the resulting value is not a String, an Error is returned.
pub fn yaml_hash_require_string(hash: &Hash, key: &str) -> anyhow::Result<String> {
  if let Some(value) = hash.get(&Yaml::String(key.to_string())) {
    if let Some(value) = value.as_str() {
      Ok(value.to_string())
    } else {
      Err(anyhow!("Value for key '{}' in hash was not a string, was {}", key, yaml_type_name(value)))
    }
  } else {
    Err(anyhow!("Did not find key '{}' in hash", key))
  }
}

/// Looks up a String key in the given hash, calling the provided callback if it is found.
pub fn yaml_hash_lookup<F, U>(
  hash: &Hash,
  key: &str,
  callback: F
) -> Option<U> where F: FnOnce(&Yaml) -> Option<U> {
  if let Some(value) = hash.get(&Yaml::String(key.to_string())) {
    callback(value)
  } else {
    None
  }
}

/// Looks up an Array of String values with the given String key in a YAML Hash. If each value
/// is easily convertable to a String (is a Number or Boolean), `to_string()` will be called on it.
/// All other values are ignored.
pub fn yaml_hash_lookup_string_list(hash: &Hash, key: &str) -> Option<Vec<String>> {
  if let Some(value) = hash.get(&Yaml::String(key.to_string())) {
    if let Some(array) = value.as_vec() {
      Some(array.iter().flat_map(|value| {
        match value {
          Yaml::Real(s) => Some(s.clone()),
          Yaml::Integer(i) => Some(i.to_string()),
          Yaml::String(s) => Some(s.clone()),
          Yaml::Boolean(b) => Some(b.to_string()),
          _ => None
        }
      }).collect())
    } else {
      None
    }
  } else {
    None
  }
}

/// Looks up the entry in the hash and converts it to JSON. If there is no entry with that key,
/// JSON Null is returned.
pub fn yaml_hash_entry_to_json(hash: &Hash, key: &str) -> anyhow::Result<Value> {
  if let Some(value) = hash.get(&Yaml::String(key.to_string())) {
    yaml_to_json(value)
  } else {
    Ok(Value::Null)
  }
}

/// Converts the Yaml value to the equivalent JSON value
pub fn yaml_to_json(yaml: &Yaml) -> anyhow::Result<Value> {
  match yaml {
    Yaml::Null => Ok(Value::Null),
    Yaml::Boolean(b) => Ok(Value::Bool(*b)),
    Yaml::Integer(i) => Ok(json!(*i)),
    Yaml::Real(f) => f.parse::<f64>()
      .map(|f| json!(f))
      .map_err(|err| anyhow!(err)),
    Yaml::String(s) => Ok(Value::String(s.clone())),
    Yaml::Array(a) => {
      let mut array = vec![];

      for value in a {
        array.push(yaml_to_json(value)?);
      }

      Ok(Value::Array(array))
    }
    Yaml::Hash(hash) => {
      let mut map = Map::new();

      for (k, v) in hash {
        let key = k.as_str()
          .ok_or_else(|| {
            anyhow!("Only String values can be used for JSON keys. Got '{}'", yaml_type_name(k))
          })?;
        map.insert(key.to_string(), yaml_to_json(v)?);
      }

      Ok(Value::Object(map))
    }
    _ => Err(anyhow!("YAML '{}' value can not be converted to JSON", yaml_type_name(yaml)))
  }
}

#[cfg(test)]
mod tests {
  use expectest::prelude::*;
  use serde_json::{json, Value};
  use yaml_rust2::Yaml;

  use crate::yaml::yaml_to_json;

  #[test]
  fn yaml_to_json_test() {
    expect!(yaml_to_json(&Yaml::Null)).to(be_ok().value(Value::Null));
    expect!(yaml_to_json(&Yaml::Boolean(true))).to(be_ok().value(Value::Bool(true)));
    expect!(yaml_to_json(&Yaml::Integer(100))).to(be_ok().value(json!(100)));
    expect!(yaml_to_json(&Yaml::Real("123.45".to_string()))).to(be_ok()
      .value(json!(123.45)));
    expect!(yaml_to_json(&Yaml::String("123.45".to_string()))).to(be_ok()
      .value(Value::String("123.45".to_string())));

    let array = Yaml::Array(vec![
      Yaml::Null,
      Yaml::Boolean(false),
      Yaml::Integer(100)
    ]);
    expect!(yaml_to_json(&array)).to(be_ok().value(json!([ null, false, 100 ])));
  }
}
