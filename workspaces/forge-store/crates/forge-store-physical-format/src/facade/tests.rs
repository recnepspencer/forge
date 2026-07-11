use crate::{
    ExtentGenerationCell, PersistedPhysicalLayout, PhysicalExtentId, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalRecordSlot, PhysicalReferenceKind, PhysicalSegmentId,
    PlatformPhysicalAppendRequest, PlatformPhysicalFacade, PlatformPhysicalFacadeDenialKind,
    PlatformPhysicalLayoutAccessRequest, PlatformPhysicalOpenRequest, SlotGenerationCell,
};
use forge_store_budgets::S8PreExecutionPlanBinding;
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};

#[test]
fn facade_append_publish_scan_reopen_and_locate_stays_physical() {
    let mut facade = open_facade();
    let slot_cell = slot_cell();
    let append = facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            slot_cell, b"small",
        ))
        .expect("append through facade");

    assert_eq!(append.reference().kind(), PhysicalReferenceKind::PageSlot);

    let mut page_layout = facade.page_access();
    let located = page_layout
        .locate_record(append.reference())
        .expect("locate through facade");
    assert_eq!(located.record_view().payload().as_bytes(), b"small");

    let mut page_layout = facade.page_access();
    let read = page_layout
        .read_record(append.reference())
        .expect("read through facade");
    assert_eq!(read.record_view().payload().as_bytes(), b"small");
    let mut frame_layout = facade.frame_access();
    let framed = frame_layout
        .read_frame(append.reference())
        .expect("frame-admitted read through facade");
    assert_eq!(framed.frame_view().payload().as_bytes(), b"small");
    assert_eq!(facade.counters().locates(), 1);
    assert_eq!(facade.counters().reads(), 2);

    let published = facade
        .publish_physical_root()
        .expect("clean root publication");
    let scan = facade.scan_physical_layout().expect("verifier scan");
    assert!(scan.platform_evidence().proves_platform_boundary());
    assert_eq!(facade.counters().appends(), 1);
    assert_eq!(facade.counters().root_publications(), 1);
    assert_eq!(facade.counters().scans(), 1);

    let mut reopened = PlatformPhysicalFacade::reopen(
        readiness(),
        PlatformPhysicalOpenRequest::physical_format_canonical(),
        published.replay_artifact(),
    )
    .expect("reopen through verifier");
    let mut reopened_page_layout = reopened.page_access();
    let reopened_locate = reopened_page_layout
        .locate_record(append.reference())
        .expect("reopen locate by physical reference");
    assert_eq!(reopened_locate.record_view().payload().as_bytes(), b"small");
}

#[test]
fn hidden_broad_scan_is_rejected_before_physical_traversal_with_owner_receipt() {
    let mut facade = open_facade();
    let denial =
        facade.reject_hidden_broad_scan(PlatformPhysicalLayoutAccessRequest::hidden_broad_scan(
            S8PreExecutionPlanBinding::new(1, 2, 3, 4, 0),
        ));

    assert!(denial.is_owner_denial());
    assert_eq!(
        denial.request().plan_binding(),
        S8PreExecutionPlanBinding::new(1, 2, 3, 4, 0)
    );
    assert_eq!(denial.counters().scans(), 0);
    assert_eq!(denial.counters().full_store_materialization_rejections(), 1);
}

