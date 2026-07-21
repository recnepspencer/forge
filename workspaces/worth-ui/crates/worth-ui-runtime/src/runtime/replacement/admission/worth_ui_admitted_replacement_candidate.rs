use crate::runtime::replacement::admission::{
    WorthUiActiveReplacementBasis, WorthUiCandidateAdmissionReport,
};
use crate::runtime::replacement::candidate::{
    WorthUiCandidateArtifactBundle, WorthUiReplacementCandidate,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiAdmittedReplacementCandidate {
    candidate: WorthUiReplacementCandidate,
    active_basis: WorthUiActiveReplacementBasis,
    report: WorthUiCandidateAdmissionReport,
}

impl WorthUiAdmittedReplacementCandidate {
    pub(crate) fn new(
        candidate: WorthUiReplacementCandidate,
        active_basis: WorthUiActiveReplacementBasis,
        report: WorthUiCandidateAdmissionReport,
    ) -> Self {
        Self {
            candidate,
            active_basis,
            report,
        }
    }

    pub fn candidate(&self) -> &WorthUiReplacementCandidate {
        &self.candidate
    }

    pub fn active_basis(&self) -> WorthUiActiveReplacementBasis {
        self.active_basis
    }

    pub fn report(&self) -> WorthUiCandidateAdmissionReport {
        self.report.clone()
    }

    pub(crate) fn artifact_bundle(&self) -> &WorthUiCandidateArtifactBundle {
        self.candidate.artifact_bundle()
    }
}
