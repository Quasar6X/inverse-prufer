use crate::config::Inset;

pub trait TreeNode {
    fn content(&self) -> &str;

    fn children(&self) -> &Vec<impl TreeNode>;

    fn insets(&self) -> &Inset;
}

pub struct SimpleTreeNode {
    content: String,
    inset: Inset,
    children: Vec<SimpleTreeNode>,
}

impl SimpleTreeNode {
    pub fn new(content: &str, inset: Inset) -> Self {
        Self {
            content: content.to_owned(),
            inset,
            children: Vec::new(),
        }
    }

    pub fn new_empty_inset(content: &str) -> Self {
        Self::new(content, Inset::empty_inset())
    }

    pub fn add_child(&mut self, node: Box<dyn TreeNode>) {
        self.children.push(node);
    }
}

impl TreeNode for SimpleTreeNode {
    fn content(&self) -> &str {
        self.content.as_str()
    }

    fn children(&self) -> &Vec<impl TreeNode> {
        &self.children
    }

    fn insets(&self) -> &Inset {
        &self.inset
    }
}
