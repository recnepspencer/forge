use crate::{bootstrap_catalog, BootstrapOnlyAccessPath, LayoutMaterializationState};
use worth_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use worth_store_physical_format::{
    physical_bootstrap_catalog, InMemoryPhysicalFormatModel, InMemoryPhysicalFormatModelRequest,
    PhysicalExtentId, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
    PhysicalRecordSlot, PhysicalRootReference, PhysicalSegmentId, PhysicalStoreIdentity,
    PlatformPhysicalAppendRequest,
};

pub(crate) fn bootstrap_exact_materialization(
    family: crate::PhysicalArtifactFamily,
) -> LayoutMaterializationState {
    let _ = bootstrap_catalog_read_admission();
    crate::LayoutMaterializationState::exact_through_physical_basis(family)
}

pub(crate) fn bootstrap_catalog_read_admission() -> crate::BootstrapCatalogReadAdmission {
    catalog_read_admission(published_layout(false))
}

pub(crate) fn advanced_bootstrap_catalog_read_admission() -> crate::BootstrapCatalogReadAdmission {
    catalog_read_admission(published_layout(true))
}

fn catalog_read_admission(
    published: worth_store_physical_format::PlatformPhysicalRootPublicationReport,
) -> crate::BootstrapCatalogReadAdmission {
    let open = published
        .admit_bootstrap_open_witness()
        .expect("bootstrap fixture layout should admit bootstrap open witness");
    let catalog = physical_bootstrap_catalog()
        .discover_catalog(&open)
        .expect("bootstrap fixture layout should discover a catalog");
    let (catalog, admission) = bootstrap_catalog()
        .read_catalog(
            BootstrapOnlyAccessPath::fixed_bootstrap_access_path(),
            catalog,
            physical_bootstrap_catalog()
                .discover_catalog(&open)
                .expect("catalog should replay stably")
                .current_root(),
        )
        .expect("bootstrap fixture should admit catalog");
    assert_eq!(catalog.root_reference(), root_ref(1));
    assert_eq!(
        catalog.physical_format_version(),
        worth_store_physical_format::PhysicalFormatVersion::initial_format_version()
    );
    assert_eq!(catalog.layout_entry_count(), 5);
    assert_eq!(
        admission.physical_format_version(),
        catalog.physical_format_version()
    );
    admission
}

fn published_layout(
    advance_root: bool,
) -> worth_store_physical_format::PlatformPhysicalRootPublicationReport {
    let mut facade = open_physical_facade();
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            generations()
                .slot_cell(segment(7), page(3), slot(1))
                .with_slot_generation(generation(2)),
            b"small",
        ))
        .expect("append page-backed record");
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::extent(
            generations()
                .extent_cell(segment(7), PhysicalExtentId::from_raw(20).unwrap())
                .with_extent_generation(generation(3)),
            b"large",
        ))
        .expect("append extent-backed record");
    let published = facade
        .publish_physical_root()
        .expect("publish physical root");
    if advance_root {
        facade
            .publish_physical_root()
            .expect("advance physical root publication")
    } else {
        published
    }
}

pub(crate) fn open_physical_facade() -> InMemoryPhysicalFormatModel {
    open_physical_facade_for_store(PhysicalStoreIdentity::physical_format_default())
}

pub(crate) fn open_physical_facade_for_store(
    store_identity: PhysicalStoreIdentity,
) -> InMemoryPhysicalFormatModel {
    InMemoryPhysicalFormatModel::start_empty_model(
        readiness(),
        InMemoryPhysicalFormatModelRequest::physical_format_for_store(store_identity),
    )
    .expect("open physical facade")
}

fn generations() -> PhysicalGenerationAuthority {
    PhysicalGenerationAuthority::for_canonical_physical_format()
}

fn readiness() -> AcceptedHandoffReadiness {
    AcceptedHandoffReadiness::from_foundational_handoff_artifacts(ROADMAP_2_S1_SCOPE, digest_set())
        .expect("S.1 handoff readiness")
}

fn digest_set() -> HandoffEvidenceDigestSet {
    HandoffEvidenceDigestSet::new(
        digest("backend"),
        digest("deferred"),
        digest("harness"),
        digest("terms"),
        digest("audit"),
        digest("complexity"),
        digest("provenance"),
    )
}

fn digest(name: &str) -> StableDigest {
    StableDigest::new(format!("sha256:{name}")).expect("non-empty digest")
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

fn root_ref(value: u64) -> PhysicalRootReference {
    PhysicalRootReference::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
