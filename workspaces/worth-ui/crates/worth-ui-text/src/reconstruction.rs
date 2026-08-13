use std::sync::Arc;

use worth_ui_host_contract::{UiFontCollectionGeneration, UiQualifiedTextLayoutIdentity};

use crate::{
    UiGlobalFontCollection, UiQualifiedTextLayout, UiQualifiedTextLayoutRequest,
    UiQualifiedTextLayoutRequestIdentity, UiTextQualificationDenial,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiTextReconstructionDenial {
    Qualification(UiTextQualificationDenial),
    IdentityDrift,
}

pub struct UiQualifiedTextReconstructionSource {
    request: UiQualifiedTextLayoutRequest,
    expected: UiQualifiedTextLayoutIdentity,
}

impl UiQualifiedTextReconstructionSource {
    pub(crate) fn new(
        request: UiQualifiedTextLayoutRequest,
        expected: UiQualifiedTextLayoutIdentity,
    ) -> Self {
        Self { request, expected }
    }

    pub const fn request_identity(&self) -> UiQualifiedTextLayoutRequestIdentity {
        self.request.identity()
    }

    pub const fn layout_identity(&self) -> UiQualifiedTextLayoutIdentity {
        self.expected
    }

    pub fn font_collection_generation(&self) -> UiFontCollectionGeneration {
        self.request.fonts().generation()
    }

    pub fn matches_font_collection(&self, fonts: &Arc<UiGlobalFontCollection>) -> bool {
        Arc::ptr_eq(self.request.fonts(), fonts)
    }

    pub fn reconstruct(
        self: &Arc<Self>,
    ) -> Result<UiQualifiedTextLayout, UiTextReconstructionDenial> {
        let mut layout = crate::qualification::qualify_reconstruction(self.request.clone())
            .map_err(UiTextReconstructionDenial::Qualification)?;
        if layout.identity() != self.expected {
            return Err(UiTextReconstructionDenial::IdentityDrift);
        }
        layout.attach_reconstruction_source(Arc::clone(self));
        Ok(layout)
    }
}
