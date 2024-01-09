#![doc(hidden)]

use clap::Parser;
use color_eyre::eyre::Result;
use inverse_prufer::{tree_edges, PruferCode};
use std::{io::stdout, process::ExitCode};

use tree_printer::{
    printer::{
        traditional::{aligner::DefaultAligner, liner::DefaultLiner, TraditionalTreePrinter},
        TreePrinter,
    },
    tree_node::{PlaceholderTreeNode, SimpleTreeNode, TreeNode},
};

#[derive(Parser, Debug)]
#[command(about, author, version)]
struct Cli {
    /// Prüfer sequence (example: 4 1 3 4)
    #[arg(name = "SEQ", required = true, value_delimiter = ' ')]
    code: Vec<u64>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let Cli { code } = Cli::parse();
    let code = PruferCode::try_from(code.as_slice())?;

    println!("The supplied Prüfer code is:\n{code}");
    println!("The edge set is:\nE(G) = {:?}", tree_edges(&code));

    Ok(())

    // let root_node = build_tree();

    // TraditionalTreePrinter::<_, DefaultAligner, DefaultLiner>::default().print(root_node, stdout());
    // ExitCode::SUCCESS
}

// fn build_tree() -> Box<dyn TreeNode> {
//     let mut root_node = Box::new(SimpleTreeNode::new("root"));
//     let mut sub_node1 = Box::new(SimpleTreeNode::new(
//         "SUB asdf\nSSS fdsa\nxxx yyy",
//     ));
//     let mut sub_node2 = Box::new(SimpleTreeNode::new("lorem ipsum"));
//     let mut sub_node3 = Box::new(SimpleTreeNode::new("ggggg"));
//     let sub_sub_node11 = Box::new(SimpleTreeNode::new("AAA"));
//     let sub_sub_node12 = Box::new(SimpleTreeNode::new("BBB"));
//     let sub_sub_node21 = Box::new(SimpleTreeNode::new("CCC"));
//     let sub_sub_node22 = Box::new(SimpleTreeNode::new("DDD"));
//     let mut sub_sub_node23 = Box::new(SimpleTreeNode::new("EEE"));
//     let sub_sub_node24 = Box::new(SimpleTreeNode::new("FFF"));
//     let mut sub_sub_node31 = Box::new(SimpleTreeNode::new("GGG"));
//     let sub_sub_sub_node231 = Box::new(SimpleTreeNode::new("(eee)"));
//     let sub_sub_sub_node232 = Box::new(SimpleTreeNode::new("(eee2)"));
//     let sub_sub_sub_node311 = Box::new(SimpleTreeNode::new("(ggg)"));

//     let placeholder_node = Box::new(PlaceholderTreeNode::new());

//     sub_node1.add_child(sub_sub_node11);
//     sub_node1.add_child(sub_sub_node12);
//     root_node.add_child(sub_node1);
//     // root_node.add_child(placeholder_node);
//     sub_node2.add_child(sub_sub_node21);
//     sub_node2.add_child(sub_sub_node22);
//     sub_sub_node23.add_child(sub_sub_sub_node231);
//     sub_sub_node23.add_child(sub_sub_sub_node232);
//     sub_node2.add_child(sub_sub_node23);
//     sub_node2.add_child(sub_sub_node24);
//     root_node.add_child(sub_node2);
//     sub_sub_node31.add_child(sub_sub_sub_node311);
//     sub_node3.add_child(sub_sub_node31);
//     root_node.add_child(sub_node3);

//     root_node
// }

fn build_tree() -> Box<dyn TreeNode> {
    let mut root = Box::new(SimpleTreeNode::new("ROOT"));
    root.add_child(Box::new(SimpleTreeNode::new("Child #1")));
    root.add_child(Box::new(PlaceholderTreeNode::new()));
    root
}

#[cfg(test)]
mod tests;
