use super::current::current_evidence_lookup_route_packet;
use super::mismatch::EvidenceLookupRouteMismatch;
use super::support::current_evidence_lookup_route_source;
use crate::facade::replay_undo_semantic_graph::{
    current_boolean_event_ledger_spatial_boundary, current_projection_receipt_spatial_boundary,
};
use crate::workload_platform::evidence_lookup_family_catalog::current_evidence_lookup_family_catalog;

#[test]
fn evidence_lookup_route_explanation_consumes_milestone_fourteen_seed() {
    let packet = current_evidence_lookup_route_packet().expect("current route packet");
    let route_source = current_evidence_lookup_route_source().expect("current route source");
    let left_boundary =
        current_boolean_event_ledger_spatial_boundary().expect("current left replay/undo boundary");
    let right_boundary =
        current_projection_receipt_spatial_boundary().expect("current right replay/undo boundary");
    let family_catalog =
        current_evidence_lookup_family_catalog().expect("current evidence lookup family catalog");
    let left_family = family_catalog
        .family_by_identity("spatial-touch.boolean.event-ledger-evidence.v1")
        .expect("current left route family");
    let right_family = family_catalog
        .family_by_identity("spatial-touch.boolean.projection-consumption-evidence.v1")
        .expect("current right route family");
    let left_handoff = left_boundary.workload_handoff();
    let left_index_product = left_boundary.index_product();
    let right_handoff = right_boundary.workload_handoff();

    assert_eq!(
        packet.route_family_identity(),
        left_family.identity().as_str()
    );
    assert_eq!(
        packet.right_route_family_identity(),
        right_family.identity().as_str()
    );
    assert_eq!(
        packet.selected_lookup_plan_digest(),
        left_handoff.selected_lookup_plan_digest()
    );
    assert_eq!(
        packet.lookup_execution_receipt_digest(),
        left_handoff.lookup_execution_receipt_digest()
    );
    assert_eq!(
        packet.right_stage_receipt_identity(),
        right_boundary.workload_handoff().stage_receipt_identity()
    );
    assert_eq!(
        packet.right_lookup_execution_receipt_digest(),
        right_boundary
            .workload_handoff()
            .lookup_execution_receipt_digest()
    );
    assert_eq!(
        packet.lookup_product_output_digest(),
        left_handoff.lookup_product_output_digest()
    );
    assert_eq!(
        packet.compiled_product_identity_digest(),
        left_index_product.compiled_product_identity_digest()
    );
    assert_eq!(
        packet.equivalence_policy_identity_digest(),
        left_index_product.equivalence_policy_identity_digest()
    );
    assert_eq!(
        packet.selected_equivalence_family_identity(),
        left_index_product
            .selected_equivalence_family_identity()
            .as_str()
    );
    assert_eq!(
        packet.selected_reuse_basis_identity_digest(),
        left_index_product.selected_reuse_basis_identity_digest()
    );
    assert_eq!(
        packet.topology_support_digest(),
        left_index_product.topology_support_digest()
    );
    assert_eq!(
        packet.query_support_digest(),
        left_index_product.query_support_digest()
    );
    assert_eq!(
        packet.right_authority_stage_index_identity(),
        right_boundary.authority().stage_index_identity()
    );
    assert_eq!(
        left_handoff
            .milestone_twelve_seed()
            .receipt_proof_row_count(),
        left_handoff.covered_family_identities().len()
    );
    assert_eq!(
        left_handoff
            .milestone_twelve_seed()
            .non_ordinary_residue_row_count(),
        0
    );
    assert_eq!(
        packet.lowering_raw_row_revisit_count(),
        left_handoff.counters().raw_row_scan_count()
            + right_handoff.counters().raw_row_scan_count()
    );
    assert_eq!(
        packet.lowering_right_receipt_revisit_count(),
        left_handoff.counters().broad_receipt_scan_count()
            + right_handoff.counters().broad_receipt_scan_count()
    );
    assert_eq!(
        packet.lowering_caller_owned_revisit_count(),
        left_handoff.counters().caller_owned_scan_count()
            + right_handoff.counters().caller_owned_scan_count()
    );
    assert_eq!(
        packet.lowering_raw_row_revisit_count(),
        route_source.lowering_evidence().raw_row_revisit_count()
    );
    assert_eq!(
        packet.lowering_right_receipt_revisit_count(),
        route_source
            .lowering_evidence()
            .right_receipt_revisit_count()
    );
    assert_eq!(
        packet.lowering_caller_owned_revisit_count(),
        route_source
            .lowering_evidence()
            .caller_owned_revisit_count()
    );
}

