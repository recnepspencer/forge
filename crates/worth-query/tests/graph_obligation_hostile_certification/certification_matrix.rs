use std::collections::BTreeSet;

use worth_query::facade::runtime::{
    WorthQueryGraphObligationKind, WorthQueryGraphObligationSupportLane,
    WorthQueryGraphObligationSupportStatus,
};

use super::support::authority_matrix;

#[test]
fn kind_lane_support_matrix_covers_every_kind_lane_and_status() {
    let matrix = authority_matrix();
    let expected_row_count = WorthQueryGraphObligationKind::ALL.len()
        * WorthQueryGraphObligationSupportLane::MILESTONE_9_9_COVERED.len();

    assert_eq!(matrix.rows().len(), expected_row_count);
    for kind in WorthQueryGraphObligationKind::ALL {
        assert_eq!(
            matrix.rows_for_kind(kind).count(),
            WorthQueryGraphObligationSupportLane::MILESTONE_9_9_COVERED.len(),
            "phase 20 matrix must cover every lane for {kind:?}"
        );
        assert!(
            matrix.supported_lane_count_for_kind(kind) > 0,
            "every obligation kind needs at least one real supported lane"
        );
    }
    for lane in WorthQueryGraphObligationSupportLane::MILESTONE_9_9_COVERED {
        assert_eq!(
            matrix.rows_for_lane(lane).count(),
            WorthQueryGraphObligationKind::ALL.len(),
            "phase 20 matrix must cover every kind for {lane:?}"
        );
    }

    let statuses = matrix
        .rows()
        .iter()
        .map(|row| row.status())
        .collect::<BTreeSet<_>>();
    for status in WorthQueryGraphObligationSupportStatus::ALL {
        assert!(
            statuses.contains(&status),
            "phase 20 matrix must represent {status:?} without fake rows"
        );
    }
}

#[test]
fn authority_matrix_rows_all_carry_budget_and_artifact_policies() {
    for row in authority_matrix().rows() {
        assert!(!row.execution_budget().budget_digest().is_empty());
        assert!(!row.cost_class().as_str().is_empty());
        assert_eq!(row.state_load_counter_policy(), "state-load counters");
        assert_eq!(
            row.diagnostic_artifact_policy(),
            "artifact-policy-gated diagnostics"
        );
    }
}
