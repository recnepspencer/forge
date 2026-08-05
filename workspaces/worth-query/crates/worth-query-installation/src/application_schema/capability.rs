use worth_query_declaration::facade::{
    application_capability::ApplicationCapabilityRef,
    application_schema::{ApplicationOperationRef, ApplicationSchema},
};

use crate::application_capability::{
    WorthQueryApplicationCapabilityInstallationDenial,
    WorthQueryApplicationCapabilityInstallationDenialKind,
    WorthQueryInstalledApplicationCapability,
};

use super::WorthQueryInstalledApplicationSchema;

impl<Schema> WorthQueryInstalledApplicationSchema<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn capability<Capability, Operation, Input>(
        &self,
        capability: ApplicationCapabilityRef<Schema, Capability>,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
    ) -> Result<
        WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
        WorthQueryApplicationCapabilityInstallationDenial,
    > {
        WorthQueryInstalledApplicationCapability::from_installed_schema(self, capability, operation)
    }

    pub fn validate_installed_capability<Capability, Operation, Input>(
        &self,
        capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
    ) -> Result<(), WorthQueryApplicationCapabilityInstallationDenial> {
        let expected = self.binding_identity();
        let actual = capability.binding_identity();
        let kind = if actual.runtime_ordinal() != expected.runtime_ordinal() {
            Some(WorthQueryApplicationCapabilityInstallationDenialKind::ForeignRuntime)
        } else if actual.generation() != expected.generation() {
            Some(WorthQueryApplicationCapabilityInstallationDenialKind::StaleGeneration)
        } else if actual.package_identity() != expected.package_identity() {
            Some(WorthQueryApplicationCapabilityInstallationDenialKind::PackageIdentityChanged)
        } else if actual.schema_identity() != expected.schema_identity() {
            Some(WorthQueryApplicationCapabilityInstallationDenialKind::SchemaMeaningChanged)
        } else {
            None
        };
        if let Some(kind) = kind {
            return Err(WorthQueryApplicationCapabilityInstallationDenial::new(
                kind,
                capability.contract().name(),
            ));
        }
        if !capability.authority_matches(&self.package_authority) {
            return Err(WorthQueryApplicationCapabilityInstallationDenial::new(
                WorthQueryApplicationCapabilityInstallationDenialKind::AuthorityMismatch,
                capability.contract().name(),
            ));
        }
        Ok(())
    }
}
