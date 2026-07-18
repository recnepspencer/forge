use crate::runtime::WorthUiReplacementCandidateBasis;
use crate::source::{WorthUiArtifact, WorthUiArtifactDigest};
use std::rc::Rc;
use worth_ui_dsl::WorthUiDslPackage;

use super::WorthUiPreparedDeclarationSourceIdentity;

/// Canonical authoring artifact retained by one prepared application.
/// Declaration-authored applications retain their declaration package;
/// source-backed applications retain immutable artifact authority and the
/// candidate basis that admitted it, without duplicating the candidate packet.
pub(crate) enum WorthUiPreparedApplicationArtifact {
    DeclarationAuthored(WorthUiDslPackage),
    SourceBacked {
        artifact: Rc<WorthUiArtifact>,
        basis: WorthUiReplacementCandidateBasis,
    },
}

/// Observable authoring posture of the canonical artifact retained by a
/// prepared application. The artifact itself remains sealed in authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPreparedApplicationArtifactPosture {
    DeclarationAuthored,
    SourceBacked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum WorthUiPreparedApplicationArtifactIdentity {
    DeclarationAuthored(WorthUiPreparedDeclarationSourceIdentity),
    SourceBacked(WorthUiReplacementCandidateBasis),
}

impl WorthUiPreparedApplicationArtifact {
    pub(super) fn runtime_artifact_authority(
        &self,
    ) -> Option<(Rc<WorthUiArtifact>, WorthUiArtifactDigest)> {
        match self {
            Self::DeclarationAuthored(_) => None,
            Self::SourceBacked { artifact, basis } => {
                Some((Rc::clone(artifact), basis.artifact_digest()))
            }
        }
    }

    pub(super) fn identity(&self) -> WorthUiPreparedApplicationArtifactIdentity {
        match self {
            Self::DeclarationAuthored(package) => {
                WorthUiPreparedApplicationArtifactIdentity::DeclarationAuthored(
                    WorthUiPreparedDeclarationSourceIdentity::derive(package, None),
                )
            }
            Self::SourceBacked { basis, .. } => {
                WorthUiPreparedApplicationArtifactIdentity::SourceBacked(*basis)
            }
        }
    }

    pub(super) fn posture(&self) -> WorthUiPreparedApplicationArtifactPosture {
        match self {
            Self::DeclarationAuthored(_) => {
                WorthUiPreparedApplicationArtifactPosture::DeclarationAuthored
            }
            Self::SourceBacked { .. } => WorthUiPreparedApplicationArtifactPosture::SourceBacked,
        }
    }

    pub(crate) fn source_backed(candidate: &crate::runtime::WorthUiReplacementCandidate) -> Self {
        Self::SourceBacked {
            artifact: candidate.artifact_bundle().artifact_authority(),
            basis: candidate.basis(),
        }
    }
}
