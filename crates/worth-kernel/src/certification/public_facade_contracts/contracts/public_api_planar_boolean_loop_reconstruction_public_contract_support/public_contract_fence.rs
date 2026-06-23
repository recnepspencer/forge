use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopReconstructionEvidenceReceipt, PlanarBooleanLoopReconstructionLedgerReceipt,
};

use worth_kernel::workload_composition::{
    CompletedBooleanLoopReconstructionHandoff, PlanarBooleanLoopRuntimeRegistrationProof,
};

use super::proof_rows::{
    PlanarBooleanLoopPublicContractProofRow, PlanarBooleanLoopPublicContractProofRowKind,
};

#[derive(Clone, Copy)]
pub(crate) struct PlanarBooleanLoopPublicContractFenceProofInput<'a> {
    loop_ledger_receipt: &'a PlanarBooleanLoopReconstructionLedgerReceipt,
    evidence_receipt: &'a PlanarBooleanLoopReconstructionEvidenceReceipt,
    runtime_registration_proof: &'a PlanarBooleanLoopRuntimeRegistrationProof,
    workload_stage_index_identity: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PlanarBooleanLoopPublicContractFenceDenial {
    LoopEvidenceMismatch,
    RuntimeRegistrationMismatch,
    DownstreamConsumptionMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlanarBooleanLoopPublicContractFenceProof {
    proof_identity: String,
    loop_receipt_identity: String,
    evidence_receipt_identity: String,
    runtime_registration_proof_identity: String,
    downstream_consumption_identity: String,
    workload_stage_index_identity: String,
    rows: Vec<PlanarBooleanLoopPublicContractProofRow>,
}

impl<'a> PlanarBooleanLoopPublicContractFenceProofInput<'a> {
    pub(crate) fn from_parts(
        loop_ledger_receipt: &'a PlanarBooleanLoopReconstructionLedgerReceipt,
        evidence_receipt: &'a PlanarBooleanLoopReconstructionEvidenceReceipt,
        runtime_registration_proof: &'a PlanarBooleanLoopRuntimeRegistrationProof,
        workload_stage_index_identity: &'a str,
    ) -> PlanarBooleanLoopPublicContractFenceProofInput<'a> {
        Self {
            loop_ledger_receipt,
            evidence_receipt,
            runtime_registration_proof,
            workload_stage_index_identity,
        }
    }

    pub(crate) fn from_handoff(
        handoff: &'a CompletedBooleanLoopReconstructionHandoff,
    ) -> PlanarBooleanLoopPublicContractFenceProofInput<'a> {
        Self::from_parts(
            handoff.loop_ledger_receipt(),
            handoff.evidence_receipt(),
            handoff.runtime_registration_proof(),
            handoff.workload_stage_index_identity(),
        )
    }
}

impl PlanarBooleanLoopPublicContractFenceProof {
    pub(crate) fn certify(
        input: PlanarBooleanLoopPublicContractFenceProofInput<'_>,
    ) -> Result<Self, PlanarBooleanLoopPublicContractFenceDenial> {
        if input.evidence_receipt.ledger_receipt_identity()
            != input.loop_ledger_receipt.receipt_identity()
        {
            return Err(PlanarBooleanLoopPublicContractFenceDenial::LoopEvidenceMismatch);
        }
        if input.runtime_registration_proof.loop_receipt_identity()
            != input.loop_ledger_receipt.receipt_identity()
        {
            return Err(PlanarBooleanLoopPublicContractFenceDenial::RuntimeRegistrationMismatch);
        }
        if input
            .runtime_registration_proof
            .downstream_consumption_identity()
            != input.loop_ledger_receipt.downstream_consumption_identity()
        {
            return Err(PlanarBooleanLoopPublicContractFenceDenial::DownstreamConsumptionMismatch);
        }
        if input.evidence_receipt.downstream_consumption_identity()
            != input.loop_ledger_receipt.downstream_consumption_identity()
        {
            return Err(PlanarBooleanLoopPublicContractFenceDenial::DownstreamConsumptionMismatch);
        }

        let loop_receipt_identity = input.loop_ledger_receipt.receipt_identity().to_string();
        let evidence_receipt_identity = input.evidence_receipt.receipt_identity().to_string();
        let downstream_consumption_identity = input
            .loop_ledger_receipt
            .downstream_consumption_identity()
            .to_string();
        let runtime_registration_proof_identity = input
            .runtime_registration_proof
            .proof_identity()
            .to_string();
        let workload_stage_index_identity = input.workload_stage_index_identity.to_string();
        let proof_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-loop-public-contract-fence".to_string(),
                format!("loop-ledger:{loop_receipt_identity}"),
                format!("loop-evidence:{evidence_receipt_identity}"),
                format!("runtime-registration:{runtime_registration_proof_identity}"),
                format!("stage-index:{workload_stage_index_identity}"),
                format!("downstream:{downstream_consumption_identity}"),
            ],
        );
        let rows = vec![
            PlanarBooleanLoopPublicContractProofRow::new(
                PlanarBooleanLoopPublicContractProofRowKind::LoopLedgerReceipt,
                loop_receipt_identity.clone(),
            ),
            PlanarBooleanLoopPublicContractProofRow::new(
                PlanarBooleanLoopPublicContractProofRowKind::LoopEvidenceReceipt,
                evidence_receipt_identity.clone(),
            ),
            PlanarBooleanLoopPublicContractProofRow::new(
                PlanarBooleanLoopPublicContractProofRowKind::RuntimeRegistrationProof,
                runtime_registration_proof_identity.clone(),
            ),
            PlanarBooleanLoopPublicContractProofRow::new(
                PlanarBooleanLoopPublicContractProofRowKind::WorkloadStageIndex,
                workload_stage_index_identity.clone(),
            ),
            PlanarBooleanLoopPublicContractProofRow::new(
                PlanarBooleanLoopPublicContractProofRowKind::DownstreamLoopConsumption,
                downstream_consumption_identity.clone(),
            ),
        ];

        Ok(Self {
            proof_identity,
            loop_receipt_identity,
            evidence_receipt_identity,
            runtime_registration_proof_identity,
            downstream_consumption_identity,
            workload_stage_index_identity,
            rows,
        })
    }

    pub(crate) fn proof_identity(&self) -> &str {
        &self.proof_identity
    }

    pub(crate) fn loop_receipt_identity(&self) -> &str {
        &self.loop_receipt_identity
    }

    pub(crate) fn evidence_receipt_identity(&self) -> &str {
        &self.evidence_receipt_identity
    }

    pub(crate) fn runtime_registration_proof_identity(&self) -> &str {
        &self.runtime_registration_proof_identity
    }

    pub(crate) fn downstream_consumption_identity(&self) -> &str {
        &self.downstream_consumption_identity
    }

    pub(crate) fn workload_stage_index_identity(&self) -> &str {
        &self.workload_stage_index_identity
    }

    pub(crate) fn rows(&self) -> &[PlanarBooleanLoopPublicContractProofRow] {
        &self.rows
    }
}
