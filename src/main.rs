use std::process::ExitCode;

use clap::Parser;
use inverse_prufer::{tree_edges, PruferCode};

#[derive(Parser, Debug)]
#[command(about, author, version)]
struct Cli {
    /// Prüfer sequence (example: 4 1 3 4)
    #[arg(name = "SEQ", required = true, value_delimiter = ' ')]
    code: Vec<u64>,
}

#[doc(hidden)]
fn main() -> ExitCode {
    let args = Cli::parse();
    let code = PruferCode::try_from(args.code.as_slice());

    match code {
        Ok(code) => {
            println!("The edge set is:\nE(G) = {:?}", tree_edges(&code));
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}", e.to_string());
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests;
