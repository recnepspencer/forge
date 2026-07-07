#[path = "../public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

use topology::facade::PlanarBooleanLoopBlueprintRegistry;
use worth_kernel::replay_undo_consumer_cutover::{
    ReplayUndoForbiddenConsumerSurfaceKind, ReplayUndoMilestoneThirteenSeedPosture,
};
use worth_kernel::replay_undo_inventory::ReplayUndoInventoryDisposition;
use worth_kernel::replay_undo_transaction_boundary::{
    assemble_replay_undo_transaction_boundary_packet, ReplayUndoTransactionBoundaryAssemblyRequest,
    ReplayUndoTransactionBoundarySupportPosture, ReplayUndoTransactionBoundarySupportSource,
};
use worth_kernel::workload_composition::CompletedBooleanLoopReconstructionHandoff;
use worth_kernel::workload_composition::WorkloadCompositionError;

use super::metaboss_support::MetabossEventExtractionSubject;
use super::alternate_ordinary_topology_undo_support::alternate_ordinary_traversal_views_undo_scope_support;
use super::ordinary_topology_undo_support::ordinary_traversal_views_undo_scope_support;
use super::real_handoff_support::{
    foreign_packet_backed_boundary_error, packet_backed_loop_handoff_for_branch,
    packet_backed_replay_undo_chain_for_branch, real_loop_handoff_for_branch,
    with_packet_backed_loop_boundary_basis, ReplayBranch,
};

pub(crate) fn assert_packet_backed_loop_closeout_matches_legacy_vertical_slice() {
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let subject = MetabossEventExtractionSubject::certify("phase12.10 packet-backed loop closeout");
    let packet_backed = packet_backed_loop_handoff_for_branch(
        &subject,
        ReplayBranch::Original,
        &matrix,
        &validators,
    )
    .expect("packet-backed loop closeout");
    let legacy =
        packetless_legacy_loop_handoff_witness(&subject, &matrix, &validators, &packet_backed);

    assert_eq!(
        packet_backed.loop_ledger_receipt().receipt_identity(),
        legacy.loop_ledger_receipt().receipt_identity()
    );
    assert_eq!(
        packet_backed.evidence_receipt().receipt_identity(),
        legacy.evidence_receipt().receipt_identity()
    );
    assert_same_legacy_runtime_registration_basis(&packet_backed, &legacy);
    assert_ne!(
        packet_backed.workload_stage_index_identity(),
        legacy.workload_stage_index_identity(),
        "migrated closeout must carry a stronger upstream workload stage identity than the packetless witness"
    );
    assert!(
        legacy.replay_undo_transaction_boundary_packet().is_none(),
        "legacy witness must stay packetless so parity is not an alias of the migrated path"
    );
    let packet = packet_backed
        .require_replay_undo_transaction_boundary_packet()
        .expect("migrated loop closeout must retain the replay/undo transaction boundary packet");
    with_packet_backed_loop_boundary_basis(&subject, |topology_undo, replay_scope, undo_scope| {
        assert_eq!(
            packet.touched_digest(),
            topology_undo.touched_closure().closure_digest()
        );
        assert_eq!(
            packet.invalidation_receipt_identity().digest(),
            topology_undo.prior_proof_identity().digest()
        );
        assert_eq!(
            packet.counters().topology_touched_subject_count(),
            topology_undo.counters().touched_subject_count()
        );
        assert_eq!(
            packet.replay_scope_identity().digest(),
            replay_scope.scope_identity().digest()
        );
        assert_eq!(
            packet.undo_scope_identity().digest(),
            undo_scope.scope_identity().digest()
        );
        assert_eq!(
            packet.support_posture(),
            &ReplayUndoTransactionBoundarySupportPosture::Ordinary
        );
        assert_eq!(
            packet.counters().replay_raw_row_scan_count(),
            replay_scope.counters().raw_row_scan_count()
        );
        assert_eq!(
            packet.counters().replay_broad_receipt_scan_count(),
            replay_scope.counters().broad_receipt_scan_count()
        );
        assert_eq!(
            packet.counters().replay_caller_owned_scan_count(),
            replay_scope.counters().caller_owned_scan_count()
        );
        assert_eq!(
            packet.counters().undo_raw_row_scan_count(),
            undo_scope.counters().raw_row_scan_count()
        );
        assert_eq!(
            packet.counters().undo_broad_receipt_scan_count(),
            undo_scope.counters().broad_receipt_scan_count()
        );
        assert_eq!(
            packet.counters().undo_caller_owned_scan_count(),
            undo_scope.counters().caller_owned_scan_count()
        );
        assert_eq!(packet.counters().replay_raw_row_scan_count(), 0);
        assert_eq!(packet.counters().replay_broad_receipt_scan_count(), 0);
        assert_eq!(packet.counters().replay_caller_owned_scan_count(), 0);
        assert_eq!(packet.counters().undo_raw_row_scan_count(), 0);
        assert_eq!(packet.counters().undo_broad_receipt_scan_count(), 0);
        assert_eq!(packet.counters().undo_caller_owned_scan_count(), 0);
    });
    assert_eq!(
        packet.packet_identity(),
        packet_backed
            .replay_undo_transaction_boundary_packet()
            .expect("packet-backed handoff should expose the same retained packet")
            .packet_identity()
    );
}

