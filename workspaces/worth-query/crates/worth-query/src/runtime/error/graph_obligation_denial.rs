use crate::runtime::{
    WorthQueryAuthoritativeMutationObligationDispatch,
    WorthQueryGraphObligationDenialAttachmentProjection, WorthQueryGraphObligationDenialProjection,
    WorthQueryGraphObligationDenialProjectionRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationDenial {
    projection: WorthQueryGraphObligationDenialProjection,
    attachment_projection: WorthQueryGraphObligationDenialAttachmentProjection,
}

impl WorthQueryGraphObligationDenial {
    pub(crate) fn from_dispatch(
        dispatch: &WorthQueryAuthoritativeMutationObligationDispatch,
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

    pub fn projection(&self) -> &WorthQueryGraphObligationDenialProjection {
        &self.projection
    }

    pub fn attachment_projection(&self) -> &WorthQueryGraphObligationDenialAttachmentProjection {
        &self.attachment_projection
    }

    pub fn blocking_count(&self) -> usize {
        self.projection.blocking_count()
    }

    pub fn rows(&self) -> &[WorthQueryGraphObligationDenialProjectionRow] {
        self.projection.rows()
    }

    pub fn projection_digest(&self) -> &str {
        self.projection.projection_digest()
    }
}
