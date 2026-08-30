/// Stable denial posture returned when a weak service cannot enter its owner.
///
/// The owner issues this value. `non_exhaustive` keeps composition callers from
/// constructing a counterfeit denial while preserving equality for one stable
/// closing-or-gone posture.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalOwnerUnavailable;
