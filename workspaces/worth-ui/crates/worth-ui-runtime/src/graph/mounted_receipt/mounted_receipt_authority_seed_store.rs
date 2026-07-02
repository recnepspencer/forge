use std::collections::BTreeMap;

use crate::declaration::stable_text_digest;
use crate::graph::{
    UiGraphMountedPostureRelationship, UiGraphMountedReceiptAuthoritySeed,
    UiGraphMountedReceiptSlot, UiGraphNodeIdentity, UiMountedReceiptIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphMountedReceiptReservation {
    mounted_receipt_identity: UiMountedReceiptIdentity,
    graph_node_identity: UiGraphNodeIdentity,
    authority_seed: UiGraphMountedReceiptAuthoritySeed,
    mounted_posture_relationship: UiGraphMountedPostureRelationship,
}

impl UiGraphMountedReceiptReservation {
    pub(crate) fn graph_owned_seed_slot(
        graph_node_identity: UiGraphNodeIdentity,
        authority_seed: UiGraphMountedReceiptAuthoritySeed,
    ) -> Self {
        Self {
            mounted_receipt_identity: UiMountedReceiptIdentity::graph_owned_seed_slot(graph_node_identity),
            graph_node_identity,
            authority_seed,
            mounted_posture_relationship: UiGraphMountedPostureRelationship::ReservedMountedAuthoritySlot,
        }
    }

    pub fn mounted_receipt_identity(self) -> UiMountedReceiptIdentity {
        self.mounted_receipt_identity
    }

    pub fn graph_node_identity(self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn authority_seed(self) -> UiGraphMountedReceiptAuthoritySeed {
        self.authority_seed
    }

    pub fn mounted_posture_relationship(self) -> UiGraphMountedPostureRelationship {
        self.mounted_posture_relationship
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphMountedReceiptAuthoritySeedStore {
    slots: Vec<UiGraphMountedReceiptSlot>,
    slot_lookup: BTreeMap<UiMountedReceiptIdentity, usize>,
}

impl UiGraphMountedReceiptAuthoritySeedStore {
    pub(in crate::graph::mounted_receipt) fn new(slots: Vec<UiGraphMountedReceiptSlot>) -> Self {
        let slot_lookup = slots
            .iter()
            .enumerate()
            .map(|(index, slot)| (slot.mounted_receipt_identity(), index))
            .collect();

        Self { slots, slot_lookup }
    }

    pub fn slots(&self) -> &[UiGraphMountedReceiptSlot] {
        &self.slots
    }

    pub fn slot(
        &self,
        mounted_receipt_identity: UiMountedReceiptIdentity,
    ) -> Option<&UiGraphMountedReceiptSlot> {
        self.slot_lookup
            .get(&mounted_receipt_identity)
            .and_then(|index| self.slots.get(*index))
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        self.slots.iter().fold(
            stable_text_digest("graph-mounted-receipt-seed-store"),
            |digest, slot| {
                digest.rotate_left(7)
                    ^ slot.mounted_receipt_identity().digest()
                    ^ slot.graph_node_identity().digest().rotate_left(13)
            },
        )
    }
}

pub(crate) fn materialize_graph_mounted_receipts(
    reservations: &[UiGraphMountedReceiptReservation],
) -> UiGraphMountedReceiptAuthoritySeedStore {
    UiGraphMountedReceiptAuthoritySeedStore::new(
        reservations
            .iter()
            .map(|reservation| {
                UiGraphMountedReceiptSlot::new(
                    reservation.mounted_receipt_identity,
                    reservation.graph_node_identity,
                    reservation.authority_seed,
                    reservation.mounted_posture_relationship,
                )
            })
            .collect(),
    )
}
