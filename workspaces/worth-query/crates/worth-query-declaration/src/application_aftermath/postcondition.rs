//! Postcondition carried as a field of a correction mechanism.

/// Expected postcondition of a correction mechanism.
///
/// This is a field of the mechanism contract, never a variant axis that
/// duplicates mechanism kinds.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DeclaredAftermathPostcondition {
    ExactPriorTruth,
    InvariantRestored { invariant: String },
    BusinessPostcondition { identity: String },
}
