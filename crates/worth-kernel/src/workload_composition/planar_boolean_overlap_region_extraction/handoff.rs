use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionCheckpointParityReceipt, PlanarBooleanOverlapRegionEvidenceReceipt,
    PlanarBooleanOverlapRegionLedgerAssemblyBundle, PlanarBooleanOverlapRegionLedgerReceipt,
    PlanarBooleanOverlapRegionReplayParityReceipt, PlanarBooleanPostAdmissionNormalizationBundle,
    PlanarBooleanSharedAreaAdmissionBundle,
};

use crate::workload_composition::WorthWorkload;

use super::{
    PlanarBooleanOverlapRegionAntiTheatreFenceProof,
    PlanarBooleanOverlapRegionPublicContractFenceProof, PlanarBooleanOverlapReplayCertifiedPeer,
    PlanarBooleanOverlapRuntimeRegistrationProof,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletedPlanarBooleanOverlapRegionExtractionHandoff {
    completed_workload: WorthWorkload,
    shared_area_bundle: PlanarBooleanSharedAreaAdmissionBundle,
    canonical_winding_bundle: PlanarBooleanPostAdmissionNormalizationBundle,
    overlap_ledger_bundle: PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    overlap_ledger_receipt: PlanarBooleanOverlapRegionLedgerReceipt,
    replay_certified_peer: PlanarBooleanOverlapReplayCertifiedPeer,
    evidence_receipt: PlanarBooleanOverlapRegionEvidenceReceipt,
    replay_parity_receipt: PlanarBooleanOverlapRegionReplayParityReceipt,
    checkpoint_parity_receipt: PlanarBooleanOverlapRegionCheckpointParityReceipt,
    runtime_registration_proof: PlanarBooleanOverlapRuntimeRegistrationProof,
    public_contract_fence_proof: PlanarBooleanOverlapRegionPublicContractFenceProof,
    anti_theatre_fence_proof: PlanarBooleanOverlapRegionAntiTheatreFenceProof,
}

impl CompletedPlanarBooleanOverlapRegionExtractionHandoff {
    pub(crate) fn new(
        completed_workload: WorthWorkload,
        shared_area_bundle: PlanarBooleanSharedAreaAdmissionBundle,
        canonical_winding_bundle: PlanarBooleanPostAdmissionNormalizationBundle,
        overlap_ledger_bundle: PlanarBooleanOverlapRegionLedgerAssemblyBundle,
        overlap_ledger_receipt: PlanarBooleanOverlapRegionLedgerReceipt,
        replay_certified_peer: PlanarBooleanOverlapReplayCertifiedPeer,
        evidence_receipt: PlanarBooleanOverlapRegionEvidenceReceipt,
        replay_parity_receipt: PlanarBooleanOverlapRegionReplayParityReceipt,
        checkpoint_parity_receipt: PlanarBooleanOverlapRegionCheckpointParityReceipt,
        runtime_registration_proof: PlanarBooleanOverlapRuntimeRegistrationProof,
        public_contract_fence_proof: PlanarBooleanOverlapRegionPublicContractFenceProof,
        anti_theatre_fence_proof: PlanarBooleanOverlapRegionAntiTheatreFenceProof,
    ) -> Self {
        Self {
            completed_workload,
            shared_area_bundle,
            canonical_winding_bundle,
            overlap_ledger_bundle,
            overlap_ledger_receipt,
            replay_certified_peer,
            evidence_receipt,
            replay_parity_receipt,
            checkpoint_parity_receipt,
            runtime_registration_proof,
            public_contract_fence_proof,
            anti_theatre_fence_proof,
        }
    }

    pub fn completed_workload(&self) -> &WorthWorkload {
        &self.completed_workload
    }
    pub(crate) fn shared_area_bundle(&self) -> &PlanarBooleanSharedAreaAdmissionBundle {
        &self.shared_area_bundle
    }
    pub(crate) fn canonical_winding_bundle(
        &self,
    ) -> &PlanarBooleanPostAdmissionNormalizationBundle {
        &self.canonical_winding_bundle
    }
    pub fn overlap_ledger_bundle(&self) -> &PlanarBooleanOverlapRegionLedgerAssemblyBundle {
        &self.overlap_ledger_bundle
    }
    pub fn overlap_ledger_receipt(&self) -> &PlanarBooleanOverlapRegionLedgerReceipt {
        &self.overlap_ledger_receipt
    }
    pub fn evidence_receipt(&self) -> &PlanarBooleanOverlapRegionEvidenceReceipt {
        &self.evidence_receipt
    }
    pub(crate) fn replay_certified_peer(&self) -> &PlanarBooleanOverlapReplayCertifiedPeer {
        &self.replay_certified_peer
    }
    pub fn replay_parity_receipt(&self) -> &PlanarBooleanOverlapRegionReplayParityReceipt {
        &self.replay_parity_receipt
    }
    pub fn checkpoint_parity_receipt(&self) -> &PlanarBooleanOverlapRegionCheckpointParityReceipt {
        &self.checkpoint_parity_receipt
    }
    pub fn runtime_registration_proof(&self) -> &PlanarBooleanOverlapRuntimeRegistrationProof {
        &self.runtime_registration_proof
    }
    pub fn public_contract_fence_proof(
        &self,
    ) -> &PlanarBooleanOverlapRegionPublicContractFenceProof {
        &self.public_contract_fence_proof
    }
    pub fn anti_theatre_fence_proof(&self) -> &PlanarBooleanOverlapRegionAntiTheatreFenceProof {
        &self.anti_theatre_fence_proof
    }
    pub fn workload_stage_index_identity(&self) -> &str {
        self.completed_workload
            .evidence_ledger()
            .stage_index()
            .index_identity()
    }
}