#[test]
fn reopen_rejects_ambiguous_root_candidates_without_guessing() {
    let mut facade = open_facade();
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            slot_cell(),
            b"small",
        ))
        .expect("append through facade");
    let published = facade
        .publish_physical_root()
        .expect("clean root publication");
    let layout = published.persisted_layout();
    let root = layout.root_manifest_candidates()[0].clone();
    let mut builder = PersistedPhysicalLayout::builder()
        .root_manifest(root.clone())
        .root_manifest(root)
        .segment_manifest(layout.segment_manifest().to_vec())
        .extent_manifest(layout.extent_manifest().to_vec())
        .free_space_map(layout.free_space_map().to_vec());
    for page in layout.pages() {
        builder = builder.page(page.clone());
    }
    for extent in layout.extents() {
        builder = builder.extent(extent.clone());
    }

    let denial = PlatformPhysicalFacade::reopen(
        readiness(),
        PlatformPhysicalOpenRequest::physical_format_canonical(),
        crate::PlatformPhysicalReplayArtifact::from_persisted_layout(
            PlatformPhysicalOpenRequest::physical_format_canonical()
                .headers()
                .clone(),
            builder.build(),
        ),
    )
    .expect_err("ambiguous persisted root candidates deny reopen");

    assert_eq!(
        denial.kind(),
        PlatformPhysicalFacadeDenialKind::AmbiguousRootPublication
    );
}

#[test]
fn extent_records_route_through_facade_as_peer_physical_placement() {
    let mut facade = open_facade();
    let extent_cell = extent_cell();
    let append = facade
        .append_physical_record(PlatformPhysicalAppendRequest::extent(extent_cell, b"large"))
        .expect("append extent-backed record");

    let mut extent_layout = facade.extent_access();
    let located = extent_layout
        .locate_record(append.reference())
        .expect("locate extent-backed record");
    assert_eq!(located.record_view().payload().as_bytes(), b"large");
}

#[test]
fn segment_records_route_through_admitted_segment_layout() {
    let mut facade = open_facade();
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            slot_cell(),
            b"small",
        ))
        .expect("append page-backed record");
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::extent(
            extent_cell(),
            b"large",
        ))
        .expect("append extent-backed record");

    let report = facade
        .segment_access()
        .read_segment(segment(1))
        .expect("read admitted segment");

    assert_eq!(report.segment_id(), segment(1));
    assert_eq!(report.page_slots(), 1);
    assert_eq!(report.extents(), 1);
    assert_eq!(report.counters().index_probes(), 1);
}

#[test]
fn interrupted_root_publication_denies_as_ambiguous_root() {
    let mut facade = open_facade();
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            slot_cell(),
            b"small",
        ))
        .expect("append through facade");

    let denial = facade
        .publish_interrupted_physical_root()
        .expect_err("interrupted publication is ambiguous");

    assert_eq!(
        denial.kind(),
        PlatformPhysicalFacadeDenialKind::AmbiguousRootPublication
    );
}

#[test]
fn forbidden_shortcuts_are_typed_and_counted() {
    let mut facade = open_facade();
    let materialization = facade
        .reject_full_store_heap_materialization()
        .expect_err("materialization shortcut is rejected");
    let residue = facade
        .reject_backend_residue_guess()
        .expect_err("backend residue shortcut is rejected");

    assert_eq!(
        materialization.kind(),
        PlatformPhysicalFacadeDenialKind::FullStoreMaterializationRejected
    );
    assert_eq!(
        residue.kind(),
        PlatformPhysicalFacadeDenialKind::BackendResidueGuessRejected
    );
    assert_eq!(facade.counters().appends(), 0);
    assert_eq!(facade.counters().reads(), 0);
    assert_eq!(facade.counters().locates(), 0);
    assert_eq!(facade.counters().root_publications(), 0);
    assert_eq!(facade.counters().scans(), 0);
    assert_eq!(facade.counters().full_store_materialization_rejections(), 1);
    assert_eq!(facade.counters().backend_residue_guess_rejections(), 1);
}

fn open_facade() -> PlatformPhysicalFacade {
    PlatformPhysicalFacade::open_physical_format(readiness(), PlatformPhysicalOpenRequest::physical_format_canonical())
        .expect("open S.1 facade")
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

fn slot_cell() -> SlotGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment(1), page(1), slot(1))
        .with_slot_generation(generation(5))
}

fn extent_cell() -> ExtentGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .extent_cell(segment(1), PhysicalExtentId::from_raw(1).unwrap())
        .with_extent_generation(generation(7))
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> crate::PhysicalPageId {
    crate::PhysicalPageId::from_raw(value).unwrap()
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
