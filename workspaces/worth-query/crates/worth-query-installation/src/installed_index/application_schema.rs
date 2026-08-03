use worth_query_declaration::facade::application_schema::{
    ApplicationSchema, ApplicationSchemaDeclaration,
};

use crate::application_ability::{
    WorthQueryAbilityInstallationDenial, WorthQueryAbilityInstallationDenialKind,
    WorthQueryInstalledAbility,
};
use crate::application_operation::{
    WorthQueryApplicationOperationInstallationDenial,
    WorthQueryApplicationOperationInstallationDenialKind, WorthQueryInstalledApplicationOperation,
};
use crate::application_principal_binding::{
    WorthQueryInstalledPrincipalBinding, WorthQueryPrincipalBindingInstallationDenial,
    WorthQueryPrincipalBindingInstallationDenialKind,
};
use crate::application_schema::{
    ApplicationSchemaCompilationDenial, WorthQueryInstalledApplicationSchema,
    WorthQueryInstalledApplicationSchemaDenial, WorthQueryInstalledApplicationSchemaDenialKind,
};

use super::{
    WorthQueryInstalledPackageIndex, WorthQueryInstalledPackageIndexDenial,
    WorthQueryInstalledPackageIndexDenialKind,
};

impl WorthQueryInstalledPackageIndex {
    pub fn bind_application_schema<Schema>(
        &self,
        declaration: ApplicationSchemaDeclaration<Schema>,
    ) -> Result<
        WorthQueryInstalledApplicationSchema<Schema>,
        WorthQueryInstalledApplicationSchemaDenial,
    >
    where
        Schema: ApplicationSchema,
    {
        let schema = declaration.erased();
        let installed = self
            .application_schemas
            .get(&(schema.owner().to_string(), schema.name().to_string()))
            .ok_or_else(|| {
                WorthQueryInstalledApplicationSchemaDenial::new(
                    WorthQueryInstalledApplicationSchemaDenialKind::SchemaNotInstalled,
                    schema.name(),
                )
            })?;
        if installed != schema {
            return Err(WorthQueryInstalledApplicationSchemaDenial::new(
                WorthQueryInstalledApplicationSchemaDenialKind::SchemaMeaningChanged,
                schema.name(),
            ));
        }
        let authority = self
            .domain(schema.owner())
            .map_err(map_index_denial_to_schema_denial)?;
        WorthQueryInstalledApplicationSchema::new(
            authority,
            &declaration,
            self.installation_canonical_work(),
        )
        .map_err(|denial| map_compilation_denial(schema.name(), denial))
    }

    pub fn validate_application_schema<Schema>(
        &self,
        installed: &WorthQueryInstalledApplicationSchema<Schema>,
    ) -> Result<(), WorthQueryInstalledApplicationSchemaDenial>
    where
        Schema: ApplicationSchema,
    {
        self.validate(&installed.package_authority)
            .map_err(map_index_denial_to_schema_denial)?;
        let current = self
            .application_schemas
            .get(&(
                installed.package_authority.owner.clone(),
                installed.schema_name.clone(),
            ))
            .ok_or_else(|| {
                WorthQueryInstalledApplicationSchemaDenial::new(
                    WorthQueryInstalledApplicationSchemaDenialKind::SchemaNotInstalled,
                    &installed.schema_name,
                )
            })?;
        if current != &installed.schema {
            return Err(WorthQueryInstalledApplicationSchemaDenial::new(
                WorthQueryInstalledApplicationSchemaDenialKind::SchemaMeaningChanged,
                &installed.schema_name,
            ));
        }
        Ok(())
    }

