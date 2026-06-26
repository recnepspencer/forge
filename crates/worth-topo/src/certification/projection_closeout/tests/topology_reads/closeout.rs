use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use crate::certification::support::read_proof_harness::TopologyReadProofHarness;
use crate::projection::read_views::domain::closeout::{
    TopologyReadCloseoutStatus, TopologyReadPhaseThreeBlocker, TopologyReadPhaseThreeBlockerStatus,
};
use crate::projection::read_views::domain::parity::{
    build_topology_read_view_parity_artifact, TopologyReadParityKind, TopologyReadViewRef,
};
use crate::projection::read_views::domain::{
    TopologyNoNPlusOneContract, TopologyNoNPlusOneContractStatus,
};
use crate::projection::read_views::domain::{
    TopologyReadAnchorIdentity, TopologyReadRequestFamily,
};
use crate::query_domain::TopologyCurrentHeadReadHandleExt;
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::validation::reference_integrity::build_milestone_one_runtime;

use super::support::{
    current_head_query_handle, current_lookup_rows, seeded_sheet_disk_workspace,
    snapshot_basis_workspace,
};

#[test]
fn topology_read_closeout_reports_unobserved_families_before_any_requests() {
    let query = TopologyReadProofHarness::current_head();
    let closeout_report = query.closeout_report();

    assert_eq!(closeout_report.query_executed_family_count, 0);
    assert_eq!(closeout_report.query_executed_debt_free_family_count, 0);
    assert_eq!(closeout_report.query_executed_debt_backed_family_count, 0);
    assert_eq!(closeout_report.debt_family_count, 0);
    assert_eq!(closeout_report.whole_view_debt_request_count, 0);
    assert_eq!(closeout_report.row_scan_fallback_request_count, 0);
    assert_eq!(closeout_report.repeated_rediscovery_denied_count, 0);
    assert_eq!(closeout_report.family_rows().len(), 4);
    assert!(closeout_report.family_rows().iter().all(|row| {
        row.status() == TopologyReadCloseoutStatus::Unobserved
            && row.request_count() == 0
            && row.query_execution_count() == 0
    }));
    assert_eq!(
        closeout_report
            .phase_three_blocker_status(TopologyReadPhaseThreeBlocker::NoObservedRequests),
        TopologyReadPhaseThreeBlockerStatus::Blocked
    );
    assert_eq!(
        closeout_report
            .phase_three_blocker_status(TopologyReadPhaseThreeBlocker::ParityDeterminismGap),
        TopologyReadPhaseThreeBlockerStatus::Clear
    );
    assert_eq!(closeout_report.no_n_plus_one_contract_rows().len(), 4);
    assert_eq!(
        closeout_report.no_n_plus_one_contract_status(TopologyNoNPlusOneContract::LoweringBreadth),
        TopologyNoNPlusOneContractStatus::Blocked
    );
    assert_eq!(
        closeout_report.no_n_plus_one_contract_status(TopologyNoNPlusOneContract::ViewParity),
        TopologyNoNPlusOneContractStatus::Blocked
    );
    assert!(!closeout_report.phase_three_ready);
}

#[test]
fn topology_read_closeout_requires_no_n_plus_one_contracts_before_phase_three_ready() {
    let (mut workspace, surfaces, _read_basis) =
        seeded_sheet_disk_workspace("query.topology-read-closeout.contract-gate");
    let handle = current_head_query_handle();
    let lookup_rows = current_lookup_rows(&mut workspace, &surfaces);
    let moved_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("sheet disk should expose successor source");
    let moved_anchor = TopologyReadAnchorIdentity::from_runtime_row_label(&moved_identity);
    let mut reads = handle.topology_reads(&mut workspace);

    let _ = reads
        .local_rewire_neighborhood(&moved_anchor, 4)
        .expect("query-native local rewire neighborhood should load");
    let closeout_report = reads.closeout_report();

    assert_eq!(
        closeout_report
            .phase_three_blocker_status(TopologyReadPhaseThreeBlocker::NoObservedRequests),
        TopologyReadPhaseThreeBlockerStatus::Clear
    );
    assert_eq!(
        closeout_report
            .phase_three_blocker_status(TopologyReadPhaseThreeBlocker::ParityDeterminismGap),
        TopologyReadPhaseThreeBlockerStatus::Clear
    );
    assert_eq!(
        closeout_report.no_n_plus_one_contract_status(TopologyNoNPlusOneContract::LoweringBreadth),
        TopologyNoNPlusOneContractStatus::Satisfied
    );
    assert_eq!(
        closeout_report.no_n_plus_one_contract_status(TopologyNoNPlusOneContract::FallbackPosture),
        TopologyNoNPlusOneContractStatus::Satisfied
    );
    assert_eq!(
        closeout_report.no_n_plus_one_contract_status(TopologyNoNPlusOneContract::ViewParity),
        TopologyNoNPlusOneContractStatus::Blocked
    );
    assert!(!closeout_report.phase_three_ready);
}

