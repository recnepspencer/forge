use worth_store_contracts::{
    AcceptedHandoffReadiness, DurableArtifactFamilyId, HandoffEvidenceDigestSet, StableDigest,
    ROADMAP_2_S1_SCOPE,
};
use worth_store_layout_indexes::{
    bootstrap_catalog, declarations::layout_declarations, BootstrapOnlyAccessDenied,
    BootstrapOnlyAccessPath,
};
use worth_store_physical_certification::{
    FixtureCapabilityDeclaration, FixtureMutationBoundary, LargeStoreFixtureProfile,
    PhysicalFixtureBuilder, ProductionBackedFixtureMaterialization,
};
use worth_store_physical_format::{
    physical_bootstrap_catalog, PhysicalExtentId, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalPageId, PhysicalRecordSlot, PhysicalRootReference, PhysicalSegmentId,
    PhysicalStoreRuntime, PlatformPhysicalAppendRequest, PlatformPhysicalOpenRequest,
};

#[test]
fn bootstrap_catalog_admits_minimal_root_discovery_and_typed_read_access() {
    let published = published_layout();
    let open = published
        .admit_bootstrap_open_witness()
        .expect("published layout should admit bootstrap open witness");
    let physical_catalog = physical_bootstrap_catalog()
        .discover_catalog(&open)
        .expect("physical bootstrap catalog should derive");

    let outcome = bootstrap_catalog().read_catalog(
        BootstrapOnlyAccessPath::fixed_bootstrap_access_path(),
        physical_catalog.clone(),
        physical_catalog.current_root(),
    );
    let counters = outcome.counters();
    let (catalog, admission) = outcome.expect("bootstrap facade should admit catalog");

    assert_eq!(
        catalog.root_reference(),
        PhysicalRootReference::from_raw(1).unwrap()
    );
    assert_eq!(
        catalog.discovery_layout().physical_format_version(),
        worth_store_physical_format::PhysicalFormatVersion::initial_format_version()
    );
    assert_eq!(
        catalog.discovery_layout().checksum_bytes_checked(),
        physical_catalog.checksum().bytes_checked()
    );
    assert_eq!(catalog.root_entry_count(), 4);
    assert_eq!(catalog.segment_count(), 1);
    assert_eq!(catalog.page_slot_count(), 1);
    assert_eq!(catalog.extent_count(), 1);
    assert_eq!(catalog.allocation_class_count(), 2);
    assert_eq!(catalog.free_space_count(), 0);
    assert_eq!(catalog.layout_entry_count(), 5);
    assert_eq!(counters.catalog_candidates_read(), 1);
    assert_eq!(
        counters.checksum_bytes_verified(),
        physical_catalog.checksum().bytes_checked()
    );
    assert_eq!(counters.root_entries_read(), 4);
    assert_eq!(counters.layout_entries_read(), 5);
    assert_eq!(counters.admitted_catalogs(), 1);

    assert_eq!(
        admission.physical_format_version(),
        catalog.physical_format_version()
    );
    assert_eq!(
        admission.root_owner(),
        physical_catalog.current_root().root_owner()
    );
}

#[test]
fn bootstrap_only_lane_denies_ordinary_family_access() {
    let denial = bootstrap_catalog().deny_ordinary_family_access(
        layout_declarations()
            .declaration(DurableArtifactFamilyId::PhysicalPage)
            .expect("physical page family must remain declared")
            .family(),
    );

    assert!(matches!(
        denial,
        BootstrapOnlyAccessDenied::OrdinaryFamilyAccessForbidden { .. }
    ));
}

#[test]
fn mismatched_current_root_readmission_is_rejected() {
    let published = published_layout();
    let open = published
        .admit_bootstrap_open_witness()
        .expect("published layout should admit bootstrap open witness");
    let physical_catalog = physical_bootstrap_catalog()
        .discover_catalog(&open)
        .expect("physical bootstrap catalog should derive");
    let other_published = republished_layout();
    let other_open = other_published
        .admit_bootstrap_open_witness()
        .expect("other published layout should admit bootstrap open witness");
    let other_catalog = physical_bootstrap_catalog()
        .discover_catalog(&other_open)
        .expect("other physical bootstrap catalog should derive");

    let denial = bootstrap_catalog()
        .read_catalog(
            BootstrapOnlyAccessPath::fixed_bootstrap_access_path(),
            physical_catalog,
            other_catalog.current_root(),
        )
        .expect_err("stale or imported root admission cannot unlock ordinary bootstrap access");

    assert!(matches!(
        denial,
        BootstrapOnlyAccessDenied::CurrentRootReadmissionRequired { .. }
    ));
}

