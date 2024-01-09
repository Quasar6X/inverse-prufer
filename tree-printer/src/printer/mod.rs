use std::io::Write;

use crate::tree_node;

pub mod traditional;

pub trait TreePrinter {
    fn print(root_node: impl tree_node::TreeNode, out: impl Write);
}
