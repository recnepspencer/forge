use crate::runtime::WorthUiReplacementCandidateBasis;
use crate::source::{WorthUiArtifact, WorthUiArtifactDigest};
use std::rc::Rc;

/// Canonical authoring artifact retained by one prepared application.
/// Every authored lane converges before this type exists, so it always retains
/// immutable runtime artifact authority and the candidate basis that admitted
/// it without duplicating the candidate packet.
pub(crate) struct WorthUiPreparedApplicationArtifact {
    artifact: Rc<WorthUiArtifact>,
    basis: WorthUiReplacementCandidateBasis,
}

/// Observable authoring posture of the canonical artifact retained by a
/// prepared application. The artifact itself remains sealed in authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPreparedApplicationArtifactPosture {
    SourceBacked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthUiPreparedApplicationArtifactIdentity(WorthUiReplacementCandidateBasis);

impl WorthUiPreparedApplicationArtifact {
    pub(super) fn runtime_artifact_authority(
        &self,
    ) -> (Rc<WorthUiArtifact>, WorthUiArtifactDigest) {
        (Rc::clone(&self.artifact), self.basis.artifact_digest())
    }

    pub(super) fn identity(&self) -> WorthUiPreparedApplicationArtifactIdentity {
        WorthUiPreparedApplicationArtifactIdentity(self.basis)
    }

    pub(super) fn posture(&self) -> WorthUiPreparedApplicationArtifactPosture {
        WorthUiPreparedApplicationArtifactPosture::SourceBacked
    }

    pub(super) fn candidate_basis(&self) -> WorthUiReplacementCandidateBasis {
        self.basis
    }

    pub(crate) fn source_backed(candidate: &crate::runtime::WorthUiReplacementCandidate) -> Self {
        Self {
            artifact: candidate.artifact_bundle().artifact_authority(),
            basis: candidate.basis(),
        }
    }
}