pub(crate) fn assert_topology_undo_product_changes_packet_identity() {
    let subject =
        MetabossEventExtractionSubject::certify("phase12.10 topology undo packet binding drift");
    let ordinary_topology_support = ordinary_traversal_views_undo_scope_support();
    let alternate_topology_support = alternate_ordinary_traversal_views_undo_scope_support();
    let ordinary_topology = ordinary_topology_support
        .lower_undo_scope_product()
        .expect("ordinary topology undo scope");
    let alternate_topology = alternate_topology_support
        .lower_undo_scope_product()
        .expect("alternate topology undo scope");

    with_packet_backed_loop_boundary_basis(&subject, |_, replay_scope, undo_scope| {
        let ordinary_packet = assemble_replay_undo_transaction_boundary_packet(
            ReplayUndoTransactionBoundaryAssemblyRequest::new(
                &ordinary_topology,
                replay_scope,
                undo_scope,
                ReplayUndoTransactionBoundarySupportSource::Ordinary,
            ),
        )
        .expect("ordinary topology undo product should assemble packet");
        let alternate_packet = assemble_replay_undo_transaction_boundary_packet(
            ReplayUndoTransactionBoundaryAssemblyRequest::new(
                &alternate_topology,
                replay_scope,
                undo_scope,
                ReplayUndoTransactionBoundarySupportSource::Ordinary,
            ),
        )
        .expect("alternate topology undo product should assemble packet");

        assert_ne!(
            ordinary_packet.touched_digest(),
            alternate_packet.touched_digest(),
            "packet must bind topology undo touched digest"
        );
        assert_ne!(
            ordinary_packet.invalidation_receipt_identity().digest(),
            alternate_packet.invalidation_receipt_identity().digest(),
            "packet must bind topology invalidation prior proof"
        );
        assert_ne!(
            ordinary_packet.packet_identity(),
            alternate_packet.packet_identity(),
            "packet identity must drift when topology undo authority changes"
        );
        assert_eq!(
            ordinary_packet.replay_scope_identity().digest(),
            alternate_packet.replay_scope_identity().digest()
        );
        assert_eq!(
            ordinary_packet.undo_scope_identity().digest(),
            alternate_packet.undo_scope_identity().digest()
        );
    });
}

