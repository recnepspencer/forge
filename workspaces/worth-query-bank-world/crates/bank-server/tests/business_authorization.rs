mod support;

use bank_domain::model::{
    AccountId, AccountName, BankPrincipalId, BankSnapshotVersion, BusinessId, CustomerRole,
    InstitutionId, Money, PaymentId,
};
use bank_domain::proposals::{
    BankIdempotencyKey, BankOperationScopeBinding, BankProposalDenial, BankProposalEngine,
    BankSnapshot, BankSnapshotBuilder,
};
use bank_domain::schema::{
    ApplyOpeningFunding, ApprovePayment, CreateBusinessAccount, CreatePersonalAccount,
    GrantAccountAuthorization, InitiateBusinessPayment, RevokeAccountAuthorization,
};
use bank_server::{
    BankBusinessOwnerSeed, BankEmployeeAssignmentSeed, BankOperationAdmissionError,
    BankOperationProposals, BankPrincipalSeed, BankWorldSeed,
};
use worth_query_host::facade::primary_graph::WorthQueryOperationAuthorizationDenialKind;

use support::{block_on, request_scope, runtime, CausalCredential, DynamicIdentity};

fn id<T>(constructor: impl FnOnce(u64) -> Option<T>, value: u64) -> T {
    constructor(value).unwrap()
}

fn key(value: &str) -> BankIdempotencyKey {
    BankIdempotencyKey::new(value).unwrap()
}

fn binding(value: u8) -> BankOperationScopeBinding {
    BankOperationScopeBinding::from_fingerprint_bytes([value; 32])
}

fn pending_business_payment_world() -> (BankSnapshot, PaymentId) {
    let empty = BankSnapshotBuilder::new(id(BankSnapshotVersion::new, 1))
        .institution(id(InstitutionId::new, 1))
        .principal(id(BankPrincipalId::new, 1))
        .principal(id(BankPrincipalId::new, 2))
        .principal(id(BankPrincipalId::new, 3))
        .business(id(BusinessId::new, 1))
        .business(id(BusinessId::new, 2))
        .institution_cash_account(id(AccountId::new, 100), id(InstitutionId::new, 1))
        .build()
        .unwrap();
    let recipient = BankProposalEngine::prepare_create_personal_account(
        &empty,
        binding(1),
        &key("recipient"),
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
        &key("business"),
        &CreateBusinessAccount {
            institution: id(InstitutionId::new, 1),
            business: id(BusinessId::new, 1),
            display_name: AccountName::new("Operations").unwrap(),
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone();
    let second_business = BankProposalEngine::prepare_create_business_account(
        &business,
        binding(2),
        &key("second-business"),
        &CreateBusinessAccount {
            institution: id(InstitutionId::new, 1),
            business: id(BusinessId::new, 2),
            display_name: AccountName::new("Other operations").unwrap(),
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone();
    let source = second_business
        .business_account(id(BusinessId::new, 1))
        .unwrap();
    let other_source = second_business
        .business_account(id(BusinessId::new, 2))
        .unwrap();
    let funded = BankProposalEngine::prepare_opening_funding(
        &second_business,
        binding(3),
        &key("fund"),
        &ApplyOpeningFunding {
            institution: id(InstitutionId::new, 1),
            account: source,
            amount: Money::from_minor(50_000).unwrap(),
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone();
    let initiator_access = grant_access(
        &funded,
        source,
        1,
        CustomerRole::Approver,
        "initiator-approval-role",
    );
    let approver_access = grant_access(
        &initiator_access,
        source,
        3,
        CustomerRole::Approver,
        "approver-role",
    );
    let viewer_access = grant_access(
        &approver_access,
        source,
        2,
        CustomerRole::Viewer,
        "viewer-role",
    );
    let cross_business_access = grant_access(
        &viewer_access,
        other_source,
        2,
        CustomerRole::Approver,
        "cross-business-role",
    );
    let pending = BankProposalEngine::prepare_initiate_business_payment(
        &cross_business_access,
        binding(4),
        &key("initiate"),
        id(BankPrincipalId::new, 1),
        &InitiateBusinessPayment {
            business: id(BusinessId::new, 1),
            from: source,
            recipient: id(BankPrincipalId::new, 2),
            amount: Money::from_minor(8_000).unwrap(),
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone();
    let payment = pending.payments().next().unwrap().id();
    (pending, payment)
}

fn grant_access(
    snapshot: &BankSnapshot,
    account: AccountId,
    principal: u64,
    role: CustomerRole,
    key_value: &str,
) -> BankSnapshot {
    BankProposalEngine::prepare_grant_account_authorization(
        snapshot,
        binding(5),
        &key(key_value),
        &GrantAccountAuthorization {
            account,
            principal: id(BankPrincipalId::new, principal),
            role,
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone()
}

#[test]
fn real_graph_allows_distinct_approver_and_deny_precedence_blocks_initiator() {
    let (snapshot, payment) = pending_business_payment_world();
    let initiator = DynamicIdentity::new("initiator");
    let recipient = DynamicIdentity::new("recipient");
    let approver = DynamicIdentity::new("approver");
    let world = runtime(
        BankWorldSeed::new(snapshot.clone())
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 1),
                initiator.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 2),
                recipient.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 3),
                approver.external(),
            ))
            .business_owner(BankBusinessOwnerSeed::new(
                id(BusinessId::new, 1),
                id(BankPrincipalId::new, 1),
            )),
    );
    let request = request_scope();
    let approver_actor = block_on(world.runtime.authenticate_with(
        &world.authentication,
        CausalCredential::for_identity(&approver),
        &request,
    ))
    .unwrap();
    let admission = world
        .runtime
        .authorize_approve_payment(&approver_actor, payment, &request)
        .unwrap();
    let approved = BankOperationProposals::prepare_approve_payment(
        &snapshot,
        admission,
        &key("approve"),
        &ApprovePayment {
            payment,
            approver: id(BankPrincipalId::new, 3),
        },
    )
    .unwrap();
    assert_eq!(approved.invariant().effects().len(), 4);

    let initiator_actor = block_on(world.runtime.authenticate_with(
        &world.authentication,
        CausalCredential::for_identity(&initiator),
        &request,
    ))
    .unwrap();
    let denial = world
        .runtime
        .authorize_approve_payment(&initiator_actor, payment, &request)
        .err()
        .expect("initiator deny path must override approver role");
    assert!(matches!(
        denial,
        BankOperationAdmissionError::Authorization(ref denial)
            if denial.kind() == WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    ));
}

#[test]
fn authenticated_actor_cannot_be_relabelled_in_payment_input() {
    let (snapshot, payment) = pending_business_payment_world();
    let initiator = DynamicIdentity::new("initiator");
    let recipient = DynamicIdentity::new("recipient");
    let approver = DynamicIdentity::new("approver");
    let world = runtime(
        BankWorldSeed::new(snapshot.clone())
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 1),
                initiator.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 2),
                recipient.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 3),
                approver.external(),
            ))
            .business_owner(BankBusinessOwnerSeed::new(
                id(BusinessId::new, 1),
                id(BankPrincipalId::new, 1),
            )),
    );
    let request = request_scope();
    let actor = block_on(world.runtime.authenticate_with(
        &world.authentication,
        CausalCredential::for_identity(&approver),
        &request,
    ))
    .unwrap();
    let admission = world
        .runtime
        .authorize_approve_payment(&actor, payment, &request)
        .unwrap();
    let denial = BankOperationProposals::prepare_approve_payment(
        &snapshot,
        admission,
        &key("relabel"),
        &ApprovePayment {
            payment,
            approver: id(BankPrincipalId::new, 2),
        },
    )
    .err()
    .expect("input actor cannot differ from authenticated actor");
    assert_eq!(denial, BankProposalDenial::AuthenticatedActorMismatch);
}

