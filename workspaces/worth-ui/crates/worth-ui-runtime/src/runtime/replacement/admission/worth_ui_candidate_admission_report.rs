use crate::runtime::replacement::admission::{
    WorthUiActiveReplacementBasis, WorthUiCandidateAdmissionCounters,
    WorthUiCandidateAdmissionDenial,
};
use crate::runtime::replacement::candidate::WorthUiReplacementCandidateBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCandidateAdmissionReport {
    candidate_basis: WorthUiReplacementCandidateBasis,
    active_basis: WorthUiActiveReplacementBasis,
    counters: Box<WorthUiCandidateAdmissionCounters>,
    denial: Option<Box<WorthUiCandidateAdmissionDenial>>,
}

impl WorthUiCandidateAdmissionReport {
    pub(crate) fn admitted(
        candidate_basis: WorthUiReplacementCandidateBasis,
        active_basis: WorthUiActiveReplacementBasis,
        counters: WorthUiCandidateAdmissionCounters,
    ) -> Self {
        Self {
            candidate_basis,
            active_basis,
            counters: Box::new(counters),
            denial: None,
        }
    }

    pub(crate) fn denied(
        candidate_basis: WorthUiReplacementCandidateBasis,
        active_basis: WorthUiActiveReplacementBasis,
        counters: WorthUiCandidateAdmissionCounters,
        denial: WorthUiCandidateAdmissionDenial,
    ) -> Self {
        Self {
            candidate_basis,
            active_basis,
            counters: Box::new(counters),
            denial: Some(Box::new(denial)),
        }
    }

    pub fn candidate_basis(&self) -> WorthUiReplacementCandidateBasis {
        self.candidate_basis
    }

    pub fn active_basis(&self) -> WorthUiActiveReplacementBasis {
        self.active_basis
    }

    pub fn counters(&self) -> WorthUiCandidateAdmissionCounters {
        *self.counters
    }

    pub fn denial(&self) -> Option<WorthUiCandidateAdmissionDenial> {
        self.denial.as_deref().copied()
    }
}