    pub fn validate_principal_binding<Schema, Binding, Mapping, Principal, PrincipalIdentity>(
        &self,
        installed: &WorthQueryInstalledPrincipalBinding<
            Schema,
            Binding,
            Mapping,
            Principal,
            PrincipalIdentity,
        >,
    ) -> Result<(), WorthQueryPrincipalBindingInstallationDenial> {
        let identity = installed.binding_identity();
        if identity.runtime_ordinal() != self.runtime_ordinal() {
            return Err(principal_binding_denial(
                WorthQueryPrincipalBindingInstallationDenialKind::ForeignRuntime,
                installed,
            ));
        }
        if identity.generation() != self.generation().ordinal() {
            return Err(principal_binding_denial(
                WorthQueryPrincipalBindingInstallationDenialKind::StaleGeneration,
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
                principal_binding_denial(
                    WorthQueryPrincipalBindingInstallationDenialKind::SchemaMeaningChanged,
                    installed,
                )
            })?;
        let package = self.domain(installed.owner()).map_err(|_| {
            principal_binding_denial(
                WorthQueryPrincipalBindingInstallationDenialKind::PackageIdentityChanged,
                installed,
            )
        })?;
        if package.package_identity().digest() != identity.package_identity() {
            return Err(principal_binding_denial(
                WorthQueryPrincipalBindingInstallationDenialKind::PackageIdentityChanged,
                installed,
            ));
        }
        if !installed.authority_matches(&package) {
            return Err(principal_binding_denial(
                WorthQueryPrincipalBindingInstallationDenialKind::AuthorityMismatch,
                installed,
            ));
        }
        let meaning_matches = schema
            .members()
            .iter()
            .any(|member| installed.meaning_matches(member));
        if !meaning_matches {
            return Err(principal_binding_denial(
                WorthQueryPrincipalBindingInstallationDenialKind::BindingMeaningChanged,
                installed,
            ));
        }
        Ok(())
    }

    pub fn validate_ability<Schema, Ability, Scope>(
        &self,
        installed: &WorthQueryInstalledAbility<Schema, Ability, Scope>,
    ) -> Result<(), WorthQueryAbilityInstallationDenial> {
        let identity = installed.binding_identity();
        if identity.runtime_ordinal() != self.runtime_ordinal() {
            return Err(ability_denial(
                WorthQueryAbilityInstallationDenialKind::ForeignRuntime,
                installed,
            ));
        }
        if identity.generation() != self.generation().ordinal() {
            return Err(ability_denial(
                WorthQueryAbilityInstallationDenialKind::StaleGeneration,
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
                ability_denial(
                    WorthQueryAbilityInstallationDenialKind::SchemaMeaningChanged,
                    installed,
                )
            })?;
        let package = self.domain(installed.owner()).map_err(|_| {
            ability_denial(
                WorthQueryAbilityInstallationDenialKind::PackageIdentityChanged,
                installed,
            )
        })?;
        if package.package_identity().digest() != identity.package_identity() {
            return Err(ability_denial(
                WorthQueryAbilityInstallationDenialKind::PackageIdentityChanged,
                installed,
            ));
        }
        if !installed.authority_matches(&package) {
            return Err(ability_denial(
                WorthQueryAbilityInstallationDenialKind::AuthorityMismatch,
                installed,
            ));
        }
        if !installed.meaning_matches(schema.members()) {
            return Err(ability_denial(
                WorthQueryAbilityInstallationDenialKind::AbilityMeaningChanged,
                installed,
            ));
        }
        Ok(())
    }

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
        if !installed.meaning_matches(schema.members()) {
            return Err(application_operation_denial(
                WorthQueryApplicationOperationInstallationDenialKind::OperationMeaningChanged,
                installed,
            ));
        }
        Ok(())
    }
}

fn map_compilation_denial(
    schema: &str,
    denial: ApplicationSchemaCompilationDenial,
) -> WorthQueryInstalledApplicationSchemaDenial {
    let (kind, subject) = match denial {
        ApplicationSchemaCompilationDenial::Capability(denial) => {
            let kind = match denial.kind() {
                crate::application_capability::WorthQueryApplicationCapabilityInstallationDenialKind::CanonicalEntryLimitExceeded => {
                    WorthQueryInstalledApplicationSchemaDenialKind::CanonicalEntryBudgetExceeded
                }
                crate::application_capability::WorthQueryApplicationCapabilityInstallationDenialKind::CanonicalByteLimitExceeded => {
                    WorthQueryInstalledApplicationSchemaDenialKind::CanonicalEncodedByteBudgetExceeded
                }
                _ => WorthQueryInstalledApplicationSchemaDenialKind::CapabilityInstallationDenied,
            };
            (kind, denial.subject().to_string())
        }
        ApplicationSchemaCompilationDenial::Canonical(
            worth_foundational::facade::CanonicalDigestDerivationDenial::EntryLimitExceeded {
                ..
            },
        ) => (
            WorthQueryInstalledApplicationSchemaDenialKind::CanonicalEntryBudgetExceeded,
            schema.to_string(),
        ),
        ApplicationSchemaCompilationDenial::Canonical(
            worth_foundational::facade::CanonicalDigestDerivationDenial::EncodedByteLimitExceeded {
                ..
            },
        ) => (
            WorthQueryInstalledApplicationSchemaDenialKind::CanonicalEncodedByteBudgetExceeded,
            schema.to_string(),
        ),
        ApplicationSchemaCompilationDenial::Canonical(_) => (
            WorthQueryInstalledApplicationSchemaDenialKind::CanonicalDigestSlotRejected,
            schema.to_string(),
        ),
    };
    WorthQueryInstalledApplicationSchemaDenial::new(kind, subject)
}

fn ability_denial<Schema, Ability, Scope>(
    kind: WorthQueryAbilityInstallationDenialKind,
    installed: &WorthQueryInstalledAbility<Schema, Ability, Scope>,
) -> WorthQueryAbilityInstallationDenial {
    WorthQueryAbilityInstallationDenial::new(kind, installed.ability())
}

fn application_operation_denial<Schema, Operation, Input>(
    kind: WorthQueryApplicationOperationInstallationDenialKind,
    installed: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
) -> WorthQueryApplicationOperationInstallationDenial {
    WorthQueryApplicationOperationInstallationDenial::new(kind, installed.operation())
}

fn principal_binding_denial<Schema, Binding, Mapping, Principal, PrincipalIdentity>(
    kind: WorthQueryPrincipalBindingInstallationDenialKind,
    installed: &WorthQueryInstalledPrincipalBinding<
        Schema,
        Binding,
        Mapping,
        Principal,
        PrincipalIdentity,
    >,
) -> WorthQueryPrincipalBindingInstallationDenial {
    WorthQueryPrincipalBindingInstallationDenial::new(kind, installed.binding())
}

fn map_index_denial_to_schema_denial(
    denial: WorthQueryInstalledPackageIndexDenial,
) -> WorthQueryInstalledApplicationSchemaDenial {
    let kind = match denial.kind() {
        WorthQueryInstalledPackageIndexDenialKind::DomainNotInstalled => {
            WorthQueryInstalledApplicationSchemaDenialKind::DomainNotInstalled
        }
        WorthQueryInstalledPackageIndexDenialKind::ForeignRuntime => {
            WorthQueryInstalledApplicationSchemaDenialKind::ForeignRuntime
        }
        WorthQueryInstalledPackageIndexDenialKind::StaleGeneration => {
            WorthQueryInstalledApplicationSchemaDenialKind::StaleGeneration
        }
        WorthQueryInstalledPackageIndexDenialKind::PackageIdentityChanged => {
            WorthQueryInstalledApplicationSchemaDenialKind::PackageIdentityChanged
        }
        WorthQueryInstalledPackageIndexDenialKind::AdmissionIdentityChanged => {
            WorthQueryInstalledApplicationSchemaDenialKind::AdmissionIdentityChanged
        }
        WorthQueryInstalledPackageIndexDenialKind::AuthorityMismatch => {
            WorthQueryInstalledApplicationSchemaDenialKind::AuthorityMismatch
        }
        _ => WorthQueryInstalledApplicationSchemaDenialKind::SchemaMeaningChanged,
    };
    WorthQueryInstalledApplicationSchemaDenial::new(kind, denial.subject())
}
