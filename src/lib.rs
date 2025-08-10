//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Main lib file

use core::fmt;

// (destiny, weight)
type Edge = (u8, i32);

#[derive(Default)]
pub struct Graph {
  adjacency_list: Vec<Vec<Edge>>,
  is_directed: bool,
}

impl Graph {
  /// Creates a new instance of the struct
  pub fn new(is_directed: bool) -> Self {
    Graph {
      is_directed,
      ..Default::default()
    }
  }
} // impl Graph

/// Enum representing all possible errors
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Error {}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}