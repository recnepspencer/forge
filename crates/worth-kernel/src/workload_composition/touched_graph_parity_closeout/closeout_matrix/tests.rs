use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

use crate::workload_composition::{
    planner_owned_routing::run_stack_heavy_planner_owned_routing_test,
    touched_graph_parity_closeout::current_touched_graph_parity_closeout_authorities,
};

#[test]
fn cross_family_closeout_matrix_matches_live_counts() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let authorities = current_touched_graph_parity_closeout_authorities()
            .expect("current touched-graph parity closeout authorities");
        let ledger = authorities.live_coverage_ledger();
        let representative_path = authorities.representative_path();
        let matrix = authorities.closeout_matrix();

        assert_eq!(matrix.rows().len(), TouchedGraphParityFamilyKind::ALL.len());
        assert_eq!(matrix.covered_surface_count(), ledger.covered_count());
        assert_eq!(matrix.capped_residue_count(), ledger.capped_residue_count());
        assert_eq!(matrix.query_gap_count(), ledger.query_gap_count());
        assert_eq!(
            matrix.blocked_outside_roadmap_count(),
            ledger.blocked_outside_roadmap_count()
        );
        assert_eq!(
            matrix.deleted_count(),
            authorities
                .public_closeout()
                .architecture_alignment_report()
                .deleted_authority_rows()
                .len()
        );
        assert_eq!(
            matrix.total_certified_rows(),
            matrix.covered_surface_count()
                + matrix.deleted_count()
                + matrix.capped_residue_count()
                + matrix.query_gap_count()
                + matrix.blocked_outside_roadmap_count()
        );
        assert_eq!(
            matrix.closeout_architecture_claim_digest(),
            ledger.closeout_architecture_claim_digest()
        );
        assert!(!matrix.matrix_digest().is_empty());

        let representative_coverage = representative_path.covered_family_kinds();
        for row in matrix.rows() {
            assert!(TouchedGraphParityFamilyKind::ALL.contains(&row.family_kind()));
            assert_eq!(
                row.representative_path_covered(),
                representative_coverage.contains(&row.family_kind())
            );
            assert!(row.declare_once_parity_passed());
            assert!(row.public_proof_parity_passed());
            assert!(row.diagnostic_parity_passed());
            assert!(row.readiness_handoff_passed());
        }
    });
}
