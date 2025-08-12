//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Private interface and trait implementation.

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

  assert!(x.edge_exists(0, 1));
  assert!(x.edge_exists(0, 4));
  assert!(!x.edge_exists(1, 4));
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

#[test]
fn delete_vertex() {
  let mut x = Graph::new(false);
  x.add_vertex(0).expect("error first add");
  x.add_vertex(1).expect("error second add");
  x.add_vertex(4).expect("error third add");

  x.update_edge(0, 1, 100).expect("error adding first edge");
  x.update_edge(0, 4, 500).expect("error adding second edge");

  x.update_edge(0, 1, 5).expect("error modifying edge");

  x.clean_vertex(0);
  assert!(!x.vertex_exists(0));
  println!("{}", x);

  x.clean_vertex(1);
  assert!(!x.vertex_exists(1));
  println!("{}", x);
}