#[test]
fn evidence_lookup_route_denial_localizes_family_or_support_mismatch() {
    let packet = current_evidence_lookup_route_packet().expect("current route packet");

    let family_error = packet
        .require_matches_selected_contract(
            packet.route_authority_digest(),
            "foreign-route-family",
            packet.right_route_family_identity(),
            packet.stage_receipt_family_identity(),
            packet.right_stage_receipt_identity(),
            packet.selected_lookup_plan_digest(),
            packet.right_lookup_execution_receipt_digest(),
            packet.compiled_product_identity_digest(),
            packet.equivalence_policy_identity_digest(),
            packet.selected_equivalence_family_identity(),
            packet.selected_equivalence_basis_identity_digest(),
            packet.selected_compatibility_basis_identity_digest(),
            packet.selected_reuse_basis_identity_digest(),
            packet.topology_support_digest(),
            packet.query_support_digest(),
            packet.right_authority_stage_index_identity(),
        )
        .expect_err("foreign route family should be rejected");
    assert!(matches!(
        family_error.mismatch(),
        Some(EvidenceLookupRouteMismatch::RouteFamilyIdentity { .. })
    ));

    let support_error = packet
        .require_matches_selected_contract(
            packet.route_authority_digest(),
            packet.route_family_identity(),
            packet.right_route_family_identity(),
            packet.stage_receipt_family_identity(),
            packet.right_stage_receipt_identity(),
            packet.selected_lookup_plan_digest(),
            packet.right_lookup_execution_receipt_digest(),
            packet.compiled_product_identity_digest(),
            packet.equivalence_policy_identity_digest(),
            packet.selected_equivalence_family_identity(),
            packet.selected_equivalence_basis_identity_digest(),
            packet.selected_compatibility_basis_identity_digest(),
            packet.selected_reuse_basis_identity_digest(),
            packet.topology_support_digest(),
            "foreign-query-support",
            packet.right_authority_stage_index_identity(),
        )
        .expect_err("foreign query support should be rejected");
    assert!(matches!(
        support_error.mismatch(),
        Some(EvidenceLookupRouteMismatch::QuerySupportDigest { .. })
    ));

    let right_boundary_error = packet
        .require_matches_selected_contract(
            packet.route_authority_digest(),
            packet.route_family_identity(),
            packet.right_route_family_identity(),
            packet.stage_receipt_family_identity(),
            packet.right_stage_receipt_identity(),
            packet.selected_lookup_plan_digest(),
            "foreign-right-lookup-receipt",
            packet.compiled_product_identity_digest(),
            packet.equivalence_policy_identity_digest(),
            packet.selected_equivalence_family_identity(),
            packet.selected_equivalence_basis_identity_digest(),
            packet.selected_compatibility_basis_identity_digest(),
            packet.selected_reuse_basis_identity_digest(),
            packet.topology_support_digest(),
            packet.query_support_digest(),
            packet.right_authority_stage_index_identity(),
        )
        .expect_err("foreign right boundary receipt should be rejected");
    assert!(matches!(
        right_boundary_error.mismatch(),
        Some(EvidenceLookupRouteMismatch::RightLookupExecutionReceiptDigest { .. })
    ));
}
