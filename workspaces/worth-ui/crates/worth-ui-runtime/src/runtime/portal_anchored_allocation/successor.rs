#[derive(Clone, Debug, PartialEq)]
pub struct UiPortalAllocationPlanningBasis {
    observation: super::UiAdmittedPortalAnchorObservation,
    identity_transition: super::UiPortalAnchorIdentityTransition,
    prior_receipt_identity: crate::runtime::UiAllocationReceiptIdentity,
    prior_receipt_generation: crate::runtime::UiAllocationReceiptGeneration,
    neighborhood_identity_digest: u64,
    identity_digest: u64,
}

impl Eq for UiPortalAllocationPlanningBasis {}

impl UiPortalAllocationPlanningBasis {
    pub(crate) fn seal(
        movement: &crate::runtime::UiAdmittedPortalMovement,
        neighborhood: &crate::evidence::UiAllocationNeighborhoodIdentity,
    ) -> Option<Self> {
        let target_matches = movement.target().primary().neighborhood_identity() == neighborhood;
        if !target_matches {
            return None;
        }
        let observation = movement.observation();
        let identity_transition = movement.identity_transition();
        let prior_receipt_identity = movement.receipt_identity().clone();
        let prior_receipt_generation = movement.receipt_generation();
        let neighborhood_identity_digest = neighborhood.identity_digest();
        let identity_digest =
            crate::declaration::stable_text_digest("worth-ui.portal-allocation-planning-basis")
                ^ observation.identity().identity_digest().rotate_left(7)
                ^ observation.evidence_generation().as_u64().rotate_left(17)
                ^ prior_receipt_generation
                    .planning_evidence_digest()
                    .rotate_left(29)
                ^ prior_receipt_identity.identity_digest().rotate_left(37)
                ^ prior_receipt_generation.identity_digest().rotate_left(43)
                ^ neighborhood_identity_digest.rotate_left(53);
        Some(Self {
            observation,
            identity_transition,
            prior_receipt_identity,
            prior_receipt_generation,
            neighborhood_identity_digest,
            identity_digest,
        })
    }

    pub fn observation(&self) -> super::UiAdmittedPortalAnchorObservation {
        self.observation
    }
    pub fn identity_transition(&self) -> super::UiPortalAnchorIdentityTransition {
        self.identity_transition
    }
    pub fn prior_receipt_identity(&self) -> &crate::runtime::UiAllocationReceiptIdentity {
        &self.prior_receipt_identity
    }
    pub fn prior_receipt_generation(&self) -> crate::runtime::UiAllocationReceiptGeneration {
        self.prior_receipt_generation
    }
    pub fn neighborhood_identity_digest(&self) -> u64 {
        self.neighborhood_identity_digest
    }
    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}
