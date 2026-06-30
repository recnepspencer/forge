use crate::runtime::admission::{
    WorthUiActiveReplacementBasis, WorthUiCandidateAdmissionCounters,
    WorthUiCandidateAdmissionDenial, WorthUiQuerySupportReceipt,
};
use crate::runtime::candidate::WorthUiReplacementCandidateBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCandidateAdmissionReport {
    candidate_basis: WorthUiReplacementCandidateBasis,
    active_basis: WorthUiActiveReplacementBasis,
    query_support_receipt: WorthUiQuerySupportReceipt,
    counters: WorthUiCandidateAdmissionCounters,
    denial: Option<WorthUiCandidateAdmissionDenial>,
}

impl WorthUiCandidateAdmissionReport {
    pub(crate) fn admitted(
        candidate_basis: WorthUiReplacementCandidateBasis,
        active_basis: WorthUiActiveReplacementBasis,
        query_support_receipt: WorthUiQuerySupportReceipt,
        counters: WorthUiCandidateAdmissionCounters,
    ) -> Self {
        Self {
            candidate_basis,
            active_basis,
            query_support_receipt,
            counters,
            denial: None,
        }
    }

    pub(crate) fn denied(
        candidate_basis: WorthUiReplacementCandidateBasis,
        active_basis: WorthUiActiveReplacementBasis,
        query_support_receipt: WorthUiQuerySupportReceipt,
        counters: WorthUiCandidateAdmissionCounters,
        denial: WorthUiCandidateAdmissionDenial,
    ) -> Self {
        Self {
            candidate_basis,
            active_basis,
            query_support_receipt,
            counters,
            denial: Some(denial),
        }
    }

    pub fn candidate_basis(&self) -> WorthUiReplacementCandidateBasis {
        self.candidate_basis
    }

    pub fn active_basis(&self) -> WorthUiActiveReplacementBasis {
        self.active_basis
    }

    pub fn query_support_receipt(&self) -> WorthUiQuerySupportReceipt {
        self.query_support_receipt
    }

    pub fn counters(&self) -> WorthUiCandidateAdmissionCounters {
        self.counters
    }

    pub fn denial(&self) -> Option<WorthUiCandidateAdmissionDenial> {
        self.denial
    }
}
