//! Version 1.0.x specification models

use anyhow::anyhow;
#[cfg(feature = "yaml")] use yaml_rust2::Yaml;
use yaml_rust2::yaml::Hash;
#[cfg(feature = "yaml")] use crate::yaml::{hash_require_string, type_name};
use crate::yaml::{hash_lookup, hash_lookup_string};

/// 4.6.1 Arazzo Description is the root object of the loaded specification.
/// [Reference](https://spec.openapis.org/arazzo/latest.html#arazzo-description)
#[derive(Debug, Clone)]
pub struct ArazzoDescription {
  /// Version number of the Arazzo Specification
  pub arazzo: String,
  /// Metadata about API workflows defined in the Arazzo document
  pub info: Info
}

#[cfg(feature = "yaml")]
impl TryFrom<&Yaml> for ArazzoDescription {
  type Error = anyhow::Error;

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    if let Some(hash) = value.as_hash() {
      if let Ok(version) = hash_require_string(hash, "arazzo") {
        let info = Info::try_from(hash)?;
        Ok(ArazzoDescription {
          arazzo: version,
          info
        })
      } else {
        Err(anyhow!("Arazzo version number is required [4.6.1.1 Fixed Fields]"))
      }
    } else {
      Err(anyhow!("YAML document must be a Hash, got {}", type_name(value)))
    }
  }
}

/// 4.6.2 Info Object
/// [Reference](https://spec.openapis.org/arazzo/latest.html#info-object)
#[derive(Debug, Clone)]
pub struct Info {
  /// A human readable title of the Arazzo Description.
  pub title: String,
  /// A short summary of the Arazzo Description.
  pub summary: Option<String>,
  /// A description of the purpose of the workflows defined.
  pub description: Option<String>,
  /// Document version
  pub version: String
}

#[cfg(feature = "yaml")]
impl TryFrom<&Hash> for Info {
  type Error = anyhow::Error;

  fn try_from(value: &Hash) -> Result<Self, Self::Error> {
    if let Some(hash) = hash_lookup(value, "info", | v | v.as_hash().cloned()) {
      Ok(Info {
        title: hash_require_string(&hash, "title")?,
        summary: hash_lookup_string(&hash, "summary"),
        description: hash_lookup_string(&hash, "description"),
        version: hash_require_string(&hash, "version")?
      })
    } else {
      Err(anyhow!("Info Object is required [4.6.1.1 Fixed Fields]"))
    }
  }
}

#[cfg(test)]
mod tests {
  use expectest::expect;
  use expectest::prelude::be_err;
  use yaml_rust2::Yaml;
  use yaml_rust2::yaml::Hash;
  use crate::v1_0::ArazzoDescription;

  #[test]
  #[cfg(feature = "yaml")]
  fn fails_to_load_if_the_main_value_is_not_a_yaml_hash() {
    expect!(ArazzoDescription::try_from(&Yaml::String("test".to_string()))).to(be_err());
  }

  #[test]
  #[cfg(feature = "yaml")]
  fn fails_to_load_if_the_version_is_missing() {
    expect!(ArazzoDescription::try_from(&Yaml::Hash(Hash::new()))).to(be_err());
  }

  #[test]
  #[cfg(feature = "yaml")]
  fn fails_to_load_if_the_version_is_not_a_string() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::Hash(Hash::new()));
    expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  }

  #[test]
  #[cfg(feature = "yaml")]
  fn fails_to_load_if_the_info_is_missing() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
    expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  }
}
