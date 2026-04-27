use super::{
    RuntimeApiStabilizationAdapter, RUNTIME_API_STABILIZATION_REQUIRED_CANONICAL_ROW_NAMES,
    RUNTIME_API_STABILIZATION_REQUIRED_REJECTION_ROW_NAMES,
};
use crate::harness::certification::{
    contains_row, unmet_required_assertion_classes, HostileExpectation, ParityAnchor,
    RequiredAssertionClass,
};
use crate::runtime::{ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupportStatus};

#[test]
fn runtime_api_stabilization_adapter_emits_named_matrix() {
    let artifact =
        RuntimeApiStabilizationAdapter::runtime_api_golden_dx_and_async_safe_facade_artifact();

    assert_eq!(
        artifact.suite_name,
        "Runtime API Golden DX And Async-Safe Facade Test"
    );
    assert!(!artifact.certification_bundle_digest.is_empty());
    assert!(!artifact.coverage_matrix_digest.is_empty());
}

#[test]
fn runtime_api_stabilization_matrix_covers_required_rows() {
    let matrix = RuntimeApiStabilizationAdapter::runtime_api_golden_dx_and_async_safe_facade_test();

    for row_name in RUNTIME_API_STABILIZATION_REQUIRED_CANONICAL_ROW_NAMES {
        assert!(contains_row(&matrix, row_name), "missing {row_name}");
    }
    for row_name in RUNTIME_API_STABILIZATION_REQUIRED_REJECTION_ROW_NAMES {
        assert!(contains_row(&matrix, row_name), "missing {row_name}");
    }
}

#[test]
fn runtime_api_stabilization_rows_have_required_outputs_and_meaningful_assertions() {
    let matrix = RuntimeApiStabilizationAdapter::runtime_api_golden_dx_and_async_safe_facade_test();

    for row in &matrix.rows {
        for lane in [&row.control_lane, &row.hostile_lane, &row.parity_lane] {
            assert!(lane.has_required_outputs(), "row '{}'", row.row_name);
            assert!(lane.public_facade_only, "row '{}'", row.row_name);
            assert_eq!(
                lane.lower_runtime_plumbing_count, 0,
                "row '{}'",
                row.row_name
            );
            assert!(
                lane.meaningful_assertion_count >= 8,
                "golden DX rows must assert proof artifacts, not only compilation"
            );
            assert!(lane.stable_family_count >= 7);
            assert!(lane.deferred_family_count >= 5);
        }
    }
}

#[test]
fn runtime_api_stabilization_rows_enforce_required_assertion_classes() {
    let matrix = RuntimeApiStabilizationAdapter::runtime_api_golden_dx_and_async_safe_facade_test();
    let mut covered = Vec::new();

    for row in &matrix.rows {
        let control = row.control_lane.semantic_signature();
        let hostile = row.hostile_lane.semantic_signature();
        let parity = row.parity_lane.semantic_signature();
        match row.hostile_expectation {
            HostileExpectation::EquivalentToControl => {
                assert_eq!(control, hostile, "row '{}'", row.row_name);
                covered.push(RequiredAssertionClass::Equality);
            }
            HostileExpectation::DistinctFromControl => {
                assert_ne!(control, hostile, "row '{}'", row.row_name);
                covered.push(RequiredAssertionClass::Inequality);
            }
        }
        match row.parity_anchor {
            ParityAnchor::Control => assert_eq!(parity, control, "row '{}'", row.row_name),
            ParityAnchor::Hostile => assert_eq!(parity, hostile, "row '{}'", row.row_name),
        }
    }

    for row in &matrix.rejection_rows {
        covered.push(RequiredAssertionClass::TypedFailure);
        assert_eq!(
            row.hostile_lane.status,
            ForgeQueryRuntimeFamilySupportStatus::DeferredDebt
        );
        assert!(row
            .hostile_lane
            .counter_snapshot
            .contains("authority_residue=0"));
        covered.push(RequiredAssertionClass::ZeroResidue);
    }

    covered.sort();
    covered.dedup();
    let required = [
        RequiredAssertionClass::Equality,
        RequiredAssertionClass::TypedFailure,
        RequiredAssertionClass::ZeroResidue,
    ];
    let missing = unmet_required_assertion_classes(&covered, &required);
    assert!(missing.is_empty(), "missing assertion classes: {missing:?}");
}

#[test]
fn runtime_api_stabilization_deferred_gates_name_future_owners() {
    let matrix = RuntimeApiStabilizationAdapter::runtime_api_golden_dx_and_async_safe_facade_test();
    let expected = [
        (
            "temporal-basis-deferred-gate",
            ForgeQueryRuntimeFacadeFamily::Temporal,
        ),
        (
            "async-resource-deferred-gate",
            ForgeQueryRuntimeFacadeFamily::AsyncResource,
        ),
        (
            "mixed-cause-delivery-deferred-gate",
            ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
        ),
        (
            "store-backed-parity-deferred-gate",
            ForgeQueryRuntimeFacadeFamily::StoreBackedExecution,
        ),
        (
            "durable-restart-deferred-gate",
            ForgeQueryRuntimeFacadeFamily::DurableArtifacts,
        ),
    ];

    for (row_name, family) in expected {
        let row = matrix
            .rejection_rows
            .iter()
            .find(|row| row.row_name == row_name)
            .expect("deferred gate row should exist");
        assert_eq!(row.hostile_lane.family, family);
        assert_eq!(
            row.hostile_lane.status,
            ForgeQueryRuntimeFamilySupportStatus::DeferredDebt
        );
        assert_ne!(
            row.hostile_lane.deferred_temporal_async_gate_digest,
            row.hostile_lane.failure_digest
        );
    }
}
