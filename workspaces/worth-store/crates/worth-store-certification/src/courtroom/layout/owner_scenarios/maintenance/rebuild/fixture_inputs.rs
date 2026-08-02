use worth_store_layout_indexes::strategy_declarations::LayoutStrategyFamily;
use worth_store_layout_indexes::{
    access_planning, access_shapes, AdmittedLayoutMaterialization, DerivedIndexRebuildRequest,
    DerivedIndexRebuildSourceInput, LayoutStrategyRegistrySnapshot,
};
use worth_store_physical_format::{
    PhysicalReferenceAuthority, PhysicalRootManifestRebuildSource, PhysicalStoreIdentity,
};

use super::super::strategy as maintenance_strategy;

pub(super) fn btree_strategy() -> LayoutStrategyRegistrySnapshot {
    maintenance_strategy::btree_strategy(
        worth_store_layout_indexes::IndexMaintenanceMode::SynchronousExact,
        worth_store_layout_indexes::PhysicalMutationShape::PointRewrite,
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
    worth_store_test_support::execute_root_manifest_rebuild_source(store, 7, page, 1)
}

pub(super) fn root_materialization(
    strategy: &LayoutStrategyRegistrySnapshot,
    source: &PhysicalRootManifestRebuildSource,
) -> AdmittedLayoutMaterialization {
    let catalog = worth_store_test_support::admitted_layout_bootstrap_catalog();
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
                worth_store_layout_indexes::AccessLaneClassification::Maintenance,
            )
            .expect("maintenance declares rebuild reads"),
        materialization,
        source,
    )
}

pub(super) fn request_with(
    strategy: &LayoutStrategyRegistrySnapshot,
    requested_strategy: LayoutStrategyFamily,
    shape: worth_store_layout_indexes::AccessShapeContract,
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
