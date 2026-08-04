//! Stage B translated-geometry and initial-bound evaluation.

use super::super::super::parameters::PARAMS;
use super::final_expansion::FinalExpansion;
use super::translated_geometry::{AdaptiveInput, LiftedBaseDeterminants, TranslatedGeometry};

pub(in crate::predicates::vendored::incircle::adaptive) enum StageBResult {
    Resolved(f64),
    Continue(StageBState),
}

pub(in crate::predicates::vendored::incircle::adaptive) struct StageBState {
    pub(in crate::predicates::vendored::incircle::adaptive) input: AdaptiveInput,
    pub(in crate::predicates::vendored::incircle::adaptive) geometry: TranslatedGeometry,
    pub(in crate::predicates::vendored::incircle::adaptive) lifted: LiftedBaseDeterminants,
    pub(in crate::predicates::vendored::incircle::adaptive) final_expansion: FinalExpansion,
    pub(in crate::predicates::vendored::incircle::adaptive) det: f64,
}

pub(super) fn run(
    pa: [f64; 2],
    pb: [f64; 2],
    pc: [f64; 2],
    pd: [f64; 2],
    permanent: f64,
) -> StageBResult {
    let input = AdaptiveInput::new(pa, pb, pc, pd, permanent);
    let (geometry, lifted) = TranslatedGeometry::from_input(&input);
    let final_expansion = FinalExpansion::from_initial(&lifted);
    let det = final_expansion.initial_sum();
    let errbound = PARAMS.iccerrbound_b * permanent;
    if det >= errbound || -det >= errbound {
        StageBResult::Resolved(det)
    } else {
        StageBResult::Continue(StageBState {
            input,
            geometry,
            lifted,
            final_expansion,
            det,
        })
    }
}
