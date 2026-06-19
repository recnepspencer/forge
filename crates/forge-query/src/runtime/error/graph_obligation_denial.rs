use crate::runtime::{
    ForgeQueryAuthoritativeMutationObligationDispatch,
    ForgeQueryGraphObligationDenialAttachmentProjection, ForgeQueryGraphObligationDenialProjection,
    ForgeQueryGraphObligationDenialProjectionRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationDenial {
    projection: ForgeQueryGraphObligationDenialProjection,
    attachment_projection: ForgeQueryGraphObligationDenialAttachmentProjection,
}

impl ForgeQueryGraphObligationDenial {
    pub(crate) fn from_dispatch(
        dispatch: &ForgeQueryAuthoritativeMutationObligationDispatch,
    ) -> Option<Self> {
        let projection = dispatch.blocking_denial_projection()?;
        let attachment_projection = dispatch
            .attachment_evidence()
            .denial_projection()
            .cloned()?;
        Some(Self {
            projection,
            attachment_projection,
        })
    }

    pub fn projection(&self) -> &ForgeQueryGraphObligationDenialProjection {
        &self.projection
    }

    pub fn attachment_projection(&self) -> &ForgeQueryGraphObligationDenialAttachmentProjection {
        &self.attachment_projection
    }

    pub fn blocking_count(&self) -> usize {
        self.projection.blocking_count()
    }

    pub fn rows(&self) -> &[ForgeQueryGraphObligationDenialProjectionRow] {
        self.projection.rows()
    }

    pub fn projection_digest(&self) -> &str {
        self.projection.projection_digest()
    }
}
