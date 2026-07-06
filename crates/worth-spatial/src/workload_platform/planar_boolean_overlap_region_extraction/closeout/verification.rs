use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityReadinessInput;
use topology::facade::TopologyMilestoneSevenFiveOverlapReadinessConsumer;

use super::denial::{
    PlanarBooleanOverlapRegionSummumBonumCloseoutDenial,
    PlanarBooleanOverlapRegionSummumBonumCloseoutDenialKind as Kind,
};
use super::subcases::{
    PlanarBooleanOverlapRegionSummumBonumSubcaseKind as SubcaseKind,
    PlanarBooleanOverlapRegionSummumBonumSubcaseRow,
};
use super::summum_bonum::{
    PlanarBooleanOverlapRegionBoundaryOnlyOutcomeWitness,
    PlanarBooleanOverlapRegionCanonicalWindingOutcomeWitness,
    PlanarBooleanOverlapRegionCheckpointOutcomeWitness,
    PlanarBooleanOverlapRegionMixedBoundaryAreaWitness,
    PlanarBooleanOverlapRegionNestedIdentityOutcomeWitness,
    PlanarBooleanOverlapRegionOrderingParityWitness, PlanarBooleanOverlapRegionReplayParityWitness,
    PlanarBooleanOverlapRegionSharedAreaOutcomeWitness, PlanarBooleanOverlapRegionStormWitness,
    PlanarBooleanOverlapRegionSummumBonumCloseoutCounters,
};
use super::witness_material::{build_counters, witness_material};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapReadinessLoopLedgerBinding,
    PlanarBooleanOverlapRegionCheckpointParityReceipt, PlanarBooleanOverlapRegionEvidenceReceipt,
    PlanarBooleanOverlapRegionLedgerAssemblyBundle, PlanarBooleanOverlapRegionReplayParityReceipt,
    PlanarBooleanOverlapRegionReplayParityRowKind, PlanarBooleanPostAdmissionNormalizationBundle,
    PlanarBooleanSharedAreaAdmissionBundle,
};

const PHASE_FIFTEEN_FENCE_SUBCASE: &str = "phase_fifteen_fence";

pub(super) struct CertifiedPlanarBooleanOverlapRegionSummumBonumWitnessSet {
    pub(super) boundary_only_outcome: PlanarBooleanOverlapRegionBoundaryOnlyOutcomeWitness,
    pub(super) shared_area_outcome: PlanarBooleanOverlapRegionSharedAreaOutcomeWitness,
    pub(super) canonical_winding_outcome: PlanarBooleanOverlapRegionCanonicalWindingOutcomeWitness,
    pub(super) nested_identity_outcome: PlanarBooleanOverlapRegionNestedIdentityOutcomeWitness,
    pub(super) mixed_boundary_area_outcome: PlanarBooleanOverlapRegionMixedBoundaryAreaWitness,
    pub(super) ordering_parity: PlanarBooleanOverlapRegionOrderingParityWitness,
    pub(super) replay_parity: PlanarBooleanOverlapRegionReplayParityWitness,
    pub(super) checkpoint_parity: PlanarBooleanOverlapRegionCheckpointOutcomeWitness,
    pub(super) storm_witness: PlanarBooleanOverlapRegionStormWitness,
    pub(super) subcases: Vec<PlanarBooleanOverlapRegionSummumBonumSubcaseRow>,
    pub(super) counters: PlanarBooleanOverlapRegionSummumBonumCloseoutCounters,
}

pub(super) fn verify_readiness_consumer(
    readiness: &TouchedGraphParityReadinessInput,
    consumer: &TopologyMilestoneSevenFiveOverlapReadinessConsumer,
) -> Result<(), PlanarBooleanOverlapRegionSummumBonumCloseoutDenial> {
    let matches = readiness.selected_route_identity_digest()
        == consumer.selected_route_identity_digest()
        && readiness.selected_family_identity() == consumer.selected_family_identity()
        && readiness.selected_product_identity_digest()
            == consumer.selected_product_identity_digest()
        && readiness.selected_witness_identity_digest()
            == consumer.selected_witness_identity_digest()
        && readiness.touched_closure_digest() == consumer.touched_closure_digest()
        && readiness.selected_plan_digest() == consumer.selected_plan_digest()
        && readiness.topology_query_posture_digest() == consumer.topology_query_posture_digest()
        && readiness.spatial_query_posture_digest() == consumer.spatial_query_posture_digest()
        && readiness.residue_digest() == consumer.residue_digest()
        && readiness.source_firewall_digest() == consumer.source_firewall_digest()
        && readiness.architecture_claim_digest() == consumer.architecture_claim_digest();
    if matches {
        Ok(())
    } else {
        Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::ReadinessConsumerMismatch,
            PHASE_FIFTEEN_FENCE_SUBCASE,
            "phase-16 closeout requires one matching readiness handoff and readiness consumer",
        ))
    }
}

