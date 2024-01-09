use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::config::Inset;
use uuid::Uuid;

pub trait TreeNode {
    fn id(&self) -> Uuid;

    fn content(&self) -> String;

    fn children(&self) -> &[Box<dyn TreeNode>];

    fn children_mut(&mut self) -> &mut [Box<dyn TreeNode>];

    fn insets(&self) -> &Inset {
        &Inset::EMPTY
    }

    fn is_decorable(&self) -> bool {
        true
    }

    fn is_placeholder(&self) -> bool {
        false
    }
}

// trait DynHash {
//     fn dyn_hash(&self, state: &mut dyn Hasher);
// }

// impl<T: Hash> DynHash for T {
//     fn dyn_hash(&self, mut state: &mut dyn Hasher) {
//         self.hash(&mut state);
//     }
// }

impl Hash for dyn TreeNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl PartialEq for dyn TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Debug for dyn TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreeNode")
            .field("id", &self.id())
            .field("content", &self.content())
            .finish()
    }
}

impl Eq for dyn TreeNode {}

pub struct SimpleTreeNode {
    id: Uuid,
    content: String,
    inset: Inset,
    children: Vec<Box<dyn TreeNode>>,
}

impl SimpleTreeNode {
    pub fn with_inset(content: &str, inset: Inset) -> Self {
        Self {
            id: Uuid::new_v4(),
            content: content.to_owned(),
            inset,
            children: Vec::new(),
        }
    }

    pub fn new(content: &str) -> Self {
        Self::with_inset(content, Inset::EMPTY)
    }

    pub fn add_child(&mut self, child: Box<dyn TreeNode>) {
        self.children.push(child);
    }

    pub fn add_children(&mut self, children: impl IntoIterator<Item = Box<dyn TreeNode>>) {
        self.children.extend(children);
    }
}

impl TreeNode for SimpleTreeNode {
    fn id(&self) -> Uuid {
        self.id
    }

    fn content(&self) -> String {
        self.content.clone()
    }

    fn children(&self) -> &[Box<dyn TreeNode>] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut [Box<dyn TreeNode>] {
        &mut self.children
    }

    fn insets(&self) -> &Inset {
        &self.inset
    }
}

pub struct PlaceholderTreeNode(Vec<Box<dyn TreeNode>>);

impl PlaceholderTreeNode {
    pub const fn new() -> Self {
        Self(Vec::new())
    }
}

impl TreeNode for PlaceholderTreeNode {
    fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn content(&self) -> String {
        "PLACEHOLDER".to_owned()
    }

    fn children(&self) -> &[Box<dyn TreeNode>] {
        &self.0
    }

    fn children_mut(&mut self) -> &mut [Box<dyn TreeNode>] {
        &mut self.0
    }

    fn is_decorable(&self) -> bool {
        false
    }

    fn is_placeholder(&self) -> bool {
        true
    }
}
