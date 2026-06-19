use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopReconstructionEvidenceReceipt, PlanarBooleanLoopReconstructionLedgerReceipt,
};

use super::guard_coverage::loop_reconstruction_guard_names;
use super::proof_rows::{
    PlanarBooleanLoopPublicContractProofRow, PlanarBooleanLoopPublicContractProofRowKind,
};
use super::public_contract_fence::PlanarBooleanLoopPublicContractFenceProof;

#[derive(Clone, Copy)]
pub(crate) struct PlanarBooleanLoopAntiTheatreFenceProofInput<'a> {
    loop_ledger_receipt: &'a PlanarBooleanLoopReconstructionLedgerReceipt,
    evidence_receipt: &'a PlanarBooleanLoopReconstructionEvidenceReceipt,
    public_contract_fence: &'a PlanarBooleanLoopPublicContractFenceProof,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PlanarBooleanLoopAntiTheatreFenceDenial {
    PublicContractFenceMismatch,
    DownstreamConsumptionMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlanarBooleanLoopAntiTheatreFenceProof {
    proof_identity: String,
    loop_receipt_identity: String,
    evidence_receipt_identity: String,
    downstream_consumption_identity: String,
    guard_names: Vec<String>,
    rows: Vec<PlanarBooleanLoopPublicContractProofRow>,
}

impl PlanarBooleanLoopAntiTheatreFenceProof {
    pub(crate) fn certify(
        input: PlanarBooleanLoopAntiTheatreFenceProofInput<'_>,
    ) -> Result<Self, PlanarBooleanLoopAntiTheatreFenceDenial> {
        if input.public_contract_fence.loop_receipt_identity()
            != input.loop_ledger_receipt.receipt_identity()
            || input.public_contract_fence.evidence_receipt_identity()
                != input.evidence_receipt.receipt_identity()
        {
            return Err(PlanarBooleanLoopAntiTheatreFenceDenial::PublicContractFenceMismatch);
        }
        if input
            .public_contract_fence
            .downstream_consumption_identity()
            != input.loop_ledger_receipt.downstream_consumption_identity()
            || input.evidence_receipt.downstream_consumption_identity()
                != input.loop_ledger_receipt.downstream_consumption_identity()
        {
            return Err(PlanarBooleanLoopAntiTheatreFenceDenial::DownstreamConsumptionMismatch);
        }

        let loop_receipt_identity = input.loop_ledger_receipt.receipt_identity().to_string();
        let evidence_receipt_identity = input.evidence_receipt.receipt_identity().to_string();
        let downstream_consumption_identity = input
            .loop_ledger_receipt
            .downstream_consumption_identity()
            .to_string();
        let guard_names = loop_reconstruction_guard_names();
        let proof_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-loop-anti-theatre-fence".to_string(),
                format!(
                    "public-contract:{}",
                    input.public_contract_fence.proof_identity()
                ),
                format!("loop-ledger:{loop_receipt_identity}"),
                format!("loop-evidence:{evidence_receipt_identity}"),
                format!("downstream:{downstream_consumption_identity}"),
                format!("guards:{}", guard_names.join("|")),
            ],
        );
        let mut rows = input.public_contract_fence.rows().to_vec();
        rows.extend(guard_names.iter().cloned().map(|guard_name| {
            PlanarBooleanLoopPublicContractProofRow::new(
                PlanarBooleanLoopPublicContractProofRowKind::AntiTheatreGuard,
                guard_name,
            )
        }));
        rows.push(PlanarBooleanLoopPublicContractProofRow::new(
            PlanarBooleanLoopPublicContractProofRowKind::AntiTheatreFence,
            proof_identity.clone(),
        ));
        Ok(Self {
            proof_identity,
            loop_receipt_identity,
            evidence_receipt_identity,
            downstream_consumption_identity,
            guard_names,
            rows,
        })
    }

    pub(crate) fn proof_identity(&self) -> &str {
        &self.proof_identity
    }

    pub(crate) fn guard_names(&self) -> &[String] {
        &self.guard_names
    }

    pub(crate) fn rows(&self) -> &[PlanarBooleanLoopPublicContractProofRow] {
        &self.rows
    }
}

impl<'a> PlanarBooleanLoopAntiTheatreFenceProofInput<'a> {
    pub(crate) fn from_parts(
        loop_ledger_receipt: &'a PlanarBooleanLoopReconstructionLedgerReceipt,
        evidence_receipt: &'a PlanarBooleanLoopReconstructionEvidenceReceipt,
        public_contract_fence: &'a PlanarBooleanLoopPublicContractFenceProof,
    ) -> Self {
        Self {
            loop_ledger_receipt,
            evidence_receipt,
            public_contract_fence,
        }
    }
}
