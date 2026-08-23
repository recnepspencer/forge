use super::{LayoutOwnerCaseDeclarations, LayoutOwnerFamily};

pub(super) fn register(declarations: &mut LayoutOwnerCaseDeclarations) {
    use worth_store_layout_indexes::evolution::migration;
    declarations.insert(
        LayoutOwnerFamily::LayoutBindingAdmission,
        migration::layout_binding_admission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::MigrationPlanning,
        migration::migration_planning_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::RollbackPlanning,
        migration::rollback_planning_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::BackwardReadCompatibility,
        migration::layout_backward_read_compatibility_cases().map(|case| case.as_str()),
    );
}
