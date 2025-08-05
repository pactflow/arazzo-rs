//! Version 1.0.x specification models (https://spec.openapis.org/arazzo/v1.0.1.html)

use std::collections::HashMap;

use anyhow::anyhow;
#[cfg(feature = "yaml")] use yaml_rust2::Yaml;
#[cfg(feature = "yaml")] use yaml_rust2::yaml::Hash;

use crate::extensions::ExtensionValue;
#[cfg(feature = "yaml")] use crate::extensions::yaml_extract_extensions;
#[cfg(feature = "yaml")] use crate::yaml::{
  hash_lookup,
  hash_lookup_string,
  hash_lookup_string_list,
  hash_require_string,
  type_name
};

/// 4.6.1 Arazzo Description is the root object of the loaded specification.
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#arazzo-description)
#[derive(Debug, Clone)]
pub struct ArazzoDescription {
  /// Version number of the Arazzo Specification
  pub arazzo: String,
  /// Metadata about API workflows defined in the Arazzo document
  pub info: Info,
  /// List of source descriptions
  pub source_descriptions: Vec<SourceDescription>,
  /// List of workflows
  pub workflows: Vec<Workflow>,
  /// Extension values
  pub extensions: HashMap<String, ExtensionValue>,
}

#[cfg(feature = "yaml")]
impl TryFrom<&Yaml> for ArazzoDescription {
  type Error = anyhow::Error;

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    if let Some(hash) = value.as_hash() {
      if let Ok(version) = hash_require_string(hash, "arazzo") {
        let info = Info::try_from(hash)?;
        let source_descriptions = yaml_load_source_descriptions(hash)?;
        let workflows = yaml_load_workflows(hash)?;

        Ok(ArazzoDescription {
          arazzo: version,
          info,
          source_descriptions,
          workflows,
          extensions: yaml_extract_extensions(&hash)?
        })
      } else {
        Err(anyhow!("Arazzo version number is required [4.6.1.1 Fixed Fields]"))
      }
    } else {
      Err(anyhow!("YAML value must be a Hash, got {}", type_name(value)))
    }
  }
}

/// 4.6.2 Info Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#info-object)
#[derive(Debug, Clone)]
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
  pub extensions: HashMap<String, ExtensionValue>
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
        version: hash_require_string(&hash, "version")?,
        extensions: yaml_extract_extensions(&hash)?
      })
    } else {
      Err(anyhow!("Info Object is required [4.6.1.1 Fixed Fields]"))
    }
  }
}

/// 4.6.3 Source Description Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#source-description-object)
#[derive(Debug, Clone)]
pub struct SourceDescription {
  /// Unique name for the source description.
  pub name: String,
  /// URL to a source description to be used by a workflow.
  pub url: String,
  /// The type of source description.
  pub r#type: Option<String>,
  /// Extension values
  pub extensions: HashMap<String, ExtensionValue>
}

#[cfg(feature = "yaml")]
fn yaml_load_source_descriptions(hash: &Hash) -> anyhow::Result<Vec<SourceDescription>> {
  if let Some(array) = hash_lookup(hash, "sourceDescriptions", | v | v.as_vec().cloned()) {
    if array.is_empty() {
      Err(anyhow!("Source Description list must have at least one entry [4.6.1.1 Fixed Fields]"))
    } else {
      let mut list = vec![];

      for item in &array {
        list.push(SourceDescription::try_from(item)?);
      }

      Ok(list)
    }
  } else {
    Err(anyhow!("Source Description Object is required [4.6.1.1 Fixed Fields]"))
  }
}

#[cfg(feature = "yaml")]
impl TryFrom<&Yaml> for SourceDescription {
  type Error = anyhow::Error;

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    if let Some(hash) = value.as_hash() {
      Ok(SourceDescription {
        name: hash_require_string(&hash, "name")?,
        url: hash_require_string(&hash, "url")?,
        r#type: hash_lookup_string(&hash, "type"),
        extensions: yaml_extract_extensions(&hash)?
      })
    } else {
      Err(anyhow!("YAML value must be a Hash, got {}", type_name(value)))
    }
  }
}

/// 4.6.4 Workflow Object
/// [Reference](https://spec.openapis.org/arazzo/v1.0.1.html#workflow-object)
#[derive(Debug, Clone)]
pub struct Workflow {
  /// Unique string to represent the workflow.
  pub workflow_id: String,
  /// Summary of the purpose or objective of the workflow.
  pub summary: Option<String>,
  /// Description of the workflow.
  pub description: Option<String>,
  /// List of workflows that must be completed before this workflow can be processed.
  pub depends_on: Vec<String>,
  /// Extension values
  pub extensions: HashMap<String, ExtensionValue>
}

#[cfg(feature = "yaml")]
fn yaml_load_workflows(hash: &Hash) -> anyhow::Result<Vec<Workflow>> {
  if let Some(array) = hash_lookup(hash, "workflows", | v | v.as_vec().cloned()) {
    if array.is_empty() {
      Err(anyhow!("Workflows list must have at least one entry [4.6.1.1 Fixed Fields]"))
    } else {
      let mut list = vec![];

      for item in &array {
        list.push(Workflow::try_from(item)?);
      }

      Ok(list)
    }
  } else {
    Err(anyhow!("Workflow Object is required [4.6.1.1 Fixed Fields]"))
  }
}

