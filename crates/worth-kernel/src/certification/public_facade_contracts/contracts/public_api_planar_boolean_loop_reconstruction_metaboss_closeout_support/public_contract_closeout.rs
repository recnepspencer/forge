use super::public_contract_support::public_contract_fence::{
    PlanarBooleanLoopPublicContractFenceProof, PlanarBooleanLoopPublicContractFenceProofInput,
};
use worth_kernel::workload_composition::CompletedBooleanLoopReconstructionHandoff;

pub(crate) fn certify_public_contract_closeout(
    handoff: &CompletedBooleanLoopReconstructionHandoff,
) -> PlanarBooleanLoopPublicContractFenceProof {
    PlanarBooleanLoopPublicContractFenceProof::certify(
        PlanarBooleanLoopPublicContractFenceProofInput::from_handoff(handoff),
    )
    .expect("canonical loop handoff should certify the public contract fence")
}
