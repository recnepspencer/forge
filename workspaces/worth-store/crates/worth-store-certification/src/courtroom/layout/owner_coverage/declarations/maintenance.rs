use super::{LayoutOwnerCaseDeclarations, LayoutOwnerFamily};

pub(super) fn register(declarations: &mut LayoutOwnerCaseDeclarations) {
    declarations.insert(
        LayoutOwnerFamily::LiveMaintenancePosture,
        worth_store_layout_indexes::live_maintenance_posture_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::LayoutMutationAdmission,
        worth_store_layout_indexes::layout_mutation_admission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::DerivedIndexParity,
        worth_store_layout_indexes::derived_index_parity_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::DerivedIndexRebuildAdmission,
        worth_store_layout_indexes::derived_index_rebuild_admission_cases()
            .map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::DerivedIndexRebuildExecution,
        worth_store_layout_indexes::derived_index_rebuild_execution_cases()
            .map(|case| case.as_str()),
    );
    use worth_store_layout_indexes::LsmMaintenanceOperation;
    for (family, operation) in [
        (
            LayoutOwnerFamily::LsmRunPublicationAdmission,
            LsmMaintenanceOperation::AdmitRunPublication,
        ),
        (
            LayoutOwnerFamily::LsmReplayAdmission,
            LsmMaintenanceOperation::AdmitReplay,
        ),
        (
            LayoutOwnerFamily::LsmCompactionAdmission,
            LsmMaintenanceOperation::AdmitCompaction,
        ),
    ] {
        declarations.insert(
            family,
            worth_store_layout_indexes::lsm_maintenance_owner_case_inventory()
                .filter(move |case| case.id().operation() == operation)
                .map(|case| case.id().disposition().as_str()),
        );
    }
}
