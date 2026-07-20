use worth_ui_runtime::facade::evidence::UiMeasurementResult;

fn main() {
    let _mint = UiMeasurementResult::from_host_observation;
}
// measurement authority denials share one compiler process.
mod covered_001 { include!("external_callers_cannot_mint_projection_fact_receipt_via_struct_literal.rs"); }
mod covered_002 { include!("../measurement_boundary_purity/forbidden_measurement_request_family_variant.rs"); }
mod covered_003 { include!("../measurement_boundary_purity/forbidden_measurement_request_constructor.rs"); }
