fn main() {
    let prufer = [1, 1, 1, 1, 6, 5];
    let edges = tree_edges(&prufer);
    println!("The edge set E(G) is:\n{:?}", &edges);
}

fn tree_edges(prufer_code: &[usize]) -> Vec<(usize, usize)> {
    let vertecies = prufer_code.len() + 2;
    let mut vertex_set: Vec<i64> = [0].repeat(vertecies);

    for &code in prufer_code.iter() {
        if code > vertecies {
            panic!(r#"Invalid code in sequence: "{code}""#);
        }

        vertex_set[code - 1] += 1;
    }

    let mut edges: Vec<(usize, usize)> = Vec::with_capacity(vertecies - 1);

    for &code in prufer_code.iter() {
        for (j, v) in vertex_set.iter_mut().enumerate() {
            if *v == 0 {
                *v = -1;
                edges.push((j + 1, code));
                vertex_set[code - 1] -= 1;
                break;
            }
        }
    }

    fn create_last_pair(vertex_set: &[i64]) -> (usize, usize) {
        let res = vertex_set
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v == 0 { Some(i + 1) } else { None })
            .take(2)
            .collect::<Vec<_>>();
        (res[0], res[1])
    }

    edges.push(create_last_pair(&vertex_set));
    edges
}

#[test]
fn test_1() {
    const PRUFER_CODE: [usize; 5] = [5, 1, 2, 4, 3];
    let edges = tree_edges(&PRUFER_CODE);
    assert_eq!(
        [(6, 5), (5, 1), (1, 2), (2, 4), (4, 3), (3, 7)],
        edges.as_slice()
    );
}

#[test]
fn test_2() {
    const PRUFER_CODE: [usize; 4] = [4, 1, 3, 4];
    let edges = tree_edges(&PRUFER_CODE);
    assert_eq!([(2, 4), (5, 1), (1, 3), (3, 4), (4, 6)], edges.as_slice());
}

#[test]
fn test_3() {
    const PRUFER_CODE: [usize; 6] = [1, 1, 1, 1, 6, 5];
    let edges = tree_edges(&PRUFER_CODE);
    assert_eq!(
        [(2, 1), (3, 1), (4, 1), (7, 1), (1, 6), (6, 5), (5, 8)],
        edges.as_slice()
    );
}
