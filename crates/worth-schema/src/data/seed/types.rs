use forge_relational::facade::identity::EntityId;
use forge_relational::facade::snapshots::SnapshotHandle;
use serde::{Deserialize, Serialize};

use crate::data::authority::{
    CertifiedTopologyInterpretation, DerivedTopologyReadBasis, PersistedTopologyTruthBatch,
    WorthTopologyReadArtifact,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthMinimalTopologySeed {
    pub snapshot: SnapshotHandle,
    pub model: EntityId,
    pub body: EntityId,
    pub lump: EntityId,
    pub region: EntityId,
    pub shell: EntityId,
    pub face: EntityId,
    pub outer_loop: EntityId,
    pub wire: EntityId,
    pub half_edge: EntityId,
    pub edge: EntityId,
    pub vertex: EntityId,
    pub persistent_name_ids: Vec<EntityId>,
    pub persisted_truth: PersistedTopologyTruthBatch,
    pub read_basis: DerivedTopologyReadBasis,
    pub read_artifact: WorthTopologyReadArtifact,
    pub certified_interpretation: CertifiedTopologyInterpretation,
}
