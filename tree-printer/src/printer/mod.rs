use std::{io::Write, marker::PhantomData};

use crate::tree_node::TreeNode;

pub mod traditional;

pub trait TreePrinter<W: Write> {
    fn print(&self, root_node: Box<dyn TreeNode>, out: W);
}

pub struct MappingTreePrinter<W, P, F>
where
    W: Write,
    P: TreePrinter<W>,
    F: Fn(Box<dyn TreeNode>) -> Box<dyn TreeNode>,
{
    printer: P,
    mapper: F,
    _phantom: PhantomData<W>,
}

impl<W, P, F> MappingTreePrinter<W, P, F>
where
    W: Write,
    P: TreePrinter<W>,
    F: Fn(Box<dyn TreeNode>) -> Box<dyn TreeNode>,
{
    pub const fn new(printer: P, mapper: F) -> Self {
        Self {
            printer,
            mapper,
            _phantom: PhantomData,
        }
    }
}

impl<W, P, F> TreePrinter<W> for MappingTreePrinter<W, P, F>
where
    W: Write,
    P: TreePrinter<W>,
    F: Fn(Box<dyn TreeNode>) -> Box<dyn TreeNode>,
{
    fn print(&self, root_node: Box<dyn TreeNode>, out: W) {
        self.printer.print((self.mapper)(root_node), out);
    }
}
