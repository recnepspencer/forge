use worth_query::facade::{
    BridgePreviewSessionIdentity, ResolvedSnapshotBasis, SnapshotResolutionReport,
    WorthQuerySessionLabel,
};

use crate::declaration::stable_text_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiGraphWorldProfile {
    Authoritative,
    PreviewSessionLabel {
        session_label: WorthQuerySessionLabel,
    },
    PreviewSessionIdentity {
        preview_session_identity: BridgePreviewSessionIdentity,
    },
    BranchSessionLabel {
        session_label: WorthQuerySessionLabel,
    },
    HotReloadCandidate {
        session_label: WorthQuerySessionLabel,
    },
    Diagnostic {
        session_label: WorthQuerySessionLabel,
    },
    HostObservation {
        session_label: WorthQuerySessionLabel,
    },
    TestCertification {
        session_label: WorthQuerySessionLabel,
    },
    QuerySnapshotBasis {
        basis: ResolvedSnapshotBasis,
        resolution_report: SnapshotResolutionReport,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiGraphWorldProfileError {
    ResolutionReportMismatch,
}

impl UiGraphWorldProfile {
    pub const fn authoritative() -> Self {
        Self::Authoritative
    }

    pub fn preview_session_label(session_label: WorthQuerySessionLabel) -> Self {
        Self::PreviewSessionLabel { session_label }
    }

    pub fn preview_session_identity(
        preview_session_identity: BridgePreviewSessionIdentity,
    ) -> Self {
        Self::PreviewSessionIdentity {
            preview_session_identity,
        }
    }

    pub fn branch_session_label(session_label: WorthQuerySessionLabel) -> Self {
        Self::BranchSessionLabel { session_label }
    }

    pub fn hot_reload_candidate(session_label: WorthQuerySessionLabel) -> Self {
        Self::HotReloadCandidate { session_label }
    }

    pub fn diagnostic(session_label: WorthQuerySessionLabel) -> Self {
        Self::Diagnostic { session_label }
    }

    pub fn host_observation(session_label: WorthQuerySessionLabel) -> Self {
        Self::HostObservation { session_label }
    }

    pub fn test_certification(session_label: WorthQuerySessionLabel) -> Self {
        Self::TestCertification { session_label }
    }

    pub fn query_snapshot_basis(
        basis: ResolvedSnapshotBasis,
        resolution_report: SnapshotResolutionReport,
    ) -> Result<Self, UiGraphWorldProfileError> {
        if resolution_report.basis_digest() != basis.proof().digest()
            || resolution_report.resolution_mode() != basis.resolution_mode()
        {
            return Err(UiGraphWorldProfileError::ResolutionReportMismatch);
        }
        Ok(Self::QuerySnapshotBasis {
            basis,
            resolution_report,
        })
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        match self {
            Self::Authoritative => stable_text_digest("graph-world:authoritative"),
            Self::PreviewSessionLabel { session_label } => {
                stable_text_digest("graph-world:preview-label")
                    ^ stable_text_digest(session_label.display()).rotate_left(13)
            }
            Self::PreviewSessionIdentity {
                preview_session_identity,
            } => {
                stable_text_digest("graph-world:preview-session")
                    ^ stable_text_digest(
                        preview_session_identity.terminal_projection_for_reporting(),
                    )
                    .rotate_left(19)
            }
            Self::BranchSessionLabel { session_label } => {
                stable_text_digest("graph-world:branch-label")
                    ^ stable_text_digest(session_label.display()).rotate_left(23)
            }
            Self::HotReloadCandidate { session_label } => {
                stable_text_digest("graph-world:hot-reload-candidate")
                    ^ stable_text_digest(session_label.display()).rotate_left(29)
            }
            Self::Diagnostic { session_label } => {
                stable_text_digest("graph-world:diagnostic")
                    ^ stable_text_digest(session_label.display()).rotate_left(31)
            }
            Self::HostObservation { session_label } => {
                stable_text_digest("graph-world:host-observation")
                    ^ stable_text_digest(session_label.display()).rotate_left(37)
            }
            Self::TestCertification { session_label } => {
                stable_text_digest("graph-world:test-certification")
                    ^ stable_text_digest(session_label.display()).rotate_left(41)
            }
            Self::QuerySnapshotBasis {
                basis,
                resolution_report,
            } => {
                stable_text_digest("graph-world:query-basis")
                    ^ stable_text_digest(
                        basis.proof().identity().terminal_projection_for_reporting(),
                    )
                    .rotate_left(29)
                    ^ stable_text_digest(resolution_report.basis_digest().as_str()).rotate_left(41)
            }
        }
    }

    pub(crate) fn comparison_family(&self) -> u64 {
        match self {
            Self::Authoritative => stable_text_digest("graph-world-family:authoritative"),
            Self::PreviewSessionLabel { .. } => {
                stable_text_digest("graph-world-family:preview-label")
            }
            Self::PreviewSessionIdentity { .. } => {
                stable_text_digest("graph-world-family:preview-session")
            }
            Self::BranchSessionLabel { .. } => {
                stable_text_digest("graph-world-family:branch-label")
            }
            Self::HotReloadCandidate { .. } => {
                stable_text_digest("graph-world-family:hot-reload-candidate")
            }
            Self::Diagnostic { .. } => stable_text_digest("graph-world-family:diagnostic"),
            Self::HostObservation { .. } => {
                stable_text_digest("graph-world-family:host-observation")
            }
            Self::TestCertification { .. } => {
                stable_text_digest("graph-world-family:test-certification")
            }
            Self::QuerySnapshotBasis { basis, .. } => {
                stable_text_digest("graph-world-family:query")
                    ^ stable_text_digest(match basis.identity().authority_family() {
                        worth_query::facade::BasisAuthorityFamily::Runtime => "runtime",
                        worth_query::facade::BasisAuthorityFamily::Store => "store",
                    })
                    .rotate_left(11)
            }
        }
    }
}
