use forge_query::facade::{
    ForgeQueryGraphObligationExecutionStatus, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationSupportLane,
};

use super::super::{
    primitive_construction_graph_obligation_execution_matrix,
    primitive_construction_graph_obligation_replay_pair,
};
use crate::construction::request::PRIMITIVE_CONSTRUCTION_FAMILIES;

#[test]
fn executed_result_and_outcome_matrix_covers_every_current_family() {
    let rows = primitive_construction_graph_obligation_execution_matrix();
    let executed_families = rows.iter().map(|row| row.family()).collect::<Vec<_>>();

    assert_eq!(executed_families, PRIMITIVE_CONSTRUCTION_FAMILIES);
    assert_eq!(rows.len(), PRIMITIVE_CONSTRUCTION_FAMILIES.len());
    for row in rows {
        assert_eq!(row.selected_count(), 1);
        assert!(!row.result_digest().is_empty());
        assert!(!row.outcome_digest().is_empty());
        assert!(!row.evidence_digest().is_empty());
        assert!(!row.envelope_digest().is_empty());
        assert!(!row.selected_row_digest().is_empty());
        assert!(!row.rule_identity_digest().is_empty());
        assert_eq!(
            row.obligation_kind(),
            ForgeQueryGraphObligationKind::AdvisoryObligation
        );
        assert_eq!(
            row.support_lane(),
            ForgeQueryGraphObligationSupportLane::GraphComposition
        );
        assert_eq!(
            row.execution_status(),
            Some(ForgeQueryGraphObligationExecutionStatus::Executed)
        );
        assert_eq!(row.verdict(), "advise");
        assert_eq!(row.verdict_context(), Some("advisory-obligation-selected"));
        assert!(row.has_authoritative_dispatch_identity());
    }
}

#[test]
fn executed_obligation_selection_replays_to_same_authoritative_row_identity() {
    for family in PRIMITIVE_CONSTRUCTION_FAMILIES {
        let (first, second) = primitive_construction_graph_obligation_replay_pair(family);

        assert_eq!(first.selected_count(), 1);
        assert_eq!(second.selected_count(), 1);
        assert_eq!(first.selected_row_digest(), second.selected_row_digest());
        assert_eq!(first.rule_identity_digest(), second.rule_identity_digest());
        assert_eq!(first.obligation_kind(), second.obligation_kind());
        assert_eq!(first.support_lane(), second.support_lane());
        assert_eq!(first.execution_status(), second.execution_status());
        assert_eq!(first.verdict(), second.verdict());
        assert_eq!(first.verdict_context(), second.verdict_context());
    }
}
