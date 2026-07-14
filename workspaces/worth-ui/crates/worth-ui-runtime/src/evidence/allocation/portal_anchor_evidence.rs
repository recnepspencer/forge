#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiPortalAnchorMovementEvidence {
    identity_transition: crate::runtime::UiPortalAnchorIdentityTransition,
    evidence_generation: worth_ui_inspection::UiEvidenceAuthorityGeneration,
    receipt_identity_digest: u64,
    receipt_generation_digest: u64,
    neighborhood_identity_digests: Box<[u64]>,
    authority_probes: u16,
}

impl UiPortalAnchorMovementEvidence {
    pub(crate) fn from_movement(movement: &crate::runtime::UiAdmittedPortalMovement) -> Self {
        let mut neighborhoods = std::iter::once(movement.target().primary())
            .chain(movement.target().widened().iter())
            .map(|target| target.neighborhood_identity().identity_digest())
            .collect::<Vec<_>>();
        neighborhoods.sort_unstable();
        neighborhoods.dedup();
        Self {
            identity_transition: movement.identity_transition(),
            evidence_generation: movement.observation().evidence_generation(),
            receipt_identity_digest: movement.receipt_identity().identity_digest(),
            receipt_generation_digest: movement.receipt_generation().identity_digest(),
            neighborhood_identity_digests: neighborhoods.into_boxed_slice(),
            authority_probes: movement.authority_probes(),
        }
    }

    pub(crate) fn from_committed(
        graph_evidence: &Self,
        portal: &crate::runtime::UiPortalAllocationPlanningBasis,
        receipt: &crate::runtime::UiAllocationReceipt,
    ) -> Option<Self> {
        (receipt.identity().portal_anchor() == Some(portal.identity_transition().current()))
            .then_some(())?;
        let mut neighborhoods = graph_evidence.neighborhood_identity_digests().to_vec();
        neighborhoods.sort_unstable();
        neighborhoods.dedup();
        Some(Self {
            identity_transition: portal.identity_transition(),
            evidence_generation: portal.observation().evidence_generation(),
            receipt_identity_digest: receipt.identity().identity_digest(),
            receipt_generation_digest: receipt.generation().identity_digest(),
            neighborhood_identity_digests: neighborhoods.into_boxed_slice(),
            authority_probes: graph_evidence.authority_probes(),
        })
    }

    pub fn identity_transition(&self) -> crate::runtime::UiPortalAnchorIdentityTransition {
        self.identity_transition
    }
    pub fn evidence_generation(&self) -> worth_ui_inspection::UiEvidenceAuthorityGeneration {
        self.evidence_generation
    }
    pub fn receipt_identity_digest(&self) -> u64 {
        self.receipt_identity_digest
    }
    pub fn receipt_generation_digest(&self) -> u64 {
        self.receipt_generation_digest
    }
    pub fn neighborhood_identity_digests(&self) -> &[u64] {
        &self.neighborhood_identity_digests
    }
    pub fn authority_probes(&self) -> u16 {
        self.authority_probes
    }
    pub(crate) fn identity_digest(&self) -> u64 {
        let mut digest = crate::declaration::stable_text_digest("worth-ui.portal-anchor-movement");
        digest ^= self
            .identity_transition
            .current()
            .identity_digest()
            .rotate_left(7);
        digest ^= self.evidence_generation.as_u64().rotate_left(17);
        digest ^= self.receipt_identity_digest.rotate_left(29);
        digest ^= self.receipt_generation_digest.rotate_left(37);
        for neighborhood in &self.neighborhood_identity_digests {
            digest = digest.wrapping_mul(0x100000001b3) ^ neighborhood.rotate_left(43);
        }
        digest ^ u64::from(self.authority_probes).rotate_left(53)
    }
}
