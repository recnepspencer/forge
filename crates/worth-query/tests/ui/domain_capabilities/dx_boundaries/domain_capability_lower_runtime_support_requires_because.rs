#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::runtime::WorthQueryLowerRuntimeBoundaryEnvelope;

fn envelope() -> WorthQueryLowerRuntimeBoundaryEnvelope {
    todo!()
}

fn main() {
    let installation = installed_domain::install("lower-runtime-support-requires-because");
    let _ = installation
        .contributions()
        .for_lower_runtime_boundary_envelope(&envelope()).expect("installed contribution authority must remain current")
        .supports_boundary_traceability("routing.signal_invalidation")
        .materialize();
}
