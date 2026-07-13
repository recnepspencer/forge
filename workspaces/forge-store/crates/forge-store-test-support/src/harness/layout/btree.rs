use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_layout_indexes::{
    encode_baseline_btree_leaf_record, encode_baseline_btree_root_record,
    BaselineBTreeCorruptionMarker, BaselineBTreeExecutionWitness, BaselineBTreeReadPreflight,
};
use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReference, PhysicalSegmentId, PhysicalStoreIdentity, PlatformPhysicalAppendRequest,
    PlatformPhysicalFacade, PlatformPhysicalOpenRequest, PlatformPhysicalReplayArtifact,
    SlotGenerationCell,
};

use super::bootstrap::foreign_layout_physical_store_identity;

#[derive(Debug, Clone)]
pub struct DeterministicBTreeReplayWorld {
    readiness: AcceptedHandoffReadiness,
    root_reference: PhysicalReference,
    replay_artifact: PlatformPhysicalReplayArtifact,
}

impl DeterministicBTreeReplayWorld {
    pub const fn readiness(&self) -> &AcceptedHandoffReadiness {
        &self.readiness
    }
    pub const fn root_reference(&self) -> PhysicalReference {
        self.root_reference
    }
    pub const fn replay_artifact(&self) -> &PlatformPhysicalReplayArtifact {
        &self.replay_artifact
    }
}

pub fn deterministic_baseline_btree_read_preflight() -> BaselineBTreeReadPreflight {
    deterministic_btree_read_source(false, false, None)
}

pub fn deterministic_cross_store_btree_read_preflight() -> BaselineBTreeReadPreflight {
    deterministic_btree_read_source(false, false, Some(foreign_layout_physical_store_identity()))
}

pub fn deterministic_corrupt_leaf_btree_read_preflight() -> BaselineBTreeReadPreflight {
    deterministic_btree_read_source(true, false, None)
}

pub fn deterministic_stale_child_btree_read_preflight() -> BaselineBTreeReadPreflight {
    deterministic_btree_read_source(false, true, None)
}

fn deterministic_btree_read_source(
    corrupt_left_leaf: bool,
    stale_left_reference: bool,
    store_identity: Option<PhysicalStoreIdentity>,
) -> BaselineBTreeReadPreflight {
    let world = deterministic_btree_replay_world_with_damage(
        corrupt_left_leaf,
        stale_left_reference,
        store_identity,
    );
    BaselineBTreeExecutionWitness::admit_published_layout(
        world.readiness,
        world.root_reference,
        world.replay_artifact,
    )
    .expect("deterministic B-tree fixture must admit")
    .preflight_stable_read()
    .expect("published B-tree root must produce a bounded read preflight")
}

pub fn deterministic_btree_replay_world() -> DeterministicBTreeReplayWorld {
    deterministic_btree_replay_world_with_damage(false, false, None)
}

fn deterministic_btree_replay_world_with_damage(
    corrupt_left_leaf: bool,
    stale_left_reference: bool,
    store_identity: Option<PhysicalStoreIdentity>,
) -> DeterministicBTreeReplayWorld {
    let readiness = readiness();
    let open_request = store_identity.map_or_else(
        PlatformPhysicalOpenRequest::physical_format_canonical,
        PlatformPhysicalOpenRequest::physical_format_for_store,
    );
    let mut facade = PlatformPhysicalFacade::open_physical_format(readiness.clone(), open_request)
        .expect("open deterministic B-tree fixture");
    let left_payload = if corrupt_left_leaf {
        *b"broken"
    } else {
        encode_baseline_btree_leaf_record([slot(10), slot(11)], false, false)
    };
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            left_slot_cell(),
            &left_payload,
        ))
        .expect("append left B-tree leaf");
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            right_slot_cell(),
            &encode_baseline_btree_leaf_record([slot(12), slot(13)], false, false),
        ))
        .expect("append right B-tree leaf");
    let left_reference = if stale_left_reference {
        cell(11, 1, 99)
    } else {
        left_slot_cell()
    };
    let root = facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            root_slot_cell(),
            &encode_baseline_btree_root_record(
                BaselineBTreeCorruptionMarker::Header,
                slot(12),
                left_reference,
                right_slot_cell(),
            ),
        ))
        .expect("append B-tree root");
    let published = facade.publish_physical_root().expect("publish B-tree root");
    DeterministicBTreeReplayWorld {
        readiness,
        root_reference: root.reference(),
        replay_artifact: published.replay_artifact(),
    }
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
