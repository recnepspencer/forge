use worth_query::facade::runtime::{worth_query_domain, WorthQueryLowerRuntimeBoundaryEnvelope, WorthQueryLowerRuntimeExplanationRequest};

fn main() {
    let _ = worth_query_domain("worth.spatial")
        .for_lower_runtime_boundary_envelope(&envelope())
        .explains_store_backed_replay_gap(
            "routing.store_backed_replay",
            explanation_request(),
        )
        .review();
}

fn envelope() -> WorthQueryLowerRuntimeBoundaryEnvelope {
    todo!()
}

fn explanation_request() -> WorthQueryLowerRuntimeExplanationRequest {
    todo!()
}
