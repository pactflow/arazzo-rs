//! Structs and Traits for dealing with body payloads

use std::any::Any;
use std::fmt::Debug;

use bytes::Bytes;
use serde::{Serialize, Serializer};
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

impl Serialize for dyn Payload + Send + Sync {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer
  {
    let payload: &dyn Any = self;
    if let Some(string_payload) = payload.downcast_ref::<StringPayload>() {
      string_payload.serialize(serializer)
    } else if let Some(json_payload) = payload.downcast_ref::<JsonPayload>() {
      json_payload.serialize(serializer)
    } else {
      serializer.serialize_unit()
    }
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

impl Serialize for StringPayload {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer
  {
    serializer.serialize_str(self.0.as_str())
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

impl Serialize for EmptyPayload {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer
  {
    serializer.serialize_str("")
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

impl Serialize for JsonPayload {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer
  {
    self.0.serialize(serializer)
  }
}
