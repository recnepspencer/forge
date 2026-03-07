use std::collections::HashMap;

use forge_signal::facade::NodeId;
use forge_spec::facade::{MakeVertexFaceMutation, SpecState};

use crate::configuration::facade::FingerprintDetail;
use crate::engine::contracts::contract::ConditioningMode;
use crate::engine::output::spec_envelope::SpecEnvelope;
use crate::engine::pipeline::fingerprint::compute_spec_pipeline_fingerprint;
use crate::geometry::facade::GeometryStore;

fn build_spec_envelope() -> SpecEnvelope {
    let mut draft = SpecState::empty().into_draft();
    draft.execute(MakeVertexFaceMutation).unwrap();
    SpecEnvelope::new(draft.commit().unwrap(), GeometryStore::default())
}

#[test]
fn spec_pipeline_fingerprint_is_stable_for_same_inputs() {
    let mut inputs = HashMap::new();
    inputs.insert(NodeId::new(1, 0), build_spec_envelope());

    let a = compute_spec_pipeline_fingerprint(
        &inputs,
        "make_cube",
        ConditioningMode::None,
        1e-7,
        1.0,
        1e-6,
        FingerprintDetail::Standard,
    )
    .unwrap();
    let b = compute_spec_pipeline_fingerprint(
        &inputs,
        "make_cube",
        ConditioningMode::None,
        1e-7,
        1.0,
        1e-6,
        FingerprintDetail::Standard,
    )
    .unwrap();

    assert_eq!(a, b);
}

#[test]
fn spec_pipeline_fingerprint_changes_with_feature_kind() {
    let mut inputs = HashMap::new();
    inputs.insert(NodeId::new(1, 0), build_spec_envelope());

    let a = compute_spec_pipeline_fingerprint(
        &inputs,
        "make_cube",
        ConditioningMode::None,
        1e-7,
        1.0,
        1e-6,
        FingerprintDetail::Standard,
    )
    .unwrap();
    let b = compute_spec_pipeline_fingerprint(
        &inputs,
        "boolean",
        ConditioningMode::None,
        1e-7,
        1.0,
        1e-6,
        FingerprintDetail::Standard,
    )
    .unwrap();

    assert_ne!(a, b);
}

#[test]
fn spec_pipeline_fingerprint_changes_with_detail_level() {
    let mut inputs = HashMap::new();
    inputs.insert(NodeId::new(1, 0), build_spec_envelope());

    let standard = compute_spec_pipeline_fingerprint(
        &inputs,
        "make_cube",
        ConditioningMode::None,
        1e-7,
        1.0,
        1e-6,
        FingerprintDetail::Standard,
    )
    .unwrap();
    let full = compute_spec_pipeline_fingerprint(
        &inputs,
        "make_cube",
        ConditioningMode::None,
        1e-7,
        1.0,
        1e-6,
        FingerprintDetail::Full,
    )
    .unwrap();

    assert_ne!(standard, full);
}
