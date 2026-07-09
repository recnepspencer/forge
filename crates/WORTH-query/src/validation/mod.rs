mod artifacts;
mod counters;
mod errors;
mod failure;
mod ordering;
mod pipeline;
mod predicate_state;
mod predicates;
mod projection;
mod report;
mod result_shape;
mod traversal;

pub use artifacts::{
    ValidatedOrderingEntry, ValidatedOrderingSet, ValidatedPredicateEntry, ValidatedPredicateSet,
    ValidatedProjectionEntry, ValidatedQueryArtifact, ValidatedQueryBundle,
    ValidatedResultShapeArtifact, ValidatedResultShapeBinding, ValidatedTraversalEntry,
};
pub use counters::QueryValidationCounters;
pub use errors::{QueryValidationError, ValidationFailureClass};
#[cfg(test)]
pub(crate) use failure::ValidationFailureArtifact;
pub use pipeline::validate_canonical_bundle;
#[cfg(test)]
pub(crate) use pipeline::validate_canonical_bundle_with_failure_artifact;
pub use report::{
    QueryValidationReport, ValidationEvent, ValidationRejectionMatrix, ValidationWarning,
};
