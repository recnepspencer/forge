//! Tests for the error taxonomy.

use super::*;

#[test]
fn merge_error_summary_preserves_boundary_reject_witness_reason() {
    let err = MergeError::BoundaryCertificationFailed {
        reason: "self-intersection".to_string(),
        witness: Some([0.25, -1.0]),
    };
    let summary = MergeErrorSummary::from(&err);

    assert_eq!(
        summary,
        MergeErrorSummary::BoundaryCertificationFailed {
            reason: "self-intersection".to_string(),
            witness: Some([0.25, -1.0]),
        }
    );
}

#[test]
fn kernel_error_summary_preserves_typed_merge_variant() {
    let err = KernelError::MergeFailure(MergeError::ProtectedUseConflict {
        face_index: 17,
        edge_index: Some(4),
    });

    match KernelErrorSummary::from(&err) {
        KernelErrorSummary::MergeFailure(MergeErrorSummary::ProtectedUseConflict {
            face_index,
            edge_index,
        }) => {
            assert_eq!(face_index, 17);
            assert_eq!(edge_index, Some(4));
        }
        other => panic!("expected typed merge failure summary, got {:?}", other),
    }
}

#[test]
fn error_summary_round_trips_json() {
    let err = KernelError::MergeFailure(MergeError::PartialMergePlanRejected {
        step_index: Some(2),
        reason: "radial ring changed during execution".to_string(),
    });

    let summary = ErrorSummary::from(&err);
    let json = serde_json::to_string(&summary).expect("serialize error summary");
    let restored: ErrorSummary = serde_json::from_str(&json).expect("deserialize error summary");

    assert_eq!(restored, summary);
    assert!(matches!(restored.category, ErrorCategory::Kernel));
    assert!(restored
        .human_message
        .as_deref()
        .unwrap_or("")
        .contains("Merge failure"));
}

#[test]
fn topology_error_summary_preserves_radial_edge_inconsistency_fields() {
    let err = TopologyError::RadialEdgeInconsistency {
        halfedge_index: 9,
        actual_edge: 3,
        seed_halfedge_index: 7,
        expected_edge: 2,
    };

    assert_eq!(
        TopologyErrorSummary::from(&err),
        TopologyErrorSummary::RadialEdgeInconsistency {
            halfedge_index: 9,
            actual_edge: 3,
            seed_halfedge_index: 7,
            expected_edge: 2,
        }
    );
}
