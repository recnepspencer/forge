use super::ForgeQueryGraphObligationDenialAttachmentProjection;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryAuthoritativeMutationObligationDispatch, ForgeQueryGraphObligationDispatchContextKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationAttachmentEvidence {
    selection_digest: String,
    dispatch_digest: String,
    envelope_digest: Option<String>,
    execution_point: Option<ForgeQueryGraphObligationDispatchContextKind>,
    touch_descriptor_digest: Option<String>,
    operating_world_digest: Option<String>,
    selected_obligation_count: usize,
    denial_projection: Option<ForgeQueryGraphObligationDenialAttachmentProjection>,
    evidence_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationAttachmentEvidence {
    pub(crate) fn from_dispatch(
        dispatch: &ForgeQueryAuthoritativeMutationObligationDispatch,
    ) -> Self {
        let dispatch_projection = dispatch.evidence_projection();
        let denial_projection = dispatch
            .blocking_denial_projection()
            .as_ref()
            .and_then(|denial| {
                ForgeQueryGraphObligationDenialAttachmentProjection::from_dispatch_projection_and_denial(
                    &dispatch_projection,
                    denial,
                )
            });
        let denial_identity = denial_projection
            .as_ref()
            .map(ForgeQueryGraphObligationDenialAttachmentProjection::projection_evidence_identity);
        let evidence_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::GraphObligationAttachmentEvidence,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("selection"),
            dispatch_projection.selection_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("dispatch"),
            dispatch_projection.dispatch_digest(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("envelope"),
            dispatch_projection.envelope_digest(),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("execution_point"),
            dispatch_projection.context_kind().map(|kind| kind.as_str()),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("touch_descriptor"),
            dispatch_projection.touch_descriptor_digest(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("operating_world"),
            dispatch_projection.operating_world_digest(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("selected_obligation_count"),
            dispatch_projection.rows().len(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("denial_projection"),
            denial_identity,
        )
        .seal();
        Self {
            selection_digest: dispatch_projection.selection_digest().to_string(),
            dispatch_digest: dispatch_projection.dispatch_digest().to_string(),
            envelope_digest: dispatch_projection.envelope_digest().map(str::to_string),
            execution_point: dispatch_projection.context_kind(),
            touch_descriptor_digest: dispatch_projection
                .touch_descriptor_digest()
                .map(str::to_string),
            operating_world_digest: dispatch_projection
                .operating_world_digest()
                .map(str::to_string),
            selected_obligation_count: dispatch_projection.rows().len(),
            denial_projection,
            evidence_digest,
        }
    }

    pub fn selection_digest(&self) -> &str {
        &self.selection_digest
    }

    pub fn dispatch_digest(&self) -> &str {
        &self.dispatch_digest
    }

    pub fn envelope_digest(&self) -> Option<&str> {
        self.envelope_digest.as_deref()
    }

    pub fn execution_point(&self) -> Option<ForgeQueryGraphObligationDispatchContextKind> {
        self.execution_point
    }

    pub fn touch_descriptor_digest(&self) -> Option<&str> {
        self.touch_descriptor_digest.as_deref()
    }

    pub fn operating_world_digest(&self) -> Option<&str> {
        self.operating_world_digest.as_deref()
    }

    pub fn selected_obligation_count(&self) -> usize {
        self.selected_obligation_count
    }

    pub fn denial_projection(
        &self,
    ) -> Option<&ForgeQueryGraphObligationDenialAttachmentProjection> {
        self.denial_projection.as_ref()
    }

    pub fn evidence_digest(&self) -> &str {
        self.evidence_digest.as_str()
    }
}
