mod support;

use bank_domain::model::{
    AccountId, AccountName, BankPrincipalId, BankSnapshotVersion, BusinessId, EmployeeAssignmentId,
    EmployeeRole, InstitutionId, Money,
};
use bank_domain::proposals::{
    BankIdempotencyKey, BankOperationScopeBinding, BankProposalDenial, BankProposalEngine,
    BankSnapshot, BankSnapshotBuilder,
};
use bank_domain::schema::{
    ApplyOpeningFunding, CreateBusinessAccount, CreatePersonalAccount, SendMoney,
};
use bank_server::{
    BankEmployeeAssignmentSeed, BankOperationAdmissionError, BankOperationProposals,
    BankPrincipalSeed, BankWorldSeed,
};
use worth_query_host::facade::primary_graph::WorthQueryOperationAuthorizationDenialKind;

use support::{block_on, request_scope, runtime, CausalCredential, DynamicIdentity};

fn id<T>(constructor: impl FnOnce(u64) -> Option<T>, value: u64) -> T {
    constructor(value).expect("test identity is nonzero")
}

fn key(value: &str) -> BankIdempotencyKey {
    BankIdempotencyKey::new(value).unwrap()
}

fn descriptive_binding(value: u8) -> BankOperationScopeBinding {
    BankOperationScopeBinding::from_fingerprint_bytes([value; 32])
}

fn funded_personal_world() -> BankSnapshot {
    let empty = BankSnapshotBuilder::new(id(BankSnapshotVersion::new, 1))
        .institution(id(InstitutionId::new, 1))
        .principal(id(BankPrincipalId::new, 1))
        .principal(id(BankPrincipalId::new, 2))
        .principal(id(BankPrincipalId::new, 3))
        .business(id(BusinessId::new, 1))
        .institution_cash_account(id(AccountId::new, 100), id(InstitutionId::new, 1))
        .build()
        .unwrap();
    let first = BankProposalEngine::prepare_create_personal_account(
        &empty,
        descriptive_binding(1),
        &key("fixture-create-1"),
        &CreatePersonalAccount {
            institution: id(InstitutionId::new, 1),
            owner: id(BankPrincipalId::new, 1),
            display_name: AccountName::new("Daily").unwrap(),
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone();
    let second = BankProposalEngine::prepare_create_personal_account(
        &first,
        descriptive_binding(1),
        &key("fixture-create-2"),
        &CreatePersonalAccount {
            institution: id(InstitutionId::new, 1),
            owner: id(BankPrincipalId::new, 2),
            display_name: AccountName::new("Recipient").unwrap(),
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone();
    let source = second.primary_account(id(BankPrincipalId::new, 1)).unwrap();
    BankProposalEngine::prepare_opening_funding(
        &second,
        descriptive_binding(2),
        &key("fixture-fund"),
        &ApplyOpeningFunding {
            institution: id(InstitutionId::new, 1),
            account: source,
            amount: Money::from_minor(10_000).unwrap(),
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone()
}

#[test]
fn consumer_authenticates_authorizes_and_prepares_a_send_proposal() {
    let snapshot = funded_personal_world();
    let owner = DynamicIdentity::new("owner");
    let recipient = DynamicIdentity::new("recipient");
    let employee = DynamicIdentity::new("employee");
    let world = runtime(
        BankWorldSeed::new(snapshot.clone())
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 1),
                owner.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 2),
                recipient.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 3),
                employee.external(),
            )),
    );
    let request = request_scope();
    let actor = block_on(world.runtime.authenticate_with(
        &world.authentication,
        CausalCredential::for_identity(&owner),
        &request,
    ))
    .unwrap();
    let source = snapshot
        .primary_account(id(BankPrincipalId::new, 1))
        .unwrap();
    let admission = world
        .runtime
        .authorize_send_money(&actor, source, &request)
        .unwrap();
    let proposal = BankOperationProposals::prepare_send_money(
        &snapshot,
        admission,
        &key("consumer-send"),
        &SendMoney {
            from: source,
            recipient: id(BankPrincipalId::new, 2),
            amount: Money::from_minor(2_500).unwrap(),
        },
    )
    .unwrap();

    assert_eq!(proposal.admission().actor(), id(BankPrincipalId::new, 1));
    assert_eq!(proposal.admission().scope(), source);
    assert_eq!(proposal.invariant().effects().len(), 3);
    assert_eq!(proposal.invariant().proposed_snapshot().journal().len(), 2);
}

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
        .authorize_create_personal_account(&owner_actor, id(InstitutionId::new, 1), &request)
        .err()
        .expect("customer role cannot substitute teller authority");
    assert!(matches!(
        creation_denial,
        BankOperationAdmissionError::Authorization(ref denial)
            if denial.kind() == WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    ));

    let denial = world
        .runtime
        .authorize_send_money(&other_actor, source, &request)
        .err()
        .expect("non-owner must be denied");
    assert!(matches!(
        denial,
        BankOperationAdmissionError::Authorization(ref denial)
            if denial.kind() == WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    ));

    let admission = world
        .runtime
        .authorize_send_money(&owner_actor, source, &request)
        .unwrap();
    let drift = BankOperationProposals::prepare_send_money(
        &snapshot,
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
    assert_eq!(drift, BankProposalDenial::ScopeInputMismatch);
}

#[test]
fn teller_authority_opens_an_account_for_a_real_customer_identity() {
    let snapshot = funded_personal_world();
    let owner = DynamicIdentity::new("owner");
    let recipient = DynamicIdentity::new("recipient");
    let teller = DynamicIdentity::new("teller");
    let world = runtime(
        BankWorldSeed::new(snapshot.clone())
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 1),
                owner.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 2),
                recipient.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 3),
                teller.external(),
            ))
            .employee(BankEmployeeAssignmentSeed::new(
                id(EmployeeAssignmentId::new, 1),
                id(InstitutionId::new, 1),
                id(BankPrincipalId::new, 3),
                EmployeeRole::Teller,
            )),
    );
    let request = request_scope();
    let actor = block_on(world.runtime.authenticate_with(
        &world.authentication,
        CausalCredential::for_identity(&teller),
        &request,
    ))
    .unwrap();
    let admission = world
        .runtime
        .authorize_create_personal_account(&actor, id(InstitutionId::new, 1), &request)
        .unwrap();
    let input = CreatePersonalAccount {
        institution: id(InstitutionId::new, 1),
        owner: id(BankPrincipalId::new, 3),
        display_name: AccountName::new("Teller-created").unwrap(),
    };
    let proposal = BankOperationProposals::prepare_create_personal_account(
        &snapshot,
        admission,
        &key("employee-open"),
        &input,
    )
    .unwrap();
    assert_eq!(proposal.admission().actor(), id(BankPrincipalId::new, 3));

    let admission = world
        .runtime
        .authorize_create_business_account(&actor, id(InstitutionId::new, 1), &request)
        .unwrap();
    let proposal = BankOperationProposals::prepare_create_business_account(
        &snapshot,
        admission,
        &key("employee-open-business"),
        &CreateBusinessAccount {
            institution: id(InstitutionId::new, 1),
            business: id(BusinessId::new, 1),
            display_name: AccountName::new("Real business").unwrap(),
        },
    )
    .unwrap();
    assert!(proposal
        .invariant()
        .proposed_snapshot()
        .business_account(id(BusinessId::new, 1))
        .is_some());
}
