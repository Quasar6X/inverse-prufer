//! Module containing the implementation of the inverse Prüfer algorithm.

/// Calculates the edge set of a given Prüfer code.
///
/// # Errors
///
/// If the given Prüfer code is invlaid the function returns the error variant.
/// See: [`error::InvalidPruferCode`] for more information.
pub fn tree_edges(prufer_code: &[u64]) -> Result<Vec<(u64, u64)>, error::InvalidPruferCode> {
    let vertecies = prufer_code.len() + 2;
    let mut vertex_set: Vec<i8> = [0].repeat(vertecies);

    for &code in prufer_code.iter() {
        if code > vertecies as u64 {
            return Err(error::InvalidPruferCode::new(code, prufer_code));
        }

        vertex_set[code as usize - 1] += 1;
    }

    let mut edges = Vec::with_capacity(vertecies + 1);

    for &code in prufer_code.iter() {
        for (j, v) in vertex_set.iter_mut().enumerate() {
            if *v == 0 {
                *v = -1;
                edges.push(((j + 1) as u64, code));
                vertex_set[code as usize - 1] -= 1;
                break;
            }
        }
    }

    fn create_last_pair(vertex_set: &[i8]) -> (u64, u64) {
        let res = vertex_set
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| (v == 0).then_some((i + 1) as u64))
            .take(2)
            .collect::<Vec<_>>();
        (res[0], res[1])
    }

    edges.push(create_last_pair(&vertex_set));
    Ok(edges)
}

/// Module for error handling.
pub mod error {

    /// Describes a Prüfer code that contains an invalid element.
    ///
    /// An element is considered invalid if it is larger than the length of
    /// the code + 2. Created by [`super::tree_edges`].
    #[derive(Debug, PartialEq)]
    pub struct InvalidPruferCode<'a> {
        invalid_value: u64,
        code: &'a [u64],
    }

    impl<'a> InvalidPruferCode<'a> {
        /// Constructs a new `InvalidPruferCode`.
        pub fn new(invalid_value: u64, code: &'a [u64]) -> Self {
            InvalidPruferCode {
                invalid_value,
                code,
            }
        }
    }

    impl<'a> ToString for InvalidPruferCode<'a> {
        /// Gives a mathematical explanation of why
        /// this [`InvalidPruferCode`] was created.
        ///
        /// # Example
        ///
        /// ```
        /// let res = inverse_prufer::tree_edges(&[4, 7, 3, 4]);
        /// # let res = match res {
        /// #   Ok(_) => panic!(),
        /// #   Err(e) => e,
        /// # };
        /// eprintln!("{}", res.to_string());
        /// ```
        /// Prints the formatted [`String`]:
        /// ```text
        /// Invalid value in code: 7
        ///     SEQ = [4, 7, 3, 4]
        ///     N := |SEQ| + 2 = 6
        ///     max(SEQ) = 7
        ///     max(SEQ) > N => Invalid prüfer code
        /// ```
        fn to_string(&self) -> String {
            let InvalidPruferCode {
                invalid_value,
                code,
            } = self;
            format!(
                r#"Invalid value in code: {invalid_value}
    SEQ = {:?}
    N := |SEQ| + 2 = {}
    max(SEQ) = {invalid_value}
    max(SEQ) > N => Invalid prüfer code"#,
                code,
                code.len() + 2,
            )
        }
    }
}
