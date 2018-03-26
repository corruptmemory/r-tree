use std::vec::Vec;
use  std::fmt::Debug;

use rtree::rect::Rect;

#[derive(Debug)]
pub struct InnerNode<T : Debug> {
    parent: Option<Box<RTreeNode<T>>>,
    rect: Rect,
    values: Vec<Box<RTreeNode<T>>>
}

impl<T:Debug> InnerNode<T> {
  pub fn new(capacity: usize) -> InnerNode<T> {
    InnerNode {parent:None, rect: Rect::empty(), values: Vec::with_capacity(capacity) }
  }

  pub fn is_full(&self) -> bool {
    self.values.len() == self.values.capacity()
  }
}

#[derive(Debug)]
pub struct LeafNode<T: Debug> {
  parent: Option<Box<RTreeNode<T>>>,
  rect: Rect,
  values: Vec<Box<T>>
}

impl<T:Debug> LeafNode<T> {
  pub fn new(capacity: usize) -> LeafNode<T> {
    LeafNode {parent:None, rect:Rect::empty(), values: Vec::with_capacity(capacity) }
  }

  pub fn is_full(&self) -> bool {
    self.values.len() == self.values.capacity()
  }
}

#[derive(Debug)]
pub enum RTreeNode<T: Debug> {
  Inner(InnerNode<T>),
  Leaf(LeafNode<T>)
}

#[derive(Debug)]
pub struct RTree<T : Debug> {
    max_per_node: usize,
    root: Box<RTreeNode<T>>
}