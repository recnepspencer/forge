use super::WorthUiPlanEquivalenceEvidenceReference;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPlanEquivalenceSummary {
    previous_fingerprint: u64,
    candidate_fingerprint: u64,
    changed_region_count: usize,
    exact_region_comparison_count: usize,
    evidence_reference: WorthUiPlanEquivalenceEvidenceReference,
}

impl WorthUiPlanEquivalenceSummary {
    pub(crate) fn new(
        previous_fingerprint: u64,
        candidate_fingerprint: u64,
        changed_region_count: usize,
        exact_region_comparison_count: usize,
        evidence_reference: WorthUiPlanEquivalenceEvidenceReference,
    ) -> Self {
        Self {
            previous_fingerprint,
            candidate_fingerprint,
            changed_region_count,
            exact_region_comparison_count,
            evidence_reference,
        }
    }

    pub fn previous_fingerprint(self) -> u64 {
        self.previous_fingerprint
    }

    pub fn candidate_fingerprint(self) -> u64 {
        self.candidate_fingerprint
    }

    pub fn changed_region_count(self) -> usize {
        self.changed_region_count
    }

    pub fn exact_region_comparison_count(self) -> usize {
        self.exact_region_comparison_count
    }

    pub fn evidence_reference(self) -> WorthUiPlanEquivalenceEvidenceReference {
        self.evidence_reference
    }
}
