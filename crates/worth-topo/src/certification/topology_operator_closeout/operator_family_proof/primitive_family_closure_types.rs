use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;
use serde::{Deserialize, Serialize};

use crate::certification::DeterministicDigest;
use crate::topology_operators::{TopologyMutationDigest, TopologyMutationFamily};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreePrimitiveFamilyClosureRow {
    pub(crate) primitive_family: String,
    pub(crate) primitive: MilestoneOnePrimitiveCase,
    pub(crate) mutation_families: Vec<TopologyMutationFamily>,
    pub(crate) topology_mutation_digest: TopologyMutationDigest,
    pub(crate) replay_verified: bool,
    pub(crate) final_materialized_topology_digest: DeterministicDigest,
    pub(crate) replay_final_materialized_topology_digest: DeterministicDigest,
    pub(crate) derived_validation_row_count: usize,
    pub(crate) row_digest: String,
}
