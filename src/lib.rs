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

/// Public interface
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

  pub fn update_edge(&mut self, origin: usize, destiny: usize, weight: i32) -> Result<(), GraphError> {
    if !self.edge_exist(origin, destiny) {
      self.add_edge(origin, destiny, weight)
    }
    else {
      self.modify_edge(origin, destiny, weight)
    }
  }

} // Public interface

/// Private interface
impl Graph {
  /// Add a non existant edge
  fn add_edge(&mut self, origin: usize, destiny: usize, weight: i32) -> Result<(), GraphError> {
    debug_assert!(!self.edge_exist(origin, destiny));
    if !self.vertex_exist(origin) || !self.vertex_exist(destiny) {
      Err(GraphError::VertexNotExist)
    } else if self.is_directed {
      self.adjacency_list[origin].as_mut().unwrap().push((destiny, weight));
      Ok(())
    } else {
      // a to b
      self.adjacency_list[origin].as_mut().unwrap().push((destiny, weight));
      // b to a
      self.adjacency_list[destiny].as_mut().unwrap().push((origin, weight));
      Ok(())
    }
  }
  /// Update an already existant edge
  fn modify_edge(&mut self, origin: usize, destiny: usize, weight: i32) -> Result<(), GraphError> {
    if !self.vertex_exist(origin) || !self.vertex_exist(destiny) {
      return Err(GraphError::VertexNotExist);
    }
    if !self.edge_exist(origin, destiny) {
      return Err(GraphError::EdgeNotExist);
    }
    if self.is_directed {
      self.adjacency_list[origin]
        .as_mut()
        .unwrap()
        .iter_mut()
        .find(|x| x.0 == destiny)
        .unwrap()
        .1 = weight;
      Ok(())
    } else {
      // origin to destiny
      self.adjacency_list[origin]
        .as_mut()
        .unwrap()
        .iter_mut()
        .find(|x| x.0 == destiny)
        .unwrap()
        .1 = weight;
      // destiny to origin
      self.adjacency_list[destiny]
        .as_mut()
        .unwrap()
        .iter_mut()
        .find(|x| x.0 == origin)
        .unwrap()
        .1 = weight;
      Ok(())
    }
  }
  /// Checks if a vertex exists in the graph
  fn vertex_exist(&self, vertex: usize) -> bool {
    if self.adjacency_list.len() < vertex {
      false
    } else {
      self.adjacency_list[vertex].is_some()
    }
  }

  /// Checks if a edge exist. It also double check when not directed, done for debugging purposes
  fn edge_exist(&self, origin: usize, destiny: usize) -> bool {
    debug_assert!(self.vertex_exist(origin) && self.vertex_exist(destiny));
    let origin_to_destiny = self.adjacency_list[origin]
      .as_ref()
      .unwrap()
      .iter()
      .position(|x| x.0 == destiny)
      .is_some();
    let destiny_to_origin = self.adjacency_list[destiny]
      .as_ref()
      .unwrap()
      .iter()
      .position(|x| x.0 == origin)
      .is_some();
    if self.is_directed {
      origin_to_destiny
    } else {
      origin_to_destiny && destiny_to_origin
    }
  }
} // Private interface

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
  EdgeNotExist,
  VertexDuplication,
}

impl fmt::Debug for GraphError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      | Self::VertexNotExist => write!(f, "Trying to access a vertex non-existent"),
      | Self::VertexDuplication => write!(f, "Trying to create the same vertex two times"),
      | Self::EdgeNotExist => write!(f, "Trying to access a edge non-existent"),
    }
  }
}

#[cfg(test)]
mod graph_test {
  use crate::Graph;

  #[test]
  fn add_vertex_test() {
    let mut x = Graph::new(false);
    x.add_vertex(0).expect("error first add");
    x.add_vertex(1).expect("error second add");
    x.add_vertex(1).expect_err("not error found at third");
    x.add_vertex(50).expect("error third add");
  }

  #[test]
  fn add_edge_test() {
    let mut x = Graph::new(false);
    x.add_vertex(0).expect("error first add");
    x.add_vertex(1).expect("error second add");
    x.add_vertex(4).expect("error third add");

    x.update_edge(0, 1, 100).expect("error adding first edge");
    x.update_edge(0, 4, 500).expect("error adding second edge");
  }

  #[test]
  fn find_edge_test() {
    let mut x = Graph::new(true);
    x.add_vertex(0).expect("error first add");
    x.add_vertex(1).expect("error second add");
    x.add_vertex(4).expect("error third add");

    x.update_edge(0, 1, 100).expect("error adding first edge");
    x.update_edge(0, 4, 500).expect("error adding second edge");

    assert!(x.edge_exist(0, 1));
    assert!(x.edge_exist(0, 4));
    assert!(!x.edge_exist(1, 4));
  }

  #[test]
  fn modify_edge_test() {
    let mut x = Graph::new(false);
    x.add_vertex(0).expect("error first add");
    x.add_vertex(1).expect("error second add");
    x.add_vertex(4).expect("error third add");

    x.update_edge(0, 1, 100).expect("error adding first edge");
    x.update_edge(0, 4, 500).expect("error adding second edge");

    x.update_edge(0, 1, 5).expect("error modifying edge");

    println!("{}", x);
  }
} // mod graph_test
