use super::milestone_fourteen_seed::WorthTouchedGraphConflictMilestoneFourteenSeed;
use super::public_closeout::{
    current_public_closeout_components,
    current_worth_touched_graph_conflict_milestone_fourteen_seed,
    current_worth_touched_graph_conflict_public_closeout, publish_from_parts,
    WorthTouchedGraphConflictPublicCloseoutErrorKind,
};
use super::residue_chain::{
    WorthTouchedGraphConflictResidueBoundaryPosture, WorthTouchedGraphConflictResidueChain,
    WorthTouchedGraphConflictResidueRow,
};
use crate::workload_composition::{
    current_worth_touched_graph_conflict_deletion_closeout,
    current_worth_touched_graph_conflict_source_firewall_report,
    worth_workload::current_worth_workload_ordinary_consumer_cutover,
};

#[test]
fn milestone_thirteen_closeout_requires_real_cutover() {
    let components = current_public_closeout_components().expect("current closeout components");
    let residue_chain = with_covered_ordinary_dependency(components.residue_chain());

    let error = publish_from_parts(
        components.input().expect("current closeout input"),
        components.cutover(),
        residue_chain,
    )
    .expect_err("public closeout must reject an open ordinary-consumer dependency");

    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictPublicCloseoutErrorKind::OrdinaryConsumerDependencyStillOpen
    );
}

#[test]
fn closeout_binds_full_conflict_authority_chain() {
    let closeout = current_worth_touched_graph_conflict_public_closeout()
        .expect("current public closeout should publish from real proof products");
    let cutover =
        current_worth_workload_ordinary_consumer_cutover().expect("current cutover should build");

    assert_eq!(
        closeout.proof_chain().selected_batch_plan_digest(),
        closeout
            .milestone_fourteen_seed()
            .selected_batch_plan_digest()
    );
    assert_eq!(
        closeout.proof_chain().batch_execution_receipt_digest(),
        closeout
            .milestone_fourteen_seed()
            .batch_execution_receipt_digest()
    );
    assert_eq!(
        closeout.proof_chain().selected_conflict_plan_digests(),
        cutover
            .batch_execution_receipt()
            .selected_conflict_plan_digests()
    );
    assert_eq!(
        closeout.proof_chain().independence_proof_digests(),
        cutover
            .batch_execution_receipt()
            .independence_proof_identities()
    );
    let route_authority_digests = cutover
        .rows()
        .iter()
        .filter(|row| {
            row.posture()
                == crate::workload_composition::worth_workload::WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer
        })
        .map(|row| {
            row.selected_plan_witness()
                .expect("selected-plan ordinary consumer rows should carry a bound receipt witness")
                .route_authority_digest()
        })
        .collect::<Vec<_>>();
    assert_eq!(route_authority_digests.len(), 3);
    assert_ne!(route_authority_digests[0], route_authority_digests[1]);
    assert_ne!(route_authority_digests[1], route_authority_digests[2]);
    for row in cutover.rows().iter().filter(|row| {
        row.posture()
            == crate::workload_composition::worth_workload::WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer
    }) {
        assert_eq!(
            row.selected_plan_witness()
                .expect("selected-plan ordinary consumer rows should carry a bound receipt witness")
                .batch_execution_receipt_digest(),
            closeout.proof_chain().batch_execution_receipt_digest()
        );
        assert!(!row
            .selected_plan_witness()
            .expect("selected-plan ordinary consumer rows should carry a bound receipt witness")
            .route_lineage_digest()
            .is_empty());
        assert!(!row
            .selected_plan_witness()
            .expect("selected-plan ordinary consumer rows should carry a bound receipt witness")
            .route_authority_digest()
            .is_empty());
    }
    let replay_undo_row = cutover
        .rows()
        .iter()
        .find(|row| row.surface_name() == "admit_boolean_split_replay_undo_boundary")
        .expect("replay/undo selected-plan row should exist");
    let replay_undo_witness = replay_undo_row
        .selected_plan_witness()
        .expect("replay/undo selected-plan row should carry a bound proof witness");
    assert!(replay_undo_witness
        .replay_undo_boundary_proof_digest()
        .is_some_and(|value| !value.is_empty()));
    assert!(replay_undo_witness
        .transaction_packet_identity()
        .is_some_and(|value| !value.is_empty()));
    assert!(replay_undo_witness
        .replay_scope_identity()
        .is_some_and(|value| !value.is_empty()));
    assert!(replay_undo_witness
        .undo_scope_identity()
        .is_some_and(|value| !value.is_empty()));
    assert_eq!(
        closeout.proof_chain().replay_undo_boundary_proof_digests(),
        &[replay_undo_witness
            .replay_undo_boundary_proof_digest()
            .expect("replay/undo proof digest should survive into public closeout")
            .to_string()]
    );
    assert_eq!(
        closeout.proof_chain().transaction_packet_identities(),
        &[replay_undo_witness
            .transaction_packet_identity()
            .expect("replay/undo packet identity should survive into public closeout")
            .to_string()]
    );
    assert_eq!(
        closeout.proof_chain().replay_scope_identities(),
        &[replay_undo_witness
            .replay_scope_identity()
            .expect("replay scope identity should survive into public closeout")
            .to_string()]
    );
    assert_eq!(
        closeout.proof_chain().undo_scope_identities(),
        &[replay_undo_witness
            .undo_scope_identity()
            .expect("undo scope identity should survive into public closeout")
            .to_string()]
    );
    assert_eq!(
        closeout.source_firewall_digest(),
        current_worth_touched_graph_conflict_source_firewall_report()
            .expect("current firewall report")
            .report_digest()
    );
    assert_eq!(
        closeout.deletion_closeout_digest(),
        current_worth_touched_graph_conflict_deletion_closeout()
            .expect("current deletion closeout")
            .closeout_digest()
    );
    assert!(!closeout.closeout_digest().is_empty());
}

