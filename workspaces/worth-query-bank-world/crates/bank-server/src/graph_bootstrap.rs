use bank_domain::accounting::account_balance;
use bank_domain::proposals::BankSnapshot;
use bank_domain::schema::*;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationEntityKey, WorthQueryApplicationEntitySeed,
    WorthQueryApplicationRelationSeed, WorthQueryPrimaryGraphBootstrap,
    WorthQueryPrimaryGraphInstallationDenial,
};

use crate::{BankBusinessOwnerSeed, BankEmployeeAssignmentSeed};

pub(crate) fn bind_bank_world(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    snapshot: &BankSnapshot,
    owners: &[BankBusinessOwnerSeed],
    employees: &[BankEmployeeAssignmentSeed],
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    bind_institutions(graph, snapshot)?;
    bind_businesses(graph, snapshot)?;
    bind_accounts(graph, snapshot)?;
    bind_authorizations(graph, snapshot)?;
    bind_employees(graph, employees)?;
    bind_payments(graph, snapshot)?;
    bind_ownership_relations(graph, snapshot, owners)?;
    bind_authorization_relations(graph, snapshot)?;
    bind_employee_relations(graph, employees)?;
    bind_payment_relations(graph, snapshot)
}

fn bind_institutions(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    snapshot: &BankSnapshot,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for institution in snapshot.institutions() {
        graph.bind_entity(
            WorthQueryApplicationEntitySeed::new(
                Institution::reference(),
                entity_key(institution_key(institution.get())),
            )
            .field(InstitutionIdentityField::reference(), institution),
        )?;
    }
    Ok(())
}

fn bind_businesses(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    snapshot: &BankSnapshot,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for business in snapshot.businesses() {
        graph.bind_entity(
            WorthQueryApplicationEntitySeed::new(
                Business::reference(),
                entity_key(business_key(business.get())),
            )
            .field(BusinessIdentityField::reference(), business),
        )?;
    }
    Ok(())
}

fn bind_accounts(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    snapshot: &BankSnapshot,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for account in snapshot.accounts() {
        let balance = account_balance(snapshot.journal(), account.id())
            .expect("validated bank snapshot balances cannot overflow");
        graph.bind_entity(
            WorthQueryApplicationEntitySeed::new(
                Account::reference(),
                entity_key(account_key(account.id().get())),
            )
            .field(AccountIdentity::reference(), account.id())
            .field(
                AccountDisplayName::reference(),
                account.display_name().clone(),
            )
            .field(Kind::reference(), account.kind())
            .field(AvailableBalance::reference(), balance)
            .field(Status::reference(), account.status()),
        )?;
    }
    Ok(())
}

fn bind_authorizations(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    snapshot: &BankSnapshot,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for authorization in snapshot.authorizations() {
        graph.bind_entity(
            WorthQueryApplicationEntitySeed::new(
                AccountAuthorization::reference(),
                entity_key(authorization_key(authorization.id().get())),
            )
            .field(AuthorizationRole::reference(), authorization.role()),
        )?;
    }
    Ok(())
}

fn bind_employees(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    employees: &[BankEmployeeAssignmentSeed],
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for employee in employees {
        graph.bind_entity(
            WorthQueryApplicationEntitySeed::new(
                EmployeeAssignment::reference(),
                entity_key(employee_key(employee.id().get())),
            )
            .field(AssignmentRole::reference(), employee.role()),
        )?;
    }
    Ok(())
}

fn bind_payments(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    snapshot: &BankSnapshot,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for payment in snapshot.payments() {
        graph.bind_entity(
            WorthQueryApplicationEntitySeed::new(
                PaymentIntent::reference(),
                entity_key(payment_key(payment.id().get())),
            )
            .field(PaymentIdentityField::reference(), payment.id())
            .field(PaymentStatusField::reference(), payment.status()),
        )?;
    }
    Ok(())
}

