use super::super::*;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::ForgeQueryAspectTouch;

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
    source_evidence_identity: ForgeQueryEvidenceIdentity,
    aspect_touches: Vec<ForgeQueryAspectTouch>,
    intent_strategy_name: Option<String>,
    execution_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryPreviewExecutionEvidence {
    pub(in crate::runtime::preview) fn for_aspect_touches(
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        kind: ForgeQueryPreviewExecutionKind,
        handle_name: &str,
        source_lane: ForgeQueryAuthorityLane,
        preview_lane: ForgeQueryAuthorityLane,
        source_evidence_identity: &ForgeQueryEvidenceIdentity,
        aspect_touches: Vec<ForgeQueryAspectTouch>,
    ) -> Self {
        Self::new(
            basis_admission,
            kind,
            handle_name,
            source_lane,
            preview_lane,
            source_evidence_identity,
            aspect_touches,
            None,
        )
    }

    pub(in crate::runtime::preview) fn for_intent_strategy(
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        handle_name: &str,
        source_evidence_identity: &ForgeQueryEvidenceIdentity,
        intent_strategy_name: &str,
    ) -> Self {
        Self::new(
            basis_admission,
            ForgeQueryPreviewExecutionKind::PendingWriteIntent,
            handle_name,
            ForgeQueryAuthorityLane::PendingWriteIntent,
            ForgeQueryAuthorityLane::PreviewTruth,
            source_evidence_identity,
            Vec::new(),
            Some(intent_strategy_name.to_string()),
        )
    }

    fn new(
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        kind: ForgeQueryPreviewExecutionKind,
        handle_name: &str,
        source_lane: ForgeQueryAuthorityLane,
        preview_lane: ForgeQueryAuthorityLane,
        source_evidence_identity: &ForgeQueryEvidenceIdentity,
        aspect_touches: Vec<ForgeQueryAspectTouch>,
        intent_strategy_name: Option<String>,
    ) -> Self {
        let execution_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewExecutionEvidence)
                .field_value(
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
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("source_evidence_identity"),
                    source_evidence_identity,
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("admitted_aspect_touch"),
                    aspect_touches
                        .iter()
                        .map(|touch| touch.admitted_touch_digest_part()),
                )
                .optional_shape(
                    ForgeQueryEvidenceTag::new("intent_strategy_name"),
                    intent_strategy_name.as_deref(),
                )
                .seal();
        Self {
            session_label: basis_admission.session_label().clone(),
            kind,
            handle_name: handle_name.to_string(),
            source_lane,
            preview_lane,
            source_evidence_identity: source_evidence_identity.clone(),
            aspect_touches,
            intent_strategy_name,
            execution_identity,
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

    pub fn source_evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.source_evidence_identity
    }

    pub fn aspect_touches(&self) -> &[ForgeQueryAspectTouch] {
        &self.aspect_touches
    }

    pub fn intent_strategy_name(&self) -> Option<&str> {
        self.intent_strategy_name.as_deref()
    }

    pub fn execution_digest(&self) -> &str {
        self.execution_identity.as_str()
    }

    pub fn execution_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.execution_identity
    }
}