#[test]
fn viewer_cross_business_and_employee_roles_do_not_combine_into_approval() {
    let (snapshot, payment) = pending_business_payment_world();
    let first = DynamicIdentity::new("initiator");
    let combined = DynamicIdentity::new("viewer-cross-business-teller");
    let third = DynamicIdentity::new("approver");
    let world = runtime(
        BankWorldSeed::new(snapshot)
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 1),
                first.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 2),
                combined.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 3),
                third.external(),
            ))
            .employee(BankEmployeeAssignmentSeed::new(
                id(bank_domain::model::EmployeeAssignmentId::new, 1),
                id(InstitutionId::new, 1),
                id(BankPrincipalId::new, 2),
                bank_domain::model::EmployeeRole::Teller,
            )),
    );
    let request = request_scope();
    let actor = block_on(world.runtime.authenticate_with(
        &world.authentication,
        CausalCredential::for_identity(&combined),
        &request,
    ))
    .unwrap();
    assert_permission_denied(
        world
            .runtime
            .authorize_approve_payment(&actor, payment, &request),
    );
}

#[test]
fn revoked_approver_membership_is_absent_from_current_authorization_graph() {
    let (snapshot, payment) = pending_business_payment_world();
    let authorization = snapshot
        .authorizations()
        .find(|candidate| {
            candidate.principal() == id(BankPrincipalId::new, 3)
                && candidate.role() == CustomerRole::Approver
        })
        .copied()
        .unwrap();
    let revoked = BankProposalEngine::prepare_revoke_account_authorization(
        &snapshot,
        binding(6),
        &key("revoke-approver"),
        &RevokeAccountAuthorization {
            authorization: authorization.id(),
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone();
    let first = DynamicIdentity::new("initiator");
    let second = DynamicIdentity::new("recipient");
    let revoked_approver = DynamicIdentity::new("revoked-approver");
    let world = runtime(
        BankWorldSeed::new(revoked)
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 1),
                first.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 2),
                second.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 3),
                revoked_approver.external(),
            )),
    );
    let request = request_scope();
    let actor = block_on(world.runtime.authenticate_with(
        &world.authentication,
        CausalCredential::for_identity(&revoked_approver),
        &request,
    ))
    .unwrap();
    assert_permission_denied(
        world
            .runtime
            .authorize_approve_payment(&actor, payment, &request),
    );
}

fn assert_permission_denied<T>(result: Result<T, BankOperationAdmissionError>) {
    assert!(matches!(
        result,
        Err(BankOperationAdmissionError::Authorization(ref denial))
            if denial.kind() == WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    ));
}
