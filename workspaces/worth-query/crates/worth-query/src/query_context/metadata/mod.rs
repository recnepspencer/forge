mod basis;
mod diff;

pub use basis::{QueryBasisMetadata, QueryBasisResultBundle};
pub use diff::{DiffQueryMetadata, QueryDiffResultBundle};

pub(crate) use basis::{
    attach_legacy_query_basis_metadata, build_legacy_query_basis_result_bundle,
};
pub(crate) use diff::{attach_diff_query_metadata, build_query_diff_result_bundle};
