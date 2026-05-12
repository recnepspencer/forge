use forge_query::facade::{CausalInspectionScaleCounterSnapshot, CausalInspectionScaleFixtureSize};

fn main() {
    let _ = CausalInspectionScaleCounterSnapshot {
        fixture_size: CausalInspectionScaleFixtureSize::Small,
        artifact_digest: String::new(),
        evidence_reference_width: 0,
        anchor_derivation_slope_counter: 0,
        reference_resolution_slope_counter: 0,
        admission_slope_counter: 0,
        bridge_envelope_slope_counter: 0,
        materialization_slope_counter: 0,
        artifact_serialization_slope_counter: 0,
        bridge_scan_fallback_count: 0,
        bridge_readmission_proof_digest: None,
        snapshot_digest: String::new(),
    };
}
