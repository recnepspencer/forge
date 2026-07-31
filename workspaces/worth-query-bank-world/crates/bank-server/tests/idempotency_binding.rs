mod support;

use bank_domain::model::{
    AccountId, BankPrincipalId, BankSnapshotVersion, EmployeeAssignmentId, EmployeeRole,
    InstitutionId,
};
use bank_domain::proposals::BankSnapshotBuilder;
use bank_server::{BankEmployeeAssignmentSeed, BankPrincipalSeed, BankWorldSeed};

use support::{block_on, request_scope, runtime, CausalCredential, DynamicIdentity};

fn id<T>(constructor: impl FnOnce(u64) -> Option<T>, value: u64) -> T {
    constructor(value).unwrap()
}

#[test]
fn real_query_binding_separates_principal_operation_and_scope_but_not_retry() {
    let snapshot = BankSnapshotBuilder::new(id(BankSnapshotVersion::new, 1))
        .institution(id(InstitutionId::new, 1))
        .institution(id(InstitutionId::new, 2))
        .principal(id(BankPrincipalId::new, 1))
        .principal(id(BankPrincipalId::new, 2))
        .institution_cash_account(id(AccountId::new, 100), id(InstitutionId::new, 1))
        .institution_cash_account(id(AccountId::new, 200), id(InstitutionId::new, 2))
        .build()
        .unwrap();
    let first = DynamicIdentity::new("first-teller");
    let second = DynamicIdentity::new("second-teller");
    let world = runtime(
        BankWorldSeed::new(snapshot)
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 1),
                first.external(),
            ))
            .principal(BankPrincipalSeed::enabled(
                id(BankPrincipalId::new, 2),
                second.external(),
            ))
            .employee(BankEmployeeAssignmentSeed::new(
                id(EmployeeAssignmentId::new, 1),
                id(InstitutionId::new, 1),
                id(BankPrincipalId::new, 1),
                EmployeeRole::Teller,
            ))
            .employee(BankEmployeeAssignmentSeed::new(
                id(EmployeeAssignmentId::new, 2),
                id(InstitutionId::new, 2),
                id(BankPrincipalId::new, 1),
                EmployeeRole::Teller,
            ))
            .employee(BankEmployeeAssignmentSeed::new(
                id(EmployeeAssignmentId::new, 3),
                id(InstitutionId::new, 1),
                id(BankPrincipalId::new, 2),
                EmployeeRole::Teller,
            )),
    );
    let request = request_scope();
    let first_actor = block_on(world.runtime.authenticate_with(
        &world.authentication,
        CausalCredential::for_identity(&first),
        &request,
    ))
    .unwrap();
    let second_actor = block_on(world.runtime.authenticate_with(
        &world.authentication,
        CausalCredential::for_identity(&second),
        &request,
    ))
    .unwrap();

    let baseline = world
        .runtime
        .authorize_deposit(
            &first_actor,
            id(InstitutionId::new, 1),
            Default::default(),
            &request,
        )
        .unwrap()
        .operation_scope_fingerprint();
    let retry = world
        .runtime
        .authorize_deposit(
            &first_actor,
            id(InstitutionId::new, 1),
            Default::default(),
            &request_scope(),
        )
        .unwrap()
        .operation_scope_fingerprint();
    let principal_drift = world
        .runtime
        .authorize_deposit(
            &second_actor,
            id(InstitutionId::new, 1),
            Default::default(),
            &request,
        )
        .unwrap()
        .operation_scope_fingerprint();
    let operation_drift = world
        .runtime
        .authorize_withdrawal(
            &first_actor,
            id(InstitutionId::new, 1),
            Default::default(),
            &request,
        )
        .unwrap()
        .operation_scope_fingerprint();
    let scope_drift = world
        .runtime
        .authorize_deposit(
            &first_actor,
            id(InstitutionId::new, 2),
            Default::default(),
            &request,
        )
        .unwrap()
        .operation_scope_fingerprint();

    assert_eq!(baseline, retry);
    assert_ne!(baseline, principal_drift);
    assert_ne!(baseline, operation_drift);
    assert_ne!(baseline, scope_drift);
}
