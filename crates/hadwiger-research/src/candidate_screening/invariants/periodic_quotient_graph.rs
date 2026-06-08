use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(F::PeriodicQuotientGraph, "periodic_quotient_graph", "Periodic quotient graph test", T::ExactCheckerReady, A::PeriodicTiling, "Periodic tilings must include wraparound conflicts across lattice translations.", "the quotient graph with translated tile conflicts is not 6-colorable or contradicts the proposed coloring", "exact translated-pair conflict and quotient-coloring certificate")
}
