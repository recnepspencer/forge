use super::current::current_worth_touched_graph_roadmap_completion_gate;
use super::validation::validate_roadmap_completion_gate;
use super::WorthTouchedGraphRoadmapCompletionGateErrorKind;
use crate::workload_composition::{
    planner_owned_routing::{
        run_stack_heavy_planner_owned_routing_test,
        WorthTouchedGraphConflictArchitectureAlignmentReportRow,
    },
    WorthTouchedGraphCrossFamilyCloseoutMatrix,
};

#[test]
fn touched_graph_roadmap_completion_gate_rejects_mostly_unified_paths() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let gate = current_worth_touched_graph_roadmap_completion_gate()
            .expect("current roadmap completion gate");
        let representative_family = gate
            .representative_path()
            .covered_family_kinds()
            .into_iter()
            .next()
            .expect("representative path should cover at least one family");
        let hostile_rows = gate
            .closeout_matrix()
            .rows()
            .iter()
            .map(|row| {
                if row.family_kind() == representative_family {
                    crate::workload_composition::WorthTouchedGraphCrossFamilyCloseoutMatrixRow::new(
                        row.family_kind(),
                        row.covered_surface_count(),
                        false,
                        row.declare_once_parity_passed(),
                        row.public_proof_parity_passed(),
                        row.diagnostic_parity_passed(),
                        row.readiness_handoff_passed(),
                        row.deleted_count(),
                        row.capped_residue_count(),
                        row.query_gap_count(),
                        row.blocked_outside_roadmap_count(),
                    )
                } else {
                    row.clone()
                }
            })
            .collect::<Vec<_>>();
        let hostile_matrix = WorthTouchedGraphCrossFamilyCloseoutMatrix::new(
            hostile_rows,
            gate.closeout_matrix().closeout_architecture_claim_digest(),
        );
        let hostile_gate = gate.with_test_closeout_matrix(hostile_matrix);

        let error = validate_roadmap_completion_gate(&hostile_gate)
            .expect_err("roadmap completion must fail when a covered family loses certification");
        assert_eq!(
            error.kind(),
            WorthTouchedGraphRoadmapCompletionGateErrorKind::RepresentativePathAuthorityMismatch
        );
    });
}

#[test]
fn touched_graph_roadmap_completion_gate_rejects_mismatched_architecture_claim() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let gate = current_worth_touched_graph_roadmap_completion_gate()
            .expect("current roadmap completion gate");
        let hostile_matrix = WorthTouchedGraphCrossFamilyCloseoutMatrix::new(
            gate.closeout_matrix().rows().to_vec(),
            "hostile-architecture-claim-digest",
        );
        let hostile_gate = gate.with_test_closeout_matrix(hostile_matrix);

        let error = validate_roadmap_completion_gate(&hostile_gate)
            .expect_err("roadmap completion must fail when architecture claims drift");
        assert_eq!(
            error.kind(),
            WorthTouchedGraphRoadmapCompletionGateErrorKind::MismatchedArchitectureClaim
        );
    });
}

#[test]
fn touched_graph_roadmap_completion_gate_rejects_reachable_second_ontology_blocker() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let gate = current_worth_touched_graph_roadmap_completion_gate()
            .expect("current roadmap completion gate");
        let representative_family = gate
            .representative_path()
            .covered_family_kinds()
            .into_iter()
            .next()
            .expect("representative path should cover at least one family");
        let hostile_row =
            WorthTouchedGraphConflictArchitectureAlignmentReportRow::hostile_second_ontology_blocker(
                representative_family,
                "crates/worth-kernel/src/workload_composition/touched_graph_parity_closeout/completion_gate/tests_failure_guards.rs",
                "touched_graph_roadmap_completion_gate_rejects_reachable_second_ontology_blocker",
                "worth-kernel",
                "hostile test reopens an ordinary-path second ontology dependency",
                "phase 17 hostile proof should fail roadmap completion",
            );
        let hostile_report = gate
            .public_closeout()
            .architecture_alignment_report()
            .clone()
            .with_test_reachable_second_ontology_blocker(hostile_row);
        let hostile_public_closeout = gate
            .public_closeout()
            .clone()
            .with_test_architecture_alignment_report(hostile_report);
        let hostile_gate = gate.with_test_public_closeout(hostile_public_closeout);

        let error = validate_roadmap_completion_gate(&hostile_gate).expect_err(
            "roadmap completion must fail when a second-ontology blocker remains reachable",
        );
        assert_eq!(
            error.kind(),
            WorthTouchedGraphRoadmapCompletionGateErrorKind::OrdinarySecondOntologyStillReachable
        );
    });
}
