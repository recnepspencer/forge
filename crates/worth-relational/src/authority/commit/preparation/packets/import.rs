use crate::authority::commit::preparation::proofs::kinds::PreparationProofKind;
use crate::authority::commit::preparation::proofs::locality::PreparationLocalityProof;
use crate::identity::data::{KindId, PartitionId};
use crate::transactions::data::{AspectFieldPatch, EntityReference};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ImportFragmentKind {
    EntityCreate,
    RelationCreate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ImportFragmentIdentity {
    pub(crate) partition_id: PartitionId,
    pub(crate) kind_id: KindId,
    pub(crate) fragment_kind: ImportFragmentKind,
    pub(crate) packet_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ImportStagedRow {
    Entity {
        fields: AspectFieldPatch,
    },
    Relation {
        source: EntityReference,
        target: EntityReference,
        fields: AspectFieldPatch,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ImportStagingHeader {
    pub(crate) packet_index_floor: usize,
    pub(crate) identity: ImportFragmentIdentity,
    pub(crate) proof_kind: PreparationProofKind,
    pub(crate) locality: PreparationLocalityProof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ImportStagingPacket {
    pub(crate) header: ImportStagingHeader,
    pub(crate) rows: Vec<ImportStagedRow>,
}
