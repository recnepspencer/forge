use crate::runtime::admission::{
    WorthUiActiveReplacementBasis, WorthUiCandidateAdmissionDenial, WorthUiCandidateAdmissionReport,
};
use crate::runtime::candidate::{WorthUiCandidateArtifactBundle, WorthUiReplacementCandidate};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiAdmittedReplacementCandidate {
    candidate: WorthUiReplacementCandidate,
    active_basis: WorthUiActiveReplacementBasis,
    report: WorthUiCandidateAdmissionReport,
    admitted_query_support_receipt_digest: u64,
}

impl WorthUiAdmittedReplacementCandidate {
    pub(crate) fn new(
        candidate: WorthUiReplacementCandidate,
        active_basis: WorthUiActiveReplacementBasis,
        report: WorthUiCandidateAdmissionReport,
    ) -> Self {
        let admitted_query_support_receipt_digest = candidate
            .lowering_basis()
            .query_support_receipt()
            .receipt_digest();
        Self {
            candidate,
            active_basis,
            report,
            admitted_query_support_receipt_digest,
        }
    }

    pub fn candidate(&self) -> &WorthUiReplacementCandidate {
        &self.candidate
    }

    pub fn active_basis(&self) -> WorthUiActiveReplacementBasis {
        self.active_basis
    }

    pub fn report(&self) -> WorthUiCandidateAdmissionReport {
        self.report
    }

    pub fn verify_receipts_unchanged(&self) -> Result<(), WorthUiCandidateAdmissionDenial> {
        let current_receipt_digest = self
            .candidate
            .lowering_basis()
            .query_support_receipt()
            .receipt_digest();
        if current_receipt_digest == self.admitted_query_support_receipt_digest {
            Ok(())
        } else {
            Err(
                WorthUiCandidateAdmissionDenial::QuerySupportReceiptChanged {
                    admitted_receipt_digest: self.admitted_query_support_receipt_digest,
                    current_receipt_digest,
                },
            )
        }
    }

    pub(crate) fn artifact_bundle(&self) -> &WorthUiCandidateArtifactBundle {
        self.candidate.artifact_bundle()
    }

    #[cfg(test)]
    pub(crate) fn verify_test_receipt_digest(
        &self,
        current_receipt_digest: u64,
    ) -> Result<(), WorthUiCandidateAdmissionDenial> {
        if current_receipt_digest == self.admitted_query_support_receipt_digest {
            Ok(())
        } else {
            Err(
                WorthUiCandidateAdmissionDenial::QuerySupportReceiptChanged {
                    admitted_receipt_digest: self.admitted_query_support_receipt_digest,
                    current_receipt_digest,
                },
            )
        }
    }

    #[cfg(test)]
    pub(crate) fn with_admitted_query_support_receipt_digest_for_test(
        mut self,
        admitted_query_support_receipt_digest: u64,
    ) -> Self {
        self.admitted_query_support_receipt_digest = admitted_query_support_receipt_digest;
        self
    }
}
