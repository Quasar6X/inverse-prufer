pub fn tree_edges(prufer_code: &[usize]) -> Result<Vec<(usize, usize)>, error::PruferError> {
    let vertecies = prufer_code.len() + 2;
    let mut vertex_set: Vec<i64> = [0].repeat(vertecies);

    for &code in prufer_code.iter() {
        if code > vertecies {
            return Err(error::PruferError::InvalidCode {
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

pub mod error {
    #[derive(Debug, PartialEq)]
    pub enum PruferError<'a> {
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
}
