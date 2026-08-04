//! Stage C coordinate-tail correction and bound evaluation.

use super::super::super::parameters::{abs, PARAMS};
use super::exact_stage::ExactStageState;
use super::stage_b::StageBState;

pub(in crate::predicates::vendored::incircle::adaptive) enum StageCResult {
    Resolved(f64),
    Continue(ExactStageState),
}

pub(super) fn run(state: StageBState) -> StageCResult {
    let StageBState {
        input,
        geometry,
        lifted,
        final_expansion,
        mut det,
    } = state;
    let tails = geometry.coordinate_tails(&input);
    if tails.all_zero() {
        return StageCResult::Resolved(det);
    }
    let errbound = PARAMS.iccerrbound_c * input.permanent + PARAMS.resulterrbound * abs(det);
    det += (geometry.adx * geometry.adx + geometry.ady * geometry.ady)
        * (geometry.bdx * tails.cdytail + geometry.cdy * tails.bdxtail
            - (geometry.bdy * tails.cdxtail + geometry.cdx * tails.bdytail))
        + 2.0
            * (geometry.adx * tails.adxtail + geometry.ady * tails.adytail)
            * (geometry.bdx * geometry.cdy - geometry.bdy * geometry.cdx)
        + ((geometry.bdx * geometry.bdx + geometry.bdy * geometry.bdy)
            * (geometry.cdx * tails.adytail + geometry.ady * tails.cdxtail
                - (geometry.cdy * tails.adxtail + geometry.adx * tails.cdytail))
            + 2.0
                * (geometry.bdx * tails.bdxtail + geometry.bdy * tails.bdytail)
                * (geometry.cdx * geometry.ady - geometry.cdy * geometry.adx))
        + ((geometry.cdx * geometry.cdx + geometry.cdy * geometry.cdy)
            * (geometry.adx * tails.bdytail + geometry.bdy * tails.adxtail
                - (geometry.ady * tails.bdxtail + geometry.bdx * tails.adytail))
            + 2.0
                * (geometry.cdx * tails.cdxtail + geometry.cdy * tails.cdytail)
                * (geometry.adx * geometry.bdy - geometry.ady * geometry.bdx));
    if det >= errbound || -det >= errbound {
        return StageCResult::Resolved(det);
    }
    StageCResult::Continue(ExactStageState::new(
        geometry,
        tails,
        lifted,
        final_expansion,
    ))
}
