use crate::runtime::candidate::{
    WorthUiCandidateArtifactBundle, WorthUiCandidateAuthoringLane, WorthUiCandidateLoweringBasis,
    WorthUiCandidateProvenanceHandle, WorthUiReplacementCandidateBasis,
    WorthUiReplacementCandidateDenial, WorthUiReplacementCause,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiReplacementCandidate {
    bundle: WorthUiCandidateArtifactBundle,
    cause: WorthUiReplacementCause,
    provenance_handle: WorthUiCandidateProvenanceHandle,
    authoring_lane: WorthUiCandidateAuthoringLane,
}

impl WorthUiReplacementCandidate {
    pub(crate) fn from_artifact_bundle(
        bundle: WorthUiCandidateArtifactBundle,
        cause: WorthUiReplacementCause,
        authoring_lane: WorthUiCandidateAuthoringLane,
    ) -> Result<Self, WorthUiReplacementCandidateDenial> {
        let provenance_handle = cause.provenance_handle();
        Ok(Self {
            bundle,
            cause,
            provenance_handle,
            authoring_lane,
        })
    }

    pub fn basis(&self) -> WorthUiReplacementCandidateBasis {
        self.bundle.basis()
    }

    pub fn lowering_basis(&self) -> WorthUiCandidateLoweringBasis {
        self.bundle.lowering_basis()
    }

    pub fn cause(&self) -> &WorthUiReplacementCause {
        &self.cause
    }

    pub fn provenance_handle(&self) -> WorthUiCandidateProvenanceHandle {
        self.provenance_handle
    }

    pub fn authoring_lane(&self) -> WorthUiCandidateAuthoringLane {
        self.authoring_lane
    }

    pub(crate) fn artifact_bundle(&self) -> &WorthUiCandidateArtifactBundle {
        &self.bundle
    }
}
