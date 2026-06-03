use crate::certification::TopologyBranchAuthoringBoundary;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::super::super::scale_pressure_proof::{
    MilestoneThreeScalePressureRow, MilestoneThreeScalePressureSweep,
};
use crate::topology_operators::{TopologyMutationDigest, TopologyMutationFamily};

impl MilestoneThreeScalePressureRow {
    pub fn sweep(&self) -> MilestoneThreeScalePressureSweep {
        self.sweep
    }

    pub fn sweep_label(&self) -> &'static str {
        self.sweep.as_str()
    }

    pub fn primitive_family(&self) -> &str {
        self.primitive_family.as_str()
    }

    pub fn primitive(&self) -> &MilestoneOnePrimitiveCase {
        &self.primitive
    }

    pub fn workload_size(&self) -> usize {
        self.workload_size
    }

    pub fn mutation_step_count(&self) -> usize {
        self.mutation_step_count
    }

    pub fn mutation_families(&self) -> &[TopologyMutationFamily] {
        self.mutation_families.as_slice()
    }

    pub fn branch_local(&self) -> bool {
        self.branch_local
    }

    pub fn branch_authoring_boundary(&self) -> Option<TopologyBranchAuthoringBoundary> {
        self.branch_authoring_boundary.clone()
    }

    pub fn topology_mutation_digest(&self) -> &TopologyMutationDigest {
        &self.topology_mutation_digest
    }

    pub fn replay_verified(&self) -> bool {
        self.replay_verified
    }

    pub fn final_state_digest(&self) -> &str {
        self.final_state_digest.as_str()
    }

    pub fn replay_final_state_digest(&self) -> &str {
        self.replay_final_state_digest.as_str()
    }

    pub fn derived_validation_row_count(&self) -> usize {
        self.derived_validation_row_count
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}
