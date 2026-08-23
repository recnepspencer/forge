use crate::application_operation::{
    WorthQueryApplicationOperationInstallationDenial,
    WorthQueryApplicationOperationInstallationDenialKind, WorthQueryInstalledApplicationOperation,
};

use super::super::super::WorthQueryInstalledPackageIndex;

impl WorthQueryInstalledPackageIndex {
    pub fn validate_application_operation<Schema, Operation, Input>(
        &self,
        installed: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    ) -> Result<(), WorthQueryApplicationOperationInstallationDenial> {
        let identity = installed.binding_identity();
        if identity.runtime_ordinal() != self.runtime_ordinal() {
            return Err(application_operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::ForeignRuntime,
                installed,
            ));
        }
        if identity.generation() != self.generation().ordinal() {
            return Err(application_operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::StaleGeneration,
                installed,
            ));
        }
        let schema = self
            .application_schemas
            .get(&(
                installed.owner().to_string(),
                installed.schema_name().to_string(),
            ))
            .ok_or_else(|| {
                application_operation_denial(
                    WorthQueryApplicationOperationInstallationDenialKind::SchemaMeaningChanged,
                    installed,
                )
            })?;
        let package = self.domain(installed.owner()).map_err(|_| {
            application_operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::PackageIdentityChanged,
                installed,
            )
        })?;
        if package.package_identity().digest() != identity.package_identity() {
            return Err(application_operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::PackageIdentityChanged,
                installed,
            ));
        }
        if !installed.authority_matches(&package) {
            return Err(application_operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::AuthorityMismatch,
                installed,
            ));
        }
        if !installed.meaning_matches(schema.declaration().members()) {
            return Err(application_operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::OperationMeaningChanged,
                installed,
            ));
        }
        Ok(())
    }
}

fn application_operation_denial<Schema, Operation, Input>(
    kind: WorthQueryApplicationOperationInstallationDenialKind,
    installed: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
) -> WorthQueryApplicationOperationInstallationDenial {
    WorthQueryApplicationOperationInstallationDenial::new(kind, installed.operation())
}
