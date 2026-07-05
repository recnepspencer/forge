use super::{
    ComparePlanarBooleanOverlapRegionReplayParity, PlanarBooleanOverlapRegionEvidenceInput,
    PlanarBooleanOverlapRegionEvidenceReceipt, PlanarBooleanOverlapRegionReplayParityInput,
};
use topology::facade::admit_milestone_seven_five_overlap_readiness_consumer;
use worth_kernel::workload_composition::current_touched_graph_readiness_handoff;

use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    retained_replay_receipt_chain, LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::arrangement_graph::tests::support::overlap_request_and_support;
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanCoplanarOverlapArrangementGraph,
    PlanarBooleanOverlapAdjacencyIndexInput, PlanarBooleanOverlapCellContainmentInput,
    PlanarBooleanOverlapCellContainmentMap, PlanarBooleanOverlapCellWindingField,
    PlanarBooleanOverlapCellWindingFieldInput, PlanarBooleanOverlapIslandCandidateInput,
    PlanarBooleanOverlapIslandComponentBundle, PlanarBooleanOverlapParticipationRecovery,
    PlanarBooleanOverlapParticipationRecoveryInput, PlanarBooleanOverlapRegionAdjacencyIndex,
    PlanarBooleanOverlapRegionExtractionRequest, PlanarBooleanOverlapRegionIdentityLineageBundle,
    PlanarBooleanOverlapRegionLedgerAssemblyBundle, PlanarBooleanOverlapArrangementGraphInput,
    PlanarBooleanPreRegionNormalizationBundle, PlanarBooleanSharedAreaAdmissionBundle,
};
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;

fn replay_receipts() -> ReplayReceiptSet {
    retained_replay_receipt_chain("overlap-replay-closeout")
}

fn clone_loop_entry_order(order: &LoopFixtureEntryOrder) -> LoopFixtureEntryOrder {
    match order {
        LoopFixtureEntryOrder::Canonical => LoopFixtureEntryOrder::Canonical,
        LoopFixtureEntryOrder::Replayed => LoopFixtureEntryOrder::Replayed,
    }
}

fn identity_lineage_for_order(
    loop_entry_order: &LoopFixtureEntryOrder,
) -> (
    PlanarBooleanOverlapRegionExtractionRequest,
    PlanarBooleanOverlapRegionIdentityLineageBundle,
) {
    let (request, support) =
        overlap_request_and_support(clone_loop_entry_order(loop_entry_order));

    let participation = PlanarBooleanOverlapParticipationRecovery::recover(
        PlanarBooleanOverlapParticipationRecoveryInput::from_request_and_loop_support(
            &request, &support,
        ),
    )
    .expect("participation");
    let adjacency = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            participation.loop_participation_map(),
            participation.island_participation_map(),
            participation.chain_lineage_map(),
        ),
    )
    .expect("adjacency");
    let arrangement = PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect("arrangement");
    let shared_area = shared_area_bundle_from_arrangement(&arrangement);
    let pre_region = PlanarBooleanPreRegionNormalizationBundle::from_shared_area_admission(
        &shared_area,
        participation.chain_lineage_map(),
    )
    .expect("pre-region normalization");
    let candidate_bundle = pre_region
        .promote_overlap_region_candidates(&shared_area)
        .expect("candidate promotion");
    let canonical_bundle = candidate_bundle
        .normalize_post_admission_canonical_winding()
        .expect("canonical winding");

    (
        request,
        canonical_bundle
            .mint_overlap_region_identity_lineage()
            .expect("identity lineage"),
    )
}

fn shared_area_bundle_from_arrangement(
    arrangement: &PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanSharedAreaAdmissionBundle {
    let containment = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(arrangement),
    )
    .expect("containment");
    let winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(arrangement, &containment),
    )
    .expect("winding");
    let island_bundle = PlanarBooleanOverlapIslandComponentBundle::admit(
        PlanarBooleanOverlapIslandCandidateInput::from_cell_classification(
            arrangement,
            &containment,
            &winding,
        ),
    )
    .expect("island components");
    let boundary_bundle: PlanarBooleanBoundaryContactClassificationBundle = island_bundle
        .classify_boundary_contact_components()
        .expect("boundary contact");
    boundary_bundle
        .admit_shared_area_components(&containment, &winding)
        .expect("shared area")
}

fn evidence_for(
    loop_entry_order: &LoopFixtureEntryOrder,
    replay_receipts: &ReplayReceiptSet,
) -> (
    PlanarBooleanOverlapRegionExtractionRequest,
    PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    PlanarBooleanOverlapRegionEvidenceReceipt,
) {
    let readiness = current_touched_graph_readiness_handoff().expect("readiness handoff");
    let readiness_consumer =
        admit_milestone_seven_five_overlap_readiness_consumer(&readiness).expect("consumer");
    let (request, identity_lineage) = identity_lineage_for_order(loop_entry_order);
    let ledger_bundle = identity_lineage
        .mint_overlap_region_ledger()
        .expect("ledger");
    let evidence = PlanarBooleanOverlapRegionEvidenceReceipt::admit(
        PlanarBooleanOverlapRegionEvidenceInput::from_readiness_and_request_and_ledger(
            &readiness,
            &readiness_consumer,
            &request,
            ledger_bundle.receipt(),
            replay_receipts,
        ),
    )
    .expect("evidence");
    (request, ledger_bundle, evidence)
}

#[test]
fn overlap_evidence_and_replay_closeout_remain_replay_stable() {
    let receipts = replay_receipts();
    let (_, canonical_ledger, canonical_evidence) =
        evidence_for(&LoopFixtureEntryOrder::Canonical, &receipts);
    let (_, replayed_ledger, replayed_evidence) =
        evidence_for(&LoopFixtureEntryOrder::Replayed, &receipts);

    let replay = ComparePlanarBooleanOverlapRegionReplayParity::compare(
        PlanarBooleanOverlapRegionReplayParityInput::admit_from_ledger_and_evidence(
            canonical_ledger.receipt(),
            replayed_ledger.receipt(),
            &canonical_evidence,
            &replayed_evidence,
            &receipts,
        )
        .expect("replay input"),
    )
    .expect("replay parity");

    assert_eq!(replay.rows().len(), 11);
    assert_eq!(
        replay.checkpoint_receipt().checkpoint_identity(),
        replay.checkpoint_receipt().checkpoint_identity()
    );
    assert_eq!(
        canonical_evidence.readiness_handoff_identity(),
        replayed_evidence.readiness_handoff_identity()
    );
    assert_eq!(
        canonical_evidence.readiness_consumer_identity(),
        replayed_evidence.readiness_consumer_identity()
    );
}
