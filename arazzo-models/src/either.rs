//! Enum to store a value that can be either one or another value

use std::fmt::Debug;
use serde::Serialize;

/// Type that can be either A or B
#[derive(Debug, Clone, PartialEq)]
pub enum Either<A, B>
  where A: Debug + Clone + PartialEq + Serialize,
        B: Debug + Clone + PartialEq + Serialize {
  /// First value (A)
  First(A),

  /// Second value (B)
  Second(B)
}

impl <A, B> Either<A, B>
  where A: Debug + Clone + PartialEq + Serialize,
        B: Debug + Clone + PartialEq + Serialize {
  /// If the value is an A
  pub fn is_first(&self) -> bool {
    match self {
      Either::First(_) => true,
      Either::Second(_) => false
    }
  }

  /// If the value is a B
  pub fn is_second(&self) -> bool {
    match self {
      Either::First(_) => false,
      Either::Second(_) => true
    }
  }

  /// Returns the value of it is an A
  pub fn first(&self) -> Option<&A> {
    match self {
      Either::First(a) => Some(a),
      Either::Second(_) => None
    }
  }

  /// Returns the value if it is a B
  pub fn second(&self) -> Option<&B> {
    match self {
      Either::First(_) => None,
      Either::Second(b) => Some(b)
    }
  }
}
