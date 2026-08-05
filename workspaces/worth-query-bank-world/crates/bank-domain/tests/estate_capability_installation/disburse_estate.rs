use std::collections::BTreeSet;

use bank_domain::schema::DisburseEstateOperation;
use worth_query_host::facade::{
    declaration::application_schema::{
        ApplicationOperationDecisionReadTarget, ApplicationOperationProgramTarget,
    },
    domain::WorthQueryInstallationRuntimeIdentity,
};

use super::installed_bank;

#[test]
fn disbursement_installs_exact_effect_integrity_reads_and_money_program() {
    let (_index, bank) = installed_bank(WorthQueryInstallationRuntimeIdentity::fresh());
    let operation = bank
        .installed_operation(DisburseEstateOperation::reference())
        .expect("estate disbursement must install one executable accounting program");

    assert_eq!(operation.contracts().decision_fact_budget(), 64);
    assert_eq!(operation.contracts().projection_work_budget(), 192);
    assert_eq!(
        operation
            .contracts()
            .decision_reads()
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        expected_reads()
    );
    assert_eq!(operation.contracts().program(), expected_program());
}

fn expected_reads() -> BTreeSet<ApplicationOperationDecisionReadTarget> {
    [
        field("EstateCase", "EstateCaseRecord", "EstateCaseIdentityField"),
        field("EstateCase", "EstateCaseRecord", "EstateCaseStatusField"),
        field("Principal", "PrincipalIdentity", "PrincipalIdentityField"),
        field(
            "LegalAuthority",
            "LegalAuthorityRecord",
            "LegalAuthorityIdentityField",
        ),
        field(
            "LegalAuthority",
            "LegalAuthorityRecord",
            "LegalAuthorityRecognizedField",
        ),
        field("Account", "Identity", "AccountIdentity"),
        field("Account", "AccountState", "AccountingRevision"),
        field("Account", "AccountState", "Status"),
        field("Account", "AccountProfile", "Kind"),
        field("Account", "AccountProfile", "AccountDisplayName"),
        field(
            "Institution",
            "InstitutionIdentity",
            "InstitutionIdentityField",
        ),
        field("Business", "BusinessIdentity", "BusinessIdentityField"),
        field("Posting", "PostingValue", "PostingAmount"),
        relation("EstateAccount", "EstateCase", "Account"),
        relation("EstateBeneficiary", "Principal", "EstateCase"),
        relation("EstateJointOwner", "Principal", "Account"),
        relation("LegalAuthorityEstate", "LegalAuthority", "EstateCase"),
        relation("LegalAuthorityHolder", "LegalAuthority", "Principal"),
        relation("EstateExecutor", "Principal", "EstateCase"),
        relation("InstitutionAccount", "Institution", "Account"),
        relation("PersonalOwner", "Principal", "Account"),
        relation("BusinessAccount", "Business", "Account"),
        relation("PostingAccount", "Posting", "Account"),
    ]
    .into_iter()
    .collect()
}

fn expected_program() -> Vec<ApplicationOperationProgramTarget> {
    let mut expected = vec![
        ApplicationOperationProgramTarget::Create {
            entity: "JournalEntry".to_owned(),
        },
        ApplicationOperationProgramTarget::Create {
            entity: "Posting".to_owned(),
        },
        write("Account", "AccountState", "AccountingRevision"),
        write("JournalEntry", "JournalIdentity", "JournalIdentityField"),
        write("JournalEntry", "JournalState", "JournalPurpose"),
        write("Posting", "PostingIdentity", "PostingIdentityField"),
        write("Posting", "PostingValue", "PostingAmount"),
        write("Posting", "PostingValue", "PostingAccountSequence"),
        write("Posting", "PostingValue", "Purpose"),
        ApplicationOperationProgramTarget::Link {
            relation: "JournalPosting".to_owned(),
            from: "JournalEntry".to_owned(),
            to: "Posting".to_owned(),
        },
        ApplicationOperationProgramTarget::Link {
            relation: "PostingAccount".to_owned(),
            from: "Posting".to_owned(),
            to: "Account".to_owned(),
        },
        ApplicationOperationProgramTarget::Emit {
            effect: "AccountActivityEffect".to_owned(),
        },
    ];
    expected.sort();
    expected
}

fn field(entity: &str, aspect: &str, field: &str) -> ApplicationOperationDecisionReadTarget {
    ApplicationOperationDecisionReadTarget::Field {
        entity: entity.to_owned(),
        aspect: aspect.to_owned(),
        field: field.to_owned(),
    }
}

fn relation(relation: &str, from: &str, to: &str) -> ApplicationOperationDecisionReadTarget {
    ApplicationOperationDecisionReadTarget::Relation {
        relation: relation.to_owned(),
        from: from.to_owned(),
        to: to.to_owned(),
    }
}

fn write(entity: &str, aspect: &str, field: &str) -> ApplicationOperationProgramTarget {
    ApplicationOperationProgramTarget::Write {
        entity: entity.to_owned(),
        aspect: aspect.to_owned(),
        field: field.to_owned(),
    }
}