pub(crate) fn assert_replay_undo_consumer_cutover_closes_from_ordinary_chain() {
    let (matrix, validators) = PlanarBooleanLoopBlueprintRegistry::phase_2()
        .into_classification_matrix_and_validator_plan();
    let subject = MetabossEventExtractionSubject::certify_event_carrier(
        "phase12.11 replay undo consumer cutover ordinary chain",
    );
    let chain = packet_backed_replay_undo_chain_for_branch(
        &subject,
        ReplayBranch::Original,
        &matrix,
        &validators,
    )
    .expect("ordinary replay/undo chain");
    let packet = chain
        .loop_handoff()
        .require_replay_undo_transaction_boundary_packet()
        .expect("ordinary chain must retain the replay/undo transaction packet");
    let closeout = chain.consumer_cutover_closeout();

    assert_eq!(
        closeout.transaction_packet_identity(),
        packet.packet_identity()
    );
    assert_eq!(
        closeout.replay_scope_identity(),
        packet.replay_scope_identity().digest()
    );
    assert_eq!(
        closeout.undo_scope_identity(),
        packet.undo_scope_identity().digest()
    );
    assert_eq!(
        closeout.boolean_chain_handoff_identity(),
        chain.chain_handoff().handoff_identity()
    );
    assert_eq!(closeout.counters().replay_raw_row_scan_count(), 0);
    assert_eq!(closeout.counters().replay_broad_receipt_scan_count(), 0);
    assert_eq!(closeout.counters().replay_caller_owned_scan_count(), 0);
    assert_eq!(closeout.counters().undo_raw_row_scan_count(), 0);
    assert_eq!(closeout.counters().undo_broad_receipt_scan_count(), 0);
    assert_eq!(closeout.counters().undo_caller_owned_scan_count(), 0);
    assert!(closeout.counters().migrated_sources() > 0);
    assert_eq!(
        closeout.residue_ledger().row_count(),
        closeout.counters().capped_residue_sources() + closeout.counters().query_gap_sources()
    );
    assert!(
        closeout.residue_ledger().rows().iter().all(|row| {
            !row.removal_trigger().is_empty()
                && matches!(
                    row.disposition(),
                    ReplayUndoInventoryDisposition::Cap | ReplayUndoInventoryDisposition::QueryGap
                )
        }),
        "every non-ordinary remainder must stay owned, counted, and trigger-bound"
    );
    assert_eq!(closeout.forbidden_surface_denials().row_count(), 5);
    for required_kind in [
        ReplayUndoForbiddenConsumerSurfaceKind::OldReplayHelper,
        ReplayUndoForbiddenConsumerSurfaceKind::BroadTopologyRediscovery,
        ReplayUndoForbiddenConsumerSurfaceKind::BroadEvidenceRediscovery,
        ReplayUndoForbiddenConsumerSurfaceKind::RawReceiptAdmission,
        ReplayUndoForbiddenConsumerSurfaceKind::LocalRollbackShortcut,
    ] {
        assert!(
            closeout
                .forbidden_surface_denials()
                .rows()
                .iter()
                .any(|row| row.kind() == required_kind && !row.removal_trigger().is_empty()),
            "Phase 11 closeout must deny {required_kind:?}"
        );
    }
    for source_firewall_kind in [
        ReplayUndoForbiddenConsumerSurfaceKind::OldReplayHelper,
        ReplayUndoForbiddenConsumerSurfaceKind::LocalRollbackShortcut,
    ] {
        let row = closeout
            .forbidden_surface_denials()
            .source_firewall()
            .row_for_kind(source_firewall_kind)
            .expect("source-firewall denied surface row");
        assert_eq!(
            row.ordinary_occurrence_count(),
            0,
            "source-firewall row must prove no ordinary production revival for {source_firewall_kind:?}"
        );
        assert!(
            !row.scanned_source().is_empty() && !row.forbidden_pattern().is_empty(),
            "source-firewall row must name the scanned source and denied pattern"
        );
    }
    let seed = closeout.milestone_thirteen_seed();
    assert_eq!(seed.transaction_packet_identity(), packet.packet_identity());
    assert_eq!(
        seed.replay_scope_identity(),
        packet.replay_scope_identity().digest()
    );
    assert_eq!(
        seed.undo_scope_identity(),
        packet.undo_scope_identity().digest()
    );
    assert_eq!(
        seed.residue_row_count(),
        closeout.residue_ledger().row_count()
    );
    assert_eq!(
        seed.migrated_source_count(),
        closeout.counters().migrated_sources()
    );
    assert!(seed.source_firewall_clean());
    assert_eq!(
        seed.posture(),
        ReplayUndoMilestoneThirteenSeedPosture::ReplayUndoOnlyNoConflictOrCacheClaim
    );
    assert!(!seed.seed_identity().is_empty());
    super::hard_deletion_closeout_assertions::assert_hard_deletion_closeout_binds_ordinary_chain(
        &chain, packet,
    );
}

