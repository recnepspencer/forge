//! Adaptive incircle cascade orchestration.

mod exact_stage;
mod final_expansion;
mod first_order;
mod second_order;
mod stage_b;
mod stage_c;
mod tail_expansion_scratch;
mod translated_geometry;

#[cfg(test)]
mod oracle_tests;

#[inline]
pub(in crate::predicates) fn incircleadapt(
    pa: [f64; 2],
    pb: [f64; 2],
    pc: [f64; 2],
    pd: [f64; 2],
    permanent: f64,
) -> f64 {
    match stage_b::run(pa, pb, pc, pd, permanent) {
        stage_b::StageBResult::Resolved(det) => det,
        stage_b::StageBResult::Continue(state) => match stage_c::run(state) {
            stage_c::StageCResult::Resolved(det) => det,
            stage_c::StageCResult::Continue(state) => exact_stage::run(state),
        },
    }
}
