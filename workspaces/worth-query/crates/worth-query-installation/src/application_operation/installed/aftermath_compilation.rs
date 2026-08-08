//! Compiling one operation's declared aftermath into an installed contract.

use worth_query_declaration::facade::application_schema::{
    ApplicationOperationDecisionReadTarget, ApplicationSchema,
};

use crate::application_aftermath::{
    derived_lowering_catalog, install_application_aftermath, InstalledExternalEffectContract,
    OperationAftermathInstallation, OperationDeclaredReadFields,
    WorthQueryInstalledAftermathContract,
};
use crate::application_operation::contract_resolution::operation_aftermath;
use crate::application_operation::installed_contract_support::operation_denial;
use crate::application_operation::{
    WorthQueryApplicationOperationInstallationDenial,
    WorthQueryApplicationOperationInstallationDenialKind,
};
use crate::application_schema::WorthQueryInstalledApplicationSchema;

/// Installs the operation's aftermath, or `None` when it declares none.
///
/// `external_effect` is the same resolution of the operation's escaping lane
/// that the compiled contracts carry. Passing it in — rather than letting
/// aftermath declare its own posture — is what keeps the reversibility guard
/// pointed at the lane that actually dispatches (Q8.25-C1).
pub(super) fn compile_operation_aftermath<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    operation: &str,
    decision_reads: &[ApplicationOperationDecisionReadTarget],
    external_effect: &InstalledExternalEffectContract,
) -> Result<
    Option<WorthQueryInstalledAftermathContract>,
    WorthQueryApplicationOperationInstallationDenial,
>
where
    Schema: ApplicationSchema,
{
    let declared = operation_aftermath(schema.installed_declaration().members(), operation)
        .map_err(|denial| operation_denial(denial.installation_kind(), operation))?;
    let Some(declared) = declared else {
        return Ok(None);
    };
    let declared_reads =
        OperationDeclaredReadFields::from_field_slots(decision_reads.iter().filter_map(|target| {
            match target {
                ApplicationOperationDecisionReadTarget::Field { field, .. } => Some(field.as_str()),
                _ => None,
            }
        }));
    let binding = schema.binding_identity();
    let denied = || {
        operation_denial(
            WorthQueryApplicationOperationInstallationDenialKind::AftermathInstallationDenied,
            operation,
        )
    };
    let catalog = derived_lowering_catalog(&binding, &declared).map_err(|_| denied())?;
    install_application_aftermath(OperationAftermathInstallation {
        binding: &binding,
        operation_slot: operation,
        declared: &declared,
        declared_reads: &declared_reads,
        external_effect,
        lowering_catalog: &catalog,
    })
    .map(Some)
    .map_err(|_| denied())
}
