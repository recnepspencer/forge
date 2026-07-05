mod current;
mod matrix;
mod row;
mod validation;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_failure_guards;

pub use current::current_worth_touched_graph_cross_family_closeout_matrix;
pub use matrix::WorthTouchedGraphCrossFamilyCloseoutMatrix;
pub use row::WorthTouchedGraphCrossFamilyCloseoutMatrixRow;
pub use validation::{
    WorthTouchedGraphCrossFamilyCloseoutMatrixError,
    WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind,
};

pub(crate) use current::closeout_matrix_from_authorities;
pub(crate) use current::current_matrix_authority;
pub(crate) use validation::validate_closeout_matrix;
