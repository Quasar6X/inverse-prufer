use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::config::Inset;

trait Id {
    fn id(&self) -> usize;
}

impl Id for dyn TreeNode {
    fn id(&self) -> usize {
        (self as *const dyn TreeNode).cast::<()>() as usize
    }
}

pub trait TreeNode {
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

impl Eq for dyn TreeNode {}

impl Debug for dyn TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreeNode")
            .field("id", &self.id())
            .field("content", &self.content())
            .finish()
    }
}

pub struct SimpleTreeNode {
    content: String,
    inset: Inset,
    children: Vec<Box<dyn TreeNode>>,
}

impl SimpleTreeNode {

    #[must_use]
    pub fn with_inset(content: &str, inset: Inset) -> Self {
        Self {
            content: content.to_owned(),
            inset,
            children: Vec::new(),
        }
    }

    #[must_use]
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

    #[must_use]
    pub const fn new() -> Self {
        Self(Vec::new())
    }
}

impl TreeNode for PlaceholderTreeNode {
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
