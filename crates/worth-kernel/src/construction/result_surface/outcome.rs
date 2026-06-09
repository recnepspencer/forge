#[path = "geometry_recovery.rs"]
mod geometry_recovery;

#[path = "rejection_facts.rs"]
mod rejection_facts;

#[path = "outcome_rejection.rs"]
mod outcome_rejection;

#[cfg(test)]
pub use crate::construction::tests::support::prepared_outcome::{
    prepare_primitive_construction_outcome, PrimitiveConstructionPreparedOutcome,
};
#[cfg(test)]
pub(crate) use geometry_recovery::PrimitiveConstructionRecoveryAction;
#[cfg(test)]
pub use geometry_recovery::{
    GeometryRecoveryAction, GeometryRecoveryActionFactReceipt, GeometryRecoverySourcePosture,
    GeometryRecoveryTargetScope,
};
#[cfg(test)]
pub use outcome_rejection::PrimitiveConstructionRejectedOutcome;
#[allow(unused_imports)]
pub(crate) use rejection_facts::PrimitiveConstructionRejectedFacts;
#[allow(dead_code)]
pub(crate) type PrimitiveConstructionRejectionClass =
    outcome_rejection::PrimitiveConstructionRejectionClass;
#[cfg(test)]
pub(crate) use outcome_rejection::PrimitiveConstructionRejectionLocality;
pub(crate) use rejection_facts::prepare_primitive_construction_rejected_facts;
#[cfg(test)]
pub(crate) fn rejected_outcome(
    family: super::request::PrimitiveConstructionFamily,
    error: &super::result::PrimitiveConstructionResultError,
) -> outcome_rejection::PrimitiveConstructionRejectedOutcome {
    outcome_rejection::rejected_outcome(family, error)
}
#[cfg(test)]
#[path = "../tests/outcome.rs"]
mod tests;