#[test]
fn topology_read_proof_report_aggregates_request_and_parity_evidence_on_the_boundary() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "query.topology-read-proof.replay",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let replay_basis = verified.read_basis().replay_of();
    let (mut left_workspace, left_assembly) = snapshot_basis_workspace(
        &runtime,
        "query.topology-read-proof.replay.left",
        &verified.read_basis(),
    );
    let (mut right_workspace, _right_assembly) = snapshot_basis_workspace(
        &runtime,
        "query.topology-read-proof.replay.right",
        &replay_basis,
    );
    let left_query = TopologyReadProofHarness::historical_from_workspace_token();
    let right_query = TopologyReadProofHarness::historical_from_workspace_token();
    let left_lookup_rows = current_lookup_rows(&mut left_workspace, &left_assembly);
    let moved_identity = left_lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("sheet disk should expose successor source");
    let left_view = left_query
        .local_rewire_neighborhood(&mut left_workspace, &moved_identity, 4)
        .expect("left local rewire neighborhood should load");
    let right_view = right_query
        .local_rewire_neighborhood(&mut right_workspace, &moved_identity, 4)
        .expect("right local rewire neighborhood should load");
    let left_artifact = build_topology_read_view_parity_artifact(
        &verified.read_basis(),
        TopologyReadViewRef::LocalRewire(&left_view),
    );
    let right_artifact = build_topology_read_view_parity_artifact(
        &replay_basis,
        TopologyReadViewRef::LocalRewire(&right_view),
    );
    let parity = left_query.record_view_parity(
        TopologyReadParityKind::Replay,
        &left_artifact,
        &right_artifact,
    );
    let proof_report = left_query.proof_report();
    let closeout_report = left_query.closeout_report();

    assert!(parity.parity_verified);
    assert_eq!(proof_report.request_aggregate.request_count, 1);
    assert_eq!(
        proof_report
            .request_aggregate
            .query_runtime_historical_execution_count,
        1
    );
    assert_eq!(
        proof_report
            .request_aggregate
            .anchored_expansion_execution_count,
        0
    );
    assert_eq!(
        proof_report
            .request_aggregate
            .explicit_broad_search_execution_count,
        1
    );
    assert_eq!(
        proof_report.request_aggregate.locality_claim_mismatch_count,
        0
    );
    assert_eq!(proof_report.request_aggregate.lowered_traversal_count, 2);
    assert_eq!(proof_report.parity_aggregate.topology_read_parity_count, 1);
    assert_eq!(
        proof_report.parity_aggregate.view_determinism_checked_count,
        1
    );
    assert_eq!(
        proof_report
            .parity_aggregate
            .view_determinism_verified_count,
        1
    );
    assert_eq!(proof_report.parity_aggregate.replay_checked_count, 1);
    assert_eq!(proof_report.parity_aggregate.replay_verified_count, 1);
    assert_eq!(proof_report.parity_aggregate.branch_local_checked_count, 0);
    assert_eq!(proof_report.parity_aggregate.branch_local_verified_count, 0);
    assert_eq!(proof_report.parity_aggregate.parity_rows.len(), 1);
    assert_eq!(
        proof_report.parity_aggregate.parity_rows[0].request_family,
        proof_report.request_aggregate.family_rows[0].request_family
    );
    assert_eq!(closeout_report.query_executed_family_count, 1);
    assert_eq!(closeout_report.query_executed_debt_free_family_count, 1);
    assert_eq!(closeout_report.query_executed_debt_backed_family_count, 0);
    assert_eq!(closeout_report.debt_family_count, 0);
    assert_eq!(closeout_report.whole_view_debt_request_count, 0);
    assert_eq!(closeout_report.row_scan_fallback_request_count, 0);
    assert_eq!(closeout_report.repeated_rediscovery_denied_count, 0);
    assert_eq!(closeout_report.family_rows().len(), 4);
    let local_rewire_row = closeout_report
        .family_rows()
        .iter()
        .find(|row| row.request_family() == TopologyReadRequestFamily::LocalRewireNeighborhood)
        .expect("local rewire closeout row");
    assert_eq!(
        local_rewire_row.status(),
        TopologyReadCloseoutStatus::QueryExecutedDebtFree
    );
    assert!(local_rewire_row
        .reason()
        .contains("without observed debt signals"));
    assert!(local_rewire_row
        .row_digest()
        .contains("request_family=LocalRewireNeighborhood"));
    assert!(local_rewire_row
        .row_digest()
        .contains("status=QueryExecutedDebtFree"));
    assert!(local_rewire_row
        .row_digest()
        .contains("query_execution_count=1"));
    let shared_vertex_row = closeout_report
        .family_rows()
        .iter()
        .find(|row| {
            row.request_family() == TopologyReadRequestFamily::HalfEdgeSharedVertexNeighborhood
        })
        .expect("shared vertex closeout row");
    assert_eq!(
        shared_vertex_row.status(),
        TopologyReadCloseoutStatus::Unobserved
    );
    assert!(shared_vertex_row
        .reason()
        .contains("no executed requests were observed"));
    assert!(shared_vertex_row.row_digest().contains("request_count=0"));
    let no_observed_requests_blocker = closeout_report
        .phase_three_blocker_rows()
        .iter()
        .find(|row| row.blocker() == TopologyReadPhaseThreeBlocker::NoObservedRequests)
        .expect("no observed requests blocker");
    assert_eq!(
        no_observed_requests_blocker.status(),
        TopologyReadPhaseThreeBlockerStatus::Clear
    );
    assert!(no_observed_requests_blocker
        .reason()
        .contains("at least one executed topology-domain read request was observed"));
    assert_eq!(
        closeout_report
            .phase_three_blocker_status(TopologyReadPhaseThreeBlocker::ParityDeterminismGap),
        TopologyReadPhaseThreeBlockerStatus::Clear
    );
    assert_eq!(
        closeout_report
            .phase_three_blocker_status(TopologyReadPhaseThreeBlocker::NoObservedRequests),
        TopologyReadPhaseThreeBlockerStatus::Clear
    );
    assert_eq!(
        closeout_report.no_n_plus_one_contract_status(TopologyNoNPlusOneContract::LoweringBreadth),
        TopologyNoNPlusOneContractStatus::Satisfied
    );
    assert_eq!(
        closeout_report.no_n_plus_one_contract_status(TopologyNoNPlusOneContract::FallbackPosture),
        TopologyNoNPlusOneContractStatus::Satisfied
    );
    assert_eq!(
        closeout_report.no_n_plus_one_contract_status(TopologyNoNPlusOneContract::ViewParity),
        TopologyNoNPlusOneContractStatus::Blocked
    );
    assert_eq!(
        closeout_report
            .no_n_plus_one_contract_status(TopologyNoNPlusOneContract::RelationshipProofPosture),
        TopologyNoNPlusOneContractStatus::Satisfied
    );
    let lowering_contract = closeout_report
        .no_n_plus_one_contract_rows()
        .iter()
        .find(|row| row.contract() == TopologyNoNPlusOneContract::LoweringBreadth)
        .expect("lowering breadth contract row");
    assert!(lowering_contract
        .row_digest()
        .contains("contract=topology_read_lowering_breadth"));
    assert!(lowering_contract
        .reason()
        .contains("exact scope-class breadth"));
    assert!(!closeout_report.phase_three_ready);
}
