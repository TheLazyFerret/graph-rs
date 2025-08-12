//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! test module.

use crate::Graph;

#[test]
fn insert_vertex_test() {
  let mut x = Graph::new(false);
  x.insert_vertex(0).expect("error first add");
  x.insert_vertex(1).expect("error second add");
  x.insert_vertex(1).expect_err("not error found at third");

  assert!(x.vertex_exists(0));
  assert!(x.vertex_exists(1));
}

#[test]
fn insert_edge_test() {
  let mut x = Graph::new(false);
  x.insert_vertex(0).expect("error first add");
  x.insert_vertex(1).expect("error second add");
  x.insert_vertex(4).expect("error third add");

  x.insert_edge(0, 1, 100).expect("error adding first edge");
  x.insert_edge(0, 4, 500).expect("error adding second edge");

  assert!(x.edge_exists(0, 1));
  assert!(x.edge_exists(0, 4));
}

#[test]
fn modify_edge_test() {
  let mut x = Graph::new(true);
  x.insert_vertex(0).expect("error first add");
  x.insert_vertex(1).expect("error second add");

  x.insert_edge(0, 1, 100).expect("error adding edge");
  x.update_edge(0, 1, 5).expect("error modifying edge");

  assert_eq!(x.adjacency_list[0].as_mut().expect("error expected Some")[0].1, 5);
}

#[test]
fn delete_vertex_test() {
  let mut x = Graph::new(false);
  x.insert_vertex(0).expect("error first add");
  x.insert_vertex(1).expect("error second add");
  x.insert_vertex(4).expect("error third add");

  x.insert_edge(0, 1, 100).expect("error adding first edge");
  x.insert_edge(0, 4, 500).expect("error adding second edge");

  x.clean_vertex(0);
  assert!(!x.vertex_exists(0));

  x.clean_vertex(1);
  assert!(!x.vertex_exists(1));
}

#[test]
fn delete_edge_test() {
  let mut x = Graph::new(false);

  x.insert_vertex(0).expect("error first add");
  x.insert_vertex(1).expect("error second add");
  x.insert_vertex(4).expect("error third add");

  x.insert_edge(0, 1, 100).expect("error adding first edge");
  x.insert_edge(0, 4, 500).expect("error adding second edge");

  x.clean_edge(0, 1);
  assert!(!x.edge_exists(0, 1));
}
