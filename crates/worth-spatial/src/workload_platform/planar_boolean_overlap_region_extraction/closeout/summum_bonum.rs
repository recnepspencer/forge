use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityReadinessInput;
use topology::facade::TopologyMilestoneSevenFiveOverlapReadinessConsumer;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::denial::PlanarBooleanOverlapRegionSummumBonumCloseoutDenial;
use super::subcases::PlanarBooleanOverlapRegionSummumBonumSubcaseRow;
pub use super::summum_bonum_witnesses::{
    PlanarBooleanOverlapRegionBoundaryOnlyOutcomeWitness,
    PlanarBooleanOverlapRegionCanonicalWindingOutcomeWitness,
    PlanarBooleanOverlapRegionCheckpointOutcomeWitness,
    PlanarBooleanOverlapRegionMixedBoundaryAreaWitness,
    PlanarBooleanOverlapRegionNestedIdentityOutcomeWitness,
    PlanarBooleanOverlapRegionOrderingParityWitness, PlanarBooleanOverlapRegionReplayParityWitness,
    PlanarBooleanOverlapRegionSharedAreaOutcomeWitness, PlanarBooleanOverlapRegionStormWitness,
    PlanarBooleanOverlapRegionSummumBonumCloseoutCounters,
};
use super::verification::{
    certify_outcome_witnesses, verify_canonical_identities, verify_readiness_binding,
    verify_readiness_consumer, verify_receipt_chain, verify_replay_and_checkpoint,
    CertifiedPlanarBooleanOverlapRegionSummumBonumWitnessSet,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapReadinessLoopLedgerBinding,
    PlanarBooleanOverlapRegionCheckpointParityReceipt, PlanarBooleanOverlapRegionEvidenceReceipt,
    PlanarBooleanOverlapRegionLedgerAssemblyBundle, PlanarBooleanOverlapRegionReplayParityReceipt,
    PlanarBooleanPostAdmissionNormalizationBundle, PlanarBooleanSharedAreaAdmissionBundle,
};

#[derive(Clone, Copy, Debug)]
pub struct PlanarBooleanOverlapRegionSummumBonumCloseoutInput<'a> {
    readiness: &'a TouchedGraphParityReadinessInput,
    readiness_consumer: &'a TopologyMilestoneSevenFiveOverlapReadinessConsumer,
    readiness_binding: &'a PlanarBooleanOverlapReadinessLoopLedgerBinding,
    shared_area_bundle: &'a PlanarBooleanSharedAreaAdmissionBundle,
    canonical_winding_bundle: &'a PlanarBooleanPostAdmissionNormalizationBundle,
    overlap_ledger_bundle: &'a PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    replayed_shared_area_bundle: &'a PlanarBooleanSharedAreaAdmissionBundle,
    replayed_canonical_winding_bundle: &'a PlanarBooleanPostAdmissionNormalizationBundle,
    replayed_overlap_ledger_bundle: &'a PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    evidence_receipt: &'a PlanarBooleanOverlapRegionEvidenceReceipt,
    replay_parity_receipt: &'a PlanarBooleanOverlapRegionReplayParityReceipt,
    checkpoint_parity_receipt: &'a PlanarBooleanOverlapRegionCheckpointParityReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionSummumBonumCloseout {
    closeout_identity: String,
    request_identity: String,
    readiness_handoff_identity: String,
    readiness_consumer_identity: String,
    readiness_binding_identity: String,
    loop_ledger_receipt_identity: String,
    overlap_ledger_receipt_identity: String,
    overlap_ledger_identity: String,
    overlap_decision_log_identity: String,
    overlap_ordering_basis_identity: String,
    overlap_region_identity_map_identity: String,
    persistent_name_propagation_map_identity: String,
    subshape_signature_map_identity: String,
    replay_identity: String,
    checkpoint_identity: String,
    replay_evidence_identity: String,
    boundary_only_outcome: PlanarBooleanOverlapRegionBoundaryOnlyOutcomeWitness,
    shared_area_outcome: PlanarBooleanOverlapRegionSharedAreaOutcomeWitness,
    canonical_winding_outcome: PlanarBooleanOverlapRegionCanonicalWindingOutcomeWitness,
    nested_identity_outcome: PlanarBooleanOverlapRegionNestedIdentityOutcomeWitness,
    mixed_boundary_area_outcome: PlanarBooleanOverlapRegionMixedBoundaryAreaWitness,
    ordering_parity: PlanarBooleanOverlapRegionOrderingParityWitness,
    replay_parity: PlanarBooleanOverlapRegionReplayParityWitness,
    checkpoint_parity: PlanarBooleanOverlapRegionCheckpointOutcomeWitness,
    storm_witness: PlanarBooleanOverlapRegionStormWitness,
    subcases: Vec<PlanarBooleanOverlapRegionSummumBonumSubcaseRow>,
    counters: PlanarBooleanOverlapRegionSummumBonumCloseoutCounters,
}

