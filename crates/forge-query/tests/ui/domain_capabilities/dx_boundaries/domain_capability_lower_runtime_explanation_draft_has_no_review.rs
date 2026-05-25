use forge_query::facade::runtime::{
    forge_query_domain, ForgeQueryLowerRuntimeBoundaryEnvelope, ForgeQueryLowerRuntimeExplanationRequest,
};

fn envelope() -> ForgeQueryLowerRuntimeBoundaryEnvelope {
    todo!()
}

fn explanation_request() -> ForgeQueryLowerRuntimeExplanationRequest {
    todo!()
}

fn main() {
    let _ = forge_query_domain("worth.spatial")
        .for_lower_runtime_boundary_envelope(&envelope())
        .explains_store_backed_replay_gap("replay.store_gap", explanation_request())
        .review();
}