fn bind_ownership_relations(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    snapshot: &BankSnapshot,
    owners: &[BankBusinessOwnerSeed],
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for account in snapshot.accounts() {
        graph.bind_relation(WorthQueryApplicationRelationSeed::new(
            InstitutionAccount::reference(),
            format!("institution-account:{}", account.id().get()),
            entity_key(institution_key(account.institution().get())),
            entity_key(account_key(account.id().get())),
        ))?;
        if let Some(owner) = account.personal_owner() {
            graph.bind_relation(WorthQueryApplicationRelationSeed::new(
                PersonalOwner::reference(),
                format!("personal-owner:{}", account.id().get()),
                entity_key(principal_key(owner.get())),
                entity_key(account_key(account.id().get())),
            ))?;
        }
        if let Some(business) = account.business_owner() {
            graph.bind_relation(WorthQueryApplicationRelationSeed::new(
                BusinessAccount::reference(),
                format!("business-account:{}", account.id().get()),
                entity_key(business_key(business.get())),
                entity_key(account_key(account.id().get())),
            ))?;
        }
    }
    for owner in owners {
        graph.bind_relation(WorthQueryApplicationRelationSeed::new(
            BusinessOwner::reference(),
            format!(
                "business-owner:{}:{}",
                owner.business().get(),
                owner.principal().get()
            ),
            entity_key(business_key(owner.business().get())),
            entity_key(principal_key(owner.principal().get())),
        ))?;
    }
    Ok(())
}

fn bind_authorization_relations(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    snapshot: &BankSnapshot,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for authorization in snapshot.authorizations() {
        graph.bind_relation(WorthQueryApplicationRelationSeed::new(
            AccountAuthorizedUser::reference(),
            format!("authorized-user:{}", authorization.id().get()),
            entity_key(principal_key(authorization.principal().get())),
            entity_key(authorization_key(authorization.id().get())),
        ))?;
        graph.bind_relation(WorthQueryApplicationRelationSeed::new(
            AuthorizationAccount::reference(),
            format!("authorization-account:{}", authorization.id().get()),
            entity_key(authorization_key(authorization.id().get())),
            entity_key(account_key(authorization.account().get())),
        ))?;
    }
    Ok(())
}

fn bind_employee_relations(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    employees: &[BankEmployeeAssignmentSeed],
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for employee in employees {
        graph.bind_relation(WorthQueryApplicationRelationSeed::new(
            InstitutionEmployee::reference(),
            format!("institution-employee:{}", employee.id().get()),
            entity_key(institution_key(employee.institution().get())),
            entity_key(employee_key(employee.id().get())),
        ))?;
        graph.bind_relation(WorthQueryApplicationRelationSeed::new(
            AssignmentPrincipal::reference(),
            format!("assignment-principal:{}", employee.id().get()),
            entity_key(employee_key(employee.id().get())),
            entity_key(principal_key(employee.principal().get())),
        ))?;
    }
    Ok(())
}

fn bind_payment_relations(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    snapshot: &BankSnapshot,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for payment in snapshot.payments() {
        let payment_key_value = payment_key(payment.id().get());
        graph.bind_relation(WorthQueryApplicationRelationSeed::new(
            PaymentSource::reference(),
            format!("payment-source:{}", payment.id().get()),
            entity_key(payment_key_value.clone()),
            entity_key(account_key(payment.source().get())),
        ))?;
        graph.bind_relation(WorthQueryApplicationRelationSeed::new(
            PaymentDestination::reference(),
            format!("payment-destination:{}", payment.id().get()),
            entity_key(payment_key_value.clone()),
            entity_key(account_key(payment.destination().get())),
        ))?;
        graph.bind_relation(WorthQueryApplicationRelationSeed::new(
            PaymentInitiator::reference(),
            format!("payment-initiator:{}", payment.id().get()),
            entity_key(principal_key(payment.initiator().get())),
            entity_key(payment_key_value),
        ))?;
    }
    Ok(())
}

fn entity_key<Schema, Entity>(value: String) -> WorthQueryApplicationEntityKey<Schema, Entity> {
    WorthQueryApplicationEntityKey::new(value)
        .expect("bank keys are bounded canonical numeric identities")
}

pub(crate) fn principal_key(id: u64) -> String {
    format!("bank-principal:{id}")
}

fn institution_key(id: u64) -> String {
    format!("bank-institution:{id}")
}

fn business_key(id: u64) -> String {
    format!("bank-business:{id}")
}

fn account_key(id: u64) -> String {
    format!("bank-account:{id}")
}

fn authorization_key(id: u64) -> String {
    format!("bank-authorization:{id}")
}

fn employee_key(id: u64) -> String {
    format!("bank-employee:{id}")
}

fn payment_key(id: u64) -> String {
    format!("bank-payment:{id}")
}
