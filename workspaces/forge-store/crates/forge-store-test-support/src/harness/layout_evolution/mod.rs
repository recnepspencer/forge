mod authority;
mod binding_planning;
mod execution;
mod interruption_compatibility;
mod world;

use forge_store_layout_indexes::evolution::migration::{
    LayoutBackwardReadCompatibilityCaseId, LayoutBindingAdmissionCaseId,
    LayoutMigrationExecutionCaseId, LayoutMigrationInterruptionCaseId,
    LayoutRollbackExecutionCaseId, LayoutRollbackInterruptionCaseId, MigrationPlanningCaseId,
    RollbackPlanningCaseId,
};
use forge_store_layout_indexes::OwnerCaseObservation;

#[derive(Debug)]
pub struct LayoutEvolutionOwnerCaseObservations {
    binding: Vec<OwnerCaseObservation<LayoutBindingAdmissionCaseId>>,
    migration_planning: Vec<OwnerCaseObservation<MigrationPlanningCaseId>>,
    migration_execution: Vec<OwnerCaseObservation<LayoutMigrationExecutionCaseId>>,
    migration_interruption: Vec<OwnerCaseObservation<LayoutMigrationInterruptionCaseId>>,
    rollback_planning: Vec<OwnerCaseObservation<RollbackPlanningCaseId>>,
    rollback_execution: Vec<OwnerCaseObservation<LayoutRollbackExecutionCaseId>>,
    rollback_interruption: Vec<OwnerCaseObservation<LayoutRollbackInterruptionCaseId>>,
    backward_read: Vec<OwnerCaseObservation<LayoutBackwardReadCompatibilityCaseId>>,
}

impl LayoutEvolutionOwnerCaseObservations {
    pub fn binding(
        &self,
    ) -> impl Iterator<Item = OwnerCaseObservation<LayoutBindingAdmissionCaseId>> + '_ {
        self.binding.iter().copied()
    }

    pub fn migration_planning(
        &self,
    ) -> impl Iterator<Item = OwnerCaseObservation<MigrationPlanningCaseId>> + '_ {
        self.migration_planning.iter().copied()
    }

    pub fn migration_execution(
        &self,
    ) -> impl Iterator<Item = OwnerCaseObservation<LayoutMigrationExecutionCaseId>> + '_ {
        self.migration_execution.iter().copied()
    }

    pub fn migration_interruption(
        &self,
    ) -> impl Iterator<Item = OwnerCaseObservation<LayoutMigrationInterruptionCaseId>> + '_ {
        self.migration_interruption.iter().copied()
    }

    pub fn rollback_planning(
        &self,
    ) -> impl Iterator<Item = OwnerCaseObservation<RollbackPlanningCaseId>> + '_ {
        self.rollback_planning.iter().copied()
    }

    pub fn rollback_execution(
        &self,
    ) -> impl Iterator<Item = OwnerCaseObservation<LayoutRollbackExecutionCaseId>> + '_ {
        self.rollback_execution.iter().copied()
    }

    pub fn rollback_interruption(
        &self,
    ) -> impl Iterator<Item = OwnerCaseObservation<LayoutRollbackInterruptionCaseId>> + '_ {
        self.rollback_interruption.iter().copied()
    }

    pub fn backward_read(
        &self,
    ) -> impl Iterator<Item = OwnerCaseObservation<LayoutBackwardReadCompatibilityCaseId>> + '_
    {
        self.backward_read.iter().copied()
    }
}

pub fn observe_layout_evolution_owner_cases() -> LayoutEvolutionOwnerCaseObservations {
    let (binding, migration_planning, rollback_planning) = binding_planning::observe();
    let (migration_execution, rollback_execution) = execution::observe();
    let (migration_interruption, rollback_interruption, backward_read) =
        interruption_compatibility::observe();
    LayoutEvolutionOwnerCaseObservations {
        binding,
        migration_planning,
        migration_execution,
        migration_interruption,
        rollback_planning,
        rollback_execution,
        rollback_interruption,
        backward_read,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use forge_store_layout_indexes::evolution::migration;

    use super::observe_layout_evolution_owner_cases;

    #[test]
    fn evolution_inventory_equals_ordinary_owner_observations() {
        let observed = observe_layout_evolution_owner_cases();
        assert_eq!(
            observed
                .binding()
                .map(|case| case.case_id())
                .collect::<BTreeSet<_>>(),
            migration::layout_binding_admission_cases().collect()
        );
        assert_eq!(
            observed
                .migration_planning()
                .map(|case| case.case_id())
                .collect::<BTreeSet<_>>(),
            migration::migration_planning_cases().collect()
        );
        assert_eq!(
            observed
                .migration_execution()
                .map(|case| case.case_id())
                .collect::<BTreeSet<_>>(),
            migration::layout_migration_execution_cases().collect()
        );
        assert_eq!(
            observed
                .migration_interruption()
                .map(|case| case.case_id())
                .collect::<BTreeSet<_>>(),
            migration::layout_migration_interruption_cases().collect()
        );
        assert_eq!(
            observed
                .rollback_planning()
                .map(|case| case.case_id())
                .collect::<BTreeSet<_>>(),
            migration::rollback_planning_cases().collect()
        );
        assert_eq!(
            observed
                .rollback_execution()
                .map(|case| case.case_id())
                .collect::<BTreeSet<_>>(),
            migration::layout_rollback_execution_cases().collect()
        );
        assert_eq!(
            observed
                .rollback_interruption()
                .map(|case| case.case_id())
                .collect::<BTreeSet<_>>(),
            migration::layout_rollback_interruption_cases().collect()
        );
        assert_eq!(
            observed
                .backward_read()
                .map(|case| case.case_id())
                .collect::<BTreeSet<_>>(),
            migration::layout_backward_read_compatibility_cases().collect()
        );
    }
}
