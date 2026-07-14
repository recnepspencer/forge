#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiPortalReplanConsequence {
    evidence: crate::evidence::UiPortalAnchorMovementEvidence,
    movement: crate::runtime::UiAdmittedPortalMovement,
}

impl UiPortalReplanConsequence {
    pub(crate) fn seal(movement: &crate::runtime::UiAdmittedPortalMovement) -> Self {
        Self {
            evidence: crate::evidence::UiPortalAnchorMovementEvidence::from_movement(movement),
            movement: movement.clone(),
        }
    }

    pub(crate) fn evidence(&self) -> &crate::evidence::UiPortalAnchorMovementEvidence {
        &self.evidence
    }

    pub(crate) fn movement(&self) -> &crate::runtime::UiAdmittedPortalMovement {
        &self.movement
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        self.evidence.identity_digest()
    }
}
