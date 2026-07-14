use super::{
    WorthQueryGraphObligationKind, WorthQueryGraphObligationSupportLane,
    WorthQueryGraphObligationSupportMatrix, WorthQueryGraphObligationSupportStatus,
};

#[test]
fn support_matrix_names_supported_lane_for_every_obligation_kind() {
    let matrix = WorthQueryGraphObligationSupportMatrix::assembly_selection_foundation();

    for kind in WorthQueryGraphObligationKind::ALL {
        assert_eq!(matrix.supported_lane_count_for_kind(kind), 1);
        assert!(matrix.rows_for_kind(kind).any(|row| {
            row.support_lane() == WorthQueryGraphObligationSupportLane::AssemblyIndexSelection
                && row.status() == WorthQueryGraphObligationSupportStatus::Supported
        }));
    }
}

#[test]
fn support_matrix_keeps_future_lanes_explicit_instead_of_invisible() {
    let matrix = WorthQueryGraphObligationSupportMatrix::assembly_selection_foundation();

    assert!(matrix.rows().iter().any(|row| {
        row.support_lane() == WorthQueryGraphObligationSupportLane::GraphComposition
            && row.status() == WorthQueryGraphObligationSupportStatus::DeferredToBackstop
    }));
    assert!(matrix.rows().iter().any(|row| {
        row.support_lane() == WorthQueryGraphObligationSupportLane::ReadFamily
            && row.status() == WorthQueryGraphObligationSupportStatus::DiagnosticOnly
    }));
}

#[test]
fn milestone_9_9_authority_matrix_covers_every_kind_and_lane_with_budget_posture() {
    let matrix = WorthQueryGraphObligationSupportMatrix::milestone_9_9_authority_surface();

    assert_eq!(
        matrix.rows().len(),
        WorthQueryGraphObligationKind::ALL.len()
            * WorthQueryGraphObligationSupportLane::MILESTONE_9_9_COVERED.len()
    );
    for kind in WorthQueryGraphObligationKind::ALL {
        assert_eq!(
            matrix.rows_for_kind(kind).count(),
            WorthQueryGraphObligationSupportLane::MILESTONE_9_9_COVERED.len()
        );
    }
    for lane in WorthQueryGraphObligationSupportLane::MILESTONE_9_9_COVERED {
        assert_eq!(
            matrix.rows_for_lane(lane).count(),
            WorthQueryGraphObligationKind::ALL.len()
        );
    }
    for status in WorthQueryGraphObligationSupportStatus::ALL {
        assert!(
            matrix.rows().iter().any(|row| row.status() == status),
            "milestone 9.9 matrix must expose {status:?}"
        );
    }
    assert!(matrix.rows().iter().all(|row| {
        !row.execution_budget().budget_digest().is_empty()
            && row.cost_class().as_str() == "sparse-topology"
            && row.state_load_counter_policy() == "state-load counters"
            && row.diagnostic_artifact_policy() == "artifact-policy-gated diagnostics"
    }));
}
