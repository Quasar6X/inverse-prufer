# Inverse Prüfer calculator in rust

Calculates the resulting tree from a given Prüfer code. The tree is given
by an edge list.

## Example

```rust
use inverse_prufer::{error::InvalidPruferCode, tree_edges};

let prufer = [4, 1, 3, 4];
let res: Result<Vec<(usize, usize)>, InvalidPruferCode> = tree_edges(&prufer);
match res {
    Ok(edges) => println!("The edge set E(G) is:\n{:#?}", &edges),
    Err(e) => println!("{}", e.to_string()),
}
```
