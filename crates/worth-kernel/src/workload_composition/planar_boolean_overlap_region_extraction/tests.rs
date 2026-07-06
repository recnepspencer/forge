#[path = "../operator_harness/tests_vertical_migration/support/spatial_batch_execution_slice.rs"]
mod spatial_batch_execution_slice_support;
#[path = "../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod workload_evidence_support;

use topology::facade::{
    admit_milestone_seven_five_overlap_readiness_consumer, PlanarBooleanLoopBlueprintRegistry,
    PlanarBooleanOverlapBlueprintRegistry,
};
use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionParticipationSupport;
use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanCoplanarOverlapArrangementGraph,
    PlanarBooleanOverlapAdjacencyIndexInput, PlanarBooleanOverlapArrangementGraphInput,
    PlanarBooleanOverlapCellContainmentInput, PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingField, PlanarBooleanOverlapCellWindingFieldInput,
    PlanarBooleanOverlapIslandCandidateInput, PlanarBooleanOverlapIslandComponentBundle,
    PlanarBooleanOverlapParticipationRecovery, PlanarBooleanOverlapParticipationRecoveryInput,
    PlanarBooleanOverlapRegionAdjacencyIndex, PlanarBooleanOverlapRegionExtractionRequest,
    PlanarBooleanOverlapRegionExtractionRequestInput,
    PlanarBooleanPostAdmissionNormalizationBundle, PlanarBooleanPreRegionNormalizationBundle,
    PlanarBooleanSharedAreaAdmissionBundle,
};

use crate::workload_composition::{
    current_touched_graph_readiness_handoff, BooleanSplitReplayUndoBoundaryRequest,
    CompletedBooleanLoopReconstructionHandoff, PlanarBooleanLoopReconstructionCloseoutInput,
    PlanarBooleanOverlapRegionCloseoutInput, PlanarBooleanOverlapRegionPublicContractProofRowKind,
};

