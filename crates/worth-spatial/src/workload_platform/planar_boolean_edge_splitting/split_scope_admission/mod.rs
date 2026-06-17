mod counters;
mod denial;
mod identity;
mod input;
mod policy;
mod policy_outcome;
mod scope_admission;
mod scope_class;
mod validation;

#[cfg(test)]
mod tests;

pub use counters::PlanarBooleanEdgeSplitScopeAdmissionCounters;
pub use denial::{
    PlanarBooleanEdgeSplitScopeAdmissionDenial, PlanarBooleanEdgeSplitScopeAdmissionDenialKind,
};
pub use input::PlanarBooleanEdgeSplitScopeAdmissionInput;
pub use policy::{
    PlanarBooleanEdgeSplitDegeneracyPolicy, PlanarBooleanEdgeSplitDeterminismPolicy,
    PlanarBooleanEdgeSplitOverlapPolicy,
};
pub use policy_outcome::{
    PlanarBooleanEdgeSplitPolicyOutcome, PlanarBooleanEdgeSplitPolicyOutcomeKind,
};
pub use scope_admission::PlanarBooleanEdgeSplitScopeAdmission;
pub use scope_class::PlanarBooleanEdgeSplitScopeClass;
