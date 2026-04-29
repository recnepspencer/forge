use super::super::*;

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
    label: String,
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
        label: &str,
        kind: ForgeQueryPreviewExecutionKind,
        handle_name: &str,
        source_lane: ForgeQueryAuthorityLane,
        preview_lane: ForgeQueryAuthorityLane,
        commit_identity: &str,
        aspect_paths: Vec<String>,
    ) -> Self {
        let execution_digest = hash_parts(&[
            "forge_query_preview_execution_evidence_v1".to_string(),
            format!("label:{label}"),
            format!("kind:{}", kind.as_str()),
            format!("handle:{handle_name}"),
            format!("source_lane:{source_lane}"),
            format!("preview_lane:{preview_lane}"),
            format!("commit:{commit_identity}"),
            format!("aspects:{}", aspect_paths.join("|")),
        ]);
        Self {
            label: label.to_string(),
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
        &self.label
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
