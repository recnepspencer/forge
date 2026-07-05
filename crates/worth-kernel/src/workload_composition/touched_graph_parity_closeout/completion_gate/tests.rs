use super::current::current_worth_touched_graph_roadmap_completion_gate;
use crate::workload_composition::planner_owned_routing::run_stack_heavy_planner_owned_routing_test;

#[test]
fn touched_graph_roadmap_completion_gate_requires_unified_architecture() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let gate = current_worth_touched_graph_roadmap_completion_gate()
            .expect("current roadmap completion gate");

        assert!(gate.is_complete());
        assert!(!gate.completion_digest().is_empty());
        assert_eq!(
            gate.closeout_architecture_claim_digest(),
            gate.closeout_matrix().closeout_architecture_claim_digest()
        );
        assert_eq!(
            gate.closeout_architecture_claim_digest(),
            gate.readiness_handoff().architecture_claim_digest()
        );
        assert_eq!(
            gate.closeout_architecture_claim_digest(),
            gate.live_coverage_ledger()
                .closeout_architecture_claim_digest()
        );
        assert_eq!(
            gate.readiness_handoff().source_firewall_digest(),
            gate.source_firewall_report_digest()
        );
        assert!(gate.covered_forbidden_surface_count() > 0);

        let representative_coverage = gate.representative_path().covered_family_kinds();
        for family_kind in gate.covered_family_kinds() {
            let row = gate
                .closeout_matrix()
                .row(*family_kind)
                .expect("covered family row");
            assert!(row.declare_once_parity_passed());
            assert!(row.readiness_handoff_passed());
            assert!(row.public_proof_parity_passed());
            assert!(row.diagnostic_parity_passed());
            assert_eq!(
                row.representative_path_covered(),
                representative_coverage.contains(family_kind)
            );
        }
    });
}
