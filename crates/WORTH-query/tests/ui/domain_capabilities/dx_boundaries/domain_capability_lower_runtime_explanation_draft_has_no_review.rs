use worth_query::facade::runtime::{
    worth_query_domain, WorthQueryLowerRuntimeBoundaryEnvelope, WorthQueryLowerRuntimeExplanationRequest,
};

fn envelope() -> WorthQueryLowerRuntimeBoundaryEnvelope {
    todo!()
}

fn explanation_request() -> WorthQueryLowerRuntimeExplanationRequest {
    todo!()
}

fn main() {
    let _ = worth_query_domain("worth.spatial")
        .for_lower_runtime_boundary_envelope(&envelope())
        .explains_store_backed_replay_gap("replay.store_gap", explanation_request())
        .review();
}
