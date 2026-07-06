#[path = "../operator_harness/tests_vertical_migration/support/spatial_batch_execution_slice.rs"]
mod spatial_batch_execution_slice_support;
#[path = "../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod workload_evidence_support;

use topology::facade::{
    admit_milestone_seven_five_overlap_readiness_consumer, PlanarBooleanLoopBlueprintRegistry,
    PlanarBooleanOverlapBlueprintRegistry, TopologyMilestoneSevenFiveOverlapReadinessConsumer,
};
use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionParticipationSupport;
use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanCoplanarOverlapArrangementGraph,
    PlanarBooleanOverlapAdjacencyIndexInput, PlanarBooleanOverlapArrangementGraphInput,
    PlanarBooleanOverlapCellContainmentInput, PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingField, PlanarBooleanOverlapCellWindingFieldInput,
    PlanarBooleanOverlapIslandCandidateInput, PlanarBooleanOverlapIslandComponentBundle,
    PlanarBooleanOverlapParticipationRecovery, PlanarBooleanOverlapParticipationRecoveryInput,
    PlanarBooleanOverlapReadinessLoopLedgerBinding, PlanarBooleanOverlapRegionAdjacencyIndex,
    PlanarBooleanOverlapRegionExtractionRequest, PlanarBooleanOverlapRegionExtractionRequestInput,
    PlanarBooleanOverlapRegionLedgerAssemblyBundle, PlanarBooleanPreRegionNormalizationBundle,
    PlanarBooleanSharedAreaAdmissionBundle,
};
use worth_spatial::facade::retained_replay_workload::ReplayReceiptSet;

use crate::workload_composition::{
    current_touched_graph_readiness_handoff, BooleanSplitReplayUndoBoundaryRequest,
    CompletedBooleanLoopReconstructionHandoff,
    CompletedPlanarBooleanOverlapRegionExtractionHandoff,
    PlanarBooleanLoopReconstructionCloseoutInput, PlanarBooleanOverlapRegionCloseoutInput,
};

pub(crate) struct RealOverlapOwnerSeamFixture {
    pub(crate) readiness: schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityReadinessInput,
    pub(crate) readiness_consumer: TopologyMilestoneSevenFiveOverlapReadinessConsumer,
    pub(crate) request: PlanarBooleanOverlapRegionExtractionRequest,
    pub(crate) ledger_bundle: PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    pub(crate) completed: CompletedPlanarBooleanOverlapRegionExtractionHandoff,
    pub(crate) replay_receipts: ReplayReceiptSet,
}

pub(crate) fn completed_overlap_owner_seam_fixture(
    label: &'static str,
) -> RealOverlapOwnerSeamFixture {
    let loop_handoff = completed_loop_handoff_with_real_batch_authority(
        label,
        workload_evidence_support::ReplayBranch::Original,
    );
    let replayed_loop_handoff = completed_loop_handoff_with_real_batch_authority(
        label,
        workload_evidence_support::ReplayBranch::Replayed,
    );
    let replay_subject = workload_evidence_support::build_edge_split_replay_parity_subject(
        &workload_evidence_support::MetabossEventExtractionSubject::certify(label),
    );
    let readiness = current_touched_graph_readiness_handoff().expect("readiness handoff");
    let readiness_consumer =
        admit_milestone_seven_five_overlap_readiness_consumer(&readiness).expect("consumer");
    let (request, ledger_bundle) = overlap_request_and_ledger(&loop_handoff);
    let overlap_registry = PlanarBooleanOverlapBlueprintRegistry::phase_2();
    let completed = loop_handoff
        .complete_planar_boolean_overlap_region_extraction(
            PlanarBooleanOverlapRegionCloseoutInput::new(
                &readiness,
                &readiness_consumer,
                &request,
                &ledger_bundle,
                &replayed_loop_handoff,
                &replay_subject.replay_receipts,
                &overlap_registry.operator_classification_matrix(),
                &overlap_registry.validator_registration_plan(),
            ),
        )
        .expect("overlap closeout should certify through the real owner seam");
    RealOverlapOwnerSeamFixture {
        readiness,
        readiness_consumer,
        request,
        ledger_bundle,
        completed,
        replay_receipts: replay_subject.replay_receipts,
    }
}

