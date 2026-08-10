mod matrix;
mod posture;
mod row;

pub use matrix::{worth_query_lower_runtime_support_matrix, WorthQueryLowerRuntimeSupportMatrix};
pub use posture::{WorthQueryLowerRuntimeSupportDetail, WorthQueryLowerRuntimeSupportPosture};
pub use row::WorthQueryLowerRuntimeSupportRow;

pub(crate) use posture::support_posture_for_classification;
#[cfg(test)]
pub(crate) use posture::support_posture_for_closeout;

#[cfg(test)]
mod tests;
