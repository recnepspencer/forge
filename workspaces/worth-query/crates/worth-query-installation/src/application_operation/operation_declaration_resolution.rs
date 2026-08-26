use worth_query_declaration::facade::application_schema::{
    ApplicationSchema, ApplicationSchemaMember,
};
use worth_query_declaration::facade::portable_identity::WorthQueryPortableType;

use super::installed_contract_support::operation_denial;
use super::{
    WorthQueryApplicationOperationInstallationDenial,
    WorthQueryApplicationOperationInstallationDenialKind,
};
use crate::application_schema::WorthQueryInstalledApplicationSchema;

/// The exact installed declaration named by a typed operation reference.
///
/// This resolves shared declaration truth only. Executable-contract compilation
/// and capability graph authority are separate downstream consumers.
pub(super) struct ResolvedApplicationOperationDeclaration {
    operation: String,
    input_type: &'static str,
}

impl ResolvedApplicationOperationDeclaration {
    pub(super) fn operation(&self) -> &str {
        &self.operation
    }

    pub(super) const fn input_type(&self) -> &'static str {
        self.input_type
    }
}

pub(super) fn resolve_operation_declaration<Schema, Operation, Input>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    operation: &str,
) -> Result<ResolvedApplicationOperationDeclaration, WorthQueryApplicationOperationInstallationDenial>
where
    Schema: ApplicationSchema,
    Operation: 'static,
    Input: WorthQueryPortableType + 'static,
{
    let input_type = Input::PORTABLE_TYPE_IDENTITY.as_str();
    if !schema
        .member_provenance
        .admits_operation::<Operation, Input>(operation, Input::PORTABLE_TYPE_IDENTITY)
    {
        return Err(operation_denial(
            WorthQueryApplicationOperationInstallationDenialKind::OperationMeaningChanged,
            operation,
        ));
    }
    let installed = schema
        .installed_declaration()
        .members()
        .iter()
        .any(|member| {
            matches!(
                member,
                ApplicationSchemaMember::Operation {
                    operation: installed,
                    input_type: installed_input,
                } if installed == operation && installed_input.as_str() == input_type
            )
        });
    if !installed {
        return Err(operation_denial(
            WorthQueryApplicationOperationInstallationDenialKind::OperationNotInstalled,
            operation,
        ));
    }
    Ok(ResolvedApplicationOperationDeclaration {
        operation: operation.to_owned(),
        input_type,
    })
}
