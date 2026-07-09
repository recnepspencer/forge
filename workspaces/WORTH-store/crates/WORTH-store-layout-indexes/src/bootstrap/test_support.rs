use crate::{bootstrap_catalog, S8BootstrapOnlyAccessPath, S8LayoutMaterializationState};
use worth_store_physical_format::{
    physical_bootstrap_catalog, PhysicalExtentId, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalPageId, PhysicalRecordSlot, PhysicalRootReference, PhysicalSegmentId,
    PlatformPhysicalAppendRequest, PlatformPhysicalFacade, PlatformPhysicalOpenRequest,
};
use worth_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};

pub(crate) fn bootstrap_exact_materialization(
    family: crate::PhysicalArtifactFamily,
) -> S8LayoutMaterializationState {
    let published = published_layout();
    let open = published
        .admit_bootstrap_open_witness()
        .expect("bootstrap fixture layout should admit bootstrap open witness");
    let catalog = physical_bootstrap_catalog()
        .discover_catalog(&open)
        .expect("bootstrap fixture layout should discover a catalog");
    let (catalog, admission) = bootstrap_catalog()
        .read_catalog(
            S8BootstrapOnlyAccessPath::s8_fixed(),
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
        worth_store_physical_format::PhysicalFormatVersion::s1_initial()
    );
    assert_eq!(catalog.layout_entry_count(), 5);
    assert_eq!(admission.physical_format_version(), catalog.physical_format_version());

    crate::S8LayoutMaterializationState::exact_through_physical_basis(family)
}

fn published_layout() -> worth_store_physical_format::PlatformPhysicalRootPublicationReport {
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
