use worth_query_execution::facade::provider_session::{
    WorthQueryDecisionFactRequest, WorthQuerySessionEffectAuthority,
};

fn capture_through_effect_authority(
    authority: WorthQuerySessionEffectAuthority<'_>,
    request: WorthQueryDecisionFactRequest,
) {
    let _ = authority.capture_decision_read_set([request]);
}

fn main() {}
