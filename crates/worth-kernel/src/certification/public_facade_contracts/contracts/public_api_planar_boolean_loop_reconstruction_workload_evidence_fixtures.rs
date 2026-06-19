use worth_kernel::workload_composition::{
    PlanarBooleanLoopRuntimeRegistrationProof, WorkloadStageRequirement,
};
use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionLedgerReceipt;
use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, BooleanEvidenceRowAuthority, BooleanEvidenceStageKind,
    WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

pub(crate) fn assert_runtime_registration_proof(
    proof: &PlanarBooleanLoopRuntimeRegistrationProof,
    receipt: &PlanarBooleanLoopReconstructionLedgerReceipt,
    stage_index_identity: &str,
    registry_identity: &str,
) {
    assert_eq!(proof.loop_receipt_identity(), receipt.receipt_identity());
    assert_eq!(proof.loop_ledger_identity(), receipt.ledger_identity());
    assert_eq!(
        proof.downstream_consumption_identity(),
        receipt.downstream_consumption_identity()
    );
    assert_eq!(proof.stage_index_identity(), stage_index_identity);
    assert_eq!(proof.registry_identity().digest(), registry_identity);
    assert!(!proof.proof_identity().is_empty());
    assert!(proof
        .operator_names()
        .iter()
        .any(|name| name == "RegisterBooleanLoopReconstructionStageRequirement"));
    assert!(proof
        .validator_names()
        .iter()
        .any(|name| name == "ValidateLoopValidatorRuntimeRegistration"));
    assert_eq!(
        WorkloadStageRequirement::BooleanLoopReconstruction.query_key(),
        "boolean_loop_reconstruction"
    );
}

pub(crate) struct CounterlessLoopLedgerEvidence {
    identity: String,
}

impl CounterlessLoopLedgerEvidence {
    pub(crate) fn new(identity: &str) -> Self {
        Self {
            identity: identity.to_string(),
        }
    }
}

impl BooleanEvidenceReceipt for CounterlessLoopLedgerEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::LoopReconstruction
    }

    fn evidence_identity(&self) -> &str {
        &self.identity
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::default()
    }
}

impl BooleanEvidenceRowAuthority for CounterlessLoopLedgerEvidence {}
