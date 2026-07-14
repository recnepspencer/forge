mod admission;
mod artifacts;
mod bindings;
mod bundle_state;
mod errors;
mod ordering;
pub(crate) mod pipeline;
mod predicates;
mod projection;
mod query_artifact;
mod result_shape_artifact;
mod traversal;

pub use artifacts::{
    CanonicalOrderingEntry, CanonicalPredicateEntry, CanonicalPredicateFamily,
    CanonicalPredicateOperand, CanonicalProjectionEntry, CanonicalQueryArtifact,
    CanonicalResultField, CanonicalResultShapeArtifact, CanonicalScalarSet,
    CanonicalTraversalEntry,
};
pub use bundle_state::CanonicalQueryBundle;
pub use errors::{CanonicalizationFailureClass, QueryCanonicalizationError};
pub(crate) use pipeline::canonicalize_request;
