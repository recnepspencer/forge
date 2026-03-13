use crate::authority::commit::preparation::proofs::kinds::PreparationProofKind;
use crate::authority::commit::preparation::proofs::locality::PreparationLocalityProof;
use crate::authority::commit::preparation::reduction::keys::IndexReductionKey;
use crate::indexes::data::DerivedIndexDefinition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IndexFragmentIdentity {
    pub(crate) index_id: crate::indexes::data::DerivedIndexId,
    pub(crate) packet_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IndexPreparationPacket {
    pub(crate) packet_index: usize,
    pub(crate) identity: IndexFragmentIdentity,
    pub(crate) definition: DerivedIndexDefinition,
    pub(crate) reduction_key: IndexReductionKey,
    pub(crate) proof_kind: PreparationProofKind,
    pub(crate) locality: PreparationLocalityProof,
}
