mod support;

use bank_domain::model::{
    AccountId, AccountName, BankPrincipalId, BankSnapshotVersion, BusinessId, CustomerRole,
    EmployeeAssignmentId, EmployeeRole, InstitutionId, Money,
};
use bank_domain::proposals::{
    BankIdempotencyKey, BankOperationScopeBinding, BankProposalEngine, BankProposedEffect,
    BankSnapshot, BankSnapshotBuilder,
};
use bank_domain::schema::{
    ApplyOpeningFunding, CreateBusinessAccount, CreatePersonalAccount, GrantAccountAuthorization,
    InitiateBusinessPayment, RejectPayment, ReversalReason, ReverseJournal,
};
use bank_server::{
    BankBusinessOwnerSeed, BankEmployeeAssignmentSeed, BankOperationProposals, BankPrincipalSeed,
    BankWorldSeed,
};

use support::{block_on, request_scope, runtime, CausalCredential, DynamicIdentity};

#[test]
fn authenticated_business_initiation_projects_only_its_real_neighborhood() {
    let snapshot = workflow_world();
    let identities = identities("initiate");
    let world = installed_world(snapshot.clone(), &identities, None);
    let request = request_scope();
    let actor = authenticate(&world, &identities[0], &request);
    let business = id(BusinessId::new, 1);
    let input = InitiateBusinessPayment {
        business,
        from: snapshot.business_account(business).unwrap(),
        recipient: id(BankPrincipalId::new, 2),
        amount: Money::from_minor(700).unwrap(),
    };
    let admission = world
        .runtime
        .authorize_initiate_business_payment(&actor, business, Default::default(), &request)
        .unwrap();
    let proposal = BankOperationProposals::prepare_initiate_business_payment(
        &world.runtime,
        admission,
        &key("consumer-initiate"),
        &input,
    )
    .unwrap();
    assert!(matches!(
        proposal.invariant().effects(),
        [BankProposedEffect::CreatePayment(_)]
    ));
    assert_eq!(proposal.projection_work().reconstructive_scans(), 0);
}

#[test]
fn authenticated_distinct_approver_can_reject_a_pending_payment() {
    let (snapshot, payment) = pending_payment_world();
    let identities = identities("reject");
    let world = installed_world(snapshot, &identities, None);
    let request = request_scope();
    let actor = authenticate(&world, &identities[2], &request);
    let input = RejectPayment {
        payment,
        rejecting_principal: id(BankPrincipalId::new, 3),
    };
    let admission = world
        .runtime
        .authorize_reject_payment(&actor, payment, Default::default(), &request)
        .unwrap();
    let proposal = BankOperationProposals::prepare_reject_payment(
        &world.runtime,
        admission,
        &key("consumer-reject"),
        &input,
    )
    .unwrap();
    assert!(matches!(
        proposal.invariant().effects(),
        [BankProposedEffect::UpdatePayment {
            payment: updated,
            ..
        }] if *updated == payment
    ));
    assert_eq!(proposal.projection_work().reconstructive_scans(), 0);
}

#[test]
fn authenticated_employee_reversal_projects_the_exact_journal_neighborhood() {
    let snapshot = workflow_world();
    let original = snapshot.journal().last().unwrap().id();
    let identities = identities("reverse");
    let employee = BankEmployeeAssignmentSeed::new(
        id(EmployeeAssignmentId::new, 1),
        id(InstitutionId::new, 1),
        id(BankPrincipalId::new, 3),
        EmployeeRole::Teller,
    );
    let world = installed_world(snapshot, &identities, Some(employee));
    let request = request_scope();
    let actor = authenticate(&world, &identities[2], &request);
    let input = ReverseJournal {
        institution: id(InstitutionId::new, 1),
        journal: original,
        reason: ReversalReason::OperatorCorrection,
    };
    let admission = world
        .runtime
        .authorize_reverse_journal(
            &actor,
            id(InstitutionId::new, 1),
            Default::default(),
            &request,
        )
        .unwrap();
    let proposal = BankOperationProposals::prepare_reverse_journal(
        &world.runtime,
        admission,
        &key("consumer-reverse"),
        &input,
    )
    .unwrap();
    assert!(matches!(
        proposal.invariant().effects(),
        [BankProposedEffect::ReverseJournal {
            original: reversed,
            ..
        }] if *reversed == original
    ));
    assert_eq!(proposal.projection_work().reconstructive_scans(), 0);
}

