use crate::graph::{
    UiGraphAxisParticipation, UiGraphNodeIdentity, UiGraphParticipationAxis,
    UiGraphParticipationPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphPageParticipationMutationKind {
    Added,
    Removed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphPageParticipationMutation {
    page_node_identity: UiGraphNodeIdentity,
    member_node_identity: UiGraphNodeIdentity,
    axis: UiGraphParticipationAxis,
    kind: UiGraphPageParticipationMutationKind,
    axis_participation: UiGraphAxisParticipation,
}

impl UiGraphPageParticipationMutation {
    const fn new(
        page_node_identity: UiGraphNodeIdentity,
        member_node_identity: UiGraphNodeIdentity,
        axis: UiGraphParticipationAxis,
        kind: UiGraphPageParticipationMutationKind,
        axis_participation: UiGraphAxisParticipation,
    ) -> Self {
        Self {
            page_node_identity,
            member_node_identity,
            axis,
            kind,
            axis_participation,
        }
    }

    pub fn page_node_identity(self) -> UiGraphNodeIdentity {
        self.page_node_identity
    }

    pub fn member_node_identity(self) -> UiGraphNodeIdentity {
        self.member_node_identity
    }

    pub fn axis(self) -> UiGraphParticipationAxis {
        self.axis
    }

    pub fn kind(self) -> UiGraphPageParticipationMutationKind {
        self.kind
    }

    pub fn axis_participation(self) -> UiGraphAxisParticipation {
        self.axis_participation
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphParticipationMutation {
    owner_node_identity: UiGraphNodeIdentity,
    page_node_identity: UiGraphNodeIdentity,
    axis: UiGraphParticipationAxis,
    prior_axis_participation: UiGraphAxisParticipation,
    next_axis_participation: UiGraphAxisParticipation,
    updated_posture: UiGraphParticipationPosture,
}

impl UiGraphParticipationMutation {
    pub fn axis_transition(
        owner_node_identity: UiGraphNodeIdentity,
        page_node_identity: UiGraphNodeIdentity,
        prior_posture: UiGraphParticipationPosture,
        axis: UiGraphParticipationAxis,
        next_axis_participation: UiGraphAxisParticipation,
    ) -> Self {
        let prior_axis_participation = prior_posture.axis(axis);
        let updated_posture = prior_posture.with_axis(axis, next_axis_participation);

        Self {
            owner_node_identity,
            page_node_identity,
            axis,
            prior_axis_participation,
            next_axis_participation,
            updated_posture,
        }
    }

    pub fn owner_node_identity(self) -> UiGraphNodeIdentity {
        self.owner_node_identity
    }

    pub fn page_node_identity(self) -> UiGraphNodeIdentity {
        self.page_node_identity
    }

    pub fn axis(self) -> UiGraphParticipationAxis {
        self.axis
    }

    pub fn prior_axis_participation(self) -> UiGraphAxisParticipation {
        self.prior_axis_participation
    }

    pub fn next_axis_participation(self) -> UiGraphAxisParticipation {
        self.next_axis_participation
    }

    pub fn updated_posture(self) -> UiGraphParticipationPosture {
        self.updated_posture
    }

    pub fn page_participation_mutation(self) -> Option<UiGraphPageParticipationMutation> {
        match (
            self.prior_axis_participation.status().admitted(),
            self.next_axis_participation.status().admitted(),
        ) {
            (false, true) => Some(UiGraphPageParticipationMutation::new(
                self.page_node_identity,
                self.owner_node_identity,
                self.axis,
                UiGraphPageParticipationMutationKind::Added,
                self.next_axis_participation,
            )),
            (true, false) => Some(UiGraphPageParticipationMutation::new(
                self.page_node_identity,
                self.owner_node_identity,
                self.axis,
                UiGraphPageParticipationMutationKind::Removed,
                self.next_axis_participation,
            )),
            _ => None,
        }
    }
}
