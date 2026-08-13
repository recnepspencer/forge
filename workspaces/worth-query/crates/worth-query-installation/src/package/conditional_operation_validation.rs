use std::collections::BTreeMap;

use worth_query_declaration::facade::application_schema::ApplicationSchemaMember;

use super::{WorthQueryPortableDomainPackage, WorthQueryPortablePackageValidationDenial};

pub(super) fn validate_conditional_application_operations(
    package: &mut WorthQueryPortableDomainPackage,
) -> Result<(), WorthQueryPortablePackageValidationDenial> {
    package.conditional_application_operations.sort();
    reject_duplicate_bindings(package)?;
    for binding in &package.conditional_application_operations {
        let schema = package
            .application_schemas
            .iter()
            .find(|schema| {
                schema.owner() == binding.schema_owner() && schema.name() == binding.schema_name()
            })
            .ok_or_else(|| {
                WorthQueryPortablePackageValidationDenial::conditional_application_schema_missing(
                    binding.schema_name(),
                )
            })?;
        let application_operation = schema.members().iter().find(|member| {
            matches!(
                member,
                ApplicationSchemaMember::Operation { operation, .. }
                    if operation == binding.application_operation()
            )
        });
        let Some(ApplicationSchemaMember::Operation { input_type, .. }) = application_operation
        else {
            return Err(
                WorthQueryPortablePackageValidationDenial::conditional_application_operation_missing(
                    binding.application_operation(),
                ),
            );
        };
        if input_type != binding.input_type() {
            return Err(
                WorthQueryPortablePackageValidationDenial::conditional_application_operation_changed(
                    binding.application_operation(),
                ),
            );
        }
        let domain_operation = package
            .domain_operations
            .iter()
            .find(|operation| operation.identity().slot() == binding.domain_operation_slot());
        let Some(domain_operation) = domain_operation else {
            return Err(
                WorthQueryPortablePackageValidationDenial::conditional_domain_operation_missing(
                    binding.domain_operation_slot(),
                ),
            );
        };
        if domain_operation.canonical_identity() != binding.domain_operation_canonical_identity() {
            return Err(
                WorthQueryPortablePackageValidationDenial::conditional_domain_operation_changed(
                    binding.domain_operation_slot(),
                ),
            );
        }
    }
    Ok(())
}

fn reject_duplicate_bindings(
    package: &WorthQueryPortableDomainPackage,
) -> Result<(), WorthQueryPortablePackageValidationDenial> {
    let mut bindings = BTreeMap::new();
    for binding in &package.conditional_application_operations {
        let key = (
            binding.schema_owner().to_string(),
            binding.schema_name().to_string(),
            binding.application_operation().to_string(),
        );
        if let Some(existing) = bindings.insert(key, binding) {
            return Err(if existing == binding {
                WorthQueryPortablePackageValidationDenial::duplicate_conditional_application_operation(
                    binding.application_operation(),
                )
            } else {
                WorthQueryPortablePackageValidationDenial::conflicting_conditional_application_operation(
                    binding.application_operation(),
                )
            });
        }
    }
    Ok(())
}
