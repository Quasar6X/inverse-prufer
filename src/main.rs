mod prufer;

fn main() {
    let prufer = [1, 1, 1, 1, 9, 5];
    let res = prufer::tree_edges(&prufer);
    match res {
        Ok(edges) => println!("The edge set E(G) is:\n{:#?}", &edges),
        Err(e) => println!("{}", e.to_string()),
    }
}

#[cfg(test)]
mod tests;
