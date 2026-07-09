use super::WorthQueryGraphObligationDenialAttachmentProjection;
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryAuthoritativeMutationObligationDispatch, WorthQueryGraphObligationDispatchContextKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationAttachmentEvidence {
    selection_digest: String,
    dispatch_digest: String,
    envelope_digest: Option<String>,
    execution_point: Option<WorthQueryGraphObligationDispatchContextKind>,
    touch_descriptor_digest: Option<String>,
    operating_world_digest: Option<String>,
    selected_obligation_count: usize,
    denial_projection: Option<WorthQueryGraphObligationDenialAttachmentProjection>,
    evidence_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationAttachmentEvidence {
    pub(crate) fn from_dispatch(
        dispatch: &WorthQueryAuthoritativeMutationObligationDispatch,
    ) -> Self {
        let dispatch_projection = dispatch.evidence_projection();
        let denial_projection = dispatch
            .blocking_denial_projection()
            .as_ref()
            .and_then(|denial| {
                WorthQueryGraphObligationDenialAttachmentProjection::from_dispatch_projection_and_denial(
                    &dispatch_projection,
                    denial,
                )
            });
        let denial_identity = denial_projection
            .as_ref()
            .map(WorthQueryGraphObligationDenialAttachmentProjection::projection_evidence_identity);
        let evidence_digest = worth_query_evidence_identity(
            WorthQueryEvidenceScope::GraphObligationAttachmentEvidence,
        )
        .field_value(
            WorthQueryEvidenceTag::new("selection"),
            dispatch_projection.selection_digest(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("dispatch"),
            dispatch_projection.dispatch_digest(),
        )
        .optional_value(
            WorthQueryEvidenceTag::new("envelope"),
            dispatch_projection.envelope_digest(),
        )
        .optional_shape(
            WorthQueryEvidenceTag::new("execution_point"),
            dispatch_projection.context_kind().map(|kind| kind.as_str()),
        )
        .optional_value(
            WorthQueryEvidenceTag::new("touch_descriptor"),
            dispatch_projection.touch_descriptor_digest(),
        )
        .optional_value(
            WorthQueryEvidenceTag::new("operating_world"),
            dispatch_projection.operating_world_digest(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("selected_obligation_count"),
            dispatch_projection.rows().len(),
        )
        .optional_evidence_identity(
            WorthQueryEvidenceTag::new("denial_projection"),
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

    pub fn execution_point(&self) -> Option<WorthQueryGraphObligationDispatchContextKind> {
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
    ) -> Option<&WorthQueryGraphObligationDenialAttachmentProjection> {
        self.denial_projection.as_ref()
    }

    pub fn evidence_digest(&self) -> &str {
        self.evidence_digest.as_str()
    }
}