fn completed_loop_handoff_with_real_batch_authority(
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

fn overlap_request_and_ledger(
    loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
) -> OverlapExtractionAuthorityProducts {
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
    let participation_input =
        PlanarBooleanOverlapParticipationRecoveryInput::from_request_and_loop_support(
            &request, &support,
        );
    let participation = PlanarBooleanOverlapParticipationRecovery::recover(participation_input)
        .expect(
            "real loop handoff should recover overlap participation from carried 7.4 provenance",
        );
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

    OverlapExtractionAuthorityProducts {
        request,
        shared_area_bundle: shared_area,
        canonical_winding_bundle: canonical_bundle.clone(),
        ledger_bundle: identity_lineage
            .mint_overlap_region_ledger()
            .expect("ledger"),
    }
}

#[test]
fn overlap_closeout_real_owner_seam_exposes_stage_runtime_and_fence_proof() {
    let label = "phase7.5 overlap extraction real owner seam";
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
    let overlap_products = overlap_request_and_ledger(&loop_handoff);
    let overlap_registry = PlanarBooleanOverlapBlueprintRegistry::phase_2();
    let completed = loop_handoff
        .complete_planar_boolean_overlap_region_extraction(
            PlanarBooleanOverlapRegionCloseoutInput::new(
                &readiness,
                &readiness_consumer,
                &overlap_products.request,
                &overlap_products.shared_area_bundle,
                &overlap_products.canonical_winding_bundle,
                &overlap_products.ledger_bundle,
                &replayed_loop_handoff,
                &replay_subject.replay_receipts,
                &overlap_registry.operator_classification_matrix(),
                &overlap_registry.validator_registration_plan(),
            ),
        )
        .expect("overlap closeout should certify through the real owner seam");

    completed
        .completed_workload()
        .require_boolean_overlap_region_extraction(completed.evidence_receipt())
        .expect("completed workload must require the overlap extraction stage");
    assert_eq!(
        completed
            .runtime_registration_proof()
            .evidence_receipt_identity(),
        completed.evidence_receipt().receipt_identity()
    );
    assert_eq!(
        completed
            .runtime_registration_proof()
            .overlap_ledger_receipt_identity(),
        completed.overlap_ledger_receipt().receipt_identity()
    );
    assert_eq!(
        completed.runtime_registration_proof().request_identity(),
        completed.evidence_receipt().request_identity()
    );
    assert_eq!(
        completed.replay_parity_receipt().checkpoint_receipt(),
        completed.checkpoint_parity_receipt()
    );
    assert_eq!(completed.replay_parity_receipt().rows().len(), 11);
    assert_eq!(
        completed
            .replay_parity_receipt()
            .checkpoint_receipt()
            .replay_evidence_identity(),
        replay_subject.replay_receipts.replay_evidence_identity()
    );
    assert_eq!(
        completed
            .runtime_registration_proof()
            .stage_index_identity(),
        completed.workload_stage_index_identity()
    );
    for kind in [
        PlanarBooleanOverlapRegionPublicContractProofRowKind::ReadinessHandoff,
        PlanarBooleanOverlapRegionPublicContractProofRowKind::ReadinessConsumer,
        PlanarBooleanOverlapRegionPublicContractProofRowKind::ReadinessBinding,
        PlanarBooleanOverlapRegionPublicContractProofRowKind::OverlapLedgerReceipt,
        PlanarBooleanOverlapRegionPublicContractProofRowKind::OverlapEvidenceReceipt,
        PlanarBooleanOverlapRegionPublicContractProofRowKind::RuntimeRegistrationProof,
        PlanarBooleanOverlapRegionPublicContractProofRowKind::WorkloadStageIndex,
        PlanarBooleanOverlapRegionPublicContractProofRowKind::RequestIdentity,
        PlanarBooleanOverlapRegionPublicContractProofRowKind::AntiTheatreFence,
    ] {
        assert!(
            completed
                .public_contract_fence_proof()
                .rows()
                .iter()
                .chain(completed.anti_theatre_fence_proof().rows().iter())
                .any(|row| row.kind() == kind),
            "completed overlap handoff must record {kind:?}",
        );
    }
    assert_eq!(
        completed.anti_theatre_fence_proof().guard_names(),
        &[
            "synthetic_readiness_rejected".to_string(),
            "raw_loop_ledger_rejected".to_string(),
            "copied_overlap_rows_rejected".to_string(),
            "bypassed_arrangement_or_cell_proof_rejected".to_string(),
        ]
    );
}

#[test]
fn overlap_closeout_rejects_foreign_retained_replay_authority_for_replay_peer() {
    let label = "phase7.5 overlap extraction replay peer rejection";
    let loop_handoff = completed_loop_handoff_with_real_batch_authority(
        label,
        workload_evidence_support::ReplayBranch::Original,
    );
    let replayed_loop_handoff = completed_loop_handoff_with_real_batch_authority(
        label,
        workload_evidence_support::ReplayBranch::Replayed,
    );
    let readiness = current_touched_graph_readiness_handoff().expect("readiness handoff");
    let readiness_consumer =
        admit_milestone_seven_five_overlap_readiness_consumer(&readiness).expect("consumer");
    let overlap_products = overlap_request_and_ledger(&loop_handoff);
    let overlap_registry = PlanarBooleanOverlapBlueprintRegistry::phase_2();
    let foreign_replay_subject = workload_evidence_support::build_edge_split_replay_parity_subject(
        &workload_evidence_support::MetabossEventExtractionSubject::certify(
            "phase7.5 overlap extraction replay peer rejection foreign",
        ),
    );

    let denial = loop_handoff
        .complete_planar_boolean_overlap_region_extraction(
            PlanarBooleanOverlapRegionCloseoutInput::new(
                &readiness,
                &readiness_consumer,
                &overlap_products.request,
                &overlap_products.shared_area_bundle,
                &overlap_products.canonical_winding_bundle,
                &overlap_products.ledger_bundle,
                &replayed_loop_handoff,
                &foreign_replay_subject.replay_receipts,
                &overlap_registry.operator_classification_matrix(),
                &overlap_registry.validator_registration_plan(),
            ),
        )
        .expect_err("foreign retained replay receipts must not certify the owner-seam replay peer");

    assert!(matches!(
        denial,
        crate::workload_composition::WorkloadCompositionError::OverlapRegionCloseout(detail)
            if detail.contains("LoopReplayParityRejected")
    ));
}
struct OverlapExtractionAuthorityProducts {
    request: PlanarBooleanOverlapRegionExtractionRequest,
    shared_area_bundle: PlanarBooleanSharedAreaAdmissionBundle,
    canonical_winding_bundle: PlanarBooleanPostAdmissionNormalizationBundle,
    ledger_bundle:
        worth_spatial::facade::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionLedgerAssemblyBundle,
}