pub(super) fn verify_readiness_binding(
    readiness: &TouchedGraphParityReadinessInput,
    consumer: &TopologyMilestoneSevenFiveOverlapReadinessConsumer,
    binding: &PlanarBooleanOverlapReadinessLoopLedgerBinding,
) -> Result<(), PlanarBooleanOverlapRegionSummumBonumCloseoutDenial> {
    let matches = binding.selected_route_identity_digest()
        == readiness.selected_route_identity_digest()
        && binding.selected_route_identity_digest() == consumer.selected_route_identity_digest()
        && binding.selected_family_identity() == readiness.selected_family_identity()
        && binding.selected_family_identity() == consumer.selected_family_identity()
        && binding.selected_product_identity_digest()
            == readiness.selected_product_identity_digest()
        && binding.selected_product_identity_digest()
            == consumer.selected_product_identity_digest()
        && binding.selected_witness_identity_digest()
            == readiness.selected_witness_identity_digest()
        && binding.selected_witness_identity_digest()
            == consumer.selected_witness_identity_digest()
        && binding.selected_plan_digest() == readiness.selected_plan_digest()
        && binding.selected_plan_digest() == consumer.selected_plan_digest()
        && binding.touched_closure_digest() == readiness.touched_closure_digest()
        && binding.touched_closure_digest() == consumer.touched_closure_digest()
        && binding.topology_query_posture_digest() == readiness.topology_query_posture_digest()
        && binding.topology_query_posture_digest() == consumer.topology_query_posture_digest()
        && binding.spatial_query_posture_digest() == readiness.spatial_query_posture_digest()
        && binding.spatial_query_posture_digest() == consumer.spatial_query_posture_digest()
        && binding.residue_digest() == readiness.residue_digest()
        && binding.residue_digest() == consumer.residue_digest()
        && binding.source_firewall_digest() == readiness.source_firewall_digest()
        && binding.source_firewall_digest() == consumer.source_firewall_digest()
        && binding.architecture_claim_digest() == readiness.architecture_claim_digest()
        && binding.architecture_claim_digest() == consumer.architecture_claim_digest();
    if matches {
        Ok(())
    } else {
        Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::ReadinessBindingMismatch,
            "synthetic_readiness_or_mismatched_loop_ledger_is_rejected",
            "phase-16 closeout requires the carried readiness-to-loop binding to match readiness authority",
        ))
    }
}

pub(super) fn verify_receipt_chain(
    binding: &PlanarBooleanOverlapReadinessLoopLedgerBinding,
    bundle: &PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    evidence_receipt: &PlanarBooleanOverlapRegionEvidenceReceipt,
) -> Result<(), PlanarBooleanOverlapRegionSummumBonumCloseoutDenial> {
    let receipt = bundle.receipt();
    if binding.loop_ledger_receipt_identity() != evidence_receipt.loop_ledger_receipt_identity() {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::LoopLedgerMismatch,
            "synthetic_readiness_or_mismatched_loop_ledger_is_rejected",
            "phase-16 closeout requires one readiness-bound loop-ledger receipt identity",
        ));
    }
    if receipt.receipt_identity() != evidence_receipt.overlap_ledger_receipt_identity() {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::OverlapLedgerMismatch,
            "synthetic_overlap_ledger_is_rejected",
            "phase-16 closeout requires one overlap-ledger receipt carried by evidence",
        ));
    }
    if receipt.request_identity() != evidence_receipt.request_identity() {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::RequestIdentityMismatch,
            "synthetic_overlap_ledger_is_rejected",
            "phase-16 closeout requires one request identity across overlap ledger and evidence",
        ));
    }
    Ok(())
}

