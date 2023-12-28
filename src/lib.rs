//! Module containing the implementation of the inverse Prüfer algorithm.

/// Calculates the edge set of a given Prüfer code.
///
/// # Errors
///
/// If the given Prüfer code is invlaid the function returns the error variant.
/// See: [`error::InvalidPruferCode`] for more information.
pub fn tree_edges(prufer_code: &[usize]) -> Result<Vec<(usize, usize)>, error::InvalidPruferCode> {
    let vertecies = prufer_code.len() + 2;
    let mut vertex_set: Vec<i64> = [0].repeat(vertecies);

    for &code in prufer_code.iter() {
        if code > vertecies {
            return Err(error::InvalidPruferCode::new(code, prufer_code));
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
            .filter_map(|(i, &v)| (v == 0).then_some(i + 1))
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
        invalid_value: usize,
        code: &'a [usize],
    }

    impl<'a> InvalidPruferCode<'a> {
        /// Constructs a new `InvalidPruferCode`.
        pub fn new(invalid_value: usize, code: &'a [usize]) -> Self {
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
        /// println!("{}", res.to_string());
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
            match self {
                InvalidPruferCode {
                    invalid_value,
                    code,
                } => {
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
    }
}
