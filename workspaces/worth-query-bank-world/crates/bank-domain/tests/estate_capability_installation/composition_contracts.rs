use bank_domain::{
    estate::RestrictedBankField,
    schema::{
        ApproveEstateEmergencyAccessCapability, ApproveEstateEmergencyAccessOperation,
        CompleteEstateMandatoryReviewCapability, CompleteEstateMandatoryReviewOperation,
        ViewEstateAdministrationCapability, ViewRestrictedEstateOperation,
    },
};
use worth_query_host::facade::{
    declaration::{
        application_capability::{
            ApplicationCapabilityDelegationRule, ApplicationCapabilityDisclosureRule,
            ApplicationCapabilityGraphRule,
        },
        application_schema::TypedApplicationValue,
    },
    domain::WorthQueryInstallationRuntimeIdentity,
};

use super::installed_bank;

#[test]
fn estate_view_contract_installs_exact_role_and_disclosure_composition() {
    let (_index, bank) = installed_bank(WorthQueryInstallationRuntimeIdentity::fresh());
    let capability = bank
        .capability(
            ViewEstateAdministrationCapability::reference(),
            ViewRestrictedEstateOperation::reference(),
        )
        .unwrap();
    let composition = capability.contract().composition();

    assert_eq!(composition.decision().allow().graph().clauses().len(), 4);
    assert_eq!(
        composition
            .decision()
            .allow()
            .graph()
            .clauses()
            .iter()
            .map(|clause| clause.path().predicates().len())
            .sum::<usize>(),
        4
    );
    let deny = composition
        .decision()
        .deny()
        .graph()
        .expect("estate disclosure declares beneficiary denial");
    assert_eq!(path_relations(deny), vec!["EstateBeneficiary"]);
    assert!(composition.decision().conflict().graph().is_none());
    assert_eq!(
        composition.propagation().delegation(),
        ApplicationCapabilityDelegationRule::NarrowAllDimensions
    );

    let ApplicationCapabilityDisclosureRule::Permit(guards) =
        composition.propagation().disclosure()
    else {
        panic!("estate view field scope must carry an explicit disclosure matrix");
    };
    assert_eq!(guards.len(), 1);
    assert_eq!(guards[0].requirements().len(), 1);
    assert_eq!(
        guards[0].requirements()[0].values(),
        &expected_values([
            RestrictedBankField::CustomerIdentity,
            RestrictedBankField::BeneficiaryIdentity,
            RestrictedBankField::AccountDetails,
            RestrictedBankField::PostingHistory,
        ])
    );
}

#[test]
fn emergency_approval_contract_installs_conflict_and_requester_separation() {
    let (_index, bank) = installed_bank(WorthQueryInstallationRuntimeIdentity::fresh());
    let capability = bank
        .capability(
            ApproveEstateEmergencyAccessCapability::reference(),
            ApproveEstateEmergencyAccessOperation::reference(),
        )
        .unwrap();
    let composition = capability.contract().composition();

    assert_eq!(composition.decision().allow().graph().clauses().len(), 3);
    assert_eq!(
        path_relations(
            composition
                .decision()
                .conflict()
                .graph()
                .expect("approval declares beneficiary conflict"),
        ),
        vec!["EstateBeneficiary"]
    );
    assert_eq!(
        path_relations(
            composition
                .actors()
                .distinct_actor()
                .graph()
                .expect("approval declares requester separation"),
        ),
        vec!["CapabilityEstate", "EmergencyGrant", "EmergencyRequester"]
    );
    assert!(matches!(
        composition.propagation().disclosure(),
        ApplicationCapabilityDisclosureRule::NotApplicable
    ));
}

#[test]
fn mandatory_review_contract_installs_every_actor_exclusion() {
    let (_index, bank) = installed_bank(WorthQueryInstallationRuntimeIdentity::fresh());
    let capability = bank
        .capability(
            CompleteEstateMandatoryReviewCapability::reference(),
            CompleteEstateMandatoryReviewOperation::reference(),
        )
        .unwrap();
    let composition = capability.contract().composition();

    assert_eq!(
        path_relations(
            composition
                .decision()
                .conflict()
                .graph()
                .expect("review declares beneficiary conflict"),
        ),
        vec!["EstateBeneficiary"]
    );
    assert_eq!(
        path_relations(
            composition
                .actors()
                .separation_of_duty()
                .graph()
                .expect("review declares executor separation"),
        ),
        vec!["EstateExecutor"]
    );
    let distinct = composition
        .actors()
        .distinct_actor()
        .graph()
        .expect("review declares requester and approver separation");
    assert_eq!(distinct.clauses().len(), 2);
    assert_eq!(
        path_relations(distinct),
        vec![
            "EmergencyApprover",
            "EmergencyRequester",
            "EmergencyReview",
            "ReviewEstate"
        ]
    );
}

fn path_relations(rule: &ApplicationCapabilityGraphRule) -> Vec<&str> {
    let mut relations = rule
        .clauses()
        .iter()
        .flat_map(|clause| clause.path().traversals())
        .map(|traversal| traversal.relation())
        .collect::<Vec<_>>();
    relations.sort_unstable();
    relations.dedup();
    relations
}

fn expected_values(
    fields: impl IntoIterator<Item = RestrictedBankField>,
) -> Vec<worth_foundational::facade::AspectValue> {
    let mut values = fields
        .into_iter()
        .map(TypedApplicationValue::into_foundational_value)
        .collect::<Vec<_>>();
    values.sort();
    values
}
