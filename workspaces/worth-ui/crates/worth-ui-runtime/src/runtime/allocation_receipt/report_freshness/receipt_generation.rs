use crate::evidence::UiMeasurementBasisGeneration;
use crate::graph::UiGraphGeneration;

/// Generation compatibility carried forward from admitted planning.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAllocationReceiptGeneration {
    /// The planning lane's canonical identity is evidence, never the receipt's
    /// sole identity authority. The other fields are compared independently.
    planning_evidence_digest: u64,
    measurement_basis_generation: UiMeasurementBasisGeneration,
    portal_evidence_generation: Option<worth_ui_inspection::UiEvidenceAuthorityGeneration>,
    neighborhood_generation: UiGraphGeneration,
}

impl UiAllocationReceiptGeneration {
    pub(crate) fn from_candidate(candidate: &super::UiAllocationCandidate) -> Self {
        Self {
            planning_evidence_digest: candidate.planning_identity_digest(),
            measurement_basis_generation: candidate.measurement_basis().generation(),
            portal_evidence_generation: candidate
                .portal_allocation_input()
                .map(|basis| basis.observation().evidence_generation()),
            neighborhood_generation: candidate.allocation_neighborhood().graph_generation(),
        }
    }
    pub fn planning_evidence_digest(&self) -> u64 {
        self.planning_evidence_digest
    }
    pub fn measurement_basis_generation(&self) -> UiMeasurementBasisGeneration {
        self.measurement_basis_generation
    }
    pub fn neighborhood_generation(&self) -> UiGraphGeneration {
        self.neighborhood_generation
    }
    pub fn portal_evidence_generation(
        &self,
    ) -> Option<worth_ui_inspection::UiEvidenceAuthorityGeneration> {
        self.portal_evidence_generation
    }
    pub(crate) fn identity_digest(&self) -> u64 {
        self.planning_evidence_digest
            ^ self.measurement_basis_generation.raw().rotate_left(17)
            ^ self.neighborhood_generation.as_u64().rotate_left(37)
            ^ self
                .portal_evidence_generation
                .map_or(0, |generation| generation.as_u64().rotate_left(47))
    }
}