fn pending_payment_world() -> (BankSnapshot, bank_domain::model::PaymentId) {
    let snapshot = workflow_world();
    let business = id(BusinessId::new, 1);
    let proposal = BankProposalEngine::prepare_initiate_business_payment(
        &snapshot,
        binding(8),
        &key("fixture-pending-payment"),
        id(BankPrincipalId::new, 1),
        &InitiateBusinessPayment {
            business,
            from: snapshot.business_account(business).unwrap(),
            recipient: id(BankPrincipalId::new, 2),
            amount: Money::from_minor(900).unwrap(),
        },
    )
    .unwrap();
    let snapshot = proposal.proposed_snapshot().clone();
    let payment = snapshot.payments().next().unwrap().id();
    (snapshot, payment)
}

fn workflow_world() -> BankSnapshot {
    let empty = BankSnapshotBuilder::new(id(BankSnapshotVersion::new, 1))
        .institution(id(InstitutionId::new, 1))
        .principal(id(BankPrincipalId::new, 1))
        .principal(id(BankPrincipalId::new, 2))
        .principal(id(BankPrincipalId::new, 3))
        .business(id(BusinessId::new, 1))
        .institution_cash_account(id(AccountId::new, 100), id(InstitutionId::new, 1))
        .build()
        .unwrap();
    let recipient = BankProposalEngine::prepare_create_personal_account(
        &empty,
        binding(1),
        &key("fixture-recipient"),
        &CreatePersonalAccount {
            institution: id(InstitutionId::new, 1),
            owner: id(BankPrincipalId::new, 2),
            display_name: AccountName::new("Recipient").unwrap(),
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone();
    let business = BankProposalEngine::prepare_create_business_account(
        &recipient,
        binding(2),
        &key("fixture-business"),
        &CreateBusinessAccount {
            institution: id(InstitutionId::new, 1),
            business: id(BusinessId::new, 1),
            display_name: AccountName::new("Operations").unwrap(),
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone();
    let account = business.business_account(id(BusinessId::new, 1)).unwrap();
    let funded = BankProposalEngine::prepare_opening_funding(
        &business,
        binding(3),
        &key("fixture-funding"),
        &ApplyOpeningFunding {
            institution: id(InstitutionId::new, 1),
            account,
            amount: Money::from_minor(10_000).unwrap(),
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone();
    let initiated = grant_role(
        &funded,
        account,
        id(BankPrincipalId::new, 1),
        CustomerRole::Initiator,
        "fixture-initiator-role",
    );
    grant_role(
        &initiated,
        account,
        id(BankPrincipalId::new, 3),
        CustomerRole::Approver,
        "fixture-approver-role",
    )
}

fn grant_role(
    snapshot: &BankSnapshot,
    account: AccountId,
    principal: BankPrincipalId,
    role: CustomerRole,
    operation_key: &str,
) -> BankSnapshot {
    BankProposalEngine::prepare_grant_account_authorization(
        snapshot,
        binding(4),
        &key(operation_key),
        &GrantAccountAuthorization {
            account,
            principal,
            role,
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone()
}

fn installed_world(
    snapshot: BankSnapshot,
    identities: &[DynamicIdentity; 3],
    employee: Option<BankEmployeeAssignmentSeed>,
) -> support::TestIdentityWorld {
    let mut seed = BankWorldSeed::new(snapshot)
        .principal(BankPrincipalSeed::enabled(
            id(BankPrincipalId::new, 1),
            identities[0].external(),
        ))
        .principal(BankPrincipalSeed::enabled(
            id(BankPrincipalId::new, 2),
            identities[1].external(),
        ))
        .principal(BankPrincipalSeed::enabled(
            id(BankPrincipalId::new, 3),
            identities[2].external(),
        ))
        .business_owner(BankBusinessOwnerSeed::new(
            id(BusinessId::new, 1),
            id(BankPrincipalId::new, 1),
        ));
    if let Some(employee) = employee {
        seed = seed.employee(employee);
    }
    runtime(seed)
}

fn authenticate(
    world: &support::TestIdentityWorld,
    identity: &DynamicIdentity,
    request: &worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope,
) -> bank_server::BankAuthenticatedPrincipal {
    block_on(world.runtime.authenticate_with(
        &world.authentication,
        CausalCredential::for_identity(identity),
        request,
    ))
    .unwrap()
}

fn identities(prefix: &str) -> [DynamicIdentity; 3] {
    [
        DynamicIdentity::new(&format!("{prefix}-initiator")),
        DynamicIdentity::new(&format!("{prefix}-recipient")),
        DynamicIdentity::new(&format!("{prefix}-approver")),
    ]
}

fn binding(value: u8) -> BankOperationScopeBinding {
    BankOperationScopeBinding::from_fingerprint_bytes([value; 32])
}

fn key(value: &str) -> BankIdempotencyKey {
    BankIdempotencyKey::new(value).unwrap()
}

fn id<T>(constructor: impl FnOnce(u64) -> Option<T>, value: u64) -> T {
    constructor(value).unwrap()
}
