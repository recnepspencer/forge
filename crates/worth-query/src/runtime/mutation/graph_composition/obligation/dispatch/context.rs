use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::super::error::WorthQueryGraphObligationDispatchError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryGraphObligationDispatchContextKind {
    GraphComposition,
    AuthoritativeCommandBatch,
    ScalarMutation,
    EffectTriggeredWriteIntent,
    DeclarationEntry,
    ContributionComposed,
    ReadFamily,
    LiveRead,
    PreviewMutation,
    PreviewIntent,
    BranchIntent,
}

impl WorthQueryGraphObligationDispatchContextKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GraphComposition => "graph-composition",
            Self::AuthoritativeCommandBatch => "authoritative-command-batch",
            Self::ScalarMutation => "scalar-mutation",
            Self::EffectTriggeredWriteIntent => "effect-triggered-write-intent",
            Self::DeclarationEntry => "declaration-entry",
            Self::ContributionComposed => "contribution-composed",
            Self::ReadFamily => "read-family",
            Self::LiveRead => "live-read",
            Self::PreviewMutation => "preview-mutation",
            Self::PreviewIntent => "preview-intent",
            Self::BranchIntent => "branch-intent",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationDispatchContext {
    kind: WorthQueryGraphObligationDispatchContextKind,
    touch_descriptor_digest: String,
    operating_world_digest: String,
    context_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationDispatchContext {
    pub fn graph_composition(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        Self::new(
            WorthQueryGraphObligationDispatchContextKind::GraphComposition,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn authoritative_command_batch(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        Self::new(
            WorthQueryGraphObligationDispatchContextKind::AuthoritativeCommandBatch,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn scalar_mutation(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        Self::new(
            WorthQueryGraphObligationDispatchContextKind::ScalarMutation,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn effect_triggered_write_intent(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        Self::new(
            WorthQueryGraphObligationDispatchContextKind::EffectTriggeredWriteIntent,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn declaration_entry(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        Self::new(
            WorthQueryGraphObligationDispatchContextKind::DeclarationEntry,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn contribution_composed(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        Self::new(
            WorthQueryGraphObligationDispatchContextKind::ContributionComposed,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn read_family(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        Self::new(
            WorthQueryGraphObligationDispatchContextKind::ReadFamily,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn live_read(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        Self::new(
            WorthQueryGraphObligationDispatchContextKind::LiveRead,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn preview_mutation(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        Self::new(
            WorthQueryGraphObligationDispatchContextKind::PreviewMutation,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn preview_intent(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        Self::new(
            WorthQueryGraphObligationDispatchContextKind::PreviewIntent,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn branch_intent(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        Self::new(
            WorthQueryGraphObligationDispatchContextKind::BranchIntent,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    fn new(
        kind: WorthQueryGraphObligationDispatchContextKind,
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        let touch_descriptor_digest = non_empty(
            touch_descriptor_digest.into(),
            WorthQueryGraphObligationDispatchError::EmptyTouchDescriptorDigest,
        )?;
        let operating_world_digest = non_empty(
            operating_world_digest.into(),
            WorthQueryGraphObligationDispatchError::EmptyOperatingWorldDigest,
        )?;
        let context_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationDispatchContext)
                .field_shape(WorthQueryEvidenceTag::new("kind"), kind.as_str())
                .field_value(
                    WorthQueryEvidenceTag::new("touch_descriptor"),
                    touch_descriptor_digest.as_str(),
                )
                .field_value(
                    WorthQueryEvidenceTag::new("operating_world"),
                    operating_world_digest.as_str(),
                )
                .seal();
        Ok(Self {
            kind,
            touch_descriptor_digest,
            operating_world_digest,
            context_digest,
        })
    }

    pub fn kind(&self) -> WorthQueryGraphObligationDispatchContextKind {
        self.kind
    }

    pub fn touch_descriptor_digest(&self) -> &str {
        &self.touch_descriptor_digest
    }

    pub fn operating_world_digest(&self) -> &str {
        &self.operating_world_digest
    }

    pub fn context_digest(&self) -> &str {
        self.context_digest.as_str()
    }

    pub(crate) fn context_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.context_digest
    }
}

fn non_empty(
    value: String,
    error: WorthQueryGraphObligationDispatchError,
) -> Result<String, WorthQueryGraphObligationDispatchError> {
    let value = value.trim().to_string();
    if value.is_empty() {
        return Err(error);
    }
    Ok(value)
}
