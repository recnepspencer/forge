use super::counters::PlanarBooleanLoopDecisionLogCounters;
use super::denial::{
    PlanarBooleanLoopDecisionLogDenial, PlanarBooleanLoopDecisionLogDenialKind as Kind,
};
use super::input::PlanarBooleanLoopDecisionLogInput;

pub(crate) fn validate_input(
    input: PlanarBooleanLoopDecisionLogInput<'_>,
    counters: &mut PlanarBooleanLoopDecisionLogCounters,
) -> Result<(), PlanarBooleanLoopDecisionLogDenial> {
    let request_identity = input.request().request_identity();
    for observed in [
        input.continuation_index().request_identity(),
        input.walk_outcomes().request_identity(),
        input.loop_candidates().request_identity(),
        input.denied_loop_candidates().request_identity(),
        input.reconstructed_loops().request_identity(),
        input.born_loops().request_identity(),
        input.island_partition().request_identity(),
        input.split_attribution().request_identity(),
        input.role_outcomes().request_identity(),
        input.degenerate_outcomes().request_identity(),
        input.loop_identity_map().request_identity(),
        input.persistent_name_map().request_identity(),
        input.subshape_signature_map().request_identity(),
    ] {
        if observed != request_identity {
            counters.denied_request_identity_mismatch();
            return Err(PlanarBooleanLoopDecisionLogDenial::new(
                Kind::RequestIdentityMismatch,
                observed,
                *counters,
                "loop decision log only admits products from one loop reconstruction request",
            ));
        }
    }
    Ok(())
}
