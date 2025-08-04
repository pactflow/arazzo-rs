//! Version 1.0.x specification models

use anyhow::anyhow;
#[cfg(feature = "yaml")] use yaml_rust2::Yaml;

#[cfg(feature = "yaml")] use crate::yaml::{hash_require_string, type_name};

/// Arazzo Description is the root object of the loaded specification.
/// [Reference](https://spec.openapis.org/arazzo/latest.html#arazzo-description)
#[derive(Debug, Clone)]
pub struct ArazzoDescription {
  /// Version number of the Arazzo Specification
  pub arazzo: String
}

#[cfg(feature = "yaml")]
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
