use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    PlanarBooleanOverlapRegionCheckpointParityReceipt, PlanarBooleanOverlapRegionEvidenceReceipt,
    PlanarBooleanOverlapRegionLedgerReceipt, PlanarBooleanOverlapRegionReplayParityReceipt,
};

use crate::workload_composition::WorthWorkload;

use super::{
    PlanarBooleanOverlapRegionAntiTheatreFenceProof,
    PlanarBooleanOverlapRegionPublicContractFenceProof, PlanarBooleanOverlapRuntimeRegistrationProof,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletedPlanarBooleanOverlapRegionExtractionHandoff {
    completed_workload: WorthWorkload,
    overlap_ledger_bundle: PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    overlap_ledger_receipt: PlanarBooleanOverlapRegionLedgerReceipt,
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
        overlap_ledger_bundle: PlanarBooleanOverlapRegionLedgerAssemblyBundle,
        overlap_ledger_receipt: PlanarBooleanOverlapRegionLedgerReceipt,
        evidence_receipt: PlanarBooleanOverlapRegionEvidenceReceipt,
        replay_parity_receipt: PlanarBooleanOverlapRegionReplayParityReceipt,
        checkpoint_parity_receipt: PlanarBooleanOverlapRegionCheckpointParityReceipt,
        runtime_registration_proof: PlanarBooleanOverlapRuntimeRegistrationProof,
        public_contract_fence_proof: PlanarBooleanOverlapRegionPublicContractFenceProof,
        anti_theatre_fence_proof: PlanarBooleanOverlapRegionAntiTheatreFenceProof,
    ) -> Self {
        Self {
            completed_workload,
            overlap_ledger_bundle,
            overlap_ledger_receipt,
            evidence_receipt,
            replay_parity_receipt,
            checkpoint_parity_receipt,
            runtime_registration_proof,
            public_contract_fence_proof,
            anti_theatre_fence_proof,
        }
    }

    pub fn completed_workload(&self) -> &WorthWorkload { &self.completed_workload }
    pub fn overlap_ledger_bundle(&self) -> &PlanarBooleanOverlapRegionLedgerAssemblyBundle {
        &self.overlap_ledger_bundle
    }
    pub fn overlap_ledger_receipt(&self) -> &PlanarBooleanOverlapRegionLedgerReceipt { &self.overlap_ledger_receipt }
    pub fn evidence_receipt(&self) -> &PlanarBooleanOverlapRegionEvidenceReceipt { &self.evidence_receipt }
    pub fn replay_parity_receipt(&self) -> &PlanarBooleanOverlapRegionReplayParityReceipt { &self.replay_parity_receipt }
    pub fn checkpoint_parity_receipt(&self) -> &PlanarBooleanOverlapRegionCheckpointParityReceipt { &self.checkpoint_parity_receipt }
    pub fn runtime_registration_proof(&self) -> &PlanarBooleanOverlapRuntimeRegistrationProof { &self.runtime_registration_proof }
    pub fn public_contract_fence_proof(&self) -> &PlanarBooleanOverlapRegionPublicContractFenceProof { &self.public_contract_fence_proof }
    pub fn anti_theatre_fence_proof(&self) -> &PlanarBooleanOverlapRegionAntiTheatreFenceProof { &self.anti_theatre_fence_proof }
    pub fn workload_stage_index_identity(&self) -> &str {
        self.completed_workload
            .evidence_ledger()
            .stage_index()
            .index_identity()
    }
}
