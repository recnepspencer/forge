mod authority;
mod backward_read;
mod binding_planning;
mod world;

use worth_store_layout_indexes::evolution::migration::{
    LayoutBackwardReadCompatibilityCaseId, LayoutBindingAdmissionCaseId, MigrationPlanningCaseId,
    RollbackPlanningCaseId,
};
use worth_store_layout_indexes::OwnerCaseObservation;

#[derive(Debug)]
pub struct LayoutEvolutionOwnerCaseObservations {
    binding: Vec<OwnerCaseObservation<LayoutBindingAdmissionCaseId>>,
    migration_planning: Vec<OwnerCaseObservation<MigrationPlanningCaseId>>,
    rollback_planning: Vec<OwnerCaseObservation<RollbackPlanningCaseId>>,
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

    pub fn rollback_planning(
        &self,
    ) -> impl Iterator<Item = OwnerCaseObservation<RollbackPlanningCaseId>> + '_ {
        self.rollback_planning.iter().copied()
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
    let backward_read = backward_read::observe();
    LayoutEvolutionOwnerCaseObservations {
        binding,
        migration_planning,
        rollback_planning,
        backward_read,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use worth_store_layout_indexes::evolution::migration;

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
                .rollback_planning()
                .map(|case| case.case_id())
                .collect::<BTreeSet<_>>(),
            migration::rollback_planning_cases().collect()
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
