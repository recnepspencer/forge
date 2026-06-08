use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(F::DegeneracyKCoreFilter, "degeneracy_k_core_filter", "Degeneracy / k-core filter", T::GraphTheoreticBound, A::FiniteConflictGraph, "A 5-degenerate graph is greedily 6-colorable, so only nonempty 6-cores deserve serious lower-bound work.", "the 6-core is empty for a claimed 7-obstruction priority lane", "deterministic k-core peel record")
}
