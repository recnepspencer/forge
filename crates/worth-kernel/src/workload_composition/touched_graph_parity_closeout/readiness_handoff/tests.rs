use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityClaimKind, TouchedGraphParityFamilyKind,
};
use schema::facade::platform::authority::touched_graph_parity_closeout_internal::admit_touched_graph_parity_readiness_input;
use topology::facade::{
    admit_milestone_seven_five_overlap_readiness_consumer,
    TopologyMilestoneSevenFiveReadinessErrorKind,
};

use super::current_touched_graph_readiness_handoff;
use crate::workload_composition::planner_owned_routing::run_stack_heavy_planner_owned_routing_test;
use crate::workload_composition::touched_graph_parity_closeout::current_live_coverage_ledger;

#[test]
fn milestone_seven_five_readiness_input_is_sufficient_without_route_rediscovery() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let readiness = current_touched_graph_readiness_handoff()
            .expect("current readiness handoff should assemble");
        let consumer = admit_milestone_seven_five_overlap_readiness_consumer(&readiness)
            .expect("Milestone 7.5-style overlap consumer should admit from readiness alone");

        assert_eq!(
            readiness.claim().kind(),
            TouchedGraphParityClaimKind::ReadinessParity
        );
        assert!(!readiness.selected_family_identity().is_empty());
        assert!(!readiness.selected_route_identity_digest().is_empty());
        assert!(!readiness.selected_product_identity_digest().is_empty());
        assert!(!readiness.touched_closure_digest().is_empty());
        assert!(!readiness.selected_plan_digest().is_empty());
        assert!(!readiness.overlap_identity_digests().is_empty());
        assert_eq!(
            readiness.representative_family_coverage(),
            &TouchedGraphParityFamilyKind::ALL
        );
        assert!(!readiness.topology_query_posture_digest().is_empty());
        assert!(!readiness.spatial_query_posture_digest().is_empty());
        assert!(!readiness.residue_digest().is_empty());
        assert!(!readiness.source_firewall_digest().is_empty());
        assert_eq!(
            readiness.selected_route_identity_digest(),
            readiness
                .claim()
                .selected_route_identity()
                .identity_digest()
        );
        assert_eq!(
            readiness.selected_family_identity(),
            readiness
                .claim()
                .selected_family_identity()
                .selected_family_name()
        );
        assert_eq!(
            readiness.selected_product_identity_digest(),
            readiness
                .claim()
                .selected_product_identity()
                .expect("readiness claim should carry product identity")
                .identity_digest()
        );
        assert_eq!(
            readiness.selected_witness_identity_digest(),
            readiness
                .claim()
                .witness_identity()
                .map(|identity| identity.identity_digest())
        );
        assert_eq!(
            consumer.selected_route_identity_digest(),
            readiness.selected_route_identity_digest()
        );
        assert_eq!(
            consumer.selected_family_identity(),
            readiness.selected_family_identity()
        );
        assert_eq!(
            consumer.selected_product_identity_digest(),
            readiness.selected_product_identity_digest()
        );
        assert_eq!(
            consumer.selected_witness_identity_digest(),
            readiness.selected_witness_identity_digest()
        );
        assert_eq!(
            consumer.touched_closure_digest(),
            readiness.touched_closure_digest()
        );
        assert_eq!(
            consumer.selected_plan_digest(),
            readiness.selected_plan_digest()
        );
        assert_eq!(
            consumer.overlap_identity_digests(),
            readiness.overlap_identity_digests()
        );
        assert_eq!(
            consumer.topology_query_posture_digest(),
            readiness.topology_query_posture_digest()
        );
        assert_eq!(
            consumer.spatial_query_posture_digest(),
            readiness.spatial_query_posture_digest()
        );
        assert_eq!(consumer.residue_digest(), readiness.residue_digest());
        assert_eq!(
            consumer.source_firewall_digest(),
            readiness.source_firewall_digest()
        );
        assert_eq!(
            consumer.architecture_claim_digest(),
            readiness.architecture_claim_digest()
        );
    });
}

#[test]
fn milestone_seven_five_readiness_input_rejects_missing_family_proof() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let readiness = current_touched_graph_readiness_handoff()
            .expect("current readiness handoff should assemble");
        let reduced_coverage = readiness
            .representative_family_coverage()
            .iter()
            .copied()
            .filter(|family| *family != TouchedGraphParityFamilyKind::ReplayUndo)
            .collect::<Vec<_>>();
        let hostile = admit_touched_graph_parity_readiness_input(
            readiness.claim().clone(),
            readiness.residue_classification(),
            readiness.touched_closure_digest(),
            readiness.selected_plan_digest(),
            readiness.overlap_identity_digests().to_vec(),
            reduced_coverage,
            readiness.topology_query_posture_digest(),
            readiness.spatial_query_posture_digest(),
            readiness.residue_digest(),
            readiness.source_firewall_digest(),
            readiness.architecture_claim_digest(),
        )
        .expect("hostile readiness artifact should still satisfy schema shape");

        let error = admit_milestone_seven_five_overlap_readiness_consumer(&hostile)
            .expect_err("Milestone 7.5-style consumer should reject missing family coverage");

        assert_eq!(
            error.kind(),
            TopologyMilestoneSevenFiveReadinessErrorKind::MissingRepresentativeFamilyProof
        );
    });
}

#[test]
fn milestone_seven_five_readiness_input_carries_closed_architecture_digest() {
    run_stack_heavy_planner_owned_routing_test(|| {
        let readiness = current_touched_graph_readiness_handoff()
            .expect("current readiness handoff should assemble");
        let hostile = admit_touched_graph_parity_readiness_input(
            readiness.claim().clone(),
            readiness.residue_classification(),
            readiness.touched_closure_digest(),
            readiness.selected_plan_digest(),
            readiness.overlap_identity_digests().to_vec(),
            readiness.representative_family_coverage().to_vec(),
            readiness.topology_query_posture_digest(),
            readiness.spatial_query_posture_digest(),
            readiness.residue_digest(),
            readiness.source_firewall_digest(),
            "foreign-architecture-claim-digest",
        )
        .expect("hostile readiness artifact should still satisfy schema shape");

        assert_ne!(
            hostile.architecture_claim_digest(),
            current_live_coverage_ledger()
                .expect("live coverage ledger")
                .closeout_architecture_claim_digest()
        );
        assert_eq!(
            readiness.architecture_claim_digest(),
            current_live_coverage_ledger()
                .expect("live coverage ledger")
                .closeout_architecture_claim_digest()
        );
        assert_ne!(
            readiness.architecture_claim_digest(),
            current_live_coverage_ledger()
                .expect("live coverage ledger")
                .ledger_digest()
        );
    });
}
