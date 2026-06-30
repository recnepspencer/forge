use super::edge_splitting_replay_parity_support::{
    build_edge_split_replay_parity_subject, EdgeSplitReplayParitySubject,
};
use super::loop_workload_evidence_support;
use super::metaboss_support::MetabossEventExtractionSubject;
use worth_kernel::workload_composition::{
    BuiltBooleanOperandPairRecipe, WorthWorkload, WorthWorkloadParts,
};
use worth_spatial::facade::query_adoption::{
    spatial_query_graph_obligation_adoption_proof_for_descriptor,
    spatial_query_graph_obligation_residue_manifest,
};
use worth_spatial::facade::workload_vocabulary::{
    lower_spatial_touch_authority_to_query_descriptor, BooleanEvidenceReceipt,
    BooleanEvidenceRowAuthority, BooleanEvidenceStageKind, SpatialGeometryEvidenceTouchAuthority,
    WorkloadEvidenceStage,
};

pub(crate) fn assert_segment_pair_receipt_admits_from_completed_workload_handoff() {
    let subject =
        MetabossEventExtractionSubject::certify("phase4 segment-pair spatial touch handoff");
    let receipt = &subject.inputs().pair_worklist;
    let completed_workload = completed_workload_with_boolean_receipt(subject.pair(), receipt);

    completed_workload
        .require_boolean_segment_pair_enumeration(receipt)
        .expect("completed workload must require the real segment-pair receipt");
    assert_completed_workload_admits_receipt(
        &completed_workload,
        receipt,
        BooleanEvidenceStageKind::SegmentPairEnumeration,
    );
}

pub(crate) fn assert_event_ledger_receipt_admits_from_completed_workload_handoff() {
    let subject =
        MetabossEventExtractionSubject::certify("phase4 event-ledger spatial touch handoff");
    let completed_workload =
        completed_workload_with_boolean_receipt(subject.pair(), subject.ledger());

    completed_workload
        .require_boolean_event_ledger(subject.ledger())
        .expect("completed workload must require the real event-ledger receipt");
    assert_completed_workload_admits_receipt(
        &completed_workload,
        subject.ledger(),
        BooleanEvidenceStageKind::EventLedger,
    );
}

pub(crate) fn assert_split_receipt_admits_from_completed_workload_handoff() {
    let split_handoffs = completed_split_workloads("phase4 split spatial touch handoff");
    let completed_workload = split_handoffs.original_workload;
    let replay_subject = split_handoffs.replay_subject;
    let receipt = replay_subject.original_ledger.receipt();

    completed_workload
        .require_boolean_split(receipt)
        .expect("completed workload must require the real split receipt");
    assert_completed_workload_admits_receipt(
        &completed_workload,
        receipt,
        BooleanEvidenceStageKind::Split,
    );
}

pub(crate) fn assert_loop_receipt_admits_from_completed_workload_handoff() {
    loop_workload_evidence_support::assert_loop_ledger_satisfies_workload_requirement_and_runtime_registration();
}

pub(crate) fn assert_split_replay_preserves_completed_workload_spatial_touch_authority() {
    let split_handoffs = completed_split_workloads("phase4 split replay spatial touch");
    let original_workload = split_handoffs.original_workload;
    let replayed_workload = split_handoffs.replayed_workload;
    let replay_subject = split_handoffs.replay_subject;
    let original_receipt = replay_subject.original_ledger.receipt();
    let replayed_receipt = replay_subject.replayed_ledger.receipt();

    original_workload
        .require_boolean_split(original_receipt)
        .expect("original completed workload must require the split receipt");
    replayed_workload
        .require_boolean_split(replayed_receipt)
        .expect("replayed completed workload must require the split receipt");
    assert_eq!(
        original_receipt.receipt_identity(),
        replayed_receipt.receipt_identity()
    );

    let original_authority = assert_completed_workload_admits_receipt(
        &original_workload,
        original_receipt,
        BooleanEvidenceStageKind::Split,
    );
    let replayed_authority = assert_completed_workload_admits_receipt(
        &replayed_workload,
        replayed_receipt,
        BooleanEvidenceStageKind::Split,
    );

    assert_eq!(original_authority.digest(), replayed_authority.digest());
    assert_eq!(
        original_authority.stage_index_identity(),
        replayed_authority.stage_index_identity()
    );
    assert_eq!(
        original_authority.stage_link_set_identity(),
        replayed_authority.stage_link_set_identity()
    );
    assert_eq!(
        original_authority.evidence_counters(),
        replayed_authority.evidence_counters()
    );
    assert_eq!(original_authority.support(), replayed_authority.support());
}

