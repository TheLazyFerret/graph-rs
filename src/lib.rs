//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Main lib file

#[cfg(test)]
mod test;

use core::fmt;

// (destiny, weight)
type Edge = (usize, i32);

#[derive(Default)]
pub struct Graph {
  adjacency_list: Vec<Option<Vec<Edge>>>,
  is_directed: bool,
}

/// Enum representing all possible errors
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GraphError {
  VertexNotExist,
  VertexDuplication,
  EdgeNotExist,
  EdgeDuplication,
}

/// PUBLIC INTERFACE
impl Graph {
  /// Creates a new instance of the struct
  pub fn new(is_directed: bool) -> Self {
    Graph { is_directed, ..Default::default() }
  }

  /// Add a vertex to the graph. Returns Error if it is duplicated.
  pub fn insert_vertex(&mut self, vertex: usize) -> Result<(), GraphError> {
    if self.vertex_exists(vertex) {
      Err(GraphError::VertexDuplication)
    } else {
      self.add_vertex(vertex);
      Ok(())
    }
  }

  /// Add the edge, if it already exists, update its weight.
  pub fn insert_edge(&mut self, origin: usize, destiny: usize, weight: i32) -> Result<(), GraphError> {
    if !self.vertex_exists(origin) || !self.vertex_exists(destiny) {
      Err(GraphError::VertexNotExist)
    } else if self.edge_exists(origin, destiny) {
      Err(GraphError::EdgeDuplication)
    } else {
      self.add_edge(origin, destiny, weight);
      Ok(())
    }
  }

  /// Update the weight of a edge.
  pub fn update_edge(&mut self, origin: usize, destiny: usize, weight: i32) -> Result<(), GraphError> {
    if !self.vertex_exists(origin) || !self.vertex_exists(destiny) {
      Err(GraphError::VertexNotExist)
    } else if !self.edge_exists(origin, destiny) {
      Err(GraphError::EdgeNotExist)
    } else {
      self.modify_edge(origin, destiny, weight);
      Ok(())
    }
  }

  /// remove a vertex and all possible directed connected edges.
  pub fn remove_vertex(&mut self, vertex: usize) -> Result<(), GraphError> {
    if !self.vertex_exists(vertex) {
      Err(GraphError::VertexNotExist)
    } else {
      self.clean_vertex(vertex);
      Ok(())
    }
  }

  /// remove an edge from the graph.
  pub fn remove_edge(&mut self, origin: usize, destiny: usize) -> Result<(), GraphError> {
    if !self.vertex_exists(origin) || !self.vertex_exists(destiny) {
      Err(GraphError::VertexNotExist)
    } else if !self.edge_exists(origin, destiny) {
      Err(GraphError::EdgeDuplication)
    } else {
      self.clean_edge(origin, destiny);
      Ok(())
    }
  }
}
/// END PUBLIC INTERFACE

/// PRIVATE INTERFACE
impl Graph {
  /// Checks if a vertex exists in the graph
  fn vertex_exists(&self, vertex: usize) -> bool {
    if self.adjacency_list.len() <= vertex {
      false
    } else {
      self.adjacency_list[vertex].is_some()
    }
  }

  /// Checks if a edge exist. It also double check when not directed, done for debugging purposes
  fn edge_exists(&self, origin: usize, destiny: usize) -> bool {
    debug_assert!(self.vertex_exists(origin) && self.vertex_exists(destiny));
    let origin_to_destiny = self.adjacency_list[origin].as_ref().unwrap().iter().position(|x| x.0 == destiny).is_some();
    let destiny_to_origin = self.adjacency_list[destiny].as_ref().unwrap().iter().position(|x| x.0 == origin).is_some();
    if self.is_directed {
      origin_to_destiny
    } else {
      debug_assert!(origin_to_destiny == destiny_to_origin);
      origin_to_destiny || destiny_to_origin
    }
  }

  /// Add a vertex. If already exist, returns Err(GraphError::VertexDuplication)
  fn add_vertex(&mut self, vertex: usize) {
    debug_assert!(!self.vertex_exists(vertex));
    if self.adjacency_list.len() <= vertex {
      self.adjacency_list.resize(vertex + 1, None);
    }
    self.adjacency_list[vertex] = Some(Vec::new());
  }

  /// Add a non existant edge
  fn add_edge(&mut self, origin: usize, destiny: usize, weight: i32) {
    debug_assert!(self.vertex_exists(origin) && self.vertex_exists(destiny));
    debug_assert!(!self.edge_exists(origin, destiny));
    if self.is_directed {
      self.adjacency_list[origin].as_mut().unwrap().push((destiny, weight));
    } else {
      // a to b
      self.adjacency_list[origin].as_mut().unwrap().push((destiny, weight));
      // b to a
      self.adjacency_list[destiny].as_mut().unwrap().push((origin, weight));
    }
  }
  /// Update an already existant edge
  fn modify_edge(&mut self, origin: usize, destiny: usize, weight: i32) {
    debug_assert!(self.vertex_exists(origin) && self.vertex_exists(destiny));
    debug_assert!(self.edge_exists(origin, destiny));
    if self.is_directed {
      self.adjacency_list[origin].as_mut().unwrap().iter_mut().find(|x| x.0 == destiny).unwrap().1 = weight;
    } else {
      // origin to destiny
      self.adjacency_list[origin].as_mut().unwrap().iter_mut().find(|x| x.0 == destiny).unwrap().1 = weight;
      // destiny to origin
      self.adjacency_list[destiny].as_mut().unwrap().iter_mut().find(|x| x.0 == origin).unwrap().1 = weight;
    }
  }

  /// Removes a vertex, and all the edges relates.
  fn clean_vertex(&mut self, vertex: usize) {
    debug_assert!(self.vertex_exists(vertex));
    self.adjacency_list[vertex] = None;
    for adj in self.adjacency_list.iter_mut().filter_map(|x| x.as_mut()) {
      adj.retain(|x| x.0 != vertex);
    }
  }

  /// Removes a edge.
  fn clean_edge(&mut self, origin: usize, destiny: usize) {
    debug_assert!(self.vertex_exists(origin) && self.vertex_exists(destiny));
    debug_assert!(self.edge_exists(origin, destiny));
    if self.is_directed {
      let index = // vector position of the edge
        self.adjacency_list[origin].as_ref().unwrap().iter().enumerate().find(|x| x.1.0 == destiny).unwrap().0;
      self.adjacency_list[origin].as_mut().unwrap().swap_remove(index);
    } else {
      // a to b
      let ori_index = // vector position of the origin to destiny edge
        self.adjacency_list[origin].as_ref().unwrap().iter().enumerate().find(|x| x.1.0 == destiny).unwrap().0;
      self.adjacency_list[origin].as_mut().unwrap().swap_remove(ori_index);
      // b to a
      let des_index = // vector position of the destiny to origin edge
        self.adjacency_list[destiny].as_ref().unwrap().iter().enumerate().find(|x| x.1.0 == origin).unwrap().0;
      self.adjacency_list[destiny].as_mut().unwrap().swap_remove(des_index);
    }
  }
}
/// END PRIVATE INTERFACE

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

impl fmt::Debug for GraphError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      | Self::VertexNotExist => write!(f, "Trying to access a vertex non-existent"),
      | Self::VertexDuplication => write!(f, "Trying to insert the same vertex two times"),
      | Self::EdgeNotExist => write!(f, "Trying to access a edge non-existent"),
      | Self::EdgeDuplication => write!(f, "Trying to insert the same edge two times"),
    }
  }
}
