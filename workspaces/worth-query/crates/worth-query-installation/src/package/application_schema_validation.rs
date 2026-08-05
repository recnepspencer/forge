use worth_query_declaration::facade::application_schema::ErasedApplicationSchemaDeclaration;

use super::{WorthQueryPortableDomainIdentity, WorthQueryPortablePackageValidationDenial};

pub(super) fn validate_application_schemas(
    identity: &WorthQueryPortableDomainIdentity,
    schemas: &[ErasedApplicationSchemaDeclaration],
) -> Result<(), WorthQueryPortablePackageValidationDenial> {
    for schema in schemas {
        if schema.owner() != identity.owner()
            || schema.major() != identity.major()
            || schema.minor() != identity.minor()
        {
            return Err(
                WorthQueryPortablePackageValidationDenial::application_schema_identity_mismatch(
                    schema.name(),
                ),
            );
        }
    }
    for pair in schemas.windows(2) {
        if pair[0].name() != pair[1].name() {
            continue;
        }
        if pair[0] == pair[1] {
            return Err(
                WorthQueryPortablePackageValidationDenial::duplicate_application_schema(
                    pair[0].name(),
                ),
            );
        }
        return Err(
            WorthQueryPortablePackageValidationDenial::conflicting_application_schema(
                pair[0].name(),
            ),
        );
    }
    Ok(())
}
