# Inverse Prüfer calculator in rust

Calculates the resulting labeled tree from a given Prüfer code. The tree is given by an edge set.

## Run example

Command:
```sh
./inverse_prufer 4 1 3 4
```
Output:
```
The supplied Prüfer code is:
{4, 1, 3, 4}
The edge set is:
E(G) = [(2, 4), (5, 1), (1, 3), (3, 4), (4, 6)]
```

## Project stucture

- The algorithm can bo found under `src/lib.rs`.  
The function `tree_edges` calcualtes the labeled tree from the Prüfer code.
- There are some tests written in `src/test.rs`.
- The argument parsing part is in `src/main.rs`.
