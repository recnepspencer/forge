use crate::{
    physical_bootstrap_catalog, PhysicalBootstrapCatalogDenial, PhysicalBootstrapCatalogOpenWitness,
    PhysicalExtentId, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
    PhysicalRecordSlot, PhysicalRootReference, PhysicalSegmentId,
    PlatformPhysicalAppendRequest, PlatformPhysicalFacade, PlatformPhysicalOpenRequest,
};
use worth_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};

#[test]
fn bootstrap_catalog_discovery_is_tiny_fixed_versioned_and_checksummed() {
    let published = published_layout();
    let open = published
        .admit_bootstrap_open_witness()
        .expect("persisted layout should admit bootstrap open witness");
    let catalog = physical_bootstrap_catalog()
        .discover_catalog(&open)
        .expect("bootstrap catalog should derive from current root");

    assert_eq!(catalog.root_reference(), root_ref(1));
    assert_eq!(catalog.physical_format_version(), crate::PhysicalFormatVersion::s1_initial());
    assert_eq!(catalog.root_entry_count(), 4);
    assert_eq!(catalog.segment_count(), 1);
    assert_eq!(catalog.page_slot_count(), 1);
    assert_eq!(catalog.extent_count(), 1);
    assert_eq!(catalog.allocation_class_count(), 2);
    assert_eq!(catalog.free_space_count(), 0);
    assert!(catalog.checksum().bytes_checked() > 0);
}

#[test]
fn corrupted_bootstrap_bytes_are_denied_before_catalog_admission() {
    let published = published_layout();
    let layout = published.persisted_layout();
    let mut builder = crate::PersistedPhysicalLayout::builder();
    for root in layout.root_manifest_candidates() {
        builder = builder.root_manifest(root.clone());
    }
    let mut corrupted = layout.segment_manifest().to_vec();
    corrupted[0] ^= 0x01;
    builder = builder
        .segment_manifest(corrupted)
        .extent_manifest(layout.extent_manifest().to_vec())
        .free_space_map(layout.free_space_map().to_vec());
    for page in layout.pages() {
        builder = builder.page(page.clone());
    }
    for extent in layout.extents() {
        builder = builder.extent(extent.clone());
    }
    let open = PhysicalBootstrapCatalogOpenWitness::admit_persisted_layout(
        PlatformPhysicalOpenRequest::s1_canonical().headers(),
        &builder.build(),
    )
    .expect("corrupted persisted layout still admits open witness before decode");

    assert!(matches!(
        physical_bootstrap_catalog().discover_catalog(&open),
        Err(PhysicalBootstrapCatalogDenial::ManifestDecodeDenied(_))
    ));
}

#[test]
fn bootstrap_identity_replays_stably_across_publish_and_reopen() {
    let published = published_layout();
    let open = published
        .admit_bootstrap_open_witness()
        .expect("published layout should admit bootstrap open witness");
    let first = physical_bootstrap_catalog()
        .discover_catalog(&open)
        .expect("first bootstrap replay should derive");
    let mut reopened = PlatformPhysicalFacade::reopen_s1(
        readiness(),
        PlatformPhysicalOpenRequest::s1_canonical(),
        published.replay_artifact(),
    )
    .expect("reopen through verifier");
    let reopened_scan = reopened
        .scan_physical_layout()
        .expect("reopened layout should still scan");
    assert_eq!(
        reopened_scan.runtime_report().traversal().page_slot_count(),
        first.page_slot_count()
    );
    let second_open = published
        .replay_artifact()
        .admit_bootstrap_open_witness()
        .expect("reopened layout bytes should admit the same bootstrap witness");
    let second = physical_bootstrap_catalog()
        .discover_catalog(&second_open)
        .expect("second bootstrap replay should derive");

    assert_eq!(first.identity(), second.identity());
}

#[test]
fn bootstrap_identity_replays_stably_across_crash_recovery() {
    let mut facade = PlatformPhysicalFacade::open_s1(
        readiness(),
        PlatformPhysicalOpenRequest::s1_canonical(),
    )
    .expect("open S.1 facade");
    let generations = PhysicalGenerationAuthority::s1();
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            generations
                .slot_cell(segment(7), page(3), slot(1))
                .with_slot_generation(generation(2)),
            b"small",
        ))
        .expect("append page-backed record");
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::extent(
            generations
                .extent_cell(segment(7), PhysicalExtentId::from_raw(20).unwrap())
                .with_extent_generation(generation(3)),
            b"large",
        ))
        .expect("append extent-backed record");
    let durable = facade.publish_physical_root().expect("publish durable root");
    let durable_catalog = physical_bootstrap_catalog()
        .discover_catalog(
            &durable
                .admit_bootstrap_open_witness()
                .expect("durable layout should admit bootstrap witness"),
        )
        .expect("durable bootstrap should derive");
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            generations
                .slot_cell(segment(8), page(4), slot(2))
                .with_slot_generation(generation(4)),
            b"later",
        ))
        .expect("append later page-backed record");
    assert!(
        facade.publish_interrupted_physical_root().is_err(),
        "interrupted publish should simulate crash lane"
    );
    let recovered = PlatformPhysicalFacade::reopen_s1(
        readiness(),
        PlatformPhysicalOpenRequest::s1_canonical(),
        durable.replay_artifact(),
    )
    .expect("crash recovery should reopen last durable layout");
    let recovered_catalog = physical_bootstrap_catalog()
        .discover_catalog(
            &durable
                .replay_artifact()
                .admit_bootstrap_open_witness()
                .expect("recovered layout should admit bootstrap witness"),
        )
        .expect("recovered bootstrap should derive");

    assert_eq!(durable_catalog.identity(), recovered_catalog.identity());
}

fn published_layout() -> crate::PlatformPhysicalRootPublicationReport {
    let mut facade = PlatformPhysicalFacade::open_s1(
        readiness(),
        PlatformPhysicalOpenRequest::s1_canonical(),
    )
    .expect("open S.1 facade");
    let generations = PhysicalGenerationAuthority::s1();
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            generations
                .slot_cell(segment(7), page(3), slot(1))
                .with_slot_generation(generation(2)),
            b"small",
        ))
        .expect("append page-backed record");
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::extent(
            generations
                .extent_cell(segment(7), PhysicalExtentId::from_raw(20).unwrap())
                .with_extent_generation(generation(3)),
            b"large",
        ))
        .expect("append extent-backed record");
    facade.publish_physical_root().expect("publish physical root")
}

fn readiness() -> AcceptedHandoffReadiness {
    AcceptedHandoffReadiness::from_s0_artifacts(ROADMAP_2_S1_SCOPE, digest_set())
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
