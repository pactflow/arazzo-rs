//! Rust models for the [Arazzo Open API specification](https://spec.openapis.org/arazzo/latest.html)

#[cfg(feature = "yaml")] pub mod yaml;

/// Arazzo Description is the root object of the loaded specification.
/// [Reference](https://spec.openapis.org/arazzo/latest.html#arazzo-description)
pub struct ArazzoDescription {
  /// Version number of the Arazzo Specification
  pub arazzo: String
}
