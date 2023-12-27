fn main() {
    let prufer = [1, 1, 1, 1, 9, 5];
    let res = tree_edges(&prufer);
    match res {
        Ok(edges) => println!("The edge set E(G) is:\n{:#?}", &edges),
        Err(e) => println!("{}", e.to_string()),
    }
}

fn tree_edges(prufer_code: &[usize]) -> Result<Vec<(usize, usize)>, PruferError> {
    let vertecies = prufer_code.len() + 2;
    let mut vertex_set: Vec<i64> = [0].repeat(vertecies);

    for &code in prufer_code.iter() {
        if code > vertecies {
            return Err(PruferError::InvalidCode {
                code,
                seq: prufer_code,
            });
        }

        vertex_set[code - 1] += 1;
    }

    let mut edges: Vec<(usize, usize)> = Vec::with_capacity(vertecies + 1);

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
    Ok(edges)
}

#[derive(Debug, PartialEq)]
enum PruferError<'a> {
    InvalidCode { code: usize, seq: &'a [usize] },
}

impl<'a> ToString for PruferError<'a> {
    fn to_string(&self) -> String {
        match self {
            PruferError::InvalidCode { code, seq } => {
                format!(
                    r#"Invalid code in sequence:
    P = {:?}
    N = |P| + 2 = {}
    max(P) = {}
    max(P) > N => Invalid prüfer sequence"#,
                    seq,
                    seq.len() + 2,
                    code
                )
            }
        }
    }
}

#[test]
fn test_prufer_4() {
    const PRUFER_CODE: [usize; 4] = [4, 1, 3, 4];
    let res = tree_edges(&PRUFER_CODE);

    match res {
        Ok(edges) => assert_eq!([(2, 4), (5, 1), (1, 3), (3, 4), (4, 6)], edges.as_slice()),
        Err(e) => assert!(false, "{}", e.to_string()),
    }
}

#[test]
fn test_prufer_5() {
    const PRUFER_CODE: [usize; 5] = [5, 1, 2, 4, 3];
    let res = tree_edges(&PRUFER_CODE);

    match res {
        Ok(edges) => assert_eq!(
            [(6, 5), (5, 1), (1, 2), (2, 4), (4, 3), (3, 7)],
            edges.as_slice()
        ),
        Err(e) => assert!(false, "{}", e.to_string()),
    }
}

#[test]
fn test_prufer_6() {
    const PRUFER_CODE: [usize; 6] = [1, 1, 1, 1, 6, 5];
    let res = tree_edges(&PRUFER_CODE);

    match res {
        Ok(edges) => assert_eq!(
            [(2, 1), (3, 1), (4, 1), (7, 1), (1, 6), (6, 5), (5, 8)],
            edges.as_slice()
        ),
        Err(e) => assert!(false, "{}", e.to_string()),
    }
}

#[test]
fn test_invalid_code() {
    const WRONG_CODE: usize = 7;
    const PRUFER_CODE: [usize; 4] = [4, WRONG_CODE, 3, 4];
    let res = tree_edges(&PRUFER_CODE);

    assert_eq!(
        res,
        Err(PruferError::InvalidCode {
            code: WRONG_CODE,
            seq: &PRUFER_CODE
        })
    );
}
