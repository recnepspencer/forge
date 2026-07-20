mod geometry;

pub use geometry::{
    WorthQueryGeometryActiveFaceSelectionHelperFamily, WorthQueryGeometryFamilyHelpers,
    WorthQueryGeometryMaterialAttachmentHelperFamily, WorthQueryGeometryMaterialAttachmentInput,
    WorthQueryGeometryNeighborhoodHelperFamily,
};

use crate::application::{
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryInstalledDomainDeclarationContext,
};

pub struct WorthQueryFamilyHelpers<
    'a,
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
> {
    handle: &'a WorthQueryInstalledDomainDeclarationContext<D, C>,
}

impl<'a, D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryFamilyHelpers<'a, D, C>
{
    pub(crate) fn new(handle: &'a WorthQueryInstalledDomainDeclarationContext<D, C>) -> Self {
        Self { handle }
    }

    pub fn geometry(&self) -> WorthQueryGeometryFamilyHelpers<'a, D, C> {
        WorthQueryGeometryFamilyHelpers::new(self.handle)
    }
}

#[cfg(test)]
mod tests;
