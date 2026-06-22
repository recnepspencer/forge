use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::super::error::ForgeQueryGraphObligationDispatchError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphObligationDispatchContextKind {
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

impl ForgeQueryGraphObligationDispatchContextKind {
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
pub struct ForgeQueryGraphObligationDispatchContext {
    kind: ForgeQueryGraphObligationDispatchContextKind,
    touch_descriptor_digest: String,
    operating_world_digest: String,
    context_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationDispatchContext {
    pub fn graph_composition(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        Self::new(
            ForgeQueryGraphObligationDispatchContextKind::GraphComposition,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn authoritative_command_batch(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        Self::new(
            ForgeQueryGraphObligationDispatchContextKind::AuthoritativeCommandBatch,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn scalar_mutation(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        Self::new(
            ForgeQueryGraphObligationDispatchContextKind::ScalarMutation,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn effect_triggered_write_intent(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        Self::new(
            ForgeQueryGraphObligationDispatchContextKind::EffectTriggeredWriteIntent,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn declaration_entry(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        Self::new(
            ForgeQueryGraphObligationDispatchContextKind::DeclarationEntry,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn contribution_composed(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        Self::new(
            ForgeQueryGraphObligationDispatchContextKind::ContributionComposed,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn read_family(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        Self::new(
            ForgeQueryGraphObligationDispatchContextKind::ReadFamily,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn live_read(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        Self::new(
            ForgeQueryGraphObligationDispatchContextKind::LiveRead,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn preview_mutation(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        Self::new(
            ForgeQueryGraphObligationDispatchContextKind::PreviewMutation,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn preview_intent(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        Self::new(
            ForgeQueryGraphObligationDispatchContextKind::PreviewIntent,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    pub fn branch_intent(
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        Self::new(
            ForgeQueryGraphObligationDispatchContextKind::BranchIntent,
            touch_descriptor_digest,
            operating_world_digest,
        )
    }

    fn new(
        kind: ForgeQueryGraphObligationDispatchContextKind,
        touch_descriptor_digest: impl Into<String>,
        operating_world_digest: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        let touch_descriptor_digest = non_empty(
            touch_descriptor_digest.into(),
            ForgeQueryGraphObligationDispatchError::EmptyTouchDescriptorDigest,
        )?;
        let operating_world_digest = non_empty(
            operating_world_digest.into(),
            ForgeQueryGraphObligationDispatchError::EmptyOperatingWorldDigest,
        )?;
        let context_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationDispatchContext)
                .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
                .field_value(
                    ForgeQueryEvidenceTag::new("touch_descriptor"),
                    touch_descriptor_digest.as_str(),
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("operating_world"),
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

    pub fn kind(&self) -> ForgeQueryGraphObligationDispatchContextKind {
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

    pub(crate) fn context_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.context_digest
    }
}

fn non_empty(
    value: String,
    error: ForgeQueryGraphObligationDispatchError,
) -> Result<String, ForgeQueryGraphObligationDispatchError> {
    let value = value.trim().to_string();
    if value.is_empty() {
        return Err(error);
    }
    Ok(value)
}
