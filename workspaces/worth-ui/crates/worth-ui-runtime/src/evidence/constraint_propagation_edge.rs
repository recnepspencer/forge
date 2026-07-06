use crate::declaration::stable_text_digest;

use super::{
    UiConstraintCycleParticipationPosture, UiConstraintPropagationEdgeFamily,
    UiConstraintPropagationEdgePayload,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiConstraintPropagationEdge {
    family: UiConstraintPropagationEdgeFamily,
    source_member_identity_digest: u64,
    target_member_identity_digest: u64,
    payload: UiConstraintPropagationEdgePayload,
    cycle_participation_posture: UiConstraintCycleParticipationPosture,
    identity_digest: u64,
}

impl UiConstraintPropagationEdge {
    pub(crate) fn new(
        family: UiConstraintPropagationEdgeFamily,
        source_member_identity_digest: u64,
        target_member_identity_digest: u64,
        payload: UiConstraintPropagationEdgePayload,
        cycle_participation_posture: UiConstraintCycleParticipationPosture,
    ) -> Self {
        debug_assert_eq!(family, payload.family());
        Self::with_identity_digest(
            family,
            source_member_identity_digest,
            target_member_identity_digest,
            payload,
            cycle_participation_posture,
        )
    }

    pub fn family(&self) -> UiConstraintPropagationEdgeFamily {
        self.family
    }

    pub fn source_member_identity_digest(&self) -> u64 {
        self.source_member_identity_digest
    }

    pub fn target_member_identity_digest(&self) -> u64 {
        self.target_member_identity_digest
    }

    pub fn payload(&self) -> UiConstraintPropagationEdgePayload {
        self.payload
    }

    pub fn cycle_participation_posture(&self) -> UiConstraintCycleParticipationPosture {
        self.cycle_participation_posture
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }

    pub(crate) fn with_cycle_participation_posture(
        self,
        cycle_participation_posture: UiConstraintCycleParticipationPosture,
    ) -> Self {
        Self::with_identity_digest(
            self.family,
            self.source_member_identity_digest,
            self.target_member_identity_digest,
            self.payload,
            cycle_participation_posture,
        )
    }

    pub(crate) fn canonical_sort_key(&self) -> (u8, u64, u64, u8, u64) {
        (
            self.family.rank(),
            self.source_member_identity_digest,
            self.target_member_identity_digest,
            self.cycle_participation_posture.rank(),
            self.payload.identity_digest(),
        )
    }

    fn with_identity_digest(
        family: UiConstraintPropagationEdgeFamily,
        source_member_identity_digest: u64,
        target_member_identity_digest: u64,
        payload: UiConstraintPropagationEdgePayload,
        cycle_participation_posture: UiConstraintCycleParticipationPosture,
    ) -> Self {
        let identity_digest = stable_text_digest("worth-ui.constraint-propagation-edge")
            ^ (family.rank() as u64).rotate_left(7)
            ^ source_member_identity_digest.rotate_left(13)
            ^ target_member_identity_digest.rotate_left(19)
            ^ payload.identity_digest().rotate_left(23)
            ^ cycle_participation_digest(cycle_participation_posture).rotate_left(29);
        Self {
            family,
            source_member_identity_digest,
            target_member_identity_digest,
            payload,
            cycle_participation_posture,
            identity_digest,
        }
    }
}

fn cycle_participation_digest(posture: UiConstraintCycleParticipationPosture) -> u64 {
    match posture {
        UiConstraintCycleParticipationPosture::Acyclic => {
            stable_text_digest("worth-ui.constraint-cycle.acyclic")
        }
        UiConstraintCycleParticipationPosture::AdmittedFixedPoint => {
            stable_text_digest("worth-ui.constraint-cycle.admitted-fixed-point")
        }
    }
}