#[test]
fn milestone_fourteen_seed_carries_overlap_identity_without_rediscovery() {
    current_worth_touched_graph_conflict_public_closeout()
        .expect("current public closeout should publish from real proof products");
    let seed = current_worth_touched_graph_conflict_milestone_fourteen_seed()
        .expect("current milestone 14 seed should derive from the canonical closeout");
    let seed: &WorthTouchedGraphConflictMilestoneFourteenSeed = &seed;
    let cutover =
        current_worth_workload_ordinary_consumer_cutover().expect("current cutover should build");
    let receipt = cutover.batch_execution_receipt();

    assert_eq!(
        seed.overlap_identity_digests(),
        receipt.overlap_identity_digests()
    );
    assert_eq!(
        seed.locality_footprint_digests(),
        receipt.locality_footprint_digests()
    );
    assert_eq!(
        seed.selected_conflict_plan_digests(),
        receipt.selected_conflict_plan_digests()
    );
    assert_eq!(
        seed.independence_proof_digests(),
        receipt.independence_proof_identities()
    );
    assert_eq!(
        seed.selected_batch_plan_digest(),
        receipt.selected_batch_plan_digest()
    );
    assert_eq!(
        seed.batch_execution_receipt_digest(),
        receipt.execution_receipt_digest()
    );
    let replay_undo_witness = cutover
        .rows()
        .iter()
        .find(|row| row.surface_name() == "admit_boolean_split_replay_undo_boundary")
        .and_then(|row| row.selected_plan_witness())
        .expect("replay/undo selected-plan row should carry a proof witness");
    assert_eq!(
        seed.replay_undo_boundary_proof_digests(),
        &[replay_undo_witness
            .replay_undo_boundary_proof_digest()
            .expect("replay/undo boundary proof should survive into milestone 14 seed")
            .to_string()]
    );
    assert_eq!(
        seed.transaction_packet_identities(),
        &[replay_undo_witness
            .transaction_packet_identity()
            .expect("replay/undo packet identity should survive into milestone 14 seed")
            .to_string()]
    );
    assert_eq!(
        seed.replay_scope_identities(),
        &[replay_undo_witness
            .replay_scope_identity()
            .expect("replay scope identity should survive into milestone 14 seed")
            .to_string()]
    );
    assert_eq!(
        seed.undo_scope_identities(),
        &[replay_undo_witness
            .undo_scope_identity()
            .expect("undo scope identity should survive into milestone 14 seed")
            .to_string()]
    );
    assert_eq!(
        seed.source_firewall_digest(),
        current_worth_touched_graph_conflict_source_firewall_report()
            .expect("current firewall report")
            .report_digest()
    );
    assert!(!seed.residue_digest().is_empty());
    assert!(!seed.seed_digest().is_empty());
}

#[test]
fn touched_graph_closeout_rejects_foreign_replay_undo_proof_identities() {
    let components = current_public_closeout_components().expect("current closeout components");
    let foreign_cutover = current_worth_workload_ordinary_consumer_cutover()
        .expect("foreign cutover fixture should build")
        .with_test_replay_undo_selected_plan_identity_override(
            "foreign-boundary-proof-digest",
            "foreign-transaction-packet-identity",
            "foreign-replay-scope-identity",
            "foreign-undo-scope-identity",
        );

    let error = publish_from_parts(
        components.input().expect("current closeout input"),
        &foreign_cutover,
        components.residue_chain(),
    )
    .expect_err("public closeout must reject foreign replay/undo proof joins");

    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain
    );
    assert!(error
        .detail()
        .contains("current replay/undo admitted-boundary proof identities"));
}

fn with_covered_ordinary_dependency(
    residue_chain: WorthTouchedGraphConflictResidueChain,
) -> WorthTouchedGraphConflictResidueChain {
    let mut rows = residue_chain.rows().to_vec();
    rows.push(WorthTouchedGraphConflictResidueRow::new(
        "WorthWorkload::compose",
        "worth-kernel",
        "ordinary consumer still routes through local conflict composition",
        "phase 13 public closeout must delete or cap the ordinary dependency",
        WorthTouchedGraphConflictResidueBoundaryPosture::CoveredOrdinaryConsumerDependency,
    ));
    WorthTouchedGraphConflictResidueChain::from_rows(rows)
}