pub(crate) fn assert_loop_replay_preserves_completed_workload_spatial_touch_authority() {
    loop_workload_evidence_support::assert_loop_ledger_replay_branch_preserves_workload_requirement(
    );
}

pub(crate) fn assert_split_replay_preserves_cross_crate_spatial_query_handoff() {
    let split_handoffs = completed_split_workloads("phase10 cross crate spatial query handoff");
    let original_workload = split_handoffs.original_workload;
    let replayed_workload = split_handoffs.replayed_workload;
    let replay_subject = split_handoffs.replay_subject;
    let original_receipt = replay_subject.original_ledger.receipt();
    let replayed_receipt = replay_subject.replayed_ledger.receipt();

    original_workload
        .require_boolean_split(original_receipt)
        .expect("original completed workload must require the split receipt");
    replayed_workload
        .require_boolean_split(replayed_receipt)
        .expect("replayed completed workload must require the split receipt");
    assert_eq!(
        original_receipt.receipt_identity(),
        replayed_receipt.receipt_identity()
    );

    let original_authority = assert_completed_workload_admits_receipt(
        &original_workload,
        original_receipt,
        BooleanEvidenceStageKind::Split,
    );
    let replayed_authority = assert_completed_workload_admits_receipt(
        &replayed_workload,
        replayed_receipt,
        BooleanEvidenceStageKind::Split,
    );
    let original_lookup = original_authority
        .spatial_evidence_lookup(original_workload.evidence_ledger())
        .expect("original authority must derive lookup through spatial facade");
    let replayed_lookup = replayed_authority
        .spatial_evidence_lookup(replayed_workload.evidence_ledger())
        .expect("replayed authority must derive lookup through spatial facade");
    let original_descriptor =
        lower_spatial_touch_authority_to_query_descriptor(&original_authority, &original_lookup)
            .expect("original authority must lower to Query descriptor");
    let replayed_descriptor =
        lower_spatial_touch_authority_to_query_descriptor(&replayed_authority, &replayed_lookup)
            .expect("replayed authority must lower to Query descriptor");
    let original_adoption_proof =
        spatial_query_graph_obligation_adoption_proof_for_descriptor(&original_descriptor)
            .expect("original Consumer Kit adoption proof");
    let replayed_adoption_proof =
        spatial_query_graph_obligation_adoption_proof_for_descriptor(&replayed_descriptor)
            .expect("replayed Consumer Kit adoption proof");
    let residue_manifest =
        spatial_query_graph_obligation_residue_manifest().expect("Consumer Kit residue manifest");

    assert_eq!(original_authority.digest(), replayed_authority.digest());
    assert_eq!(original_lookup.lookup_key(), replayed_lookup.lookup_key());
    assert_eq!(
        original_lookup.product_digest(),
        replayed_lookup.product_digest()
    );
    assert_eq!(
        original_descriptor.touch_descriptor().descriptor_digest(),
        replayed_descriptor.touch_descriptor().descriptor_digest()
    );
    assert_eq!(
        original_descriptor.operating_world().descriptor_digest(),
        replayed_descriptor.operating_world().descriptor_digest()
    );
    assert_eq!(
        original_descriptor.product_digest(),
        replayed_descriptor.product_digest()
    );
    assert_eq!(
        original_descriptor.lookup_product_digest(),
        original_lookup.product_digest()
    );
    assert_eq!(
        replayed_descriptor.lookup_product_digest(),
        replayed_lookup.product_digest()
    );
    assert!(!original_descriptor.claims_milestone_five_selection_closeout());
    assert!(!replayed_descriptor.claims_milestone_five_selection_closeout());
    assert_eq!(original_descriptor.counters().broad_ledger_scan_count(), 0);
    assert_eq!(replayed_descriptor.counters().broad_ledger_scan_count(), 0);
    assert!(!original_adoption_proof
        .manifest()
        .manifest_digest()
        .is_empty());
    assert_eq!(
        original_adoption_proof.manifest().manifest_digest(),
        replayed_adoption_proof.manifest().manifest_digest()
    );
    assert_eq!(
        original_adoption_proof.manifest().residue_manifest_digest(),
        residue_manifest.manifest_digest()
    );
    assert_eq!(
        replayed_adoption_proof.manifest().residue_manifest_digest(),
        residue_manifest.manifest_digest()
    );
    assert_eq!(
        original_adoption_proof.manifest().execution_proof_digest(),
        Some(original_adoption_proof.execution_proof().proof_digest())
    );
    assert_eq!(
        replayed_adoption_proof.manifest().execution_proof_digest(),
        Some(replayed_adoption_proof.execution_proof().proof_digest())
    );
    assert_eq!(
        original_adoption_proof.execution_proof().proof_digest(),
        replayed_adoption_proof.execution_proof().proof_digest()
    );
    assert_eq!(
        original_adoption_proof
            .execution_proof()
            .selection_proof()
            .selection_digest(),
        replayed_adoption_proof
            .execution_proof()
            .selection_proof()
            .selection_digest()
    );
    assert_eq!(
        original_adoption_proof
            .execution_proof()
            .has_real_executor_rows(),
        replayed_adoption_proof
            .execution_proof()
            .has_real_executor_rows()
    );
    assert_eq!(residue_manifest.rows().len(), 2);
}

