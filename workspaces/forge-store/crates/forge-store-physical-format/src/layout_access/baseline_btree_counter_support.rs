use crate::{
    ExtentGenerationCell, PhysicalExtentId, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalPageId, PhysicalRecordSlot, PhysicalSegmentId, PlatformPhysicalAppendReport,
    PlatformPhysicalAppendRequest, PlatformPhysicalFacade,
};
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};

use crate::layout_access::baseline_btree_node_codec::encode_leaf_record;

pub(super) fn open_facade() -> PlatformPhysicalFacade {
    PlatformPhysicalFacade::open_s1(
        readiness(),
        crate::PlatformPhysicalOpenRequest::s1_canonical(),
    )
    .expect("open S.1 physical facade")
}

pub(super) fn readiness() -> AcceptedHandoffReadiness {
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

pub(super) fn root_slot_cell() -> crate::SlotGenerationCell {
    PhysicalGenerationAuthority::s1()
        .slot_cell(segment(7), page(9), slot(1))
        .with_slot_generation(generation(17))
}

pub(super) fn left_slot_cell() -> crate::SlotGenerationCell {
    PhysicalGenerationAuthority::s1()
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(11))
}

pub(super) fn right_slot_cell() -> crate::SlotGenerationCell {
    PhysicalGenerationAuthority::s1()
        .slot_cell(segment(7), page(13), slot(1))
        .with_slot_generation(generation(13))
}

pub(super) fn left_leaf_slots() -> [PhysicalRecordSlot; 2] {
    [slot(10), slot(11)]
}

pub(super) fn right_leaf_slots() -> [PhysicalRecordSlot; 2] {
    [slot(12), slot(13)]
}

pub(super) fn separator_slot() -> PhysicalRecordSlot {
    slot(12)
}

pub(super) fn append_leaf(
    facade: &mut PlatformPhysicalFacade,
    slot_cell: crate::SlotGenerationCell,
    slots: [PhysicalRecordSlot; 2],
    sibling_links_present: bool,
    tombstones_present: bool,
) -> PlatformPhysicalAppendReport {
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            slot_cell,
            &encode_leaf_record(slots, sibling_links_present, tombstones_present),
        ))
        .expect("baseline B-tree leaf append")
}

#[allow(dead_code)]
pub(super) fn extent_cell() -> ExtentGenerationCell {
    PhysicalGenerationAuthority::s1()
        .extent_cell(segment(1), PhysicalExtentId::from_raw(1).unwrap())
        .with_extent_generation(generation(7))
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

pub(super) fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
