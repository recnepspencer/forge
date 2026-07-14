pub(crate) fn root_materialization(
    family: crate::AdmittedPhysicalArtifactFamily,
    source: &worth_store_physical_format::PhysicalRootManifestRebuildSource,
) -> crate::AdmittedLayoutMaterialization {
    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    let publication = source.witness().manifest().root_publication();
    let references =
        worth_store_physical_format::PhysicalReferenceAuthority::for_canonical_physical_format();
    let publication = references
        .validate_root_publication(references.admit_root_publication(publication), publication)
        .expect("Store-issued rebuild source contains a valid root publication");
    crate::access_planning()
        .admit_btree_publication_materialization(family, &catalog, publication)
        .expect("rebuild fixture root materialization admission")
}

pub(crate) fn wal_materialization(
    family: crate::AdmittedPhysicalArtifactFamily,
) -> crate::AdmittedLayoutMaterialization {
    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    crate::strategy::tests_support::persisted_lsm_materialization(family, &catalog).0
}

mod source_witness;

pub(crate) use source_witness::{
    root_manifest_source_witness, root_manifest_source_witness_for_store,
    root_manifest_source_witness_rows, wal_replay_source_witness,
    wal_replay_source_witness_for_identity, wal_replay_source_witness_with_security,
};
