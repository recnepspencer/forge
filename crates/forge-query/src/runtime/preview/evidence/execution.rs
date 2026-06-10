use super::super::*;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreviewExecutionKind {
    LivePatch,
    ComputedPatch,
    EffectDelivery,
    PendingWriteIntent,
    MutedEffect,
}

impl ForgeQueryPreviewExecutionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LivePatch => "live-patch",
            Self::ComputedPatch => "computed-patch",
            Self::EffectDelivery => "effect-delivery",
            Self::PendingWriteIntent => "pending-write-intent",
            Self::MutedEffect => "muted-effect",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewExecutionEvidence {
    session_label: ForgeQuerySessionLabel,
    pub(in crate::runtime::preview) kind: ForgeQueryPreviewExecutionKind,
    handle_name: String,
    source_lane: ForgeQueryAuthorityLane,
    preview_lane: ForgeQueryAuthorityLane,
    commit_identity: String,
    aspect_paths: Vec<String>,
    execution_digest: String,
}

impl ForgeQueryPreviewExecutionEvidence {
    pub(in crate::runtime::preview) fn new(
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        kind: ForgeQueryPreviewExecutionKind,
        handle_name: &str,
        source_lane: ForgeQueryAuthorityLane,
        preview_lane: ForgeQueryAuthorityLane,
        commit_identity: &str,
        aspect_paths: Vec<String>,
    ) -> Self {
        let execution_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::PreviewExecutionEvidence,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("session_label_identity"),
            basis_admission.label_identity().as_str(),
        )
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("handle_name"), handle_name)
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            source_lane.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("preview_lane"),
            preview_lane.as_str(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("commit_identity"),
            commit_identity,
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("aspect_path"),
            aspect_paths.iter().map(String::as_str),
        )
        .seal()
        .as_str()
        .to_string();
        Self {
            session_label: basis_admission.session_label().clone(),
            kind,
            handle_name: handle_name.to_string(),
            source_lane,
            preview_lane,
            commit_identity: commit_identity.to_string(),
            aspect_paths,
            execution_digest,
        }
    }

    pub fn label(&self) -> &str {
        self.session_label.display()
    }

    pub fn session_label(&self) -> &ForgeQuerySessionLabel {
        &self.session_label
    }

    pub fn label_identity(&self) -> &crate::ForgeQueryEvidenceIdentity {
        self.session_label.identity_digest()
    }

    pub fn kind(&self) -> ForgeQueryPreviewExecutionKind {
        self.kind
    }

    pub fn handle_name(&self) -> &str {
        &self.handle_name
    }

    pub fn source_lane(&self) -> ForgeQueryAuthorityLane {
        self.source_lane
    }

    pub fn preview_lane(&self) -> ForgeQueryAuthorityLane {
        self.preview_lane
    }

    pub fn commit_identity(&self) -> &str {
        &self.commit_identity
    }

    pub fn aspect_paths(&self) -> &[String] {
        &self.aspect_paths
    }

    pub fn execution_digest(&self) -> &str {
        &self.execution_digest
    }
}