pub(crate) fn completed_loop_handoff_with_real_batch_authority(
    label: &'static str,
    branch: workload_evidence_support::ReplayBranch,
) -> CompletedBooleanLoopReconstructionHandoff {
    let batch_execution =
        spatial_batch_execution_slice_support::disjoint_parallel_spatial_batch_execution_slice()
            .execution_receipt()
            .clone();
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let subject = workload_evidence_support::MetabossEventExtractionSubject::certify(label);
    let replay_subject =
        workload_evidence_support::build_edge_split_replay_parity_subject(&subject);
    let replay_report = workload_evidence_support::replay_parity_report(&replay_subject);
    let completed_split_handoff =
        workload_evidence_support::completed_split_handoff_for(&subject, &replay_subject)
            .with_batch_admission_execution(&batch_execution)
            .expect("real loop owner-seam proof requires explicit batch-admission authority");
    let (
        decision_log_receipt,
        validation,
        naming,
        ledger,
        vertices,
        fragments,
        chains,
        split_request,
    ) = match branch {
        workload_evidence_support::ReplayBranch::Original => (
            replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            replay_subject.original_ledger.ledger(),
            &replay_subject.original_products.vertices,
            &replay_subject.original_products.fragments,
            &replay_subject.original_products.chains,
            &replay_subject.original_products.request,
        ),
        workload_evidence_support::ReplayBranch::Replayed => (
            replay_subject.replayed_decision_log.receipt(),
            &replay_subject.replayed_products.validation,
            &replay_subject.replayed_products.naming,
            replay_subject.replayed_ledger.ledger(),
            &replay_subject.replayed_products.vertices,
            &replay_subject.replayed_products.fragments,
            &replay_subject.replayed_products.chains,
            &replay_subject.replayed_products.request,
        ),
    };
    let recovered_source_carriers =
        workload_evidence_support::recovered_source_carriers(&subject, split_request);

    workload_evidence_support::with_packet_backed_loop_boundary_basis(
        &subject,
        |topology_undo_scope, replay_scope, undo_scope| {
            completed_split_handoff
                .admit_batch_execution_cluster()
                .expect("attached split handoff should admit the batch-execution cluster")
                .admit_boolean_split_replay_undo_boundary(
                    BooleanSplitReplayUndoBoundaryRequest::new(
                        topology_undo_scope,
                        replay_scope,
                        undo_scope,
                    ),
                )
                .and_then(|boundary| {
                    boundary.complete_boolean_chain_integration(
                        PlanarBooleanLoopReconstructionCloseoutInput::new(
                            decision_log_receipt,
                            validation,
                            naming,
                            replay_report.receipt(),
                            ledger,
                            &recovered_source_carriers,
                            vertices,
                            fragments,
                            chains,
                            &replay_subject.replay_receipts,
                            &matrix,
                            &validators,
                        ),
                    )
                })
        },
    )
    .expect("loop closeout should certify through the real workload-backed owner seam")
    .into_loop_handoff()
}

pub(crate) fn overlap_request_and_ledger(
    loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
) -> (
    PlanarBooleanOverlapRegionExtractionRequest,
    PlanarBooleanOverlapRegionLedgerAssemblyBundle,
) {
    let readiness = current_touched_graph_readiness_handoff().expect("readiness handoff");
    let readiness_consumer =
        admit_milestone_seven_five_overlap_readiness_consumer(&readiness).expect("consumer");
    let request = PlanarBooleanOverlapRegionExtractionRequest::admit(
        PlanarBooleanOverlapRegionExtractionRequestInput::from_readiness_consumer_and_loop_ledger(
            &readiness_consumer,
            loop_handoff.loop_ledger_receipt(),
        ),
    )
    .expect("overlap request");
    let loop_products = loop_handoff
        .products()
        .expect("real loop handoff should retain canonical phase products");
    let support =
        PlanarBooleanLoopReconstructionParticipationSupport::admit_from_ledger_and_products(
            loop_products.loop_ledger(),
            loop_products.role_outcomes(),
            loop_products.island_partition(),
            loop_products.persistent_name_propagation_map(),
            loop_products.source_provenance().fragment_membership_map(),
            loop_products
                .source_provenance()
                .overlap_chain_lineage_map(),
            loop_products.source_provenance().source_loop_carriers(),
        )
        .expect("participation support");
    let participation = PlanarBooleanOverlapParticipationRecovery::recover(
        PlanarBooleanOverlapParticipationRecoveryInput::from_request_and_loop_support(
            &request, &support,
        ),
    )
    .expect("real loop handoff should recover overlap participation from carried 7.4 provenance");
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
    let identity_lineage = canonical_bundle
        .mint_overlap_region_identity_lineage()
        .expect("identity lineage");

    (
        request,
        identity_lineage
            .mint_overlap_region_ledger()
            .expect("ledger"),
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

pub(crate) fn foreign_readiness_binding(
    label: &'static str,
) -> PlanarBooleanOverlapReadinessLoopLedgerBinding {
    let foreign_loop_handoff = completed_loop_handoff_with_real_batch_authority(
        label,
        workload_evidence_support::ReplayBranch::Original,
    );
    let foreign_readiness = current_touched_graph_readiness_handoff().expect("readiness handoff");
    let foreign_consumer =
        admit_milestone_seven_five_overlap_readiness_consumer(&foreign_readiness)
            .expect("consumer");
    let foreign_request = PlanarBooleanOverlapRegionExtractionRequest::admit(
        PlanarBooleanOverlapRegionExtractionRequestInput::from_readiness_consumer_and_loop_ledger(
            &foreign_consumer,
            foreign_loop_handoff.loop_ledger_receipt(),
        ),
    )
    .expect("foreign request");
    foreign_request.readiness_loop_ledger_binding().clone()
}
