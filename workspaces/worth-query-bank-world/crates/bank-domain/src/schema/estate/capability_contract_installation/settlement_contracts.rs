use super::*;

pub(super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    let schema = install_contract!(
        schema,
        CompleteEstateMandatoryReviewCapability,
        CompleteEstateMandatoryReviewOperation,
        EstateCapabilityOperation::CompleteMandatoryReview,
        EstateCapabilityPurpose::MandatoryReview,
        no_relation,
        no_field,
        no_magnitude
    );
    let schema = install_contract!(
        schema,
        ReleaseEstateCapability,
        ReleaseEstateOperation,
        EstateCapabilityOperation::ReleaseEstate,
        EstateCapabilityPurpose::EstateAdministration,
        no_relation,
        no_field,
        no_magnitude
    );
    install_contract!(
        schema,
        DisburseEstateCapability,
        DisburseEstateOperation,
        EstateCapabilityOperation::DisburseEstate,
        EstateCapabilityPurpose::EstateDisbursement,
        account_relation,
        no_field,
        magnitude
    )
}
