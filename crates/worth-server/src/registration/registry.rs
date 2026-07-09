use std::collections::BTreeMap;

use super::{WorthServerSurfaceFamily, WorthServerSurfaceRegistration};
use crate::diagnostics::WorthServerCounters;
use crate::surfaces::compat_http::WorthServerCompatHttpRouteFamilies;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerSurfaceRegistry {
    registrations_by_family: BTreeMap<WorthServerSurfaceFamily, WorthServerSurfaceRegistration>,
}

impl WorthServerSurfaceRegistry {
    pub fn build(
        registrations: Vec<WorthServerSurfaceRegistration>,
        counters: &WorthServerCounters,
    ) -> Result<Self, WorthServerSurfaceRegistryError> {
        let mut registrations_by_family = BTreeMap::new();

        for registration in registrations {
            let family = registration.family();
            if registrations_by_family
                .insert(family, registration)
                .is_some()
            {
                counters.increment_rejected_duplicate_surface_registrations();
                return Err(WorthServerSurfaceRegistryError::DuplicateSurfaceFamily { family });
            }
        }

        counters.record_registered_surface_families(registrations_by_family.len());

        Ok(Self {
            registrations_by_family,
        })
    }

    pub fn inventory(&self) -> WorthServerSurfaceInventory {
        WorthServerSurfaceInventory {
            registered_families: self.registrations_by_family.keys().copied().collect(),
        }
    }

    pub fn capabilities_for(
        &self,
        family: WorthServerSurfaceFamily,
    ) -> crate::surfaces::WorthServerSurfaceCapabilities {
        self.registrations_by_family
            .get(&family)
            .map(WorthServerSurfaceRegistration::capabilities)
            .unwrap_or_else(|| crate::surfaces::WorthServerSurfaceCapabilities::absent(family))
    }

    pub(crate) fn compat_http_route_families(&self) -> WorthServerCompatHttpRouteFamilies {
        self.registrations_by_family
            .get(&WorthServerSurfaceFamily::CompatHttp)
            .and_then(WorthServerSurfaceRegistration::compat_http_route_families)
            .unwrap_or_default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerSurfaceRegistryError {
    DuplicateSurfaceFamily { family: WorthServerSurfaceFamily },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerSurfaceInventory {
    pub registered_families: Vec<WorthServerSurfaceFamily>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::WorthServerCounters;

    #[test]
    fn duplicate_surface_family_rejection_increments_narrow_counter() {
        let counters = WorthServerCounters::default();
        let result = WorthServerSurfaceRegistry::build(
            vec![
                WorthServerSurfaceRegistration::disabled(WorthServerSurfaceFamily::WorthNative),
                WorthServerSurfaceRegistration::disabled(WorthServerSurfaceFamily::WorthNative),
            ],
            &counters,
        );

        assert_eq!(
            result,
            Err(WorthServerSurfaceRegistryError::DuplicateSurfaceFamily {
                family: WorthServerSurfaceFamily::WorthNative,
            })
        );
        assert_eq!(
            counters.snapshot().rejected_duplicate_surface_registrations,
            1
        );
        assert_eq!(counters.snapshot().registered_surface_families, 0);
    }
}
