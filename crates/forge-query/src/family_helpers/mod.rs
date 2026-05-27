mod geometry;

pub use geometry::{
    ForgeQueryGeometryActiveFaceSelectionHelperFamily, ForgeQueryGeometryFamilyHelpers,
    ForgeQueryGeometryMaterialAttachmentHelperFamily, ForgeQueryGeometryMaterialAttachmentInput,
};

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};

pub struct ForgeQueryFamilyHelpers<
    'a,
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
> {
    handle: &'a ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
}

impl<'a, D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryFamilyHelpers<'a, D, C>
{
    pub(crate) fn new(handle: &'a ForgeQueryAdmittedConfiguredDomainHandle<D, C>) -> Self {
        Self { handle }
    }

    pub fn geometry(&self) -> ForgeQueryGeometryFamilyHelpers<'a, D, C> {
        ForgeQueryGeometryFamilyHelpers::new(self.handle)
    }
}

#[cfg(test)]
mod tests;
