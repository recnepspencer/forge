use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;
use serde::{Deserialize, Serialize};

use crate::certification::DeterministicDigest;
use crate::topology_operators::{TopologyEditDigest, TopologyEditFamily};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreePrimitiveFamilyClosureRow {
    pub(crate) primitive_family: String,
    pub(crate) primitive: MilestoneOnePrimitiveCase,
    pub(crate) edit_families: Vec<TopologyEditFamily>,
    pub(crate) topology_edit_digest: TopologyEditDigest,
    pub(crate) replay_verified: bool,
    pub(crate) final_materialized_topology_digest: DeterministicDigest,
    pub(crate) replay_final_materialized_topology_digest: DeterministicDigest,
    pub(crate) derived_validation_row_count: usize,
    pub(crate) row_digest: String,
}
