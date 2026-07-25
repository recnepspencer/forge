use std::collections::BTreeMap;

use crate::declaration::stable_text_digest;
use crate::graph::{
    UiGraphMountEligibilityIdentity, UiGraphMountEligibilityRelationship,
    UiGraphMountEligibilitySeed, UiGraphMountEligibilitySlot, UiGraphNodeIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphMountEligibilityReservation {
    mount_eligibility_identity: UiGraphMountEligibilityIdentity,
    graph_node_identity: UiGraphNodeIdentity,
    eligibility_seed: UiGraphMountEligibilitySeed,
    eligibility_relationship: UiGraphMountEligibilityRelationship,
}

impl UiGraphMountEligibilityReservation {
    pub(crate) fn graph_owned_seed_slot(
        graph_node_identity: UiGraphNodeIdentity,
        eligibility_seed: UiGraphMountEligibilitySeed,
    ) -> Self {
        Self {
            mount_eligibility_identity: UiGraphMountEligibilityIdentity::graph_owned_seed_slot(
                graph_node_identity,
            ),
            graph_node_identity,
            eligibility_seed,
            eligibility_relationship: UiGraphMountEligibilityRelationship::ReservedMountEligibility,
        }
    }

    pub fn mount_eligibility_identity(self) -> UiGraphMountEligibilityIdentity {
        self.mount_eligibility_identity
    }

    pub fn graph_node_identity(self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn eligibility_seed(self) -> UiGraphMountEligibilitySeed {
        self.eligibility_seed
    }

    pub fn eligibility_relationship(self) -> UiGraphMountEligibilityRelationship {
        self.eligibility_relationship
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphMountEligibilityStore {
    slots: Vec<UiGraphMountEligibilitySlot>,
    slot_lookup: BTreeMap<UiGraphMountEligibilityIdentity, usize>,
}

impl UiGraphMountEligibilityStore {
    pub(in crate::graph::mount_eligibility) fn new(
        slots: Vec<UiGraphMountEligibilitySlot>,
    ) -> Self {
        let slot_lookup = slots
            .iter()
            .enumerate()
            .map(|(index, slot)| (slot.mount_eligibility_identity(), index))
            .collect();

        Self { slots, slot_lookup }
    }

    pub fn slots(&self) -> &[UiGraphMountEligibilitySlot] {
        &self.slots
    }

    pub fn slot(
        &self,
        mount_eligibility_identity: UiGraphMountEligibilityIdentity,
    ) -> Option<&UiGraphMountEligibilitySlot> {
        self.slot_lookup
            .get(&mount_eligibility_identity)
            .and_then(|index| self.slots.get(*index))
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        self.slots.iter().fold(
            stable_text_digest("graph-mount-eligibility-seed-store"),
            |digest, slot| {
                digest.rotate_left(7)
                    ^ slot.mount_eligibility_identity().digest()
                    ^ slot.graph_node_identity().digest().rotate_left(13)
            },
        )
    }
}

pub(crate) fn materialize_graph_mount_eligibilities(
    reservations: &[UiGraphMountEligibilityReservation],
) -> UiGraphMountEligibilityStore {
    UiGraphMountEligibilityStore::new(
        reservations
            .iter()
            .map(|reservation| {
                UiGraphMountEligibilitySlot::new(
                    reservation.mount_eligibility_identity,
                    reservation.graph_node_identity,
                    reservation.eligibility_seed,
                    reservation.eligibility_relationship,
                )
            })
            .collect(),
    )
}
