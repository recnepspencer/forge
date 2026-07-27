use worth_query_declaration::facade::application_schema::{
    ApplicationSchema, ApplicationSchemaDeclaration,
};

use crate::application_schema::{
    WorthQueryInstalledApplicationSchema, WorthQueryInstalledApplicationSchemaDenial,
    WorthQueryInstalledApplicationSchemaDenialKind,
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
        Ok(WorthQueryInstalledApplicationSchema::new(
            authority,
            &declaration,
        ))
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
        if current.identity() != &installed.schema_identity {
            return Err(WorthQueryInstalledApplicationSchemaDenial::new(
                WorthQueryInstalledApplicationSchemaDenialKind::SchemaMeaningChanged,
                &installed.schema_name,
            ));
        }
        Ok(())
    }
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