#[cfg(feature = "yaml")]
impl TryFrom<&Yaml> for Workflow {
  type Error = anyhow::Error;

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    if let Some(hash) = value.as_hash() {
      Ok(Workflow {
        workflow_id: hash_require_string(&hash, "workflowId")?,
        summary: hash_lookup_string(&hash, "summary"),
        description: hash_lookup_string(&hash, "description"),
        depends_on: hash_lookup_string_list(&hash, "dependsOn").unwrap_or_default(),
        extensions: yaml_extract_extensions(&hash)?
      })
    } else {
      Err(anyhow!("YAML value must be a Hash, got {}", type_name(value)))
    }
  }
}

#[cfg(test)]
#[cfg(feature = "yaml")]
mod yaml_tests {
  use expectest::prelude::*;
  use maplit::hashmap;
  use yaml_rust2::Yaml;
  use yaml_rust2::yaml::Hash;

  use crate::extensions::ExtensionValue;
  use crate::v1_0::{ArazzoDescription, Info, SourceDescription, Workflow};

  #[test]
  fn fails_to_load_if_the_main_value_is_not_a_yaml_hash() {
    expect!(ArazzoDescription::try_from(&Yaml::String("test".to_string()))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_version_is_missing() {
    expect!(ArazzoDescription::try_from(&Yaml::Hash(Hash::new()))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_version_is_not_a_string() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::Hash(Hash::new()));
    expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_info_is_missing() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
    expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_source_descriptions_are_missing() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
    hash.insert(Yaml::String("info".to_string()), Yaml::Hash(info_fixture()));
    expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_source_descriptions_are_empty() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
    hash.insert(Yaml::String("info".to_string()), Yaml::Hash(info_fixture()));
    hash.insert(Yaml::String("sourceDescriptions".to_string()), Yaml::Array(vec![]));
    expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_workflows_are_missing() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
    hash.insert(Yaml::String("info".to_string()), Yaml::Hash(info_fixture()));
    hash.insert(Yaml::String("sourceDescriptions".to_string()), Yaml::Array(source_descriptions_fixture()));
    expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  }

  #[test]
  fn fails_to_load_if_the_workflows_are_empty() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
    hash.insert(Yaml::String("info".to_string()), Yaml::Hash(info_fixture()));
    hash.insert(Yaml::String("sourceDescriptions".to_string()), Yaml::Array(source_descriptions_fixture()));
    hash.insert(Yaml::String("workflows".to_string()), Yaml::Array(vec![]));
    expect!(ArazzoDescription::try_from(&Yaml::Hash(hash))).to(be_err());
  }

  #[test]
  fn arazzo_description_supports_extensions() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("arazzo".to_string()), Yaml::String("1.0.0".to_string()));
    hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
    hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));

    hash.insert(Yaml::String("info".to_string()), Yaml::Hash(info_fixture()));
    hash.insert(Yaml::String("sourceDescriptions".to_string()), Yaml::Array(source_descriptions_fixture()));
    hash.insert(Yaml::String("workflows".to_string()), Yaml::Array(workflows_fixture()));

    let desc = ArazzoDescription::try_from(&Yaml::Hash(hash)).unwrap();
    expect!(desc.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => ExtensionValue::String("1".to_string()),
      "two".to_string() => ExtensionValue::Integer(2)
    }));
  }

  fn info_fixture() -> Hash {
    let mut info = Hash::new();
    info.insert(Yaml::String("title".to_string()), Yaml::String("test".to_string()));
    info.insert(Yaml::String("version".to_string()), Yaml::String("1.0.0".to_string()));
    info
  }

  fn source_descriptions_fixture() -> Vec<Yaml> {
    let mut desc = Hash::new();
    desc.insert(Yaml::String("name".to_string()), Yaml::String("test".to_string()));
    desc.insert(Yaml::String("url".to_string()), Yaml::String("http://test".to_string()));
    vec![Yaml::Hash(desc)]
  }

  fn workflows_fixture() -> Vec<Yaml> {
    let mut desc = Hash::new();
    desc.insert(Yaml::String("workflowId".to_string()), Yaml::String("test".to_string()));
    vec![Yaml::Hash(desc)]
  }

  #[test]
  fn info_supports_extensions() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("title".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("version".to_string()), Yaml::String("1.0.0".to_string()));
    hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
    hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));

    let mut outer = Hash::new();
    outer.insert(Yaml::String("info".to_string()), Yaml::Hash(hash));
    let info = Info::try_from(&outer).unwrap();
    expect!(info.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => ExtensionValue::String("1".to_string()),
      "two".to_string() => ExtensionValue::Integer(2)
    }));
  }

  #[test]
  fn source_description_supports_extensions() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("name".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("url".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
    hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));

    let desc = SourceDescription::try_from(&Yaml::Hash(hash)).unwrap();
    expect!(desc.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => ExtensionValue::String("1".to_string()),
      "two".to_string() => ExtensionValue::Integer(2)
    }));
  }

  #[test]
  fn worlflow_supports_extensions() {
    let mut hash = Hash::new();
    hash.insert(Yaml::String("workflowId".to_string()), Yaml::String("test".to_string()));
    hash.insert(Yaml::String("x-one".to_string()), Yaml::String("1".to_string()));
    hash.insert(Yaml::String("x-two".to_string()), Yaml::Integer(2));

    let wf = Workflow::try_from(&Yaml::Hash(hash)).unwrap();
    expect!(wf.extensions).to(be_equal_to(hashmap!{
      "one".to_string() => ExtensionValue::String("1".to_string()),
      "two".to_string() => ExtensionValue::Integer(2)
    }));
  }
}
