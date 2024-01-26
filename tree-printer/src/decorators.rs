use crate::{config::Inset, tree_node::TreeNode};

pub trait TreeNodeDecorationStrategy {
    fn decorated_content(&self) -> String;

    fn wrap_child(&self, child_node: &mut Box<dyn TreeNode>, index: usize);
}

pub struct TreeNodeDecorator<S: TreeNodeDecorationStrategy> {
    base_node: Box<dyn TreeNode>,
    decorable: bool,
    strategy: S,
}

impl<S: TreeNodeDecorationStrategy> TreeNodeDecorator<S> {
    pub fn new(base_node: Box<dyn TreeNode>, strategy: S) -> Self {
        Self::builder(base_node, strategy).build()
    }

    pub fn builder(base_node: Box<dyn TreeNode>, strategy: S) -> TreeNodeDecoratorBuilder<S> {
        TreeNodeDecoratorBuilder::new(base_node, strategy)
    }

    pub const fn decorated_tree_node(&self) -> &dyn TreeNode {
        &*self.base_node
    }
}

impl<S: TreeNodeDecorationStrategy> TreeNode for TreeNodeDecorator<S> {
    fn content(&self) -> String {
        if self.base_node.is_decorable() {
            self.strategy.decorated_content()
        } else {
            self.base_node.content()
        }
    }

    fn children(&self) -> &[Box<dyn TreeNode>] {
        self.base_node.children()
    }

    fn children_mut(&mut self) -> &mut [Box<dyn TreeNode>] {
        self.base_node.children_mut()
    }

    fn insets(&self) -> &Inset {
        self.base_node.insets()
    }

    fn is_decorable(&self) -> bool {
        self.decorable
    }

    fn is_placeholder(&self) -> bool {
        self.base_node.is_placeholder()
    }
}

pub struct TreeNodeDecoratorBuilder<S: TreeNodeDecorationStrategy> {
    base_node: Box<dyn TreeNode>,
    inherit: bool,
    decorable: bool,
    strategy: S,
}

impl<S: TreeNodeDecorationStrategy> TreeNodeDecoratorBuilder<S> {
    fn new(base_node: Box<dyn TreeNode>, strategy: S) -> Self {
        Self {
            decorable: base_node.is_decorable(),
            base_node,
            inherit: true,
            strategy,
        }
    }

    pub fn inherit(&mut self, inherit: bool) -> &mut Self {
        self.inherit = inherit;
        self
    }

    pub fn decorable(&mut self, decorable: bool) -> &mut Self {
        self.decorable = decorable;
        self
    }

    pub fn build(self) -> TreeNodeDecorator<S> {
        let Self {
            mut base_node,
            inherit,
            decorable,
            strategy,
        } = self;

        for (i, child) in base_node.children_mut().iter_mut().enumerate() {
            if inherit {
                strategy.wrap_child(child, i);
            }
        }

        TreeNodeDecorator {
            base_node,
            decorable,
            strategy,
        }
    }
}
