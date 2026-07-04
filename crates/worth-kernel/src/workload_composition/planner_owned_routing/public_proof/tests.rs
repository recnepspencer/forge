use super::publish_from_parts;
use super::{
    current_public_closeout_components, current_worth_touched_graph_conflict_public_closeout,
    current_worth_touched_graph_conflict_public_closeout_with_route_loader,
};
use crate::workload_composition::{
    admit_worth_touched_graph_conflict_public_proof_input,
    planner_owned_routing::PlannerOwnedRoutingError,
    planner_owned_routing::{
        PlannerOwnedRoutingErrorKind, WorthTouchedGraphConflictPublicCloseoutErrorKind,
    },
    WorthTouchedGraphConflictAdmittedPublicProofInput,
};
use std::collections::BTreeSet;

#[test]
fn public_proof_chain_consumes_planner_route_products_only() {
    let closeout = current_worth_touched_graph_conflict_public_closeout()
        .expect("current public proof should assemble from planner-owned route products");
    let selected_route_packet =
        crate::workload_composition::current_worth_touched_graph_conflict_selected_route_packet()
            .expect("current selected-route packet should build");

    assert_eq!(
        closeout.proof_chain().selected_route_packet_digest(),
        selected_route_packet.packet_digest()
    );
}

#[test]
fn kernel_public_closeout_routes_through_planner_owned_proof_only() {
    let error = current_worth_touched_graph_conflict_public_closeout_with_route_loader(|| {
        Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
            "planner-owned selected-route packet is unavailable",
        ))
    })
    .expect_err(
        "covered current public-closeout entrypoint must fail closed when planner-owned selected-route authority is unavailable",
    );

    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable
    );
    assert_eq!(
        error.detail(),
        "planner-owned selected-route packet is unavailable"
    );
}

#[test]
fn public_proof_rejects_foreign_route_or_firewall_identity() {
    let components =
        current_public_closeout_components().expect("current public closeout components");
    let hostile_public_proof_input = hostile_public_proof_input_with_foreign_reuse_basis(
        "foreign-topology-selected-reuse-basis",
    );

    let error = publish_from_parts(
        components.input().expect("current closeout input"),
        components.cutover(),
        components.selected_route_packet(),
        &hostile_public_proof_input,
    )
    .expect_err("public proof assembly must reject foreign planner proof input");

    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain
    );
}

#[test]
fn public_and_diagnostic_residue_is_exact() {
    let components =
        current_public_closeout_components().expect("current public closeout components");
    let residue_rows = components.residue_chain();

    let expected_query_gaps = topology::facade::current_query_backed_consumer_residue_manifest()
        .iter()
        .filter(|row| {
            matches!(
                row.disposition(),
                topology::facade::QueryBackedConsumerResidueDisposition::QueryGap
            )
        })
        .map(|row| row.current_surface().to_string())
        .chain(
            worth_spatial::facade::evidence_lookup_public_closeout::current_evidence_lookup_public_closeout_residue_manifest()
                .iter()
                .filter(|row| {
                    matches!(
                        row.disposition(),
                        worth_spatial::facade::evidence_lookup_public_closeout::EvidenceLookupPublicCloseoutResidueDisposition::QueryGap
                    )
                })
                .map(|row| row.current_surface().to_string()),
        )
        .collect::<BTreeSet<_>>();
    let actual_query_gaps = residue_rows
        .rows()
        .iter()
        .filter(|row| row.disposition().as_str() == "query-gap")
        .map(|row| row.surface_name().to_string())
        .collect::<BTreeSet<_>>();

    let expected_topo_local_residue =
        topology::facade::current_query_backed_consumer_residue_manifest()
            .iter()
            .filter(|row| {
                row.owner() == topology::facade::QueryBackedConsumerResidueOwner::WorthTopo
                    && row.disposition()
                        == topology::facade::QueryBackedConsumerResidueDisposition::ExplicitResidue
            })
            .map(|row| row.current_surface().to_string())
            .collect::<BTreeSet<_>>();
    let actual_topo_local_residue = residue_rows
        .rows()
        .iter()
        .filter(|row| {
            row.owner() == "worth-topo" && row.disposition().as_str() == "explicit-residue"
        })
        .map(|row| row.surface_name().to_string())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_query_gaps, expected_query_gaps);
    assert!(actual_topo_local_residue.is_superset(&expected_topo_local_residue));
    assert!(residue_rows
        .rows()
        .iter()
        .all(|row| !row.owner().is_empty()));
    assert!(residue_rows
        .rows()
        .iter()
        .all(|row| !row.blocker().is_empty()));
    assert!(residue_rows
        .rows()
        .iter()
        .all(|row| !row.removal_trigger().is_empty()));
}

