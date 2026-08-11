use super::*;

pub(super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    let schema = install_view_contract!(
        schema,
        ViewEstateAdministrationCapability,
        EstateCapabilityPurpose::EstateAdministration
    );
    let schema = install_view_contract!(
        schema,
        ViewEstateIdentityVerificationCapability,
        EstateCapabilityPurpose::IdentityVerification
    );
    let schema = install_view_contract!(
        schema,
        ViewEstateLegalComplianceCapability,
        EstateCapabilityPurpose::LegalCompliance
    );
    let schema = install_view_contract!(
        schema,
        ViewEstateEmergencyProtectionCapability,
        EstateCapabilityPurpose::EmergencyProtection
    );
    install_view_contract!(
        schema,
        ViewEstateMandatoryReviewCapability,
        EstateCapabilityPurpose::MandatoryReview
    )
}
