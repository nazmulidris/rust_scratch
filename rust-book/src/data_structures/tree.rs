/*
 Copyright 2022 Nazmul Idris

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

      https://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
*/

//! # Tree data structure
//! ----------------------------------------------------------------------------
//! - Rust book use of enums that are struct-like:
//!   <https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#:~:text=this%20one%20has%20a%20wide%20variety%20of%20types%20embedded%20in%20its%20variants>
//! - Examples of enums that are struct-like: <https://stackoverflow.com/q/29088633/2085356>
//!   - Approach 1: <https://stackoverflow.com/q/29088633/2085356>
//!   - Approach 2: <https://stackoverflow.com/a/29101091/2085356>
//! - Easy Rust book: <https://fongyoong.github.io/easy_rust/Chapter_25.html>
//! - `From` trait: <https://stackoverflow.com/a/42278050/2085356>
//! - Don't try to write Java in Rust:
//!   <https://users.rust-lang.org/t/passing-self-as-a-parameter/18069>
//!
//! # Weak refs for child's parent (ownership edge vs non-ownership edge)
//! ----------------------------------------------------------------------------
//! - Diagram
//!   - <https://github.com/nazmulidris/rust_scratch/blob/main/rust-book/docs/weak-ref.svg>
//!   - [SVG file](../../docs/weak-ref.svg)
//! - <https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#adding-a-reference-from-a-child-to-its-parent>
//! - Thinking about the relationships another way, a parent node should own its children: if a
//!   parent node is dropped, its child nodes should be dropped as well. However, a child should not
//!   own its parent: if we drop a child node, the parent should still exist. This is a case for weak
//!   references!
//!
//! # RwLock
//! ----------------------------------------------------------------------------
//! - <https://doc.rust-lang.org/std/sync/struct.RwLock.html>
//!
//! # Other implementations
//! ----------------------------------------------------------------------------
//! 1. RBTree
//!   - Code:
//!     <https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=9444cbeadcfdbef32c664ae2946e636a>
//!   - SO answer: <https://stackoverflow.com/a/65179837/2085356>
//! 2. Simple: <https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370>
//!

use core::fmt::Debug;
use rust_book_lib::utils::{print_header, style_dimmed, style_error, style_primary, style_prompt};
use std::{
  borrow::{Borrow, BorrowMut},
  cell::RefCell,
  fmt::{self, Display},
  sync::{Arc, RwLock, Weak},
};

pub fn run() {}

type NodeRef<T> = Arc<NodeData<T>>;
type WeakNodeRef<T> = Weak<NodeData<T>>;
/// Parent relationship is one of non-ownership.
type Parent<T> = RwLock<WeakNodeRef<T>>; // not `RwLock<NodeRef<T>>` which would cause memory leak.
/// Children relationship is one of ownership.
type Children<T> = RwLock<Vec<Child<T>>>;
type Child<T> = NodeRef<T>;

/// This struct holds underlying data. It shouldn't be created directly, instead use:
/// [`NodeRefHolder`](struct@NodeRefHolder).
pub struct NodeData<T>
where
  T: Display,
{
  value: T,
  parent: Parent<T>,
  children: Children<T>,
}

/// This struct is used to own a [`NodeData`] inside an [`Arc`], which can be shared, so that it can
/// have multiple owners. It also has getter methods for all of [`NodeData`]'s properties.
///
/// # Shared ownership
///
/// After an instance of this struct is created and it's internal reference is cloned (and given to
/// another) dropping this instance will not drop the cloned internal reference.
///
/// ```text
/// NodeRefHolder { strong_ref: Arc<NodeData> }
///      ▲                ▲
///      │                │
///      │     This atomic ref owns the
///      │     `NodeData` & is shared
///      │
///  1. Has methods to manipulate parent and children.
///  2. When it is dropped, if there are other `Arc`s
///     pointing to the same `NodeData`, then the
///    `NodeData` will not be dropped.
/// ```

