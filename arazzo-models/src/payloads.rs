//! Structs and Traits for dealing with body payloads

use std::any::Any;
use std::fmt::Debug;

use bytes::Bytes;
use serde_json::Value;

/// Body Payload
pub trait Payload: Debug + Any {
  /// Returns the raw bytes of the payload. Note that in some cases this will return a new copy
  /// of the payload bytes.
  fn as_bytes(&self) -> Bytes;

  /// Returns the payload as a String.
  fn as_string(&self) -> String;

  /// Returns the payload as a JSON document if it is easily convertable, otherwise returns None.
  fn as_json(&self) -> Option<Value> {
    None
  }
}

/// Payload stored as a String value
#[derive(Clone, Debug)]
pub struct StringPayload(pub String);

impl Payload for StringPayload {
  fn as_bytes(&self) -> Bytes {
    Bytes::from(self.0.clone())
  }

  fn as_string(&self) -> String {
    self.0.clone()
  }
}

/// Empty Payload
#[derive(Clone, Debug)]
pub struct EmptyPayload;

impl Payload for EmptyPayload {
  fn as_bytes(&self) -> Bytes {
    Bytes::new()
  }

  fn as_string(&self) -> String {
    String::new()
  }
}

/// Payload stored as a JSON document. Note that this does not mean a JSON payload (that would be
/// depending on the content type), but that the source of the payload is stored as JSON.
#[derive(Clone, Debug)]
pub struct JsonPayload(pub Value);

impl Payload for JsonPayload {
  fn as_bytes(&self) -> Bytes {
    Bytes::from(self.0.to_string())
  }

  fn as_string(&self) -> String {
    self.0.to_string()
  }

  fn as_json(&self) -> Option<Value> {
    Some(self.0.clone())
  }
}
