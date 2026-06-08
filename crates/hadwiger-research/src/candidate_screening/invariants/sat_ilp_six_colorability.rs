use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(F::SatIlpSixColorability, "sat_ilp_six_colorability", "SAT / ILP 6-colorability test", T::ExactCheckerReady, A::FiniteConflictGraph, "Direct six-colorability encoding decides whether the finite conflict graph is 6-colorable.", "the checked SAT/ILP six-color instance is UNSAT", "model replay for SAT or checked refutation certificate for UNSAT")
}
