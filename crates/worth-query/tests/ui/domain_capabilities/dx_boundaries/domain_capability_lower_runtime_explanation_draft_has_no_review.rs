#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::domain::WorthQueryLowerRuntimeExplanationRequest;
use worth_query::facade::runtime::WorthQueryLowerRuntimeBoundaryEnvelope;

fn envelope() -> WorthQueryLowerRuntimeBoundaryEnvelope {
    todo!()
}

fn explanation_request() -> WorthQueryLowerRuntimeExplanationRequest {
    todo!()
}

fn main() {
    let installation = installed_domain::install("explanation-draft-has-no-review");
    let _ = installation
        .contributions()
        .for_lower_runtime_boundary_envelope(&envelope()).expect("installed contribution authority must remain current")
        .explains_store_backed_replay_gap("replay.store_gap", explanation_request())
        .review();
}
