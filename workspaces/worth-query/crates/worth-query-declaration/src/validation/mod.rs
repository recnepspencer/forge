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

#[cfg(test)]
mod native_predicate_contract_tests;

pub use artifacts::{
    ValidatedOrderingEntry, ValidatedOrderingSet, ValidatedPredicateEntry, ValidatedPredicateSet,
    ValidatedProjectionEntry, ValidatedQueryArtifact, ValidatedQueryBundle,
    ValidatedResultShapeArtifact, ValidatedResultShapeBinding, ValidatedTraversalEntry,
};
pub use counters::QueryValidationCounters;
pub use errors::{QueryValidationError, ValidationFailureClass};
pub use pipeline::validate_canonical_bundle;
pub use report::{
    QueryValidationReport, ValidationEvent, ValidationRejectionMatrix, ValidationWarning,
};
