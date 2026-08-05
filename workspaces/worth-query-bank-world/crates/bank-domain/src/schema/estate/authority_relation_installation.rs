use worth_query_decl::facade::application_schema::ApplicationSchemaDeclarationBuilder;

use super::{
    Branch, CapabilityAccount, CapabilityBranch, CapabilityEstate, CapabilityGrant,
    CapabilityGrantee, CapabilityGrantor, CapabilityInstitution, CapabilityParent, EmergencyAccess,
    EmergencyApprover, EmergencyEstate, EmergencyGrant, EmergencyRequester, EmergencyReview,
    EstateCase, MandatoryReview, ReviewEstate, ReviewPrincipal,
};
use crate::schema::{Account, BankSchema, Institution, Principal};

pub(super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    install_elevation_relations(install_capability_relations(schema))
}

fn install_capability_relations(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    schema
        .relation(
            CapabilityGrantee::reference(),
            Principal::reference(),
            CapabilityGrant::reference(),
        )
        .relation(
            CapabilityGrantor::reference(),
            Principal::reference(),
            CapabilityGrant::reference(),
        )
        .relation(
            CapabilityEstate::reference(),
            CapabilityGrant::reference(),
            EstateCase::reference(),
        )
        .relation(
            CapabilityAccount::reference(),
            CapabilityGrant::reference(),
            Account::reference(),
        )
        .relation(
            CapabilityInstitution::reference(),
            CapabilityGrant::reference(),
            Institution::reference(),
        )
        .relation(
            CapabilityBranch::reference(),
            CapabilityGrant::reference(),
            Branch::reference(),
        )
        .relation(
            CapabilityParent::reference(),
            CapabilityGrant::reference(),
            CapabilityGrant::reference(),
        )
}

fn install_elevation_relations(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    schema
        .relation(
            EmergencyRequester::reference(),
            Principal::reference(),
            EmergencyAccess::reference(),
        )
        .relation(
            EmergencyApprover::reference(),
            Principal::reference(),
            EmergencyAccess::reference(),
        )
        .relation(
            EmergencyGrant::reference(),
            EmergencyAccess::reference(),
            CapabilityGrant::reference(),
        )
        .relation(
            EmergencyEstate::reference(),
            EmergencyAccess::reference(),
            EstateCase::reference(),
        )
        .relation(
            EmergencyReview::reference(),
            EmergencyAccess::reference(),
            MandatoryReview::reference(),
        )
        .relation(
            ReviewPrincipal::reference(),
            Principal::reference(),
            MandatoryReview::reference(),
        )
        .relation(
            ReviewEstate::reference(),
            MandatoryReview::reference(),
            EstateCase::reference(),
        )
}
