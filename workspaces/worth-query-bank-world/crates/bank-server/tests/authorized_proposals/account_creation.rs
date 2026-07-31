use bank_domain::model::{
    AccountName, BankPrincipalId, BusinessId, EmployeeAssignmentId, EmployeeRole, InstitutionId,
};
use bank_domain::schema::{CreateBusinessAccount, CreatePersonalAccount};
use bank_server::{
    BankEmployeeAssignmentSeed, BankOperationProposals, BankPrincipalSeed, BankWorldSeed,
};

use super::fixture::{funded_personal_world, id, key};
use crate::support::{block_on, request_scope, runtime, CausalCredential, DynamicIdentity};

#[test]
fn teller_authority_opens_an_account_for_a_real_customer_identity() {
    let snapshot = funded_personal_world();
    let owner = DynamicIdentity::new("owner");
    let recipient = DynamicIdentity::new("recipient");
    let teller = DynamicIdentity::new("teller");
    let world = runtime(
        BankWorldSeed::new(snapshot)
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
        .authorize_create_personal_account(
            &actor,
            id(InstitutionId::new, 1),
            Default::default(),
            &request,
        )
        .unwrap();
    let proposal = BankOperationProposals::prepare_create_personal_account(
        &world.runtime,
        admission,
        &key("employee-open"),
        &CreatePersonalAccount {
            institution: id(InstitutionId::new, 1),
            owner: id(BankPrincipalId::new, 3),
            display_name: AccountName::new("Teller-created").unwrap(),
        },
    )
    .unwrap();
    assert_eq!(proposal.admission().actor(), id(BankPrincipalId::new, 3));

    let admission = world
        .runtime
        .authorize_create_business_account(
            &actor,
            id(InstitutionId::new, 1),
            Default::default(),
            &request,
        )
        .unwrap();
    let proposal = BankOperationProposals::prepare_create_business_account(
        &world.runtime,
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
