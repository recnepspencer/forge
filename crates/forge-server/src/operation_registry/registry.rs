use std::collections::BTreeMap;

use crate::{
    diagnostics::ForgeServerCounters, ForgeServerOperationAuthorityMetadata,
    ForgeServerOperationAuthorizationPolicy, ForgeServerOperationCapabilities,
    ForgeServerOperationDenial, ForgeServerOperationFamily, ForgeServerOperationInventory,
    ForgeServerOperationInventoryRow, ForgeServerOperationRegistration,
    ForgeServerOperationRequest, ForgeServerSurfaceFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationRegistry {
    registrations_by_family: BTreeMap<ForgeServerOperationFamily, ForgeServerOperationRegistration>,
}

impl ForgeServerOperationRegistry {
    pub fn build(
        registrations: Vec<ForgeServerOperationRegistration>,
        counters: &ForgeServerCounters,
    ) -> Result<Self, ForgeServerOperationRegistryError> {
        let mut registrations_by_family = BTreeMap::new();

        for registration in registrations {
            let family = registration.family();
            if let Err(detail) = registration.validate() {
                return Err(ForgeServerOperationRegistryError::InvalidRegistration {
                    family,
                    detail,
                });
            }
            if registrations_by_family
                .insert(family, registration)
                .is_some()
            {
                counters.increment_rejected_duplicate_operation_registrations();
                return Err(ForgeServerOperationRegistryError::DuplicateOperationFamily { family });
            }
        }

        counters.record_registered_operation_families(registrations_by_family.len());

        Ok(Self {
            registrations_by_family,
        })
    }

    pub fn inventory(&self) -> ForgeServerOperationInventory {
        ForgeServerOperationInventory::new(
            self.registrations_by_family
                .values()
                .map(|registration| {
                    ForgeServerOperationInventoryRow::new(
                        registration.family(),
                        registration.is_enabled(),
                        registration.exposed_surfaces().to_vec(),
                    )
                })
                .collect(),
        )
    }

    pub fn capabilities_for(
        &self,
        family: ForgeServerOperationFamily,
    ) -> ForgeServerOperationCapabilities {
        self.registrations_by_family
            .get(&family)
            .map(|registration| {
                if registration.is_enabled() {
                    ForgeServerOperationCapabilities::enabled(
                        family,
                        registration.exposed_surfaces().to_vec(),
                    )
                } else {
                    ForgeServerOperationCapabilities::disabled(
                        family,
                        registration.exposed_surfaces().to_vec(),
                    )
                }
            })
            .unwrap_or_else(|| ForgeServerOperationCapabilities::absent(family))
    }

    pub fn declared_authority_for(
        &self,
        operation_request: &ForgeServerOperationRequest,
    ) -> Result<ForgeServerOperationAuthorityMetadata, String> {
        let family = operation_request.identity().operation_family();
        let Some(registration) = self.registrations_by_family.get(&family) else {
            return Err(format!(
                "operation family `{}` has no registered authority declaration",
                family.as_str()
            ));
        };
        let Some(declaration) = registration.authority_declaration() else {
            return Err(format!(
                "operation family `{}` has no declared authority template",
                family.as_str()
            ));
        };
        declaration.lower(operation_request)
    }

    pub fn authorization_policy_for(
        &self,
        family: ForgeServerOperationFamily,
    ) -> Option<&ForgeServerOperationAuthorizationPolicy> {
        self.registrations_by_family
            .get(&family)
            .and_then(|registration| registration.authorization_policy())
    }

    pub fn admit(
        &self,
        surface_family: ForgeServerSurfaceFamily,
        family: ForgeServerOperationFamily,
    ) -> Result<ForgeServerOperationCapabilities, ForgeServerOperationDenial> {
        let capabilities = self.capabilities_for(family);
        if capabilities.is_absent() {
            return Err(ForgeServerOperationDenial::UnregisteredFamily {
                family,
                surface_family,
            });
        }
        if capabilities.is_disabled() {
            return Err(ForgeServerOperationDenial::DisabledFamily {
                family,
                surface_family,
            });
        }
        if !capabilities.exposed_surfaces().contains(&surface_family) {
            return Err(ForgeServerOperationDenial::SurfaceFamilyNotExposed {
                family,
                surface_family,
            });
        }
        Ok(capabilities)
    }

    pub fn admit_operation_name(
        &self,
        family: ForgeServerOperationFamily,
        operation_name: &str,
    ) -> Result<(), ForgeServerOperationDenial> {
        let Some(registration) = self.registrations_by_family.get(&family) else {
            return Ok(());
        };
        if registration.admitted_operation_names().is_empty() {
            return Ok(());
        }
        let canonical_operation_name = operation_name.trim().to_ascii_lowercase();
        if registration
            .admitted_operation_names()
            .iter()
            .any(|candidate| candidate == &canonical_operation_name)
        {
            return Ok(());
        }
        Err(ForgeServerOperationDenial::UnknownOperationName {
            family,
            operation_name: canonical_operation_name,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerOperationRegistryError {
    DuplicateOperationFamily {
        family: ForgeServerOperationFamily,
    },
    InvalidRegistration {
        family: ForgeServerOperationFamily,
        detail: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::ForgeServerCounters;

    #[test]
    fn duplicate_operation_family_rejection_increments_narrow_counter() {
        let counters = ForgeServerCounters::default();
        let result = ForgeServerOperationRegistry::build(
            vec![
                ForgeServerOperationRegistration::disabled(
                    ForgeServerOperationFamily::QueryDirectRead,
                ),
                ForgeServerOperationRegistration::disabled(
                    ForgeServerOperationFamily::QueryDirectRead,
                ),
            ],
            &counters,
        );

        assert_eq!(
            result,
            Err(
                ForgeServerOperationRegistryError::DuplicateOperationFamily {
                    family: ForgeServerOperationFamily::QueryDirectRead,
                },
            )
        );
        assert_eq!(
            counters
                .snapshot()
                .rejected_duplicate_operation_registrations,
            1
        );
        assert_eq!(counters.snapshot().registered_operation_families, 0);
    }
}