pub(super) fn verify_replay_and_checkpoint(
    replay_parity_receipt: &PlanarBooleanOverlapRegionReplayParityReceipt,
    checkpoint_parity_receipt: &PlanarBooleanOverlapRegionCheckpointParityReceipt,
    evidence_receipt: &PlanarBooleanOverlapRegionEvidenceReceipt,
) -> Result<usize, PlanarBooleanOverlapRegionSummumBonumCloseoutDenial> {
    if replay_parity_receipt.checkpoint_receipt() != checkpoint_parity_receipt {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::ReplayParityMismatch,
            SubcaseKind::CheckpointReplayPreservesRegionIdentityAndNames.spec_name(),
            "phase-16 closeout requires replay parity to carry the same checkpoint receipt",
        ));
    }
    if checkpoint_parity_receipt.replay_evidence_identity()
        != evidence_receipt.replay_evidence_identity()
    {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::CheckpointParityMismatch,
            SubcaseKind::CheckpointReplayPreservesRegionIdentityAndNames.spec_name(),
            "phase-16 closeout requires checkpoint parity to stay bound to retained replay evidence",
        ));
    }
    for kind in [
        PlanarBooleanOverlapRegionReplayParityRowKind::OverlapEvidenceReceipt,
        PlanarBooleanOverlapRegionReplayParityRowKind::RequestIdentity,
        PlanarBooleanOverlapRegionReplayParityRowKind::ReadinessHandoff,
        PlanarBooleanOverlapRegionReplayParityRowKind::ReadinessConsumer,
        PlanarBooleanOverlapRegionReplayParityRowKind::ReadinessBinding,
        PlanarBooleanOverlapRegionReplayParityRowKind::OverlapDecisionLog,
        PlanarBooleanOverlapRegionReplayParityRowKind::OverlapLedgerReceipt,
        PlanarBooleanOverlapRegionReplayParityRowKind::OverlapIdentityMap,
        PlanarBooleanOverlapRegionReplayParityRowKind::PersistentNamePropagationMap,
        PlanarBooleanOverlapRegionReplayParityRowKind::SubshapeSignatureMap,
        PlanarBooleanOverlapRegionReplayParityRowKind::RetainedReplayCheckpoint,
    ] {
        if replay_parity_receipt
            .rows()
            .iter()
            .all(|row| row.kind() != kind)
        {
            return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
                Kind::MissingReplayParityRow,
                SubcaseKind::CheckpointReplayPreservesRegionIdentityAndNames.spec_name(),
                format!("phase-16 closeout is missing replay row {kind:?}"),
            ));
        }
    }
    Ok(replay_parity_receipt.rows().len())
}

pub(super) fn verify_canonical_identities(
    bundle: &PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    evidence_receipt: &PlanarBooleanOverlapRegionEvidenceReceipt,
) -> Result<(), PlanarBooleanOverlapRegionSummumBonumCloseoutDenial> {
    let receipt = bundle.receipt();
    let identities = [
        receipt.overlap_region_identity_map_identity(),
        receipt.persistent_name_propagation_map_identity(),
        receipt.subshape_signature_map_identity(),
        evidence_receipt.overlap_region_identity_map_identity(),
        evidence_receipt.persistent_name_propagation_map_identity(),
        evidence_receipt.subshape_signature_map_identity(),
    ];
    if identities.iter().all(|identity| !identity.is_empty()) {
        Ok(())
    } else {
        Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::MissingCanonicalIdentity,
            PHASE_FIFTEEN_FENCE_SUBCASE,
            "phase-16 closeout requires canonical overlap identity, persistent-name, and subshape-signature authority",
        ))
    }
}

pub(super) fn certify_outcome_witnesses(
    original_shared_area_bundle: &PlanarBooleanSharedAreaAdmissionBundle,
    original_canonical_winding_bundle: &PlanarBooleanPostAdmissionNormalizationBundle,
    original_bundle: &PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    replayed_shared_area_bundle: &PlanarBooleanSharedAreaAdmissionBundle,
    replayed_canonical_winding_bundle: &PlanarBooleanPostAdmissionNormalizationBundle,
    _replayed_bundle: &PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    replay_parity_receipt: &PlanarBooleanOverlapRegionReplayParityReceipt,
    checkpoint_parity_receipt: &PlanarBooleanOverlapRegionCheckpointParityReceipt,
    replay_rows_verified: usize,
) -> Result<
    CertifiedPlanarBooleanOverlapRegionSummumBonumWitnessSet,
    PlanarBooleanOverlapRegionSummumBonumCloseoutDenial,
