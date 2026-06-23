use super::public_contract_support::anti_theatre_fence::{
    PlanarBooleanLoopAntiTheatreFenceProof, PlanarBooleanLoopAntiTheatreFenceProofInput,
};
use super::public_contract_support::public_contract_fence::PlanarBooleanLoopPublicContractFenceProof;
use worth_kernel::workload_composition::CompletedBooleanLoopReconstructionHandoff;

pub(crate) fn certify_anti_theatre_closeout(
    handoff: &CompletedBooleanLoopReconstructionHandoff,
    public_contract: &PlanarBooleanLoopPublicContractFenceProof,
) -> PlanarBooleanLoopAntiTheatreFenceProof {
    PlanarBooleanLoopAntiTheatreFenceProof::certify(
        PlanarBooleanLoopAntiTheatreFenceProofInput::from_parts(
            handoff.loop_ledger_receipt(),
            handoff.evidence_receipt(),
            public_contract,
        ),
    )
    .expect("canonical loop handoff should certify the anti-theatre fence")
}
