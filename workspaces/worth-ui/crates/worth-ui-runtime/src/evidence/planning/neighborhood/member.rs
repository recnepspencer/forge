use crate::declaration::{stable_text_digest, UiDeclaredMeasurementConstraintModifier};
use crate::graph::{UiGraphAxisParticipation, UiGraphNodeIdentity, UiRepeatedInstanceBasis};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiAllocationNeighborhoodMemberRole {
    Root,
    Peer,
    ScopedParticipant,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationNeighborhoodMember {
    graph_node_identity: UiGraphNodeIdentity,
    authored_provenance_digest: u64,
    repeated_instance_basis: UiRepeatedInstanceBasis,
    layout_participation: UiGraphAxisParticipation,
    role: UiAllocationNeighborhoodMemberRole,
    measurement_constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
    identity_digest: u64,
}

impl UiAllocationNeighborhoodMember {
    pub(crate) fn new(
        graph_node_identity: UiGraphNodeIdentity,
        authored_provenance_digest: u64,
        repeated_instance_basis: UiRepeatedInstanceBasis,
        layout_participation: UiGraphAxisParticipation,
        role: UiAllocationNeighborhoodMemberRole,
        measurement_constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
    ) -> Self {
        let identity_digest = stable_text_digest("allocation-neighborhood-member")
            ^ graph_node_identity.digest().rotate_left(7)
            ^ repeated_instance_basis.identity_digest().rotate_left(13)
            ^ (role as u64).rotate_left(19)
            ^ measurement_constraint_modifier_digest(measurement_constraint_modifier)
                .rotate_left(23);

        Self {
            graph_node_identity,
            authored_provenance_digest,
            repeated_instance_basis,
            layout_participation,
            role,
            measurement_constraint_modifier,
            identity_digest,
        }
    }

    pub fn graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn repeated_instance_basis(&self) -> &UiRepeatedInstanceBasis {
        &self.repeated_instance_basis
    }

    pub fn authored_provenance_digest(&self) -> u64 {
        self.authored_provenance_digest
    }

    pub fn layout_participation(&self) -> UiGraphAxisParticipation {
        self.layout_participation
    }

    pub fn layout_participates(&self) -> bool {
        matches!(
            self.layout_participation.status(),
            crate::graph::UiGraphParticipationStatus::Admitted
        )
    }

    pub fn role(&self) -> UiAllocationNeighborhoodMemberRole {
        self.role
    }

    pub fn measurement_constraint_modifier(
        &self,
    ) -> Option<UiDeclaredMeasurementConstraintModifier> {
        self.measurement_constraint_modifier
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }

    pub(crate) fn canonical_sort_key(&self) -> (u64, u64, u8) {
        (
            self.graph_node_identity.digest(),
            self.repeated_instance_basis.identity_digest(),
            self.role as u8
                ^ measurement_constraint_modifier_rank(self.measurement_constraint_modifier),
        )
    }
}

fn measurement_constraint_modifier_digest(
    modifier: Option<UiDeclaredMeasurementConstraintModifier>,
) -> u64 {
    match modifier {
        Some(UiDeclaredMeasurementConstraintModifier::Bounded) => {
            stable_text_digest("allocation-neighborhood-member.constraint.bounded")
        }
        None => stable_text_digest("allocation-neighborhood-member.constraint.none"),
    }
}

fn measurement_constraint_modifier_rank(
    modifier: Option<UiDeclaredMeasurementConstraintModifier>,
) -> u8 {
    match modifier {
        Some(UiDeclaredMeasurementConstraintModifier::Bounded) => 1,
        None => 0,
    }
}
