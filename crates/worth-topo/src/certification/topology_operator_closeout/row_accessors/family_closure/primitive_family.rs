use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::super::super::operator_family_proof::MilestoneThreePrimitiveFamilyClosureRow;
use crate::certification::DeterministicDigest;
use crate::topology_operators::{TopologyMutationDigest, TopologyMutationFamily};

impl MilestoneThreePrimitiveFamilyClosureRow {
    pub fn primitive_family(&self) -> &str {
        self.primitive_family.as_str()
    }

    pub fn primitive(&self) -> &MilestoneOnePrimitiveCase {
        &self.primitive
    }

    pub fn mutation_families(&self) -> &[TopologyMutationFamily] {
        self.mutation_families.as_slice()
    }

    pub fn topology_mutation_digest(&self) -> &TopologyMutationDigest {
        &self.topology_mutation_digest
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
