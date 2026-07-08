use super::super::baseline_btree_node_codec::{
    decode_leaf_record, decode_root_record, encode_root_record, BaselineBTreeCorruptionMarker,
};
use super::baseline_btree_counter_support::{
    append_leaf, left_leaf_slots, left_slot_cell, open_facade, right_leaf_slots, right_slot_cell,
    root_slot_cell, separator_slot, slot,
};
use super::{
    reopen_facade, BaselineBTreeCounterObservation, BaselineBTreeExecutionTranscript,
    BaselineBTreeExecutionWitness, BaselineBTreeLookupBranch, BaselineBTreeLookupExecution,
    BaselineBTreeReplayRecoveryExecution, BaselineBTreeRootPublicationExecution,
};
use crate::{PersistedPhysicalLayout, PhysicalReferenceAuthority, PlatformPhysicalAppendRequest};

impl BaselineBTreeExecutionWitness {
    pub fn seeded() -> Self {
        let mut facade = open_facade();
        let _left = append_leaf(
            &mut facade,
            left_slot_cell(),
            left_leaf_slots(),
            false,
            false,
        );
        let _right = append_leaf(
            &mut facade,
            right_slot_cell(),
            right_leaf_slots(),
            false,
            false,
        );
        let root = facade
            .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
                root_slot_cell(),
                &encode_root_record(
                    BaselineBTreeCorruptionMarker::Header,
                    separator_slot(),
                    left_slot_cell(),
                    right_slot_cell(),
                ),
            ))
            .expect("baseline B-tree root node append");
        let published = facade
            .publish_physical_root()
            .expect("baseline B-tree root publication");

        Self {
            root_reference: root.reference(),
            published_layout: published.persisted_layout().clone(),
        }
    }

    pub fn execute_separator_directed_lookup(
        &self,
        probe_slot: crate::PhysicalRecordSlot,
    ) -> BaselineBTreeLookupExecution {
        let mut facade = reopen_facade(self.published_layout.clone());
        let root = facade
            .read_physical_record(self.root_reference)
            .expect("baseline B-tree root read");
        let node = decode_root_record(root.framed_record().payload().as_bytes())
            .expect("baseline B-tree root payload");
        let branch = if probe_slot.get() < node.separator_slot.get() {
            BaselineBTreeLookupBranch::Left
        } else {
            BaselineBTreeLookupBranch::Right
        };
        let selected_cell = match branch {
            BaselineBTreeLookupBranch::Left => node.left_child,
            BaselineBTreeLookupBranch::Right => node.right_child,
        };
        let selected_reference = PhysicalReferenceAuthority::s1()
            .admit_page_slot(selected_cell)
            .reference();
        let _ = facade
            .locate_physical_record(selected_reference)
            .expect("baseline B-tree locate through selected branch");
        let selected_leaf = facade
            .read_physical_record(selected_reference)
            .expect("baseline B-tree read through selected branch");
        let leaf = decode_leaf_record(selected_leaf.framed_record().payload().as_bytes())
            .expect("baseline B-tree leaf payload");
        assert!(leaf.slots.iter().any(|slot| *slot == probe_slot));
        BaselineBTreeLookupExecution::new(
            probe_slot,
            node.separator_slot,
            branch,
            selected_reference,
            BaselineBTreeCounterObservation::new(1, 1, 0, 0),
        )
    }

    pub fn execute_root_publication(self) -> BaselineBTreeRootPublicationExecution {
        let mut facade = open_facade();
        let _ = append_leaf(
            &mut facade,
            left_slot_cell(),
            left_leaf_slots(),
            false,
            false,
        );
        let _ = append_leaf(
            &mut facade,
            right_slot_cell(),
            right_leaf_slots(),
            false,
            false,
        );
        let root_payload = encode_root_record(
            BaselineBTreeCorruptionMarker::Header,
            separator_slot(),
            left_slot_cell(),
            right_slot_cell(),
        );
        let root = facade
            .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
                root_slot_cell(),
                &root_payload,
            ))
            .expect("append for root publication");
        let published = facade
            .publish_physical_root()
            .expect("baseline B-tree root publication");
        BaselineBTreeRootPublicationExecution::new(
            published,
            root.reference(),
            root_payload,
            left_slot_cell(),
            right_slot_cell(),
        )
    }

    pub fn execute_replay_recovery(&self) -> BaselineBTreeReplayRecoveryExecution {
        let mut facade = reopen_facade(self.published_layout.clone());
        let root = facade
            .read_physical_record(self.root_reference)
            .expect("baseline B-tree replay root read");
        let node = decode_root_record(root.framed_record().payload().as_bytes())
            .expect("baseline B-tree replay root payload");
        let left_payload = facade
            .read_physical_record(
                PhysicalReferenceAuthority::s1()
                    .admit_page_slot(node.left_child)
                    .reference(),
            )
            .expect("baseline B-tree replay left leaf")
            .framed_record()
            .payload()
            .as_bytes()
            .to_vec();
        let right_payload = facade
            .read_physical_record(
                PhysicalReferenceAuthority::s1()
                    .admit_page_slot(node.right_child)
                    .reference(),
            )
            .expect("baseline B-tree replay right leaf")
            .framed_record()
            .payload()
            .as_bytes()
            .to_vec();
        let authority_records = decode_leaf_record(left_payload.as_slice())
            .expect("baseline B-tree replay left payload")
            .slots
            .len()
            + decode_leaf_record(right_payload.as_slice())
                .expect("baseline B-tree replay right payload")
                .slots
                .len();
        BaselineBTreeReplayRecoveryExecution::new(
            self.published_layout.clone(),
            self.root_reference,
            self.published_layout.root_manifest_candidates()[0].clone(),
            authority_records as u16,
            authority_records as u16,
            self.published_layout.root_manifest_candidates().len() == 1,
        )
    }

    pub const fn root_reference(&self) -> crate::PhysicalReference {
        self.root_reference
    }

    pub fn published_layout(&self) -> &PersistedPhysicalLayout {
        &self.published_layout
    }
}

pub fn execute_baseline_btree_lookup() -> BaselineBTreeLookupExecution {
    BaselineBTreeExecutionWitness::seeded().execute_separator_directed_lookup(slot(11))
}

pub fn execute_baseline_btree_root_publication() -> BaselineBTreeRootPublicationExecution {
    BaselineBTreeExecutionWitness::seeded().execute_root_publication()
}

pub fn execute_baseline_btree_replay_recovery() -> BaselineBTreeReplayRecoveryExecution {
    BaselineBTreeExecutionWitness::seeded().execute_replay_recovery()
}

pub fn execute_baseline_btree_transcript() -> BaselineBTreeExecutionTranscript {
    let lookup_witness = BaselineBTreeExecutionWitness::seeded();
    let publication_witness = BaselineBTreeExecutionWitness::seeded();
    let recovery_witness = BaselineBTreeExecutionWitness::seeded();
    BaselineBTreeExecutionTranscript::new(
        lookup_witness.execute_separator_directed_lookup(slot(11)),
        publication_witness.execute_root_publication(),
        recovery_witness.execute_replay_recovery(),
    )
}
