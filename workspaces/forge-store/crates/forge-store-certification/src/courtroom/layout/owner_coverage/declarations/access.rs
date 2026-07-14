use super::{LayoutOwnerCaseDeclarations, LayoutOwnerFamily};

pub(super) fn register(declarations: &mut LayoutOwnerCaseDeclarations) {
    declarations.insert(
        LayoutOwnerFamily::ArtifactFamilyAdmission,
        forge_store_layout_indexes::artifact_family_admission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::PhysicalKeyDomainAdmission,
        forge_store_layout_indexes::physical_key_domain_admission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::BootstrapCatalogRead,
        forge_store_layout_indexes::bootstrap_catalog_read_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::AccessPlanSelection,
        forge_store_layout_indexes::access_plan_selection_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::PreExecutionBudgetAdmission,
        forge_store_budgets::pre_execution_budget_admission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::FullDeclaredScan,
        forge_store_layout_indexes::full_declared_scan_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::BTreeLookupReadiness,
        forge_store_layout_indexes::btree_lookup_readiness_cases().map(|case| case.name()),
    );
    declarations.insert(
        LayoutOwnerFamily::BTreeLookupExecution,
        forge_store_layout_indexes::btree_lookup_execution_cases().map(|case| case.name()),
    );
    declarations.insert(
        LayoutOwnerFamily::BTreeReplayExecution,
        forge_store_layout_indexes::btree_replay_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::DegradedScanReadiness,
        forge_store_layout_indexes::degraded_scan_readiness_cases().map(|case| case.name()),
    );
    declarations.insert(
        LayoutOwnerFamily::LsmLookupReadiness,
        forge_store_layout_indexes::baseline_lsm_lookup_admission_cases().map(|case| case.name()),
    );
    declarations.insert(
        LayoutOwnerFamily::LsmLookupExecution,
        forge_store_layout_indexes::baseline_lsm_lookup_cases().map(|case| case.name()),
    );
    declarations.insert(
        LayoutOwnerFamily::ImportedBlobReadAdmission,
        forge_store_layout_indexes::imported_blob_read_admission_cases().map(|case| case.as_str()),
    );
}
