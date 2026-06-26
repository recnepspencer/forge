use forge_query::facade::ForgeQueryGraphObligationSupportStatus;

use super::super::{
    primitive_construction_graph_obligation_execution_matrix,
    primitive_construction_touched_basis_fixture::primitive_construction_touched_basis_for_family,
};
use crate::query_obligation_selection::selection_substrate::{
    QueryObligationSelectionInput, QueryObligationSelectionSubstrate,
};

#[test]
fn parallel_query_selection_substrate_matches_trusted_primitive_execution_matrix() {
    for old_row in primitive_construction_graph_obligation_execution_matrix() {
        let touched_basis = primitive_construction_touched_basis_for_family(old_row.family());
        let selected = QueryObligationSelectionSubstrate::select_execution_backed_obligations(
            QueryObligationSelectionInput::from_topology_touched_basis(touched_basis.proof())
                .expect("touched basis should lower into query selection input"),
        )
        .expect("query substrate should select primitive construction obligation");
        let closeout = selected.closeout();
        let selected_obligations = selected
            .execution_proof()
            .selection_proof()
            .selected_obligations();
        let execution_rows = selected.execution_proof().rows();

        assert_eq!(
            closeout.selected_obligation_count(),
            old_row.selected_count()
        );
        assert_eq!(closeout.execution_row_count(), old_row.selected_count());
        assert_eq!(selected_obligations.len(), 1);
        assert_eq!(
            selected_obligations[0].rule_identity_digest(),
            old_row.rule_identity_digest()
        );
        assert_eq!(
            selected_obligations[0].obligation_kind(),
            old_row.obligation_kind()
        );
        assert_eq!(
            selected_obligations[0].support_lane(),
            old_row.support_lane()
        );
        assert_eq!(
            selected_obligations[0].support_status(),
            ForgeQueryGraphObligationSupportStatus::Supported
        );
        assert_eq!(
            selected_obligations[0].execution_budget_digest(),
            old_row.execution_budget_digest()
        );
        assert_eq!(execution_rows.len(), 1);
        assert_eq!(
            execution_rows[0].status(),
            old_row.execution_status().unwrap()
        );
        assert_eq!(execution_rows[0].verdict(), Some(old_row.verdict()));
        assert_eq!(
            execution_rows[0].verdict_context(),
            old_row.verdict_context()
        );
        assert_eq!(
            closeout.selection_counters().matched_obligation_count(),
            old_row.selected_count()
        );
        assert_eq!(
            closeout.selection_counters().registration_full_scan_count(),
            0
        );
        assert!(!selected.execution_proof().envelope_digest().is_empty());
        assert!(
            closeout
                .selection_counters()
                .attempted_bucket_lookup_count()
                > 0
        );
        assert!(closeout.selection_counters().candidate_registration_count() > 0);
        assert!(selected.execution_proof().has_real_executor_rows());
        assert_eq!(selected.residue_manifest().rows().len(), 3);
    }
}
