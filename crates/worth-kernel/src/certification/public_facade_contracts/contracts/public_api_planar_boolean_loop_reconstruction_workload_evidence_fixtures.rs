use worth_kernel::workload_composition::{
    PlanarBooleanLoopRuntimeRegistrationProof, WorkloadStageRequirement,
};
use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionLedgerReceipt;

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
