use std::collections::BTreeMap;

use super::{ForgeServerSurfaceFamily, ForgeServerSurfaceRegistration};
use crate::diagnostics::ForgeServerCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerSurfaceRegistry {
    registrations_by_family: BTreeMap<ForgeServerSurfaceFamily, ForgeServerSurfaceRegistration>,
}

impl ForgeServerSurfaceRegistry {
    pub fn build(
        registrations: Vec<ForgeServerSurfaceRegistration>,
        counters: &ForgeServerCounters,
    ) -> Result<Self, ForgeServerSurfaceRegistryError> {
        let mut registrations_by_family = BTreeMap::new();

        for registration in registrations {
            let family = registration.family();
            if registrations_by_family
                .insert(family, registration)
                .is_some()
            {
                counters.increment_rejected_duplicate_surface_registrations();
                return Err(ForgeServerSurfaceRegistryError::DuplicateSurfaceFamily { family });
            }
        }

        counters.record_registered_surface_families(registrations_by_family.len());

        Ok(Self {
            registrations_by_family,
        })
    }

    pub fn inventory(&self) -> ForgeServerSurfaceInventory {
        ForgeServerSurfaceInventory {
            registered_families: self.registrations_by_family.keys().copied().collect(),
        }
    }

    pub fn capabilities_for(
        &self,
        family: ForgeServerSurfaceFamily,
    ) -> crate::surfaces::ForgeServerSurfaceCapabilities {
        self.registrations_by_family
            .get(&family)
            .map(ForgeServerSurfaceRegistration::capabilities)
            .unwrap_or_else(|| crate::surfaces::ForgeServerSurfaceCapabilities::absent(family))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerSurfaceRegistryError {
    DuplicateSurfaceFamily { family: ForgeServerSurfaceFamily },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerSurfaceInventory {
    pub registered_families: Vec<ForgeServerSurfaceFamily>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::ForgeServerCounters;

    #[test]
    fn duplicate_surface_family_rejection_increments_narrow_counter() {
        let counters = ForgeServerCounters::default();
        let result = ForgeServerSurfaceRegistry::build(
            vec![
                ForgeServerSurfaceRegistration::disabled(ForgeServerSurfaceFamily::ForgeNative),
                ForgeServerSurfaceRegistration::disabled(ForgeServerSurfaceFamily::ForgeNative),
            ],
            &counters,
        );

        assert_eq!(
            result,
            Err(ForgeServerSurfaceRegistryError::DuplicateSurfaceFamily {
                family: ForgeServerSurfaceFamily::ForgeNative,
            })
        );
        assert_eq!(
            counters.snapshot().rejected_duplicate_surface_registrations,
            1
        );
        assert_eq!(counters.snapshot().registered_surface_families, 0);
    }
}