> {
    let original = witness_material(
        original_shared_area_bundle,
        original_canonical_winding_bundle,
    )?;
    let replayed = witness_material(
        replayed_shared_area_bundle,
        replayed_canonical_winding_bundle,
    )?;
    let counters = build_counters(original_bundle, replay_rows_verified, &original);

    if original.boundary_only_outcome.digest() != replayed.boundary_only_outcome.digest()
        || original.shared_area_outcome.digest() != replayed.shared_area_outcome.digest()
        || original.canonical_winding_outcome.digest()
            != replayed.canonical_winding_outcome.digest()
        || original.nested_identity_outcome.digest() != replayed.nested_identity_outcome.digest()
        || original.mixed_boundary_area_outcome.digest()
            != replayed.mixed_boundary_area_outcome.digest()
    {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::ReplayParityMismatch,
            SubcaseKind::CheckpointReplayPreservesRegionIdentityAndNames.spec_name(),
            "phase-16 closeout requires replay to preserve boundary-only, shared-area, winding, nested-identity, and mixed-contact outcomes",
        ));
    }
    if original.ordering_parity.canonical_digest()
        != original.ordering_parity.order_invariant_digest()
        || replayed.ordering_parity.canonical_digest()
            != replayed.ordering_parity.order_invariant_digest()
    {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::OrderingParityInstability,
            SubcaseKind::BenignLoopOrderVariationPreservesLedgerDigest.spec_name(),
            "phase-16 closeout requires order-invariant hostile outcome digests",
        ));
    }
    if counters.pairwise_rediscovery_attempts() != 0
        || counters.decision_rows_verified() < counters.ledger_rows_verified()
    {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::OverlapStormShapeViolation,
            SubcaseKind::OverlapStormUsesIndexNotPairwiseRediscovery.spec_name(),
            "phase-16 closeout requires workload-backed storm counters that stay on the indexed overlap-ledger path",
        ));
    }

    let replay_parity = PlanarBooleanOverlapRegionReplayParityWitness {
        original_outcome_digest: original.ordering_parity.canonical_digest().to_string(),
        replayed_outcome_digest: replayed.ordering_parity.canonical_digest().to_string(),
        replay_row_count: replay_parity_receipt.rows().len(),
    };
    let checkpoint_parity = PlanarBooleanOverlapRegionCheckpointOutcomeWitness {
        checkpoint_identity: checkpoint_parity_receipt.checkpoint_identity().to_string(),
        replay_evidence_identity: checkpoint_parity_receipt
            .replay_evidence_identity()
            .to_string(),
        certified_outcome_digest: original.ordering_parity.canonical_digest().to_string(),
    };
    let storm_witness = PlanarBooleanOverlapRegionStormWitness {
        identity_rows_examined: original_bundle.counters().identity_rows_examined(),
        decision_rows_verified: counters.decision_rows_verified(),
        ledger_rows_verified: counters.ledger_rows_verified(),
        pairwise_rediscovery_attempts: counters.pairwise_rediscovery_attempts(),
    };
    let subcases = vec![
        PlanarBooleanOverlapRegionSummumBonumSubcaseRow::new(
            SubcaseKind::BoundaryOnlyCoincidentEdgesDoNotAdmitArea,
            format!("digest={}", original.boundary_only_outcome.digest()),
        ),
        PlanarBooleanOverlapRegionSummumBonumSubcaseRow::new(
            SubcaseKind::OppositeSenseSameAreaOverlapHasStableWinding,
            format!("digest={}", original.canonical_winding_outcome.digest()),
        ),
        PlanarBooleanOverlapRegionSummumBonumSubcaseRow::new(
            SubcaseKind::NestedOverlapIslandsPreserveRegionIdentity,
            format!("digest={}", original.nested_identity_outcome.digest()),
        ),
        PlanarBooleanOverlapRegionSummumBonumSubcaseRow::new(
            SubcaseKind::MixedBoundaryAndAreaContactDoesNotCollapse,
            format!("digest={}", original.mixed_boundary_area_outcome.digest()),
        ),
        PlanarBooleanOverlapRegionSummumBonumSubcaseRow::new(
            SubcaseKind::BenignLoopOrderVariationPreservesLedgerDigest,
            format!("digest={}", original.ordering_parity.canonical_digest()),
        ),
        PlanarBooleanOverlapRegionSummumBonumSubcaseRow::new(
            SubcaseKind::CheckpointReplayPreservesRegionIdentityAndNames,
            format!("digest={}", replay_parity.original_outcome_digest()),
        ),
        PlanarBooleanOverlapRegionSummumBonumSubcaseRow::new(
            SubcaseKind::OverlapStormUsesIndexNotPairwiseRediscovery,
            format!(
                "decision_rows={}; ledger_rows={}",
                storm_witness.decision_rows_verified(),
                storm_witness.ledger_rows_verified()
            ),
        ),
    ];
    Ok(CertifiedPlanarBooleanOverlapRegionSummumBonumWitnessSet {
        boundary_only_outcome: original.boundary_only_outcome,
        shared_area_outcome: original.shared_area_outcome,
        canonical_winding_outcome: original.canonical_winding_outcome,
        nested_identity_outcome: original.nested_identity_outcome,
        mixed_boundary_area_outcome: original.mixed_boundary_area_outcome,
        ordering_parity: original.ordering_parity,
        replay_parity,
        checkpoint_parity,
        storm_witness,
        subcases,
        counters,
    })
}
