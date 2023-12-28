use inverse_prufer::tree_edges;

#[doc(hidden)]
fn main() {
    let prufer = [4, 7, 3, 4];
    let res = tree_edges(&prufer);
    match res {
        Ok(edges) => println!("The edge set E(G) is:\n{:#?}", &edges),
        Err(e) => println!("{}", e.to_string()),
    }
}

#[cfg(test)]
mod tests;
