use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_layout_indexes::{
    bootstrap_catalog, BootstrapCatalogReadAdmission, BootstrapOnlyAccessPath,
};
use forge_store_physical_format::{
    physical_bootstrap_catalog, PhysicalExtentId, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalPageId, PhysicalRecordSlot, PhysicalSegmentId, PlatformPhysicalAppendRequest,
    PlatformPhysicalFacade, PlatformPhysicalOpenRequest,
};

pub fn admitted_layout_bootstrap_catalog() -> BootstrapCatalogReadAdmission {
    let published = published_layout();
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

pub fn open_layout_physical_facade() -> PlatformPhysicalFacade {
    open_layout_physical_facade_for_store(
        &forge_store_physical_format::PhysicalStoreIdentity::physical_format_default(),
    )
}

pub fn open_layout_physical_facade_for_store(
    store_identity: &forge_store_physical_format::PhysicalStoreIdentity,
) -> PlatformPhysicalFacade {
    PlatformPhysicalFacade::open_physical_format(
        readiness(),
        PlatformPhysicalOpenRequest::physical_format_for_store(store_identity.clone()),
    )
    .expect("open layout physical fixture")
}

pub fn foreign_layout_physical_store_identity() -> forge_store_physical_format::PhysicalStoreIdentity
{
    let key = forge_foundational::aspects()
        .vocabulary()
        .key("store.physical.foreign_instance")
        .unwrap();
    forge_store_physical_format::PhysicalStoreIdentity::from_aspect_identity(
        forge_store_aspect_native::StoreAspectIdentity::from_aspect_key(key),
    )
}

fn published_layout() -> forge_store_physical_format::PlatformPhysicalRootPublicationReport {
    let mut facade = PlatformPhysicalFacade::open_physical_format(
        readiness(),
        PlatformPhysicalOpenRequest::physical_format_canonical(),
    )
    .unwrap();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            generations
                .slot_cell(segment(7), page(3), slot(1))
                .with_slot_generation(generation(2)),
            b"small",
        ))
        .unwrap();
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::extent(
            generations
                .extent_cell(segment(7), PhysicalExtentId::from_raw(20).unwrap())
                .with_extent_generation(generation(3)),
            b"large",
        ))
        .unwrap();
    facade.publish_physical_root().unwrap()
}

fn readiness() -> AcceptedHandoffReadiness {
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

fn digest(value: &str) -> StableDigest {
    StableDigest::new(format!("sha256:{value}")).unwrap()
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
