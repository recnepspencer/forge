use super::{LayoutOwnerCaseDeclarations, LayoutOwnerFamily};

pub(super) fn register(declarations: &mut LayoutOwnerCaseDeclarations) {
    use worth_store_layout_indexes::materialization;
    declarations.insert(
        LayoutOwnerFamily::CatalogRootMaterialization,
        materialization::catalog_root_materialization_admission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::BTreePublicationMaterialization,
        materialization::btree_publication_materialization_admission_cases()
            .map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::BTreeLookupMaterialization,
        materialization::btree_lookup_materialization_admission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::BTreeReplayMaterialization,
        materialization::btree_replay_materialization_admission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::LsmLookupMaterialization,
        materialization::lsm_lookup_materialization_admission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::LsmPublicationMaterialization,
        materialization::lsm_publication_materialization_admission_cases()
            .map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::LsmReplayMaterialization,
        materialization::lsm_replay_materialization_admission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::ImportedBlobMaterialization,
        materialization::imported_blob_materialization_admission_cases().map(|case| case.as_str()),
    );
    declarations.insert(
        LayoutOwnerFamily::RestoredArtifactMaterialization,
        materialization::restored_artifact_materialization_admission_cases()
            .map(|case| case.as_str()),
    );
}
