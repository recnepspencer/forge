use bank_domain::model::{BankPrincipalId, Money};
use bank_domain::proposals::BankProposalDenial;
use bank_domain::schema::SendMoney;
use bank_server::{
    BankOperationAdmissionError, BankOperationProposalError, BankOperationProposals,
    BankPrincipalSeed, BankWorldSeed,
};
use worth_query_host::facade::primary_graph::WorthQueryOperationAuthorizationDenialKind;

use super::fixture::{funded_personal_world, id, key};
use crate::support::{block_on, request_scope, runtime, CausalCredential, DynamicIdentity};

#[test]
fn unauthorized_account_and_admitted_scope_drift_both_deny() {
    let snapshot = funded_personal_world();
    let owner = DynamicIdentity::new("owner");
    let other = DynamicIdentity::new("other");
    let employee = DynamicIdentity::new("employee");
    let world = runtime(
        BankWorldSeed::new(snapshot.clone())
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 1),
                owner.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 2),
                other.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 3),
                employee.external(),
            )),
    );
    let request = request_scope();
    let owner_actor = block_on(world.runtime.authenticate_with(
        &world.authentication,
        CausalCredential::for_identity(&owner),
        &request,
    ))
    .unwrap();
    let other_actor = block_on(world.runtime.authenticate_with(
        &world.authentication,
        CausalCredential::for_identity(&other),
        &request,
    ))
    .unwrap();
    let source = snapshot
        .primary_account(id(BankPrincipalId::new, 1))
        .unwrap();
    let destination = snapshot
        .primary_account(id(BankPrincipalId::new, 2))
        .unwrap();

    let creation_denial = world
        .runtime
        .authorize_create_personal_account(
            &owner_actor,
            id(bank_domain::model::InstitutionId::new, 1),
            Default::default(),
            &request,
        )
        .err()
        .expect("customer role cannot substitute teller authority");
    assert!(matches!(
        creation_denial,
        BankOperationAdmissionError::Authorization(ref denial)
            if denial.kind() == WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    ));
    let denial = world
        .runtime
        .authorize_send_money(&other_actor, source, Default::default(), &request)
        .err()
        .expect("non-owner must be denied");
    assert!(matches!(
        denial,
        BankOperationAdmissionError::Authorization(ref denial)
            if denial.kind() == WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    ));

    let admission = world
        .runtime
        .authorize_send_money(&owner_actor, source, Default::default(), &request)
        .unwrap();
    let drift = BankOperationProposals::prepare_send_money(
        &world.runtime,
        admission,
        &key("scope-drift"),
        &SendMoney {
            from: destination,
            recipient: id(BankPrincipalId::new, 1),
            amount: Money::from_minor(1).unwrap(),
        },
    )
    .err()
    .expect("admitted scope cannot be substituted");
    assert_eq!(
        drift,
        BankOperationProposalError::Invariant(BankProposalDenial::ScopeInputMismatch)
    );
}
