mod canonical_order;
mod digest;
mod fact;
mod input_basis;
mod kind;
mod math_adapter;
mod outcome;

pub(crate) use canonical_order::{
    canonical_cyclic_orient2d_points, canonical_planar_coordinate_bits,
};
pub(crate) use digest::digest_parts;
pub use fact::{PlanarPredicateFactReceipt, PlanarPredicatePerformanceCounters};
pub use input_basis::{PlanarPredicateCoincidencePolicy, PlanarPredicateInputBasis};
pub use kind::PlanarPredicateKind;
pub(crate) use math_adapter::{evaluate_planar_predicate_authority, PlanarPredicateMathEvaluation};
pub use outcome::{
    PlanarPredicateAuthorityDenial, PlanarPredicateAuthorityPosture,
    PlanarPredicateEvaluationFailureKind,
};
