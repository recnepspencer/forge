use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionEvidenceReceipt, PlanarBooleanOverlapRegionLedgerReceipt,
};

use crate::workload_composition::WorthWorkload;

use super::PlanarBooleanOverlapRuntimeRegistrationProof;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionPublicContractProofRow {
    kind: PlanarBooleanOverlapRegionPublicContractProofRowKind,
    identity: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegionPublicContractProofRowKind {
    ReadinessHandoff,
    ReadinessConsumer,
    ReadinessBinding,
    OverlapLedgerReceipt,
    OverlapEvidenceReceipt,
    RuntimeRegistrationProof,
    WorkloadStageIndex,
    RequestIdentity,
    AntiTheatreGuard,
    AntiTheatreFence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegionPublicContractFenceDenial {
    ReadinessAuthorityMismatch,
    OverlapEvidenceMismatch,
    RequestIdentityMismatch,
    RuntimeRegistrationMismatch,
    WorkloadStageMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionPublicContractFenceProof {
    proof_identity: String,
    rows: Vec<PlanarBooleanOverlapRegionPublicContractProofRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegionAntiTheatreFenceDenial {
    PublicContractFenceMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionAntiTheatreFenceProof {
    proof_identity: String,
    guard_names: Vec<String>,
    rows: Vec<PlanarBooleanOverlapRegionPublicContractProofRow>,
}

impl PlanarBooleanOverlapRegionPublicContractProofRow {
    fn new(kind: PlanarBooleanOverlapRegionPublicContractProofRowKind, identity: String) -> Self {
        Self { kind, identity }
    }

    pub fn kind(&self) -> PlanarBooleanOverlapRegionPublicContractProofRowKind { self.kind }
    pub fn identity(&self) -> &str { &self.identity }
}

impl PlanarBooleanOverlapRegionPublicContractFenceProof {
    pub(crate) fn certify(
        ledger_receipt: &PlanarBooleanOverlapRegionLedgerReceipt,
        evidence_receipt: &PlanarBooleanOverlapRegionEvidenceReceipt,
        runtime_registration_proof: &PlanarBooleanOverlapRuntimeRegistrationProof,
        completed_workload: &WorthWorkload,
    ) -> Result<Self, PlanarBooleanOverlapRegionPublicContractFenceDenial> {
        if evidence_receipt.readiness_handoff_identity().is_empty()
            || evidence_receipt.readiness_consumer_identity().is_empty()
            || evidence_receipt.readiness_binding_identity().is_empty()
        {
            return Err(PlanarBooleanOverlapRegionPublicContractFenceDenial::ReadinessAuthorityMismatch);
        }
        if evidence_receipt.overlap_ledger_receipt_identity() != ledger_receipt.receipt_identity() {
            return Err(PlanarBooleanOverlapRegionPublicContractFenceDenial::OverlapEvidenceMismatch);
        }
        if evidence_receipt.request_identity() != ledger_receipt.request_identity() {
            return Err(PlanarBooleanOverlapRegionPublicContractFenceDenial::RequestIdentityMismatch);
        }
        if runtime_registration_proof.evidence_receipt_identity() != evidence_receipt.receipt_identity()
            || runtime_registration_proof.overlap_ledger_receipt_identity() != ledger_receipt.receipt_identity()
            || runtime_registration_proof.request_identity() != evidence_receipt.request_identity()
        {
            return Err(PlanarBooleanOverlapRegionPublicContractFenceDenial::RuntimeRegistrationMismatch);
        }
        let workload_stage_index_identity = completed_workload.evidence_ledger().stage_index().index_identity();
        if runtime_registration_proof.stage_index_identity() != workload_stage_index_identity {
            return Err(PlanarBooleanOverlapRegionPublicContractFenceDenial::WorkloadStageMismatch);
        }
        let proof_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-overlap-region-public-contract-fence".to_string(),
                format!("overlap-ledger:{}", ledger_receipt.receipt_identity()),
                format!("overlap-evidence:{}", evidence_receipt.receipt_identity()),
                format!(
                    "runtime-registration:{}",
                    runtime_registration_proof.proof_identity()
                ),
                format!("stage-index:{workload_stage_index_identity}"),
            ],
        );
        Ok(Self {
            proof_identity,
            rows: vec![
                PlanarBooleanOverlapRegionPublicContractProofRow::new(
                    PlanarBooleanOverlapRegionPublicContractProofRowKind::ReadinessHandoff,
                    evidence_receipt.readiness_handoff_identity().to_string(),
                ),
                PlanarBooleanOverlapRegionPublicContractProofRow::new(
                    PlanarBooleanOverlapRegionPublicContractProofRowKind::ReadinessConsumer,
                    evidence_receipt.readiness_consumer_identity().to_string(),
                ),
                PlanarBooleanOverlapRegionPublicContractProofRow::new(
                    PlanarBooleanOverlapRegionPublicContractProofRowKind::ReadinessBinding,
                    evidence_receipt.readiness_binding_identity().to_string(),
                ),
                PlanarBooleanOverlapRegionPublicContractProofRow::new(
                    PlanarBooleanOverlapRegionPublicContractProofRowKind::OverlapLedgerReceipt,
                    ledger_receipt.receipt_identity().to_string(),
                ),
                PlanarBooleanOverlapRegionPublicContractProofRow::new(
                    PlanarBooleanOverlapRegionPublicContractProofRowKind::OverlapEvidenceReceipt,
                    evidence_receipt.receipt_identity().to_string(),
                ),
                PlanarBooleanOverlapRegionPublicContractProofRow::new(
                    PlanarBooleanOverlapRegionPublicContractProofRowKind::RuntimeRegistrationProof,
                    runtime_registration_proof.proof_identity().to_string(),
                ),
                PlanarBooleanOverlapRegionPublicContractProofRow::new(
                    PlanarBooleanOverlapRegionPublicContractProofRowKind::WorkloadStageIndex,
                    workload_stage_index_identity.to_string(),
                ),
                PlanarBooleanOverlapRegionPublicContractProofRow::new(
                    PlanarBooleanOverlapRegionPublicContractProofRowKind::RequestIdentity,
                    evidence_receipt.request_identity().to_string(),
                ),
            ],
        })
    }

    pub fn proof_identity(&self) -> &str { &self.proof_identity }
    pub fn rows(&self) -> &[PlanarBooleanOverlapRegionPublicContractProofRow] { &self.rows }
}

impl PlanarBooleanOverlapRegionAntiTheatreFenceProof {
    pub(crate) fn certify(
        evidence_receipt: &PlanarBooleanOverlapRegionEvidenceReceipt,
        public_contract_fence: &PlanarBooleanOverlapRegionPublicContractFenceProof,
    ) -> Result<Self, PlanarBooleanOverlapRegionAntiTheatreFenceDenial> {
        let required_row_kinds = [
            PlanarBooleanOverlapRegionPublicContractProofRowKind::ReadinessHandoff,
            PlanarBooleanOverlapRegionPublicContractProofRowKind::ReadinessConsumer,
            PlanarBooleanOverlapRegionPublicContractProofRowKind::ReadinessBinding,
            PlanarBooleanOverlapRegionPublicContractProofRowKind::OverlapLedgerReceipt,
            PlanarBooleanOverlapRegionPublicContractProofRowKind::OverlapEvidenceReceipt,
            PlanarBooleanOverlapRegionPublicContractProofRowKind::RuntimeRegistrationProof,
            PlanarBooleanOverlapRegionPublicContractProofRowKind::WorkloadStageIndex,
            PlanarBooleanOverlapRegionPublicContractProofRowKind::RequestIdentity,
        ];
        if required_row_kinds.iter().any(|required_kind| {
            public_contract_fence.rows().iter().all(|row| row.kind() != *required_kind)
        }) {
            return Err(PlanarBooleanOverlapRegionAntiTheatreFenceDenial::PublicContractFenceMismatch);
        }
        let guard_names = vec![
            "synthetic_readiness_rejected".to_string(),
            "raw_loop_ledger_rejected".to_string(),
            "copied_overlap_rows_rejected".to_string(),
            "bypassed_arrangement_or_cell_proof_rejected".to_string(),
        ];
        let proof_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-overlap-region-anti-theatre-fence".to_string(),
                format!("public-contract:{}", public_contract_fence.proof_identity()),
                format!("overlap-evidence:{}", evidence_receipt.receipt_identity()),
                format!("guards:{}", guard_names.join("|")),
            ],
        );
        let mut rows = public_contract_fence.rows().to_vec();
        rows.extend(guard_names.iter().cloned().map(|guard_name| {
            PlanarBooleanOverlapRegionPublicContractProofRow::new(
                PlanarBooleanOverlapRegionPublicContractProofRowKind::AntiTheatreGuard,
                guard_name,
            )
        }));
        rows.push(PlanarBooleanOverlapRegionPublicContractProofRow::new(
            PlanarBooleanOverlapRegionPublicContractProofRowKind::AntiTheatreFence,
            proof_identity.clone(),
        ));
        Ok(Self {
            proof_identity,
            guard_names,
            rows,
        })
    }

    pub fn proof_identity(&self) -> &str { &self.proof_identity }
    pub fn guard_names(&self) -> &[String] { &self.guard_names }
    pub fn rows(&self) -> &[PlanarBooleanOverlapRegionPublicContractProofRow] { &self.rows }
}
