//! Rust models for the [Arazzo Open API specification](https://spec.openapis.org/arazzo/latest.html)

pub mod v1_0;
pub mod extensions;
pub mod payloads;
#[cfg(feature = "json")] pub mod json;
#[cfg(feature = "yaml")] pub mod yaml;
