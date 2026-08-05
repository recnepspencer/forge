use worth_query_decl::facade::application_schema::ApplicationSchemaDeclarationBuilder;

use super::{
    Branch, BranchIdentity, BranchIdentityField, CapabilityGrant, CapabilityGrantRecord,
    DeathNotice, DeathNoticeIdentityField, DeathNoticeRecord, DeathNoticeStatusField,
    EmergencyAccess, EmergencyAccessRecord, EstateCase, EstateCaseIdentityField, EstateCaseRecord,
    EstateCaseStatusField, EstateWorkflowStageField, LegalAuthority, LegalAuthorityIdentityField,
    LegalAuthorityKindField, LegalAuthorityRecognizedField, LegalAuthorityRecord, MandatoryReview,
    MandatoryReviewRecord,
};
use crate::schema::BankSchema;

pub(super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    schema
        .entity(Branch::reference())
        .entity(CapabilityGrant::reference())
        .entity(DeathNotice::reference())
        .entity(EmergencyAccess::reference())
        .entity(EstateCase::reference())
        .entity(LegalAuthority::reference())
        .entity(MandatoryReview::reference())
        .aspect(Branch::reference(), BranchIdentity::reference())
        .aspect(DeathNotice::reference(), DeathNoticeRecord::reference())
        .aspect(EstateCase::reference(), EstateCaseRecord::reference())
        .aspect(
            LegalAuthority::reference(),
            LegalAuthorityRecord::reference(),
        )
        .aspect(
            CapabilityGrant::reference(),
            CapabilityGrantRecord::reference(),
        )
        .aspect(
            EmergencyAccess::reference(),
            EmergencyAccessRecord::reference(),
        )
        .aspect(
            MandatoryReview::reference(),
            MandatoryReviewRecord::reference(),
        )
        .field(Branch::reference(), BranchIdentityField::reference())
        .field(
            DeathNotice::reference(),
            DeathNoticeIdentityField::reference(),
        )
        .field(
            DeathNotice::reference(),
            DeathNoticeStatusField::reference(),
        )
        .field(
            EstateCase::reference(),
            EstateCaseIdentityField::reference(),
        )
        .field(
            EstateCase::reference(),
            EstateWorkflowStageField::reference(),
        )
        .field(EstateCase::reference(), EstateCaseStatusField::reference())
        .field(
            LegalAuthority::reference(),
            LegalAuthorityIdentityField::reference(),
        )
        .field(
            LegalAuthority::reference(),
            LegalAuthorityKindField::reference(),
        )
        .field(
            LegalAuthority::reference(),
            LegalAuthorityRecognizedField::reference(),
        )
}
