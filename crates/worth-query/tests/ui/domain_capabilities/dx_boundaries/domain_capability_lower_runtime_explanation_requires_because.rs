#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::domain::WorthQueryLowerRuntimeExplanationRequest;
use worth_query::facade::runtime::WorthQueryLowerRuntimeBoundaryEnvelope;

fn main() {
    let installation = installed_domain::install("explanation-requires-because");
    let _ = installation
        .contributions()
        .for_lower_runtime_boundary_envelope(&envelope()).expect("installed contribution authority must remain current")
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
