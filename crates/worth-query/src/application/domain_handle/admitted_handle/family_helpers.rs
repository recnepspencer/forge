use crate::family_helpers::{WorthQueryFamilyHelpers, WorthQueryGeometryFamilyHelpers};

use super::WorthQueryAdmittedConfiguredDomainHandle;
use crate::application::{WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext};

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn family_helpers(&self) -> WorthQueryFamilyHelpers<'_, D, C> {
        WorthQueryFamilyHelpers::new(self)
    }

    pub fn geometry_helpers(&self) -> WorthQueryGeometryFamilyHelpers<'_, D, C> {
        self.family_helpers().geometry()
    }
}
