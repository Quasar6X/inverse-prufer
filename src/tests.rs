use inverse_prufer::{error::InvalidPruferCode, tree_edges};

#[test]
fn test_prufer_4() {
    const PRUFER_CODE: [u64; 4] = [4, 1, 3, 4];
    let res = tree_edges(&PRUFER_CODE);

    match res {
        Ok(edges) => assert_eq!([(2, 4), (5, 1), (1, 3), (3, 4), (4, 6)], edges.as_slice()),
        Err(e) => assert!(false, "{}", e.to_string()),
    }
}

#[test]
fn test_prufer_5() {
    const PRUFER_CODE: [u64; 5] = [5, 1, 2, 4, 3];
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
    const PRUFER_CODE: [u64; 6] = [1, 1, 1, 1, 6, 5];
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
    const WRONG_CODE: u64 = 7;
    const PRUFER_CODE: [u64; 4] = [4, WRONG_CODE, 3, 4];
    let res = tree_edges(&PRUFER_CODE);

    assert_eq!(res, Err(InvalidPruferCode::new(WRONG_CODE, &PRUFER_CODE)));
}