impl<'a> PlanarBooleanOverlapRegionSummumBonumCloseoutInput<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        readiness: &'a TouchedGraphParityReadinessInput,
        readiness_consumer: &'a TopologyMilestoneSevenFiveOverlapReadinessConsumer,
        readiness_binding: &'a PlanarBooleanOverlapReadinessLoopLedgerBinding,
        shared_area_bundle: &'a PlanarBooleanSharedAreaAdmissionBundle,
        canonical_winding_bundle: &'a PlanarBooleanPostAdmissionNormalizationBundle,
        overlap_ledger_bundle: &'a PlanarBooleanOverlapRegionLedgerAssemblyBundle,
        replayed_shared_area_bundle: &'a PlanarBooleanSharedAreaAdmissionBundle,
        replayed_canonical_winding_bundle: &'a PlanarBooleanPostAdmissionNormalizationBundle,
        replayed_overlap_ledger_bundle: &'a PlanarBooleanOverlapRegionLedgerAssemblyBundle,
        evidence_receipt: &'a PlanarBooleanOverlapRegionEvidenceReceipt,
        replay_parity_receipt: &'a PlanarBooleanOverlapRegionReplayParityReceipt,
        checkpoint_parity_receipt: &'a PlanarBooleanOverlapRegionCheckpointParityReceipt,
    ) -> Self {
        Self {
            readiness,
            readiness_consumer,
            readiness_binding,
            shared_area_bundle,
            canonical_winding_bundle,
            overlap_ledger_bundle,
            replayed_shared_area_bundle,
            replayed_canonical_winding_bundle,
            replayed_overlap_ledger_bundle,
            evidence_receipt,
            replay_parity_receipt,
            checkpoint_parity_receipt,
        }
    }
}

impl PlanarBooleanOverlapRegionSummumBonumCloseout {
    pub fn certify(
        input: PlanarBooleanOverlapRegionSummumBonumCloseoutInput<'_>,
    ) -> Result<Self, PlanarBooleanOverlapRegionSummumBonumCloseoutDenial> {
        verify_readiness_consumer(input.readiness, input.readiness_consumer)?;
        verify_readiness_binding(
            input.readiness,
            input.readiness_consumer,
            input.readiness_binding,
        )?;
        verify_receipt_chain(
            input.readiness_binding,
            input.overlap_ledger_bundle,
            input.evidence_receipt,
        )?;
        let replay_rows_verified = verify_replay_and_checkpoint(
            input.replay_parity_receipt,
            input.checkpoint_parity_receipt,
            input.evidence_receipt,
        )?;
        verify_canonical_identities(input.overlap_ledger_bundle, input.evidence_receipt)?;

        let witness_set = certify_outcome_witnesses(
            input.shared_area_bundle,
            input.canonical_winding_bundle,
            input.overlap_ledger_bundle,
            input.replayed_shared_area_bundle,
            input.replayed_canonical_winding_bundle,
            input.replayed_overlap_ledger_bundle,
            input.replay_parity_receipt,
            input.checkpoint_parity_receipt,
            replay_rows_verified,
        )?;
        Ok(Self::from_verified_inputs(input, witness_set))
    }

