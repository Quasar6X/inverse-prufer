use clap::Parser;
use inverse_prufer::tree_edges;

#[derive(Parser, Debug)]
#[command(about, author, version)]
struct Cli {
    /// Prüfer sequence (example: 4 1 3 4)
    #[arg(name = "SEQ", required = true, value_delimiter = ' ', value_parser = clap::value_parser!(u64).range(1..))]
    code: Vec<u64>,
}

#[doc(hidden)]
fn main() {
    let args = Cli::parse();
    let res = tree_edges(&args.code);
    match res {
        Ok(edges) => println!("The edge set E(G) is:\n{:?}", &edges),
        Err(e) => eprintln!("{}", e.to_string()),
    }
}

#[cfg(test)]
mod tests;
