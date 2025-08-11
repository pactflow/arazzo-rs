# arazzo-models
Rust models for the [Arazzo Open API specification](https://spec.openapis.org/arazzo/latest.html)

## Loading the models from YAML

You can create a Specification document with the following snippet. This requires the `yaml` 
feature flag enabled and uses the `yaml-rust2` crate.

```rust,no_run
use std::fs;
use arazzo_models::v1_0::ArazzoDescription;
use yaml_rust2::YamlLoader;
fn main() -> anyhow::Result<()> {
  let path = "/tmp/arazzo.doc";
  let contents = fs::read_to_string(path)?;
  let yaml = YamlLoader::load_from_str(contents.as_str())?;
  let descriptor = ArazzoDescription::try_from(&yaml[0])?;
  Ok(())
}
```

## Loading the models from JSON

You can create a Specification document with the following snippet. This requires the `json`
feature flag enabled and uses the `serde_json` crate.

```rust,no_run
use std::fs::File;
use std::io::BufReader;
use arazzo_models::v1_0::ArazzoDescription;
use serde_json::Value;
fn main() -> anyhow::Result<()> {
  let path = "/tmp/arazzo.doc";
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let json: Value = serde_json::from_reader(reader)?;
  let descriptor = ArazzoDescription::try_from(&json)?;
  Ok(())
}
```

## Writing models to YAML or JSON

There are implementations of Serde Serialize for all the models (with the `serialize` feature flag enabled),
so writing to a JSON or YAML document is straight forward. Just follow the Serde documentation.

```rust
use arazzo_models::v1_0::ArazzoDescription;
fn main() -> anyhow::Result<()> {
  let serialized = serde_json::to_string(&ArazzoDescription::default())?;
  Ok(())
}
```

Note that Serde implementations (like JSON and YAML) may sort the keys on writing. So reading in a file
and then writing it out again will result in changes.

## Crate features
All features are enabled by default

* `yaml`: Enables loading the models from a YAML document (uses yaml-rust2 crate)
* `json`: Enables loading the models from a JSON document (uses serde_json crate)
* `serialize`: Adds Serde Serialize implementations

## Note on the Arazzo Specification and Any types

The specification has constructs like `Any | {expression}`. This crate only supports values for
`Any` that can be expressed in JSON form.
