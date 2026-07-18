use crate::graph::{
    UiGraphAxisParticipation, UiGraphMountedReceiptAuthorityRecord, UiGraphMountedReceiptMutation,
    UiGraphMountedReceiptMutationKind, UiGraphMountedReceiptSlot,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphMountedReceiptTransition {
    graph_authority_identity: crate::graph::UiGraphAuthorityIdentity,
    slot: UiGraphMountedReceiptSlot,
    prior_mounted_axis_participation: UiGraphAxisParticipation,
    next_mounted_axis_participation: UiGraphAxisParticipation,
    mutation_kind: UiGraphMountedReceiptMutationKind,
}

impl UiGraphMountedReceiptTransition {
    pub(crate) fn from_slot_axis_transition(
        graph_authority_identity: crate::graph::UiGraphAuthorityIdentity,
        slot: UiGraphMountedReceiptSlot,
        prior_mounted_axis_participation: UiGraphAxisParticipation,
        next_mounted_axis_participation: UiGraphAxisParticipation,
    ) -> Option<Self> {
        let mutation_kind = match (
            prior_mounted_axis_participation.status().admitted(),
            next_mounted_axis_participation.status().admitted(),
        ) {
            (false, true) => UiGraphMountedReceiptMutationKind::CreateSlot,
            (true, false) => UiGraphMountedReceiptMutationKind::RemoveSlot,
            _ => return None,
        };

        Some(Self {
            graph_authority_identity,
            slot,
            prior_mounted_axis_participation,
            next_mounted_axis_participation,
            mutation_kind,
        })
    }

    pub fn authority_record(self) -> UiGraphMountedReceiptAuthorityRecord {
        self.slot.into()
    }

    pub fn kind(self) -> UiGraphMountedReceiptMutationKind {
        self.mutation_kind
    }

    pub fn prior_mounted_axis_participation(self) -> UiGraphAxisParticipation {
        self.prior_mounted_axis_participation
    }

    pub fn next_mounted_axis_participation(self) -> UiGraphAxisParticipation {
        self.next_mounted_axis_participation
    }

    pub fn mutation(self) -> UiGraphMountedReceiptMutation {
        UiGraphMountedReceiptMutation::from_transition(self)
    }

    pub(crate) fn graph_authority_identity(self) -> crate::graph::UiGraphAuthorityIdentity {
        self.graph_authority_identity
    }
}
