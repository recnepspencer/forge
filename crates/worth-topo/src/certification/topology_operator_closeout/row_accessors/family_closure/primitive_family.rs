use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::super::super::operator_family_proof::MilestoneThreePrimitiveFamilyClosureRow;
use crate::certification::DeterministicDigest;
use crate::topology_operators::{TopologyEditDigest, TopologyEditFamily};

impl MilestoneThreePrimitiveFamilyClosureRow {
    pub fn primitive_family(&self) -> &str {
        self.primitive_family.as_str()
    }

    pub fn primitive(&self) -> &MilestoneOnePrimitiveCase {
        &self.primitive
    }

    pub fn edit_families(&self) -> &[TopologyEditFamily] {
        self.edit_families.as_slice()
    }

    pub fn topology_edit_digest(&self) -> &TopologyEditDigest {
        &self.topology_edit_digest
    }

    pub fn replay_verified(&self) -> bool {
        self.replay_verified
    }

    pub fn final_materialized_topology_digest(&self) -> &DeterministicDigest {
        &self.final_materialized_topology_digest
    }

    pub fn replay_final_materialized_topology_digest(&self) -> &DeterministicDigest {
        &self.replay_final_materialized_topology_digest
    }

    pub fn derived_validation_row_count(&self) -> usize {
        self.derived_validation_row_count
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}




