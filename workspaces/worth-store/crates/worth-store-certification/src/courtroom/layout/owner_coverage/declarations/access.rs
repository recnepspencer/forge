use super::{LayoutOwnerCaseDeclarations, LayoutOwnerFamily};

pub(super) fn register(declarations: &mut LayoutOwnerCaseDeclarations) {
    declarations.insert(
        LayoutOwnerFamily::ArtifactFamilyAdmission,
        worth_store_layout_indexes::artifact_family_admission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::PhysicalKeyDomainAdmission,
        worth_store_layout_indexes::physical_key_domain_admission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::BootstrapCatalogRead,
        worth_store_layout_indexes::bootstrap_catalog_read_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::AccessPlanSelection,
        worth_store_layout_indexes::access_plan_selection_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::PreExecutionBudgetAdmission,
        worth_store_budgets::pre_execution_budget_admission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::FullDeclaredScan,
        worth_store_layout_indexes::full_declared_scan_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::BTreeLookupReadiness,
        worth_store_layout_indexes::btree_lookup_readiness_cases().map(|case| case.name()),
    );
    declarations.insert(
        LayoutOwnerFamily::BTreeLookupExecution,
        worth_store_layout_indexes::btree_lookup_execution_cases().map(|case| case.name()),
    );
    declarations.insert(
        LayoutOwnerFamily::BTreeReplayExecution,
        worth_store_layout_indexes::btree_replay_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::DegradedScanReadiness,
        worth_store_layout_indexes::degraded_scan_readiness_cases().map(|case| case.name()),
    );
    declarations.insert(
        LayoutOwnerFamily::LsmLookupReadiness,
        worth_store_layout_indexes::baseline_lsm_lookup_admission_cases().map(|case| case.name()),
    );
    declarations.insert(
        LayoutOwnerFamily::LsmLookupExecution,
        worth_store_layout_indexes::baseline_lsm_lookup_cases().map(|case| case.name()),
    );
    declarations.insert(
        LayoutOwnerFamily::ImportedBlobReadAdmission,
        worth_store_layout_indexes::imported_blob_read_admission_cases().map(|case| case.as_str()),
    );
}
