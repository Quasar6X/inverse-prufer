//! Module containing the implementation of the inverse Prüfer algorithm.

use std::fmt::Display;

/// Calculates the edge set of a given Prüfer code.
///
/// # Panics
///
/// If the target architecture cannot represent the max value in
/// the given Prüfer code as [`usize`].
#[must_use]
pub fn tree_edges(PruferCode { code: prufer_code }: &PruferCode) -> Vec<(u64, u64)> {
    // find last two zeros in vertex set and return the corresponding vertecies
    // for the last edge pair
    fn create_last_pair(vertex_set: &[i8]) -> (u64, u64) {
        let res = vertex_set
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| (v == 0).then_some((i + 1) as u64))
            .take(2)
            .collect::<Vec<_>>();
        (res[0], res[1])
    }

    let vertecies = prufer_code.len() + 2;
    let mut vertex_set: Vec<i8> = [0].repeat(vertecies);

    // count the occurence of vertecies in `prufer_code`
    for &value in prufer_code {
        let index = usize::try_from(value - 1).expect("Index out of bounds on current target");
        vertex_set[index] += 1;
    }

    let mut edges = Vec::with_capacity(vertecies - 1);

    // create edge pairs
    for &value in prufer_code {
        for (j, v) in vertex_set.iter_mut().enumerate() {
            if *v == 0 {
                *v = -1;
                edges.push((j as u64 + 1, value));
                let index =
                    usize::try_from(value - 1).expect("Index out of bounds on current target");
                vertex_set[index] -= 1;
                break;
            }
        }
    }

    edges.push(create_last_pair(&vertex_set));
    edges
}

/// Represents a valid Prüfer code
///
/// Every number in the slice is smaller or equal to the length
/// of the code plus two.
#[derive(Debug, PartialEq, Eq)]
pub struct PruferCode {
    code: Vec<u64>,
}

impl TryFrom<&[u64]> for PruferCode {
    type Error = error::InvalidPruferCode;

    /// Validates the given Prüfer code
    ///
    // # Errors
    ///
    /// If the given Prüfer code is invlaid the function returns the error variant.
    /// See: [`error::InvalidPruferCode`] for more information.
    fn try_from(prufer_code: &[u64]) -> Result<Self, Self::Error> {
        let n = prufer_code.len() + 2;
        for &value in prufer_code {
            if value == 0 {
                return Err(error::InvalidPruferCode::ValueIsZero {
                    code: prufer_code.to_owned(),
                });
            }

            if value > n as u64 {
                return Err(error::InvalidPruferCode::ValueTooLarge {
                    invalid_value: value,
                    code: prufer_code.to_owned(),
                });
            }
        }

        Ok(Self {
            code: prufer_code.to_owned(),
        })
    }
}

impl Display for PruferCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (i, value) in self.code.iter().enumerate() {
            write!(f, "{value}")?;
            if i != self.code.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

/// Module for error handling.
pub mod error {
    use std::{error::Error, fmt::Display};

    /// Describes a Prüfer code that contains an invalid element.
    ///
    /// An element is considered invalid if it is larger than the length of
    /// the code + 2, or is 0. Created by [`super::tree_edges`].
    #[derive(Debug, PartialEq, Eq)]
    pub enum InvalidPruferCode {
        ValueTooLarge { invalid_value: u64, code: Vec<u64> },
        ValueIsZero { code: Vec<u64> },
    }

    /// Gives a mathematical explanation of why
    /// this [`InvalidPruferCode`] was created.
    ///
    /// # Examples
    ///
    /// ```
    /// # use inverse_prufer::PruferCode;
    /// let res = PruferCode::try_from([4, 7, 3, 4].as_slice());
    /// # let res = match res {
    /// #   Ok(_) => panic!(),
    /// #   Err(e) => e,
    /// # };
    /// assert_eq!(res.to_string(),
    /// r#"Invalid value in code: 7
    ///     SEQ = [4, 7, 3, 4]
    ///     N := |SEQ| + 2 = 6
    ///     max(SEQ) = 7
    ///     max(SEQ) > N => Invalid prüfer code"#);
    /// ```
    ///
    /// ```
    /// # use inverse_prufer::PruferCode;
    /// let res = PruferCode::try_from([4, 0, 3, 4].as_slice());
    /// # let res = match res {
    /// #   Ok(_) => panic!(),
    /// #   Err(e) => e,
    /// # };
    /// assert_eq!(res.to_string(),
    /// r#"Invalid value in code: 0
    ///     SEQ = [4, 0, 3, 4]
    ///     0 ∈ SEQ => Invalid prüfer code"#);
    /// ```
    impl Display for InvalidPruferCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::ValueTooLarge {
                    invalid_value,
                    code,
                } => write!(
                    f,
                    r#"Invalid value in code: {invalid_value}
    SEQ = {code:?}
    N := |SEQ| + 2 = {}
    max(SEQ) = {invalid_value}
    max(SEQ) > N => Invalid prüfer code"#,
                    code.len() + 2,
                ),
                Self::ValueIsZero { code } => write!(
                    f,
                    r#"Invalid value in code: 0
    SEQ = {code:?}
    0 ∈ SEQ => Invalid prüfer code"#
                ),
            }
        }
    }

    impl Error for InvalidPruferCode {}
}
