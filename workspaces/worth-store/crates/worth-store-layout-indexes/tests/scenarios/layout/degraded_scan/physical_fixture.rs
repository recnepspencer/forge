use worth_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use worth_store_layout_indexes::{
    bootstrap_catalog, BootstrapCatalogReadAdmission, BootstrapOnlyAccessPath,
};
use worth_store_physical_format::{
    physical_bootstrap_catalog, PhysicalExtentId, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalPageId, PhysicalRecordSlot, PhysicalSegmentId, PhysicalStoreRuntime,
    PlatformPhysicalAppendRequest, PlatformPhysicalOpenRequest,
};

pub(super) fn admitted_catalog() -> BootstrapCatalogReadAdmission {
    admitted_catalog_from_publication(published_layout(false))
}

pub(super) fn advanced_catalog() -> BootstrapCatalogReadAdmission {
    admitted_catalog_from_publication(published_layout(true))
}

pub(super) fn open_runtime() -> PhysicalStoreRuntime {
    open_runtime_for_store(
        &worth_store_physical_format::PhysicalStoreIdentity::physical_format_default(),
    )
}

pub(super) fn open_runtime_for_store(
    store_identity: &worth_store_physical_format::PhysicalStoreIdentity,
) -> PhysicalStoreRuntime {
    PhysicalStoreRuntime::open_physical_format(
        readiness(),
        PlatformPhysicalOpenRequest::physical_format_for_store(store_identity.clone()),
    )
    .expect("open degraded scan physical fixture")
}

pub(super) fn foreign_store_identity() -> worth_store_physical_format::PhysicalStoreIdentity {
    let key = worth_foundational::aspects()
        .vocabulary()
        .key("store.physical.foreign_instance")
        .unwrap();
    worth_store_physical_format::PhysicalStoreIdentity::from_aspect_identity(
        worth_store_aspect_native::StoreAspectIdentity::from_aspect_key(key),
    )
}

fn admitted_catalog_from_publication(
    published: worth_store_physical_format::PlatformPhysicalRootPublicationReport,
) -> BootstrapCatalogReadAdmission {
    let open = published.admit_bootstrap_open_witness().unwrap();
    let discovered = physical_bootstrap_catalog()
        .discover_catalog(&open)
        .unwrap();
    let current_root = physical_bootstrap_catalog()
        .discover_catalog(&open)
        .unwrap()
        .current_root();
    bootstrap_catalog()
        .read_catalog(
            BootstrapOnlyAccessPath::fixed_bootstrap_access_path(),
            discovered,
            current_root,
        )
        .unwrap()
        .1
}

fn published_layout(
    advance_root: bool,
) -> worth_store_physical_format::PlatformPhysicalRootPublicationReport {
    let mut runtime = PhysicalStoreRuntime::open_physical_format(
        readiness(),
        PlatformPhysicalOpenRequest::physical_format_canonical(),
    )
    .unwrap();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    runtime
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            generations
                .slot_cell(segment(7), page(3), slot(1))
                .with_slot_generation(generation(2)),
            b"small",
        ))
        .unwrap();
    runtime
        .append_physical_record(PlatformPhysicalAppendRequest::extent(
            generations
                .extent_cell(segment(7), PhysicalExtentId::from_raw(20).unwrap())
                .with_extent_generation(generation(3)),
            b"large",
        ))
        .unwrap();
    let published = runtime.publish_physical_root().unwrap();
    if advance_root {
        runtime.publish_physical_root().unwrap()
    } else {
        published
    }
}

fn readiness() -> AcceptedHandoffReadiness {
    let digest = |value| StableDigest::new(format!("sha256:{value}")).unwrap();
    AcceptedHandoffReadiness::from_foundational_handoff_artifacts(
        ROADMAP_2_S1_SCOPE,
        HandoffEvidenceDigestSet::new(
            digest("backend"),
            digest("deferred"),
            digest("harness"),
            digest("terms"),
            digest("audit"),
            digest("complexity"),
            digest("provenance"),
        ),
    )
    .unwrap()
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}
fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}
fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}
fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