fn completed_workload_with_boolean_receipt<T>(
    pair: &BuiltBooleanOperandPairRecipe,
    receipt: &T,
) -> WorthWorkload
where
    T: BooleanEvidenceRowAuthority + 'static,
{
    let left = pair.left().workload();
    let evidence_ledger = left
        .evidence_ledger()
        .with_boolean_evidence_receipt(receipt)
        .expect("real boolean receipt should extend the complete workload ledger");

    WorthWorkload::compose(WorthWorkloadParts {
        topology: left.topology().clone(),
        geometry_binding: left.geometry_binding().clone(),
        surface_support: left.surface_support().clone(),
        projection: left.projection().clone(),
        transform: left.transform().clone(),
        retained_replay: left.retained_replay().clone(),
        batch_admission_execution: left.batch_admission_execution().cloned(),
        diagnostics: left.diagnostics().clone(),
        response: left.response().clone(),
        evidence_ledger,
    })
    .expect("completed workload should compose from the receipt-backed complete ledger")
}

fn completed_split_workloads(label: &'static str) -> CompletedSplitSpatialTouchHandoffs {
    let subject = MetabossEventExtractionSubject::certify(label);
    let replay_subject = build_edge_split_replay_parity_subject(&subject);
    let original_workload = subject
        .pair()
        .left()
        .workload()
        .with_completed_boolean_split_ledger(replay_subject.original_ledger.receipt())
        .expect("original split receipt should complete the workload ledger");
    let replayed_workload = subject
        .pair()
        .left()
        .workload()
        .with_completed_boolean_split_ledger(replay_subject.replayed_ledger.receipt())
        .expect("replayed split receipt should complete the workload ledger");

    CompletedSplitSpatialTouchHandoffs {
        original_workload,
        replayed_workload,
        replay_subject,
    }
}

fn assert_completed_workload_admits_receipt<T>(
    completed_workload: &WorthWorkload,
    receipt: &T,
    expected_stage: BooleanEvidenceStageKind,
) -> SpatialGeometryEvidenceTouchAuthority
where
    T: BooleanEvidenceReceipt + 'static,
{
    let authority = completed_workload
        .admit_spatial_geometry_evidence_touch(receipt)
        .expect("completed workload must admit spatial touch authority");
    let expected_evidence_stage = expected_stage.evidence_stage();
    let expected_stage_link = completed_workload
        .evidence_ledger()
        .link_required_stages(&[expected_evidence_stage])
        .expect("completed workload must expose the receipt stage link");

    assert_eq!(authority.boolean_stage(), expected_stage);
    assert_eq!(authority.evidence_stage(), expected_evidence_stage);
    assert_eq!(authority.evidence_identity(), receipt.evidence_identity());
    assert_eq!(authority.evidence_counters(), receipt.evidence_counters());
    assert_eq!(authority.support(), receipt.evidence_support());
    assert_eq!(
        authority.stage_index_identity(),
        completed_workload
            .evidence_ledger()
            .stage_index()
            .index_identity()
    );
    assert_eq!(
        authority.stage_link_set_identity(),
        expected_stage_link.link_set_identity()
    );
    assert_single_indexed_lookup(&authority, expected_evidence_stage);
    authority
}

fn assert_single_indexed_lookup(
    authority: &SpatialGeometryEvidenceTouchAuthority,
    expected_stage: WorkloadEvidenceStage,
) {
    assert_eq!(authority.lookup_counters().required_stage_count(), 1);
    assert_eq!(authority.lookup_counters().indexed_lookup_count(), 1);
    assert_eq!(authority.lookup_counters().raw_row_scan_count(), 0);
    assert_eq!(authority.lookup_counters().rejected_raw_row_scan_count(), 0);
    assert_eq!(
        authority
            .lookup_counters()
            .rejected_string_prefix_stage_link_count(),
        0
    );
    assert_eq!(authority.evidence_stage(), expected_stage);
}

struct CompletedSplitSpatialTouchHandoffs {
    original_workload: WorthWorkload,
    replayed_workload: WorthWorkload,
    replay_subject: EdgeSplitReplayParitySubject,
}
