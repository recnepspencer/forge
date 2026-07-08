use super::baseline_btree_counter_observation::{
    execute_baseline_btree_transcript, BaselineBTreeLookupBranch,
};
use super::baseline_btree_node_codec::{
    decode_leaf_record, decode_root_record, BaselineBTreeCorruptionMarker,
};
use crate::{
    PersistedPhysicalLayout, PhysicalRecordSlot, PlatformPhysicalFacade,
    PlatformPhysicalOpenRequest,
};
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_budgets::S8PreExecutionPlanBinding;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaselineBTreeCorruptionObservation {
    Header,
    CellPayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineBTreeLookupInvariantWitness {
    probe_precedes_separator: bool,
    left_max_precedes_separator: bool,
    separator_precedes_right_min: bool,
    branch: BaselineBTreeLookupBranch,
}

impl BaselineBTreeLookupInvariantWitness {
    pub const fn probe_precedes_separator(self) -> bool {
        self.probe_precedes_separator
    }
    pub const fn left_max_precedes_separator(self) -> bool {
        self.left_max_precedes_separator
    }
    pub const fn separator_precedes_right_min(self) -> bool {
        self.separator_precedes_right_min
    }
    pub const fn branch(self) -> BaselineBTreeLookupBranch {
        self.branch
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineBTreeMutationInvariantWitness {
    leaf_occupancy: u16,
    split_left_occupancy: u16,
    split_right_occupancy: u16,
    promoted_separator_between_halves: bool,
    sibling_links_present: bool,
    tombstones_present: bool,
    stable_generation: u64,
    corruption: BaselineBTreeCorruptionObservation,
}

impl BaselineBTreeMutationInvariantWitness {
    pub const fn leaf_occupancy(self) -> u16 {
        self.leaf_occupancy
    }
    pub const fn split_left_occupancy(self) -> u16 {
        self.split_left_occupancy
    }
    pub const fn split_right_occupancy(self) -> u16 {
        self.split_right_occupancy
    }
    pub const fn promoted_separator_between_halves(self) -> bool {
        self.promoted_separator_between_halves
    }
    pub const fn sibling_links_present(self) -> bool {
        self.sibling_links_present
    }
    pub const fn tombstones_present(self) -> bool {
        self.tombstones_present
    }
    pub const fn stable_generation(self) -> u64 {
        self.stable_generation
    }
    pub const fn corruption(self) -> BaselineBTreeCorruptionObservation {
        self.corruption
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineBTreePublicationInvariantWitness {
    root_generation_advanced: bool,
    checksum_scope_matches: bool,
    root_manifest_candidates: u16,
}

impl BaselineBTreePublicationInvariantWitness {
    pub const fn root_generation_advanced(self) -> bool {
        self.root_generation_advanced
    }
    pub const fn checksum_scope_matches(self) -> bool {
        self.checksum_scope_matches
    }
    pub const fn root_manifest_candidates(self) -> u16 {
        self.root_manifest_candidates
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineBTreeRecoveryInvariantWitness {
    replay_generation_monotonic: bool,
    manifest_advanced: bool,
    rebuild_authority_records: u16,
    rebuild_output_records: u16,
    rebuild_source_authoritative: bool,
}

impl BaselineBTreeRecoveryInvariantWitness {
    pub const fn replay_generation_monotonic(self) -> bool {
        self.replay_generation_monotonic
    }
    pub const fn manifest_advanced(self) -> bool {
        self.manifest_advanced
    }
    pub const fn rebuild_authority_records(self) -> u16 {
        self.rebuild_authority_records
    }
    pub const fn rebuild_output_records(self) -> u16 {
        self.rebuild_output_records
    }
    pub const fn rebuild_source_authoritative(self) -> bool {
        self.rebuild_source_authoritative
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineBTreeInvariantWitness {
    lookup: BaselineBTreeLookupInvariantWitness,
    mutation: BaselineBTreeMutationInvariantWitness,
    publication: BaselineBTreePublicationInvariantWitness,
    recovery: BaselineBTreeRecoveryInvariantWitness,
}

impl BaselineBTreeInvariantWitness {
    pub const fn lookup(self) -> BaselineBTreeLookupInvariantWitness {
        self.lookup
    }
    pub const fn mutation(self) -> BaselineBTreeMutationInvariantWitness {
        self.mutation
    }
    pub const fn publication(self) -> BaselineBTreePublicationInvariantWitness {
        self.publication
    }
    pub const fn recovery(self) -> BaselineBTreeRecoveryInvariantWitness {
        self.recovery
    }
}

pub fn collect_baseline_btree_invariant_witness() -> BaselineBTreeInvariantWitness {
    let transcript = execute_baseline_btree_transcript(S8PreExecutionPlanBinding::new(1, 2, 3, 4, 0));
    let left_lookup = transcript.lookup();
    let publication = transcript.publication();
    let recovery = transcript.recovery();
    let root = read_root(
        recovery.reopened_layout().clone(),
        publication.root_reference(),
    );
    let left_slots = read_leaf_slots(recovery.reopened_layout().clone(), root.left_child);
    let right_slots = read_leaf_slots(recovery.reopened_layout().clone(), root.right_child);

    BaselineBTreeInvariantWitness {
        lookup: BaselineBTreeLookupInvariantWitness {
            probe_precedes_separator: left_lookup.probe_slot().get() < root.separator_slot.get(),
            left_max_precedes_separator: left_slots.slots[1].get() < root.separator_slot.get(),
            separator_precedes_right_min: root.separator_slot.get() <= right_slots.slots[0].get(),
            branch: left_lookup.branch(),
        },
        mutation: BaselineBTreeMutationInvariantWitness {
            leaf_occupancy: left_slots.slots.len() as u16,
            split_left_occupancy: left_slots.slots.len() as u16,
            split_right_occupancy: right_slots.slots.len() as u16,
            promoted_separator_between_halves: left_slots.slots[1].get()
                < root.separator_slot.get()
                && root.separator_slot.get() <= right_slots.slots[0].get(),
            sibling_links_present: left_slots.sibling_links_present
                || right_slots.sibling_links_present,
            tombstones_present: left_slots.tombstones_present || right_slots.tombstones_present,
            stable_generation: publication.root_reference().generation().get(),
            corruption: match root.corruption_marker {
                BaselineBTreeCorruptionMarker::Header => BaselineBTreeCorruptionObservation::Header,
                BaselineBTreeCorruptionMarker::CellPayload => {
                    BaselineBTreeCorruptionObservation::CellPayload
                }
            },
        },
        publication: BaselineBTreePublicationInvariantWitness {
            root_generation_advanced: publication.root_generation_advanced(),
            checksum_scope_matches: publication.checksum_scope_matches(),
            root_manifest_candidates: publication.root_manifest_candidate_count(),
        },
        recovery: BaselineBTreeRecoveryInvariantWitness {
            replay_generation_monotonic: recovery.replay_generation_monotonic(),
            manifest_advanced: recovery.manifest_advanced(),
            rebuild_authority_records: recovery.rebuild_authority_records(),
            rebuild_output_records: recovery.rebuild_output_records(),
            rebuild_source_authoritative: recovery.rebuild_source_authoritative(),
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DecodedRoot {
    separator_slot: PhysicalRecordSlot,
    left_child: crate::SlotGenerationCell,
    right_child: crate::SlotGenerationCell,
    corruption_marker: BaselineBTreeCorruptionMarker,
    payload: Vec<u8>,
}

fn read_root(
    layout: PersistedPhysicalLayout,
    root_reference: crate::PhysicalReference,
) -> DecodedRoot {
    let mut facade = reopen_facade(layout);
    let root = facade
        .read_physical_record(root_reference)
        .expect("read baseline root");
    let payload = root.framed_record().payload().as_bytes().to_vec();
    let decoded = decode_root_record(payload.as_slice()).expect("decode baseline root");
    DecodedRoot {
        separator_slot: decoded.separator_slot,
        left_child: decoded.left_child,
        right_child: decoded.right_child,
        corruption_marker: decoded.corruption_marker,
        payload,
    }
}

fn read_leaf_slots(
    layout: PersistedPhysicalLayout,
    cell: crate::SlotGenerationCell,
) -> super::baseline_btree_node_codec::BaselineBTreeLeafRecord {
    let mut facade = reopen_facade(layout);
    let reference = crate::PhysicalReferenceAuthority::s1()
        .admit_page_slot(cell)
        .reference();
    let leaf = facade
        .read_physical_record(reference)
        .expect("read baseline leaf");
    decode_leaf_record(leaf.framed_record().payload().as_bytes()).expect("decode baseline leaf")
}

fn reopen_facade(layout: PersistedPhysicalLayout) -> PlatformPhysicalFacade {
    PlatformPhysicalFacade::reopen_s1(
        readiness(),
        PlatformPhysicalOpenRequest::s1_canonical(),
        crate::PlatformPhysicalReplayArtifact::from_persisted_layout(
            PlatformPhysicalOpenRequest::s1_canonical().headers().clone(),
            layout,
        ),
    )
    .expect("reopen baseline btree facade")
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

#[cfg(test)]
mod tests {
    use super::{collect_baseline_btree_invariant_witness, BaselineBTreeLookupBranch};

    #[test]
    fn baseline_btree_invariant_witness_carries_execution_owned_facts() {
        let witness = collect_baseline_btree_invariant_witness();
        assert_eq!(witness.lookup().branch(), BaselineBTreeLookupBranch::Left);
        assert_eq!(witness.mutation().leaf_occupancy(), 2);
        assert!(witness.publication().checksum_scope_matches());
        assert!(witness.recovery().rebuild_source_authoritative());
    }
}
