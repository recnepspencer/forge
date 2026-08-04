mod fact;

pub use fact::{
    PhysicalRecoveryAttemptBindingFact, PhysicalRecoveryCompletedMutationFact,
    PhysicalRecoveryOperationFact, PhysicalRecoveryOperationFate,
    PhysicalRecoveryWalAttemptBinding,
};

use super::super::mutation::PhysicalMutationIdempotencyRegistry;

pub struct PhysicalRecoveryOperationFates {
    generation: crate::physical_runtime::PhysicalNamespaceDurableCheckpointGeneration,
    facts: Box<[PhysicalRecoveryOperationFact]>,
    completed_unobserved: Box<[crate::physical_runtime::CompletedUnobservedPhysicalMutation]>,
    counts: PhysicalRecoveryOperationFateCounts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecoveryOperationFateCounts {
    unresolved: u64,
    completed: u64,
    proven_no_effect: u64,
    indeterminate: u64,
    completed_unobserved: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum PhysicalIdempotencyCloseoutDenial {
    LiveCompactionAuthority,
}

impl PhysicalRecoveryOperationFates {
    pub(in crate::physical_runtime::durability) fn new(
        registry: PhysicalMutationIdempotencyRegistry,
        completed_unobserved: Box<[crate::physical_runtime::CompletedUnobservedPhysicalMutation]>,
    ) -> Self {
        let (generation, facts) = registry.into_closeout_facts();
        let mut counts = [0_u64; 4];
        for fact in &facts {
            let index = match fact.fate() {
                PhysicalRecoveryOperationFate::Unresolved => 0,
                PhysicalRecoveryOperationFate::Completed(_) => 1,
                PhysicalRecoveryOperationFate::ProvenNoEffect(_) => 2,
                PhysicalRecoveryOperationFate::Indeterminate(_) => 3,
            };
            counts[index] = counts[index].saturating_add(1);
        }
        let [unresolved, completed, proven_no_effect, indeterminate] = counts;
        let completed_unobserved_count = completed_unobserved.len() as u64;
        Self {
            generation,
            facts,
            completed_unobserved,
            counts: PhysicalRecoveryOperationFateCounts {
                unresolved,
                completed,
                proven_no_effect,
                indeterminate,
                completed_unobserved: completed_unobserved_count,
            },
        }
    }

    pub const fn generation(
        &self,
    ) -> crate::physical_runtime::PhysicalNamespaceDurableCheckpointGeneration {
        self.generation
    }

    pub fn facts(&self) -> &[PhysicalRecoveryOperationFact] {
        &self.facts
    }

    pub const fn counts(&self) -> PhysicalRecoveryOperationFateCounts {
        self.counts
    }

    pub fn completed_unobserved(
        &self,
    ) -> &[crate::physical_runtime::CompletedUnobservedPhysicalMutation] {
        &self.completed_unobserved
    }
}

impl PhysicalRecoveryOperationFateCounts {
    pub const fn unresolved(self) -> u64 {
        self.unresolved
    }

    pub const fn completed(self) -> u64 {
        self.completed
    }

    pub const fn proven_no_effect(self) -> u64 {
        self.proven_no_effect
    }

    pub const fn indeterminate(self) -> u64 {
        self.indeterminate
    }

    pub const fn completed_unobserved(self) -> u64 {
        self.completed_unobserved
    }
}