pub(crate) fn assert_packet_backed_loop_closeout_rejects_foreign_scope_products() {
    let subject = MetabossEventExtractionSubject::certify("phase12.10 packet-backed scope denial");
    let foreign_subject =
        MetabossEventExtractionSubject::certify("phase12.10 foreign packet-backed scope denial");
    let error = foreign_packet_backed_boundary_error(&subject, &foreign_subject);

    assert!(
        matches!(
            error,
            WorkloadCompositionError::ReplayUndoBoundary(_)
                | WorkloadCompositionError::ReplayUndoTransactionBoundary(_)
        ),
        "foreign replay/undo scope products must fail through the packet-backed boundary: {error:?}"
    );
}

pub(crate) fn assert_legacy_loop_closeout_cannot_claim_packet_backed_boundary() {
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let subject = MetabossEventExtractionSubject::certify(
        "phase12.10 legacy loop closeout packet-backed denial",
    );
    let migrated =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Original, &matrix, &validators)
            .expect("ordinary loop closeout");

    let packet = migrated
        .require_replay_undo_transaction_boundary_packet()
        .expect("ordinary loop closeout helper must now satisfy consumers through packet proof");
    assert!(!packet.packet_identity().is_empty());

    let packetless_loop_handoff =
        packetless_legacy_loop_handoff_witness(&subject, &matrix, &validators, &migrated);

    assert!(
        packetless_loop_handoff
            .replay_undo_transaction_boundary_packet()
            .is_none(),
        "legacy loop helper must remain visibly packetless for this denial proof"
    );
}

fn packetless_legacy_loop_handoff_witness(
    subject: &MetabossEventExtractionSubject,
    matrix: &topology::facade::PlanarBooleanLoopOperatorClassificationMatrix,
    validators: &topology::facade::PlanarBooleanLoopValidatorRegistrationPlan,
    migrated: &CompletedBooleanLoopReconstructionHandoff,
) -> CompletedBooleanLoopReconstructionHandoff {
    reduced_pair_support::rebuild_left_workload(subject.pair(), vec![])
        .with_completed_boolean_loop_reconstruction(
            migrated.loop_ledger_receipt(),
            migrated.evidence_receipt(),
            matrix,
            validators,
            migrated.lookup_consumed_workload_handoff(),
        )
        .expect("legacy loop workload helper still constructs a packetless loop handoff")
}

fn assert_same_legacy_runtime_registration_basis(
    packet_backed: &CompletedBooleanLoopReconstructionHandoff,
    legacy: &CompletedBooleanLoopReconstructionHandoff,
) {
    let packet_backed_proof = packet_backed.runtime_registration_proof();
    let legacy_proof = legacy.runtime_registration_proof();
    assert_eq!(
        packet_backed_proof.loop_receipt_identity(),
        legacy_proof.loop_receipt_identity()
    );
    assert_eq!(
        packet_backed_proof.loop_ledger_identity(),
        legacy_proof.loop_ledger_identity()
    );
    assert_eq!(
        packet_backed_proof.downstream_consumption_identity(),
        legacy_proof.downstream_consumption_identity()
    );
    assert_eq!(
        packet_backed_proof.registry_identity(),
        legacy_proof.registry_identity()
    );
    assert_eq!(
        packet_backed_proof.operator_names(),
        legacy_proof.operator_names()
    );
    assert_eq!(
        packet_backed_proof.validator_names(),
        legacy_proof.validator_names()
    );
}
