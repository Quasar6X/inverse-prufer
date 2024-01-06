use inverse_prufer::{error::InvalidPruferCode, tree_edges, PruferCode};

#[test]
fn test_prufer_4() {
    const PRUFER_CODE: [u64; 4] = [4, 1, 3, 4];
    let res = PruferCode::try_from(PRUFER_CODE.as_slice());

    match res {
        Ok(code) => assert_eq!(
            [(2, 4), (5, 1), (1, 3), (3, 4), (4, 6)],
            tree_edges(&code).as_slice()
        ),
        Err(e) => assert!(false, "{}", e.to_string()),
    }
}

#[test]
fn test_prufer_5() {
    const PRUFER_CODE: [u64; 5] = [5, 1, 2, 4, 3];
    let res = PruferCode::try_from(PRUFER_CODE.as_slice());

    match res {
        Ok(code) => assert_eq!(
            [(6, 5), (5, 1), (1, 2), (2, 4), (4, 3), (3, 7)],
            tree_edges(&code).as_slice()
        ),
        Err(e) => assert!(false, "{}", e.to_string()),
    }
}

#[test]
fn test_prufer_6() {
    const PRUFER_CODE: [u64; 6] = [1, 1, 1, 1, 6, 5];
    let res = PruferCode::try_from(PRUFER_CODE.as_slice());

    match res {
        Ok(code) => assert_eq!(
            [(2, 1), (3, 1), (4, 1), (7, 1), (1, 6), (6, 5), (5, 8)],
            tree_edges(&code).as_slice()
        ),
        Err(e) => assert!(false, "{}", e.to_string()),
    }
}

#[test]
fn test_invalid_code_large() {
    const WRONG_CODE: u64 = 7;
    const PRUFER_CODE: [u64; 4] = [4, WRONG_CODE, 3, 4];
    let res = PruferCode::try_from(PRUFER_CODE.as_slice());

    assert_eq!(
        res,
        Err(InvalidPruferCode::ValueTooLarge {
            invalid_value: WRONG_CODE,
            code: &PRUFER_CODE
        })
    );
}

#[test]
fn test_invalid_code_zero() {
    const PRUFER_CODE: [u64; 4] = [4, 0, 3, 4];
    let res = PruferCode::try_from(PRUFER_CODE.as_slice());

    assert_eq!(
        res,
        Err(InvalidPruferCode::ValueIsZero { code: &PRUFER_CODE })
    )
}
