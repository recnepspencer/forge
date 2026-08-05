use std::collections::{BTreeMap, BTreeSet};

use bank_domain::schema::BankSchema;
use worth_query_decl::facade::application_schema::{
    ApplicationAuthorizationPathEffect, ApplicationSchemaMember,
};

#[test]
fn bank_ability_inventory_and_operation_requirements_are_exact() {
    let declaration = BankSchema::declaration().expect("bank schema must declare");
    let members = declaration.erased().members();
    let abilities = members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::Ability {
                ability,
                scope_entity,
            } => Some((ability.as_str(), scope_entity.as_str())),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        abilities,
        BTreeSet::from([
            ("ApproveBusinessFunds", "PaymentIntent"),
            ("AuditInstitution", "Institution"),
            ("DiscoverOwnAccounts", "Principal"),
            ("InitiateBusinessFunds", "Business"),
            ("ManageAccountAccess", "Account"),
            ("OpenAccount", "Institution"),
            ("SendPersonalFunds", "Account"),
            ("ServiceInstitutionAccount", "Institution"),
            ("ViewAccount", "Account"),
            ("ViewAccountAccess", "Account"),
            ("ViewEstateCase", "EstateCase"),
            ("ViewEstateLegalCompliance", "EstateCase"),
            ("ViewEstateMandatoryReview", "EstateCase"),
            ("ViewPayment", "PaymentIntent"),
        ])
    );

    let requirements = members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::OperationAbility {
                operation,
                ability,
                scope_entity,
            } => Some((
                operation.as_str(),
                (ability.as_str(), scope_entity.as_str()),
            )),
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        requirements,
        BTreeMap::from([
            (
                "ApplyOpeningFundingOperation",
                ("ServiceInstitutionAccount", "Institution")
            ),
            (
                "ApprovePaymentOperation",
                ("ApproveBusinessFunds", "PaymentIntent")
            ),
            (
                "CreateBusinessAccountOperation",
                ("OpenAccount", "Institution")
            ),
            (
                "CreatePersonalAccountOperation",
                ("OpenAccount", "Institution")
            ),
            (
                "DepositOperation",
                ("ServiceInstitutionAccount", "Institution")
            ),
            (
                "GrantAccountAuthorizationOperation",
                ("ManageAccountAccess", "Account")
            ),
            (
                "InitiateBusinessPaymentOperation",
                ("InitiateBusinessFunds", "Business")
            ),
            (
                "RejectPaymentOperation",
                ("ApproveBusinessFunds", "PaymentIntent")
            ),
            (
                "ReverseJournalOperation",
                ("ServiceInstitutionAccount", "Institution")
            ),
            (
                "RevokeAccountAuthorizationOperation",
                ("ManageAccountAccess", "Account")
            ),
            ("SendMoneyOperation", ("SendPersonalFunds", "Account")),
            (
                "WithdrawOperation",
                ("ServiceInstitutionAccount", "Institution")
            ),
        ])
    );
}

#[test]
fn bank_ability_policies_are_closed_over_declared_graph_paths() {
    let declaration = BankSchema::declaration().expect("bank schema must declare");
    let policies = declaration
        .erased()
        .members()
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::AbilityPolicy {
                ability,
                scope_entity,
                paths,
                ..
            } => Some((ability.as_str(), (scope_entity.as_str(), paths))),
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();

    assert_eq!(policies.len(), 14);
    let (approval_scope, approval_paths) = policies
        .get("ApproveBusinessFunds")
        .expect("approval policy must be installed");
    assert_eq!(*approval_scope, "PaymentIntent");
    assert!(approval_paths.iter().any(|path| {
        path.effect() == ApplicationAuthorizationPathEffect::Allow
            && path
                .traversals()
                .iter()
                .any(|step| step.relation() == "AccountAuthorizedUser")
            && path
                .predicates()
                .iter()
                .any(|predicate| predicate.field() == "AuthorizationRole")
    }));
    assert!(approval_paths.iter().any(|path| {
        path.effect() == ApplicationAuthorizationPathEffect::Deny
            && path
                .traversals()
                .iter()
                .any(|step| step.relation() == "PaymentInitiator")
    }));
}
