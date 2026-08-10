mod admission;
mod binding;
mod types;

pub use admission::QueryContextBindingSource;
pub use binding::AdmittedQueryBasisContext;
#[cfg(test)]
pub use binding::QueryBasisContextBinding;
pub use types::{
    ComparisonBasisFamily, HistoricalAdmissionClass, QueryBasisContextRequest,
    QueryContextAdmissionError, QueryContextAdmissionFailureClass, QueryContextDriftOutcome,
    QueryContextFamily,
};

pub(crate) use admission::{
    admit_legacy_query_basis_context, bind_legacy_query_basis_context, historical_admission_of,
    materialization_identity_of, preview_identity_of,
};
pub(crate) use binding::QueryBasisBindingEvidenceView;
