use forge_store_layout_indexes::strategy_declarations::LayoutStrategyFamily;
use forge_store_layout_indexes::{
    access_planning, access_shapes, AdmittedLayoutMaterialization, DerivedIndexRebuildRequest,
    DerivedIndexRebuildSourceInput, LayoutStrategyRegistrySnapshot,
};
use forge_store_physical_format::{
    PhysicalReferenceAuthority, PhysicalRootManifestRebuildSource, PhysicalStoreIdentity,
};

use super::super::strategy as maintenance_strategy;

pub(super) fn btree_strategy() -> LayoutStrategyRegistrySnapshot {
    maintenance_strategy::btree_strategy(
        forge_store_layout_indexes::IndexMaintenanceMode::SynchronousExact,
        forge_store_layout_indexes::PhysicalMutationShape::PointRewrite,
    )
}

pub(super) fn lsm_strategy() -> LayoutStrategyRegistrySnapshot {
    maintenance_strategy::lsm_strategy()
}

pub(super) fn root_source(page: u64) -> PhysicalRootManifestRebuildSource {
    root_source_for_store(page, &PhysicalStoreIdentity::physical_format_default())
}

pub(super) fn root_source_for_store(
    page: u64,
    store: &PhysicalStoreIdentity,
) -> PhysicalRootManifestRebuildSource {
    forge_store_test_support::execute_root_manifest_rebuild_source(store, 7, page, 1)
}

pub(super) fn root_materialization(
    strategy: &LayoutStrategyRegistrySnapshot,
    source: &PhysicalRootManifestRebuildSource,
) -> AdmittedLayoutMaterialization {
    let catalog = forge_store_test_support::admitted_layout_bootstrap_catalog();
    let publication = source.witness().manifest().root_publication();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let validated = references
        .validate_root_publication(references.admit_root_publication(publication), publication)
        .expect("Store-issued rebuild source contains a valid root publication");
    access_planning()
        .admit_btree_publication_materialization(
            strategy.admitted_strategy().admitted_family(),
            &catalog,
            validated,
        )
        .into_result()
        .expect("published root must admit exact rebuild materialization")
}

pub(super) fn root_request(
    strategy: &LayoutStrategyRegistrySnapshot,
    materialization: AdmittedLayoutMaterialization,
    source: DerivedIndexRebuildSourceInput,
) -> DerivedIndexRebuildRequest {
    request_with(
        strategy,
        strategy.admitted_strategy().family(),
        access_shapes()
            .rebuild_read_declaration(
                forge_store_layout_indexes::AccessLaneClassification::Maintenance,
            )
            .expect("maintenance declares rebuild reads"),
        materialization,
        source,
    )
}

pub(super) fn request_with(
    strategy: &LayoutStrategyRegistrySnapshot,
    requested_strategy: LayoutStrategyFamily,
    shape: forge_store_layout_indexes::AccessShapeContract,
    materialization: AdmittedLayoutMaterialization,
    source: DerivedIndexRebuildSourceInput,
) -> DerivedIndexRebuildRequest {
    DerivedIndexRebuildRequest::new(
        strategy.admitted_strategy().admitted_family(),
        strategy.admitted_strategy().admitted_key_domain(),
        requested_strategy,
        shape,
        materialization,
        source,
    )
}

pub(super) fn lsm_materialization() -> AdmittedLayoutMaterialization {
    let strategy = lsm_strategy();
    let catalog = forge_store_test_support::admitted_layout_bootstrap_catalog();
    let published = forge_store_test_support::execute_baseline_lsm_persisted_fixture();
    access_planning()
        .admit_lsm_publication_materialization(
            strategy.admitted_strategy().admitted_family(),
            &catalog,
            &published.publication_execution(),
        )
        .into_result()
        .expect("published LSM membership must materialize")
}

pub(super) fn wal_source_for_materialization(
    materialization: &AdmittedLayoutMaterialization,
    security: &forge_store_security::StoreAdmittedSecurityScope,
) -> forge_store_wal::BlobWalReplayRebuildWitness {
    let forge_store_layout_indexes::LayoutMaterializationSourceKind::LsmReplacement(identity) =
        materialization.source().kind()
    else {
        panic!("LSM publication must retain replacement identity");
    };
    wal_source(identity, security)
}

pub(super) fn wal_source_with_next_identity(
    materialization: &AdmittedLayoutMaterialization,
    security: &forge_store_security::StoreAdmittedSecurityScope,
) -> forge_store_wal::BlobWalReplayRebuildWitness {
    let forge_store_layout_indexes::LayoutMaterializationSourceKind::LsmReplacement(identity) =
        materialization.source().kind()
    else {
        panic!("LSM publication must retain replacement identity");
    };
    let identity =
        forge_store_wal::BlobWalRecordIdentity::new(identity.sequence() + 1, identity.kind())
            .expect("the hostile WAL identity remains structurally valid");
    wal_source(identity, security)
}

fn wal_source(
    identity: forge_store_wal::BlobWalRecordIdentity,
    security: &forge_store_security::StoreAdmittedSecurityScope,
) -> forge_store_wal::BlobWalReplayRebuildWitness {
    let record = forge_store_wal::BlobWalRecordEnvelope::new(
        identity,
        forge_store_wal::DurablePublicationDeclaration::wal_frame(
            forge_store_wal::WalFrameDurablePublicationScope::new(
                1,
                1,
                10,
                20,
                "sha256:rebuild-frame",
                4096,
            )
            .expect("fixture WAL frame is valid"),
        ),
        "sha256:rebuild-payload",
    )
    .expect("fixture WAL record is valid");
    let metadata = forge_store_wal::WalSecurityMetadataCarrier::for_wal_record(
        security.witnesses(),
        forge_store_security::StoreKeyVersionPosture::Current,
        forge_store_security::StoreLegacySecurityPosture::NativeScoped,
    );
    forge_store_wal::BlobWalReplayRebuildWitness::admit(
        forge_store_wal::WalSecurityMetadataEnvelope::wal_record(record, metadata),
    )
}
