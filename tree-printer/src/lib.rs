use std::io::Write;

pub mod tree_node;
pub mod config;
pub mod traditional;
pub mod line_buffer;

pub trait TreePrinter {
    fn print(root_node: impl tree_node::TreeNode, out: impl Write);
}

#[cfg(test)]
mod tests {
}
