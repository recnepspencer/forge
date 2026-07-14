use super::{LayoutOwnerCaseDeclarations, LayoutOwnerFamily};

pub(super) fn register(declarations: &mut LayoutOwnerCaseDeclarations) {
    use worth_store_layout_indexes::integrity;
    declarations.insert(
        LayoutOwnerFamily::CorruptionClassification,
        integrity::corruption_classification_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::QuarantineReadmission,
        integrity::quarantine_readmission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::OfflineReadmission,
        integrity::offline_readmission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::ImportReadmission,
        integrity::import_readmission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::RestoredLayoutMaterialization,
        worth_store_operations::restored_layout_materialization_cases().map(|case| case.as_str()),
    );
}
