#[path = "business_authorization/fixture.rs"]
mod business_authorization_fixture;
mod support;

use bank_domain::model::{BankPrincipalId, BusinessId, CustomerRole, InstitutionId};
use bank_domain::proposals::{BankProposalDenial, BankProposalEngine};
use bank_domain::schema::{ApprovePayment, RevokeAccountAuthorization};
use bank_server::{
    BankBusinessOwnerSeed, BankEmployeeAssignmentSeed, BankOperationAdmissionError,
    BankOperationProposalError, BankOperationProposals, BankPrincipalSeed, BankWorldSeed,
};

use business_authorization_fixture::{binding, id, key, pending_business_payment_world};
use support::{block_on, request_scope, runtime, CausalCredential, DynamicIdentity};

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
        .authorize_approve_payment(&approver_actor, payment, Default::default(), &request)
        .unwrap();
    let approved = BankOperationProposals::prepare_approve_payment(
        &world.runtime,
        admission,
        &key("approve"),
        &ApprovePayment {
            payment,
            approver: id(BankPrincipalId::new, 3),
        },
    )
    .unwrap();
    assert_eq!(approved.invariant().effects().len(), 2);

    let initiator_actor = block_on(world.runtime.authenticate_with(
        &world.authentication,
        CausalCredential::for_identity(&initiator),
        &request,
    ))
    .unwrap();
    let denial = world
        .runtime
        .authorize_approve_payment(&initiator_actor, payment, Default::default(), &request)
        .err()
        .expect("initiator deny path must override approver role");
    assert!(matches!(
        denial,
        BankOperationAdmissionError::Authorization(ref denial)
            if denial.code() == "permission-denied"
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
        .authorize_approve_payment(&actor, payment, Default::default(), &request)
        .unwrap();
    let denial = BankOperationProposals::prepare_approve_payment(
        &world.runtime,
        admission,
        &key("relabel"),
        &ApprovePayment {
            payment,
            approver: id(BankPrincipalId::new, 2),
        },
    )
    .err()
    .expect("input actor cannot differ from authenticated actor");
    assert_eq!(
        denial,
        BankOperationProposalError::Invariant(BankProposalDenial::AuthenticatedActorMismatch)
    );
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
    assert_permission_denied(world.runtime.authorize_approve_payment(
        &actor,
        payment,
        Default::default(),
        &request,
    ));
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
            account: authorization.account(),
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
    assert_permission_denied(world.runtime.authorize_approve_payment(
        &actor,
        payment,
        Default::default(),
        &request,
    ));
}

fn assert_permission_denied<T>(result: Result<T, BankOperationAdmissionError>) {
    assert!(matches!(
        result,
        Err(BankOperationAdmissionError::Authorization(ref denial))
            if denial.code() == "permission-denied"
    ));
}
