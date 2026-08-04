//! Ordered second-order exact-tail contribution orchestration.

mod a_vertex;
mod b_vertex;
mod c_vertex;

use super::final_expansion::FinalExpansion;
use super::first_order::FirstOrderCrossExpansions;
use super::tail_expansion_scratch::TailExpansionScratch;
use super::translated_geometry::{CoordinateTails, LiftedBaseDeterminants, TranslatedGeometry};

pub(in crate::predicates::vendored::incircle::adaptive) struct ExactTailContext {
    pub(in crate::predicates::vendored::incircle::adaptive) geometry: TranslatedGeometry,
    pub(in crate::predicates::vendored::incircle::adaptive) tails: CoordinateTails,
    pub(in crate::predicates::vendored::incircle::adaptive) lifted: LiftedBaseDeterminants,
    pub(in crate::predicates::vendored::incircle::adaptive) aa: [f64; 4],
    pub(in crate::predicates::vendored::incircle::adaptive) bb: [f64; 4],
    pub(in crate::predicates::vendored::incircle::adaptive) cc: [f64; 4],
}

pub(super) fn apply(
    context: &ExactTailContext,
    first: &FirstOrderCrossExpansions,
    final_expansion: &mut FinalExpansion,
    scratch: &mut TailExpansionScratch,
) {
    a_vertex::apply(context, first, final_expansion, scratch);
    b_vertex::apply(context, first, final_expansion, scratch);
    c_vertex::apply(context, first, final_expansion, scratch);
}
