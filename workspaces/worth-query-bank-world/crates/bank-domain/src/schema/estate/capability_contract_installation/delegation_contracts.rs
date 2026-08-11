use super::*;

pub(super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    let schema = install_contract!(
        schema,
        DelegateEstateCapability,
        DelegateEstateCapabilityOperation,
        EstateCapabilityOperation::DelegateCapability,
        EstateCapabilityPurpose::EstateAdministration,
        no_relation,
        no_field,
        no_magnitude
    );
    install_contract!(
        schema,
        RevokeEstateCapability,
        RevokeEstateCapabilityOperation,
        EstateCapabilityOperation::RevokeCapability,
        EstateCapabilityPurpose::EstateAdministration,
        no_relation,
        no_field,
        no_magnitude
    )
}
