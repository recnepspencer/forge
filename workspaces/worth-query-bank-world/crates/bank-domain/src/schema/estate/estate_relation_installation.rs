use worth_query_decl::facade::application_schema::ApplicationSchemaDeclarationBuilder;

use super::{
    Branch, BranchInstitution, DeathNotice, DeathNoticeSubject, EstateAccount, EstateAssignment,
    EstateAuthorizedSigner, EstateBeneficiary, EstateBranch, EstateCase, EstateDeathNotice,
    EstateDeceased, EstateExecutor, EstateJointOwner, LegalAuthority, LegalAuthorityEstate,
    LegalAuthorityHolder,
};
use crate::schema::{Account, BankSchema, EmployeeAssignment, Institution, Principal};

pub(super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    install_participant_relations(install_case_relations(schema))
}

fn install_case_relations(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    schema
        .relation(
            BranchInstitution::reference(),
            Branch::reference(),
            Institution::reference(),
        )
        .relation(
            DeathNoticeSubject::reference(),
            DeathNotice::reference(),
            Principal::reference(),
        )
        .relation(
            EstateDeathNotice::reference(),
            EstateCase::reference(),
            DeathNotice::reference(),
        )
        .relation(
            EstateDeceased::reference(),
            EstateCase::reference(),
            Principal::reference(),
        )
        .relation(
            EstateAccount::reference(),
            EstateCase::reference(),
            Account::reference(),
        )
        .relation(
            EstateBranch::reference(),
            EstateCase::reference(),
            Branch::reference(),
        )
}

fn install_participant_relations(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    schema
        .relation(
            EstateExecutor::reference(),
            Principal::reference(),
            EstateCase::reference(),
        )
        .relation(
            EstateBeneficiary::reference(),
            Principal::reference(),
            EstateCase::reference(),
        )
        .relation(
            EstateJointOwner::reference(),
            Principal::reference(),
            Account::reference(),
        )
        .relation(
            EstateAuthorizedSigner::reference(),
            Principal::reference(),
            Account::reference(),
        )
        .relation(
            EstateAssignment::reference(),
            EmployeeAssignment::reference(),
            EstateCase::reference(),
        )
        .relation(
            LegalAuthorityEstate::reference(),
            LegalAuthority::reference(),
            EstateCase::reference(),
        )
        .relation(
            LegalAuthorityHolder::reference(),
            LegalAuthority::reference(),
            Principal::reference(),
        )
}
