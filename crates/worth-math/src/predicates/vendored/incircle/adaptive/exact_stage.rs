//! Exact-stage conditional lifts and ordered contribution orchestration.

use super::super::super::expansion::{square, two_two_sum};
use super::final_expansion::FinalExpansion;
use super::first_order;
use super::second_order::{self, ExactTailContext};
use super::tail_expansion_scratch::TailExpansionScratch;
use super::translated_geometry::{CoordinateTails, LiftedBaseDeterminants, TranslatedGeometry};

pub(in crate::predicates::vendored::incircle::adaptive) struct ExactStageState {
    pub(in crate::predicates::vendored::incircle::adaptive) geometry: TranslatedGeometry,
    pub(in crate::predicates::vendored::incircle::adaptive) tails: CoordinateTails,
    pub(in crate::predicates::vendored::incircle::adaptive) lifted: LiftedBaseDeterminants,
    pub(in crate::predicates::vendored::incircle::adaptive) final_expansion: FinalExpansion,
}

impl ExactStageState {
    pub(in crate::predicates::vendored::incircle::adaptive) fn new(
        geometry: TranslatedGeometry,
        tails: CoordinateTails,
        lifted: LiftedBaseDeterminants,
        final_expansion: FinalExpansion,
    ) -> Self {
        Self {
            geometry,
            tails,
            lifted,
            final_expansion,
        }
    }
}

pub(super) fn run(state: ExactStageState) -> f64 {
    let aa = if state.tails.bdxtail != 0.0
        || state.tails.bdytail != 0.0
        || state.tails.cdxtail != 0.0
        || state.tails.cdytail != 0.0
    {
        let [adxadx0, adxadx1] = square(state.geometry.adx);
        let [adyady0, adyady1] = square(state.geometry.ady);
        two_two_sum(adxadx1, adxadx0, adyady1, adyady0)
    } else {
        [0.; 4]
    };
    let bb = if state.tails.cdxtail != 0.0
        || state.tails.cdytail != 0.0
        || state.tails.adxtail != 0.0
        || state.tails.adytail != 0.0
    {
        let [bdxbdx0, bdxbdx1] = square(state.geometry.bdx);
        let [bdybdy0, bdybdy1] = square(state.geometry.bdy);
        two_two_sum(bdxbdx1, bdxbdx0, bdybdy1, bdybdy0)
    } else {
        [0.; 4]
    };
    let cc = if state.tails.adxtail != 0.0
        || state.tails.adytail != 0.0
        || state.tails.bdxtail != 0.0
        || state.tails.bdytail != 0.0
    {
        let [cdxcdx0, cdxcdx1] = square(state.geometry.cdx);
        let [cdycdy0, cdycdy1] = square(state.geometry.cdy);
        two_two_sum(cdxcdx1, cdxcdx0, cdycdy1, cdycdy0)
    } else {
        [0.; 4]
    };
    let context = ExactTailContext {
        geometry: state.geometry,
        tails: state.tails,
        lifted: state.lifted,
        aa,
        bb,
        cc,
    };
    let mut final_expansion = state.final_expansion;
    let mut scratch = TailExpansionScratch::new();
    let first = first_order::apply(&context, &mut final_expansion, &mut scratch);
    second_order::apply(&context, &first, &mut final_expansion, &mut scratch);
    final_expansion.highest_component()
}
