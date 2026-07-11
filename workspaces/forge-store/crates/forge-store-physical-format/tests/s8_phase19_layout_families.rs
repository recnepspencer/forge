use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_layout_indexes::layout_strategy_admission::{
    phase19_extent_rule, phase19_frame_rule, phase19_page_rule, phase19_segment_rule,
};
use forge_store_physical_format::{
    layout_access::{
        extent_family::extent_layout_access_counters, page_family::page_layout_access_counters,
    },
    PhysicalExtentId, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
    PhysicalRecordSlot, PhysicalSegmentId, PlatformPhysicalAppendRequest, PlatformPhysicalFacade,
    PlatformPhysicalOpenRequest,
};

#[test]
fn phase19_page_and_frame_rules_drive_public_layout_access() {
    let mut facade = open_facade();
    let append = facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            slot_cell(1),
            b"frame-backed",
        ))
        .expect("page append through public facade");
    let page_rule = phase19_page_rule().expect("phase-19 page rule");
    let frame_rule = phase19_frame_rule().expect("phase-19 frame rule");

    let mut page_layout = facade
        .page_layout(&page_rule)
        .expect("public page layout admission");
    let located = page_layout
        .read_record(append.reference())
        .expect("public page read");
    assert_eq!(located.record_view().payload().as_bytes(), b"frame-backed");

    let mut frame_layout = facade
        .frame_layout(&frame_rule)
        .expect("public frame layout admission");
    let framed = frame_layout
        .read_frame(append.reference())
        .expect("public frame read");
    assert_eq!(framed.frame_view().payload().as_bytes(), b"frame-backed");
    assert_eq!(framed.counters().point_lookups(), 1);
}

#[test]
fn phase19_extent_and_reopen_follow_public_rule_witnesses() {
    let mut facade = open_facade();
    let append = facade
        .append_physical_record(PlatformPhysicalAppendRequest::extent(
            extent_cell(1),
            b"extent-backed",
        ))
        .expect("extent append through public facade");
    let page_append = facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            slot_cell(2),
            b"root-page",
        ))
        .expect("page append before publish");
    let extent_rule = phase19_extent_rule().expect("phase-19 extent rule");
    let page_rule = phase19_page_rule().expect("phase-19 page rule");

    let mut extent_layout = facade
        .extent_layout(&extent_rule)
        .expect("public extent layout admission");
    let extent = extent_layout
        .read_record(append.reference())
        .expect("public extent read");
    assert_eq!(extent.record_view().payload().as_bytes(), b"extent-backed");

    let published = facade.publish_physical_root().expect("public root publish");
    let mut reopened = PlatformPhysicalFacade::reopen_s1(
        readiness(),
        PlatformPhysicalOpenRequest::s1_canonical(),
        published.replay_artifact(),
    )
    .expect("public replay reopen");
    let mut reopened_page_layout = reopened
        .page_layout(&page_rule)
        .expect("public page layout admission after reopen");
    let reopened_page = reopened_page_layout
        .locate_record(page_append.reference())
        .expect("public page locate after reopen");
    assert_eq!(
        reopened_page.record_view().payload().as_bytes(),
        b"root-page"
    );
}

#[test]
fn phase19_segment_rule_uses_maintained_segment_occupancy() {
    let mut facade = open_facade();
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            slot_cell(3),
            b"seg-page",
        ))
        .expect("page append");
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::extent(
            extent_cell(3),
            b"seg-extent",
        ))
        .expect("extent append");
    let segment_rule = phase19_segment_rule().expect("phase-19 segment rule");

    let report = facade
        .segment_layout(&segment_rule)
        .expect("public segment layout admission")
        .read_segment(segment(1))
        .expect("public segment read");

    assert_eq!(report.page_slots(), 1);
    assert_eq!(report.extents(), 1);
    assert_eq!(report.counters().point_lookups(), 1);
    assert_eq!(report.counters().page_touches(), 0);
    assert_eq!(report.counters().bytes_read(), 0);
}

#[test]
fn phase19_point_counters_do_not_scale_with_storage_cardinality() {
    let mut facade = open_facade();
    for value in 1..=8 {
        facade
            .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
                slot_cell_for(page(value), slot(value as u16)),
                b"page-fill",
            ))
            .expect("page append");
        facade
            .append_physical_record(PlatformPhysicalAppendRequest::extent(
                extent_cell_for(extent(value)),
                b"extent-fill",
            ))
            .expect("extent append");
    }
    let page_rule = phase19_page_rule().expect("phase-19 page rule");
    let extent_rule = phase19_extent_rule().expect("phase-19 extent rule");
    let page_reference = forge_store_physical_format::PhysicalReferenceAuthority::s1()
        .admit_page_slot(slot_cell_for(page(8), slot(8)))
        .reference();
    let extent_reference = forge_store_physical_format::PhysicalReferenceAuthority::s1()
        .admit_extent(extent_cell_for(extent(8)))
        .reference();

    let page_counters = {
        let mut page_layout = facade
            .page_layout(&page_rule)
            .expect("public page layout admission");
        let report = page_layout
            .read_record(page_reference)
            .expect("public page read");
        page_layout_access_counters(report)
    };
    let extent_counters = {
        let mut extent_layout = facade
            .extent_layout(&extent_rule)
            .expect("public extent layout admission");
        let report = extent_layout
            .read_record(extent_reference)
            .expect("public extent read");
        extent_layout_access_counters(report)
    };

    assert_eq!(page_counters.point_lookups(), 1);
    assert_eq!(page_counters.page_touches(), 1);
    assert_eq!(page_counters.index_probes(), 2);
    assert_eq!(page_counters.bytes_read(), 4_096);
    assert_eq!(extent_counters.point_lookups(), 1);
    assert_eq!(extent_counters.page_touches(), 1);
    assert_eq!(extent_counters.index_probes(), 2);
    assert_eq!(extent_counters.bytes_read(), 4_096);
}

fn open_facade() -> PlatformPhysicalFacade {
    PlatformPhysicalFacade::open_s1(readiness(), PlatformPhysicalOpenRequest::s1_canonical())
        .expect("open S.1 facade")
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

fn slot_cell(value: u16) -> forge_store_physical_format::SlotGenerationCell {
    slot_cell_for(page(1), slot(value))
}

fn slot_cell_for(
    page_id: PhysicalPageId,
    slot_id: PhysicalRecordSlot,
) -> forge_store_physical_format::SlotGenerationCell {
    PhysicalGenerationAuthority::s1()
        .slot_cell(segment(1), page_id, slot_id)
        .with_slot_generation(generation(5))
}

fn extent_cell(value: u64) -> forge_store_physical_format::ExtentGenerationCell {
    extent_cell_for(extent(value))
}

fn extent_cell_for(
    extent_id: PhysicalExtentId,
) -> forge_store_physical_format::ExtentGenerationCell {
    PhysicalGenerationAuthority::s1()
        .extent_cell(segment(1), extent_id)
        .with_extent_generation(generation(7))
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn extent(value: u64) -> PhysicalExtentId {
    PhysicalExtentId::from_raw(value).unwrap()
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
