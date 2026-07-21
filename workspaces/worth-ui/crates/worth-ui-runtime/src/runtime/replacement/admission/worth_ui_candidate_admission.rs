use crate::runtime::replacement::admission::{
    WorthUiActiveReplacementBasis, WorthUiAdmittedReplacementCandidate,
    WorthUiCandidateAdmissionCounters, WorthUiCandidateAdmissionDenial,
    WorthUiCandidateAdmissionReport, WorthUiQuerySupportStatus, WorthUiRuntimeReplacementPosture,
};
use crate::runtime::replacement::candidate::WorthUiReplacementCandidate;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCandidateAdmission {
    active_basis: WorthUiActiveReplacementBasis,
}

impl WorthUiCandidateAdmission {
    pub fn for_active_basis(active_basis: WorthUiActiveReplacementBasis) -> Self {
        Self { active_basis }
    }

    pub fn admit(
        self,
        candidate: WorthUiReplacementCandidate,
    ) -> Result<WorthUiAdmittedReplacementCandidate, WorthUiCandidateAdmissionReport> {
        let mut counters = WorthUiCandidateAdmissionCounters::default();
        counters.record_candidate_proof_check();
        let candidate_basis = candidate.basis();
        let query_support_receipt = candidate.lowering_basis().query_support_receipt();

        counters.record_snapshot_compatibility_check();
        if candidate.lowering_basis().snapshot_digest() != self.active_basis.snapshot_digest() {
            return Err(WorthUiCandidateAdmissionReport::denied(
                candidate_basis,
                self.active_basis,
                query_support_receipt,
                counters,
                WorthUiCandidateAdmissionDenial::SnapshotMismatch {
                    candidate_snapshot_digest: candidate.lowering_basis().snapshot_digest(),
                    active_snapshot_digest: self.active_basis.snapshot_digest(),
                },
            ));
        }

        counters.record_runtime_posture_check();
        if let Some(denial) = runtime_posture_denial(self.active_basis.replacement_posture()) {
            return Err(WorthUiCandidateAdmissionReport::denied(
                candidate_basis,
                self.active_basis,
                query_support_receipt,
                counters,
                denial,
            ));
        }

        counters.record_query_support_check();
        if let Some(denial) = query_support_denial(query_support_receipt) {
            return Err(WorthUiCandidateAdmissionReport::denied(
                candidate_basis,
                self.active_basis,
                query_support_receipt,
                counters,
                denial,
            ));
        }

        let report = WorthUiCandidateAdmissionReport::admitted(
            candidate_basis,
            self.active_basis,
            query_support_receipt,
            counters,
        );
        Ok(WorthUiAdmittedReplacementCandidate::new(
            candidate,
            self.active_basis,
            report,
        ))
    }
}

fn runtime_posture_denial(
    posture: WorthUiRuntimeReplacementPosture,
) -> Option<WorthUiCandidateAdmissionDenial> {
    match posture {
        WorthUiRuntimeReplacementPosture::Supported => None,
        WorthUiRuntimeReplacementPosture::Deferred => {
            Some(WorthUiCandidateAdmissionDenial::DeferredRuntimePosture { posture })
        }
        WorthUiRuntimeReplacementPosture::Unsupported => {
            Some(WorthUiCandidateAdmissionDenial::UnsupportedRuntimePosture { posture })
        }
    }
}

fn query_support_denial(
    receipt: crate::runtime::replacement::admission::WorthUiQuerySupportReceipt,
) -> Option<WorthUiCandidateAdmissionDenial> {
    match receipt.status() {
        WorthUiQuerySupportStatus::Supported => None,
        WorthUiQuerySupportStatus::Deferred => {
            Some(WorthUiCandidateAdmissionDenial::DeferredQuerySupport { receipt })
        }
        WorthUiQuerySupportStatus::Unsupported => {
            Some(WorthUiCandidateAdmissionDenial::UnsupportedQuerySupport { receipt })
        }
    }
}