#[test]
fn query_gap_rows_are_distinct_from_local_debt() {
    let components =
        current_public_closeout_components().expect("current public closeout components");
    let residue_rows = components.residue_chain();

    let query_gap_owners = residue_rows
        .rows()
        .iter()
        .filter(|row| row.disposition().as_str() == "query-gap")
        .map(|row| row.owner().to_string())
        .collect::<BTreeSet<_>>();
    let local_residue_owners = residue_rows
        .rows()
        .iter()
        .filter(|row| row.disposition().as_str() == "explicit-residue")
        .map(|row| row.owner().to_string())
        .collect::<BTreeSet<_>>();

    assert!(query_gap_owners.contains("forge-query"));
    assert!(!query_gap_owners.is_empty());
    assert!(!local_residue_owners.is_empty());
    assert_ne!(query_gap_owners, local_residue_owners);
    assert!(residue_rows
        .rows()
        .iter()
        .filter(|row| row.disposition().as_str() == "query-gap")
        .all(|row| {
            row.boundary_posture().as_str() == "query-gap-support-gap"
                && row.query_gap_kind().is_some()
        }));
}

fn hostile_public_proof_input_with_foreign_reuse_basis(
    foreign_reuse_basis_identity_digest: &str,
) -> WorthTouchedGraphConflictAdmittedPublicProofInput {
    let packet =
        crate::workload_composition::current_worth_touched_graph_conflict_selected_route_packet()
            .expect("current selected-route packet should build");
    let current_input = admit_worth_touched_graph_conflict_public_proof_input(&packet)
        .expect("current admitted public proof input should lower");
    WorthTouchedGraphConflictAdmittedPublicProofInput::from_parts(
        current_input.selected_route_packet_digest().to_string(),
        current_input.selected_route_identity_digest().to_string(),
        current_input
            .batch_admission_route_packet_identity()
            .to_string(),
        current_input
            .batch_admission_denial_witness_identity()
            .map(str::to_string),
        current_input.batch_admission_denial_witness_kind(),
        current_input
            .conflict_independence_route_packet_identity()
            .to_string(),
        current_input
            .conflict_independence_denial_witness_identity()
            .map(str::to_string),
        current_input.conflict_independence_denial_witness_kind(),
        current_input
            .replay_undo_route_packet_identity()
            .to_string(),
        current_input.replay_undo_route_family(),
        current_input.selected_family_identity().to_string(),
        current_input.selected_product_identity_digest().to_string(),
        current_input
            .compiled_product_reuse_route_packet_identity()
            .to_string(),
        current_input
            .topology_reuse_posture()
            .expect("current topology reuse posture"),
        current_input
            .spatial_reuse_posture()
            .expect("current spatial reuse posture"),
        foreign_reuse_basis_identity_digest.to_string(),
        current_input
            .selected_witness_identity_digest()
            .map(str::to_string),
        current_input
            .spatial_reuse_decision_identity_digest()
            .map(str::to_string),
        current_input
            .rebuild_denial_identity_digest()
            .map(str::to_string),
        current_input
            .spatial_rebuild_denial_identity_digest()
            .map(str::to_string),
        current_input.spatial_selected_family_identity().to_string(),
        current_input
            .spatial_selected_product_identity_digest()
            .to_string(),
        current_input
            .spatial_equivalence_policy_identity_digest()
            .to_string(),
        current_input
            .spatial_selected_compatibility_basis_identity_digest()
            .to_string(),
        current_input
            .spatial_selected_reuse_basis_identity_digest()
            .to_string(),
        current_input.topology_freshness_requirement_posture(),
        current_input.topology_rendered_output_comparison_posture(),
        current_input.spatial_freshness_requirement_posture(),
        current_input.spatial_rendered_output_comparison_posture(),
        current_input.topology_query_execution_count(),
        current_input.topology_row_scan_fallback_count(),
        current_input.topology_whole_view_fallback_count(),
        current_input.topology_repeated_rediscovery_denied_count(),
        current_input.spatial_receipt_proof_row_count(),
        current_input.spatial_non_ordinary_residue_row_count(),
    )
}
