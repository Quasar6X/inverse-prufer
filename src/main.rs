#![doc(hidden)]

use clap::Parser;
use color_eyre::eyre::Result;
use inverse_prufer::{tree_edges, PruferCode};

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
}

#[cfg(test)]
mod tests;
