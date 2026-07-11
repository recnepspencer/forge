use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_layout_indexes::{
    encode_baseline_btree_leaf_record, encode_baseline_btree_root_record,
    BaselineBTreeCorruptionMarker, BaselineBTreeExecutionWitness,
};
use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalSegmentId, PlatformPhysicalAppendRequest, PlatformPhysicalFacade,
    PlatformPhysicalOpenRequest, SlotGenerationCell,
};

pub fn deterministic_baseline_btree_witness() -> BaselineBTreeExecutionWitness {
    let readiness = readiness();
    let mut facade = PlatformPhysicalFacade::open_physical_format(
        readiness.clone(),
        PlatformPhysicalOpenRequest::physical_format_canonical(),
    )
    .expect("open deterministic B-tree fixture");
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            left_slot_cell(),
            &encode_baseline_btree_leaf_record([slot(10), slot(11)], false, false),
        ))
        .expect("append left B-tree leaf");
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            right_slot_cell(),
            &encode_baseline_btree_leaf_record([slot(12), slot(13)], false, false),
        ))
        .expect("append right B-tree leaf");
    let root = facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            root_slot_cell(),
            &encode_baseline_btree_root_record(
                BaselineBTreeCorruptionMarker::Header,
                slot(12),
                left_slot_cell(),
                right_slot_cell(),
            ),
        ))
        .expect("append B-tree root");
    let published = facade.publish_physical_root().expect("publish B-tree root");
    BaselineBTreeExecutionWitness::admit_published_layout(
        readiness,
        root.reference(),
        published.replay_artifact(),
    )
    .expect("deterministic B-tree fixture must admit")
}

pub fn baseline_btree_probe_slot() -> PhysicalRecordSlot {
    slot(11)
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
    .expect("fixture readiness")
}

fn digest(name: &str) -> StableDigest {
    StableDigest::new(format!("sha256:{name}")).expect("fixture digest")
}

fn root_slot_cell() -> SlotGenerationCell {
    cell(9, 1, 17)
}

fn left_slot_cell() -> SlotGenerationCell {
    cell(11, 1, 11)
}

fn right_slot_cell() -> SlotGenerationCell {
    cell(13, 1, 13)
}

fn cell(page: u64, slot_value: u16, generation: u64) -> SlotGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment(7), page_id(page), slot(slot_value))
        .with_slot_generation(PhysicalGeneration::from_raw(generation).unwrap())
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}
fn page_id(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}
fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}
