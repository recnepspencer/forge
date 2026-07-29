use std::collections::BTreeSet;

use bank_domain::model::BankPrincipalId;
use bank_domain::reads::VisibleAccount;
use bank_domain::schema::*;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationOperationInvariantProjectionReader, WorthQueryInvariantEntityIdentity,
};

use crate::ordinary::BankReadProjectedBatch;
use crate::BankProjectionDenial;

pub(crate) fn project_account_discovery_read(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        DiscoverAccountsOperation,
    >,
    root: &WorthQueryInvariantEntityIdentity<BankSchema, Principal>,
    principal: BankPrincipalId,
    maximum_results: usize,
) -> Result<BankReadProjectedBatch<Vec<VisibleAccount>>, BankProjectionDenial> {
    validate_principal(reader, root, principal)?;
    let mut accounts = BTreeSet::new();
    for relation in reader.relations_from(PersonalOwner::reference(), root)? {
        accounts.insert(relation.to().clone());
    }
    for relation in reader.relations_from(AccountAuthorizedUser::reference(), root)? {
        let account_relations =
            reader.relations_from(AuthorizationAccount::reference(), relation.to())?;
        accounts.insert(
            exactly_one(&account_relations, "AuthorizationAccount")?
                .to()
                .clone(),
        );
    }
    for relation in reader.relations_to(BusinessOwner::reference(), root)? {
        for account in reader.relations_from(BusinessAccount::reference(), relation.from())? {
            accounts.insert(account.to().clone());
        }
    }

    let truncated = accounts.len() > maximum_results;
    let mut visible = Vec::with_capacity(accounts.len().min(maximum_results));
    for account in accounts.into_iter().take(maximum_results) {
        let id = required(
            reader.field(&account, AccountIdentity::reference()),
            "AccountIdentity",
        )?;
        let canonical = reader.resolve_entity(AccountIdentity::reference(), id)?;
        if canonical != account {
            return Err(BankProjectionDenial::AmbiguousRelation("AccountIdentity"));
        }
        visible.push(VisibleAccount::new(id));
    }
    visible.sort_by_key(|account| account.id());
    let count = visible.len();
    Ok(if truncated {
        BankReadProjectedBatch::truncated(visible, count)
    } else {
        BankReadProjectedBatch::complete(visible, count)
    })
}

fn validate_principal(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        DiscoverAccountsOperation,
    >,
    root: &WorthQueryInvariantEntityIdentity<BankSchema, Principal>,
    expected: BankPrincipalId,
) -> Result<(), BankProjectionDenial> {
    match reader.field(root, PrincipalIdentityField::reference()) {
        Some(actual) if actual == expected => Ok(()),
        Some(_) => Err(BankProjectionDenial::AmbiguousRelation(
            "PrincipalIdentityField",
        )),
        None => Err(BankProjectionDenial::MissingField("PrincipalIdentityField")),
    }
}

fn exactly_one<'row, Row>(
    rows: &'row [Row],
    relation: &'static str,
) -> Result<&'row Row, BankProjectionDenial> {
    match rows {
        [row] => Ok(row),
        [] => Err(BankProjectionDenial::MissingRelation(relation)),
        _ => Err(BankProjectionDenial::AmbiguousRelation(relation)),
    }
}

fn required<Value>(
    value: Option<Value>,
    field: &'static str,
) -> Result<Value, BankProjectionDenial> {
    value.ok_or(BankProjectionDenial::MissingField(field))
}