#[derive(Debug)]
pub struct NodeRefHolder<T: Display> {
  strong_ref: NodeRef<T>,
}
impl<T> NodeRefHolder<T>
where
  T: Display,
{
  pub fn new(value: T) -> NodeRefHolder<T> {
    let new_node = NodeData {
      value,
      parent: RwLock::new(Weak::new()),
      children: RwLock::new(Vec::new()),
    };
    let node_arc_ref = Arc::new(new_node);
    NodeRefHolder {
      strong_ref: node_arc_ref,
    }
  }

  pub fn get_internal_ref_copy(self: &Self) -> NodeRef<T> {
    self.strong_ref.clone()
  }

  pub fn add_child(self: &Self, value: T) -> NodeRef<T> {
    let new_child = NodeRefHolder::new(value);
    self.add_child_and_update_its_parent(&new_child);
    new_child.get_internal_ref_copy()
  }

  /// 🔏 Write locks used.
  pub fn add_child_and_update_its_parent(self: &Self, child: &NodeRefHolder<T>) {
    {
      let mut my_children = self.strong_ref.children.write().unwrap();
      my_children.push(child.get_internal_ref_copy());
    } // `my_children` guard dropped.

    {
      let mut childs_parent = child.strong_ref.parent.write().unwrap();
      *childs_parent = Arc::downgrade(&self.get_internal_ref_copy());
    } // `my_parent` guard dropped.
  }

  pub fn has_parent(self: &Self) -> bool {
    self.get_parent().is_some()
  }

  /// 🔒 Read lock used.
  pub fn get_parent(self: &Self) -> Option<NodeRef<T>> {
    let my_parent_weak = self.strong_ref.parent.read().unwrap();
    if let Some(my_parent_arc_ref) = my_parent_weak.upgrade() {
      Some(my_parent_arc_ref)
    } else {
      None
    }
  }
}

#[test]
fn test_tree_low_level_node_manipulation() {
  let child_node = NodeRefHolder::new(3);
  {
    let parent_node = NodeRefHolder::new(5);
    parent_node.add_child_and_update_its_parent(&child_node);

    println!("{}: {:#?}", style_primary("[parent_node]"), parent_node); // Pretty print.
    println!("{}: {:#?}", style_primary("[child_node]"), child_node); // Pretty print.

    assert_eq!(Arc::strong_count(&child_node.get_internal_ref_copy()), 3); // `child_node` has 2 strong references.
    assert_eq!(Arc::weak_count(&child_node.get_internal_ref_copy()), 0);

    assert_eq!(Arc::strong_count(&parent_node.get_internal_ref_copy()), 2); // `parent_node` has 1 strong reference.
    assert_eq!(Arc::weak_count(&parent_node.get_internal_ref_copy()), 1); // `parent_node` also has 1 weak reference.

    assert!(child_node.has_parent());
    assert_eq!(child_node.get_parent().unwrap().value, 5);
  } // `parent_node` is dropped here.

  // `child_node`'s parent is now `None`, its an orphan.
  assert!(!child_node.has_parent());
  assert_eq!(child_node.get_internal_ref_copy().value, 3);

  assert_eq!(Arc::strong_count(&child_node.get_internal_ref_copy()), 2); // `child_node` has 1 strong references.
  assert_eq!(Arc::weak_count(&child_node.get_internal_ref_copy()), 0); // `child_node` still has no weak references.
}

// TODO: impl tree walking, find w/ comparator lambda, and print out the tree.
// TODO: impl delete, easy insert.
// TODO: impl nodelist (find multiple nodes) & return iterator.
// TODO: impl add siblings to node.

#[test]
fn test_tree_simple_api() {
  let root_ref_holder = NodeRefHolder::new(5);
  {
    let child_ref = root_ref_holder.add_child(3);

    assert_eq!(child_ref.value, 3);
    assert_eq!(root_ref_holder.get_internal_ref_copy().value, 5);
    assert_eq!(
      root_ref_holder
        .get_internal_ref_copy()
        .children
        .read()
        .unwrap()
        .len(),
      1
    );
    assert_eq!(
      child_ref.value,
      root_ref_holder
        .get_internal_ref_copy()
        .children
        .read()
        .unwrap()[0]
        .value
    );
  }
  println!("{}: {:#?}", style_primary("[tree]"), root_ref_holder); // Pretty print.
}

impl<T> fmt::Debug for NodeData<T>
where
  T: Debug + Display,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut parent_msg = String::new();
    if let Some(parent) = self.parent.read().unwrap().upgrade() {
      parent_msg.push_str(format!("📦 {}", parent.value).as_str());
    } else {
      parent_msg.push_str("🚫 None");
    }
    f.debug_struct("Node")
      .field("value", &self.value)
      // .field("parent", &self.parent)
      .field("parent", &parent_msg)
      .field("children", &self.children)
      .finish()
  }
}