#[test]
fn bootstrap_admission_cannot_unlock_a_different_same_version_catalog() {
    let published = published_layout();
    let open = published
        .admit_bootstrap_open_witness()
        .expect("first published layout should admit bootstrap open witness");
    let first_catalog = physical_bootstrap_catalog()
        .discover_catalog(&open)
        .expect("first physical bootstrap catalog should derive");
    let (_, first_admission) = bootstrap_catalog()
        .read_catalog(
            BootstrapOnlyAccessPath::fixed_bootstrap_access_path(),
            first_catalog,
            physical_bootstrap_catalog()
                .discover_catalog(&open)
                .expect("first bootstrap replay should derive")
                .current_root(),
        )
        .expect("first bootstrap facade should admit catalog");

    let other_published = republished_layout();
    let other_open = other_published
        .admit_bootstrap_open_witness()
        .expect("second published layout should admit bootstrap open witness");
    let other_catalog = physical_bootstrap_catalog()
        .discover_catalog(&other_open)
        .expect("second physical bootstrap catalog should derive");
    let (_, other_admission) = bootstrap_catalog()
        .read_catalog(
            BootstrapOnlyAccessPath::fixed_bootstrap_access_path(),
            other_catalog,
            physical_bootstrap_catalog()
                .discover_catalog(&other_open)
                .expect("second bootstrap replay should derive")
                .current_root(),
        )
        .expect("second bootstrap facade should admit catalog");
    assert_ne!(first_admission, other_admission);
}

#[test]
fn bootstrap_catalog_replays_stably_across_certification_replay() {
    let published = published_layout();
    let published_physical_catalog = physical_bootstrap_catalog()
        .discover_catalog(
            &published
                .admit_bootstrap_open_witness()
                .expect("published layout should admit bootstrap open witness"),
        )
        .expect("physical bootstrap catalog should derive");
    let (published_catalog, published_admission) = bootstrap_catalog()
        .read_catalog(
            BootstrapOnlyAccessPath::fixed_bootstrap_access_path(),
            published_physical_catalog.clone(),
            published_physical_catalog.current_root(),
        )
        .expect("published bootstrap catalog should admit");
    let certification_materialization =
        ProductionBackedFixtureMaterialization::from_replay_artifact(
            LargeStoreFixtureProfile::CheckpointHeavy,
            published_physical_catalog.root_reference().get(),
            published.replay_artifact(),
        )
        .expect("published layout should materialize for certification replay");
    let fixture = PhysicalFixtureBuilder::production_backed("bootstrap-certification")
        .materialize_with(certification_materialization)
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::Manifest,
        ))
        .and_reopen_through_physical_authority()
        .expect("certification fixture should replay through physical authority");
    let certification_replay_artifact = fixture
        .replay_artifact()
        .expect("production-backed certification replay should preserve the native replay artifact")
        .clone();
    let reopened = PhysicalStoreRuntime::reopen(
        readiness(),
        PlatformPhysicalOpenRequest::physical_format_canonical(),
        certification_replay_artifact.clone(),
    )
    .expect("certification replay layout should reopen through physical authority");
    let certification_physical_catalog = physical_bootstrap_catalog()
        .discover_catalog(
            &certification_replay_artifact
                .admit_bootstrap_open_witness()
                .expect("certification replay should admit bootstrap witness"),
        )
        .expect("certification replay should derive canonical bootstrap catalog");
    let (certification_catalog, certification_admission) = bootstrap_catalog()
        .read_catalog(
            BootstrapOnlyAccessPath::fixed_bootstrap_access_path(),
            certification_physical_catalog.clone(),
            certification_physical_catalog.current_root(),
        )
        .expect("certification replay bootstrap catalog should admit");

    assert!(fixture
        .authority_receipt()
        .reopened_through_physical_authority());
    assert_eq!(fixture.manifest().source().root_reference(), 1);
    assert_eq!(
        published_physical_catalog.identity(),
        certification_physical_catalog.identity()
    );
    assert_eq!(published_catalog, certification_catalog);
    assert_eq!(published_admission, certification_admission);
    assert_eq!(reopened.counters().reopens(), 1);
}

fn published_layout() -> worth_store_physical_format::PlatformPhysicalRootPublicationReport {
    let mut facade = PhysicalStoreRuntime::open_physical_format(
        readiness(),
        PlatformPhysicalOpenRequest::physical_format_canonical(),
    )
    .expect("open S.1 facade");
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
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
    facade
        .publish_physical_root()
        .expect("publish physical root")
}

fn republished_layout() -> worth_store_physical_format::PlatformPhysicalRootPublicationReport {
    let mut facade = PhysicalStoreRuntime::open_physical_format(
        readiness(),
        PlatformPhysicalOpenRequest::physical_format_canonical(),
    )
    .expect("open S.1 facade");
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            generations
                .slot_cell(segment(7), page(3), slot(1))
                .with_slot_generation(generation(2)),
            b"small",
        ))
        .expect("append initial page-backed record");
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::extent(
            generations
                .extent_cell(segment(7), PhysicalExtentId::from_raw(20).unwrap())
                .with_extent_generation(generation(3)),
            b"large",
        ))
        .expect("append initial extent-backed record");
    facade
        .publish_physical_root()
        .expect("publish initial physical root");
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            generations
                .slot_cell(segment(8), page(4), slot(2))
                .with_slot_generation(generation(4)),
            b"second",
        ))
        .expect("append second page-backed record");
    facade
        .publish_physical_root()
        .expect("republish physical root")
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

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