    fn from_verified_inputs(
        input: PlanarBooleanOverlapRegionSummumBonumCloseoutInput<'_>,
        witness_set: CertifiedPlanarBooleanOverlapRegionSummumBonumWitnessSet,
    ) -> Self {
        let receipt = input.overlap_ledger_bundle.receipt();
        let ledger = input.overlap_ledger_bundle.ledger();
        Self {
            closeout_identity: truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    "planar-boolean-overlap-region-summum-bonum-closeout".to_string(),
                    format!("request:{}", input.evidence_receipt.request_identity()),
                    format!("overlap-ledger-receipt:{}", receipt.receipt_identity()),
                    format!(
                        "boundary-only:{}",
                        witness_set.boundary_only_outcome.digest()
                    ),
                    format!("shared-area:{}", witness_set.shared_area_outcome.digest()),
                    format!("winding:{}", witness_set.canonical_winding_outcome.digest()),
                    format!("replay:{}", input.replay_parity_receipt.replay_identity()),
                    format!(
                        "checkpoint:{}",
                        input.checkpoint_parity_receipt.checkpoint_identity()
                    ),
                ],
            ),
            request_identity: input.evidence_receipt.request_identity().to_string(),
            readiness_handoff_identity: input
                .evidence_receipt
                .readiness_handoff_identity()
                .to_string(),
            readiness_consumer_identity: input
                .evidence_receipt
                .readiness_consumer_identity()
                .to_string(),
            readiness_binding_identity: input
                .evidence_receipt
                .readiness_binding_identity()
                .to_string(),
            loop_ledger_receipt_identity: input
                .evidence_receipt
                .loop_ledger_receipt_identity()
                .to_string(),
            overlap_ledger_receipt_identity: receipt.receipt_identity().to_string(),
            overlap_ledger_identity: ledger.ledger_identity().to_string(),
            overlap_decision_log_identity: input
                .overlap_ledger_bundle
                .decision_log()
                .decision_log_identity()
                .to_string(),
            overlap_ordering_basis_identity: ledger.ordering_basis_identity().to_string(),
            overlap_region_identity_map_identity: receipt
                .overlap_region_identity_map_identity()
                .to_string(),
            persistent_name_propagation_map_identity: receipt
                .persistent_name_propagation_map_identity()
                .to_string(),
            subshape_signature_map_identity: receipt.subshape_signature_map_identity().to_string(),
            replay_identity: input.replay_parity_receipt.replay_identity().to_string(),
            checkpoint_identity: input
                .checkpoint_parity_receipt
                .checkpoint_identity()
                .to_string(),
            replay_evidence_identity: input
                .checkpoint_parity_receipt
                .replay_evidence_identity()
                .to_string(),
            boundary_only_outcome: witness_set.boundary_only_outcome,
            shared_area_outcome: witness_set.shared_area_outcome,
            canonical_winding_outcome: witness_set.canonical_winding_outcome,
            nested_identity_outcome: witness_set.nested_identity_outcome,
            mixed_boundary_area_outcome: witness_set.mixed_boundary_area_outcome,
            ordering_parity: witness_set.ordering_parity,
            replay_parity: witness_set.replay_parity,
            checkpoint_parity: witness_set.checkpoint_parity,
            storm_witness: witness_set.storm_witness,
            subcases: witness_set.subcases,
            counters: witness_set.counters,
        }
    }

    pub fn closeout_identity(&self) -> &str {
        &self.closeout_identity
    }
    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }
    pub fn replay_identity(&self) -> &str {
        &self.replay_identity
    }
    pub fn checkpoint_identity(&self) -> &str {
        &self.checkpoint_identity
    }
    pub fn replay_evidence_identity(&self) -> &str {
        &self.replay_evidence_identity
    }
    pub fn counters(&self) -> PlanarBooleanOverlapRegionSummumBonumCloseoutCounters {
        self.counters
    }
    pub fn subcases(&self) -> &[PlanarBooleanOverlapRegionSummumBonumSubcaseRow] {
        &self.subcases
    }
    pub fn boundary_only_outcome(&self) -> &PlanarBooleanOverlapRegionBoundaryOnlyOutcomeWitness {
        &self.boundary_only_outcome
    }
    pub fn shared_area_outcome(&self) -> &PlanarBooleanOverlapRegionSharedAreaOutcomeWitness {
        &self.shared_area_outcome
    }
    pub fn canonical_winding_outcome(
        &self,
    ) -> &PlanarBooleanOverlapRegionCanonicalWindingOutcomeWitness {
        &self.canonical_winding_outcome
    }
    pub fn nested_identity_outcome(
        &self,
    ) -> &PlanarBooleanOverlapRegionNestedIdentityOutcomeWitness {
        &self.nested_identity_outcome
    }
    pub fn mixed_boundary_area_outcome(
        &self,
    ) -> &PlanarBooleanOverlapRegionMixedBoundaryAreaWitness {
        &self.mixed_boundary_area_outcome
    }
    pub fn ordering_parity(&self) -> &PlanarBooleanOverlapRegionOrderingParityWitness {
        &self.ordering_parity
    }
    pub fn replay_parity(&self) -> &PlanarBooleanOverlapRegionReplayParityWitness {
        &self.replay_parity
    }
    pub fn checkpoint_parity(&self) -> &PlanarBooleanOverlapRegionCheckpointOutcomeWitness {
        &self.checkpoint_parity
    }
    pub fn storm_witness(&self) -> &PlanarBooleanOverlapRegionStormWitness {
        &self.storm_witness
    }

    pub fn subcase(
        &self,
        kind: super::subcases::PlanarBooleanOverlapRegionSummumBonumSubcaseKind,
    ) -> Option<&PlanarBooleanOverlapRegionSummumBonumSubcaseRow> {
        self.subcases.iter().find(|row| row.kind() == kind)
    }

    pub fn is_canonical(&self) -> bool {
        !self.closeout_identity.is_empty()
            && self.replay_parity.original_outcome_digest
                == self.replay_parity.replayed_outcome_digest
            && self.ordering_parity.canonical_digest == self.ordering_parity.order_invariant_digest
            && self.storm_witness.pairwise_rediscovery_attempts == 0
    }
}
