mod absence_law;
pub(crate) mod contract;
mod equivalence_basis;
mod shape;

pub use absence_law::AbsenceLaw;
pub use contract::AspectContract;
pub use equivalence_basis::AspectEquivalenceBasis;
pub use shape::{AspectShape, OpaqueAspectType, ReferenceAspectType};
