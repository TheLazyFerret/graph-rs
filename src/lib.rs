//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Main lib file

use core::fmt;

// (destiny, weight)
type Edge = (usize, i32);

#[derive(Default)]
pub struct Graph {
  adjacency_list: Vec<Option<Vec<Edge>>>,
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

  /// Add a vertex to the graph. Returns Error if it is duplicated.
  pub fn add_vertex(&mut self, vertex: usize) -> Result<(), GraphError> {
    if self.adjacency_list.len() <= vertex {
      self.adjacency_list.resize(vertex + 1, None);
    }
    if self.adjacency_list[vertex].is_none() {
      self.adjacency_list[vertex] = Some(Vec::new());
      Ok(())
    } else {
      Err(GraphError::VertexDuplication)
    }
  }
} // impl Graph

impl fmt::Display for Graph {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (indx, adj) in self.adjacency_list.iter().enumerate() {
      if adj.is_some() {
        writeln!(f, "{} -> {:?}", indx, adj.clone().unwrap())?;
      }
    }
    Ok(())
  }
}

/// Enum representing all possible errors
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GraphError {
  VertexNotExist,
  VertexDuplication,
}

impl fmt::Debug for GraphError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      | Self::VertexNotExist => write!(f, "Trying to access a vertex non-existent"),
      | Self::VertexDuplication => write!(f, "Trying to create the same vertex two times"),
    }
  }
}


#[cfg(test)]
mod graph_test {
    use crate::Graph;

  #[test]
  fn print_test() {
    let mut x = Graph::new(false);
    x.add_vertex(0).unwrap();
    x.add_vertex(1).unwrap();
    println!("{}", x);
  }
}