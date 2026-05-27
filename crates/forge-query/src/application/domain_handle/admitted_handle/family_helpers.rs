use crate::family_helpers::{ForgeQueryFamilyHelpers, ForgeQueryGeometryFamilyHelpers};

use super::ForgeQueryAdmittedConfiguredDomainHandle;
use crate::application::{ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext};

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn family_helpers(&self) -> ForgeQueryFamilyHelpers<'_, D, C> {
        ForgeQueryFamilyHelpers::new(self)
    }

    pub fn geometry_helpers(&self) -> ForgeQueryGeometryFamilyHelpers<'_, D, C> {
        self.family_helpers().geometry()
    }
}
