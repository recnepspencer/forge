use std::collections::BTreeSet;

use bank_domain::schema::ReleaseEstateOperation;
use worth_query_host::facade::{
    declaration::application_schema::{
        ApplicationOperationDecisionReadTarget, ApplicationOperationProgramTarget,
    },
    domain::WorthQueryInstallationRuntimeIdentity,
};

use super::installed_bank;

#[test]
fn release_installs_exact_reads_and_effect() {
    let (_index, bank) = installed_bank(WorthQueryInstallationRuntimeIdentity::fresh());
    let release = bank
        .installed_operation(ReleaseEstateOperation::reference())
        .expect("release must install one exact executable program");
    assert_eq!(release.contracts().decision_fact_budget(), 32);
    assert_eq!(release.contracts().projection_work_budget(), 96);
    assert_eq!(
        release
            .contracts()
            .decision_reads()
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        expected_release_reads()
    );
    assert_eq!(
        release.contracts().program(),
        [ApplicationOperationProgramTarget::Write {
            entity: "EstateCase".to_owned(),
            aspect: "EstateCaseRecord".to_owned(),
            field: "EstateCaseStatusField".to_owned(),
        }]
    );
}

fn expected_release_reads() -> BTreeSet<ApplicationOperationDecisionReadTarget> {
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
        field(
            "MandatoryReview",
            "MandatoryReviewRecord",
            "MandatoryReviewIdentityField",
        ),
        field(
            "MandatoryReview",
            "MandatoryReviewRecord",
            "MandatoryReviewKindField",
        ),
        field(
            "MandatoryReview",
            "MandatoryReviewRecord",
            "MandatoryReviewStatusField",
        ),
        relation("EstateExecutor", "Principal", "EstateCase"),
        relation("LegalAuthorityEstate", "LegalAuthority", "EstateCase"),
        relation("LegalAuthorityHolder", "LegalAuthority", "Principal"),
        relation("ReviewEstate", "MandatoryReview", "EstateCase"),
        relation("ReviewPrincipal", "Principal", "MandatoryReview"),
    ]
    .into_iter()
    .collect()
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
