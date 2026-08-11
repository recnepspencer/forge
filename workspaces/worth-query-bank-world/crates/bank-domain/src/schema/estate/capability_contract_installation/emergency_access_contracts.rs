use super::*;

pub(super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    let schema = install_contract!(
        schema,
        RequestEstateEmergencyAccessCapability,
        RequestEstateEmergencyAccessOperation,
        EstateCapabilityOperation::RequestEmergencyAccess,
        EstateCapabilityPurpose::EmergencyProtection,
        no_relation,
        no_field,
        no_magnitude
    );
    let schema = install_contract!(
        schema,
        ApproveEstateEmergencyAccessCapability,
        ApproveEstateEmergencyAccessOperation,
        EstateCapabilityOperation::ApproveEmergencyAccess,
        EstateCapabilityPurpose::EmergencyProtection,
        no_relation,
        no_field,
        no_magnitude
    );
    install_contract!(
        schema,
        RevokeEstateEmergencyAccessCapability,
        RevokeEstateEmergencyAccessOperation,
        EstateCapabilityOperation::RevokeEmergencyAccess,
        EstateCapabilityPurpose::EmergencyProtection,
        no_relation,
        no_field,
        no_magnitude
    )
}
