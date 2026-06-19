use super::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportMatrix, ForgeQueryGraphObligationSupportStatus,
};

#[test]
fn support_matrix_names_supported_lane_for_every_obligation_kind() {
    let matrix = ForgeQueryGraphObligationSupportMatrix::assembly_selection_foundation();

    for kind in ForgeQueryGraphObligationKind::ALL {
        assert_eq!(matrix.supported_lane_count_for_kind(kind), 1);
        assert!(matrix.rows_for_kind(kind).any(|row| {
            row.support_lane() == ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection
                && row.status() == ForgeQueryGraphObligationSupportStatus::Supported
        }));
    }
}

#[test]
fn support_matrix_keeps_future_lanes_explicit_instead_of_invisible() {
    let matrix = ForgeQueryGraphObligationSupportMatrix::assembly_selection_foundation();

    assert!(matrix.rows().iter().any(|row| {
        row.support_lane() == ForgeQueryGraphObligationSupportLane::GraphComposition
            && row.status() == ForgeQueryGraphObligationSupportStatus::DeferredToBackstop
    }));
    assert!(matrix.rows().iter().any(|row| {
        row.support_lane() == ForgeQueryGraphObligationSupportLane::ReadFamily
            && row.status() == ForgeQueryGraphObligationSupportStatus::DiagnosticOnly
    }));
}

#[test]
fn milestone_9_9_authority_matrix_covers_every_kind_and_lane_with_budget_posture() {
    let matrix = ForgeQueryGraphObligationSupportMatrix::milestone_9_9_authority_surface();

    assert_eq!(
        matrix.rows().len(),
        ForgeQueryGraphObligationKind::ALL.len()
            * ForgeQueryGraphObligationSupportLane::MILESTONE_9_9_COVERED.len()
    );
    for kind in ForgeQueryGraphObligationKind::ALL {
        assert_eq!(
            matrix.rows_for_kind(kind).count(),
            ForgeQueryGraphObligationSupportLane::MILESTONE_9_9_COVERED.len()
        );
    }
    for lane in ForgeQueryGraphObligationSupportLane::MILESTONE_9_9_COVERED {
        assert_eq!(
            matrix.rows_for_lane(lane).count(),
            ForgeQueryGraphObligationKind::ALL.len()
        );
    }
    for status in ForgeQueryGraphObligationSupportStatus::ALL {
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
