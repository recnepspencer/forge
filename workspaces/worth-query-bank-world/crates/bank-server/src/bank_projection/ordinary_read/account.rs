use bank_domain::model::AccountId;
use bank_domain::reads::{AccountDetail, AccountSummary, AuthorizedAccountUser};
use bank_domain::schema::*;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationOperationInvariantProjectionReader, WorthQueryInvariantEntityIdentity,
};

use crate::ordinary::BankReadProjectedBatch;
use crate::BankProjectionDenial;

use super::account_activity::account_postings;

pub(super) type AccountEntity = WorthQueryInvariantEntityIdentity<BankSchema, Account>;

pub(crate) fn project_account_summary_read(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        ReadAccountSummaryOperation,
    >,
    root: &AccountEntity,
    expected: AccountId,
) -> Result<BankReadProjectedBatch<AccountSummary>, BankProjectionDenial> {
    let summary = project_summary(reader, root, expected)?;
    Ok(BankReadProjectedBatch::complete(summary, 1))
}

pub(crate) fn project_account_detail_read(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        ReadAccountDetailOperation,
    >,
    root: &AccountEntity,
    expected: AccountId,
) -> Result<BankReadProjectedBatch<AccountDetail>, BankProjectionDenial> {
    let summary = project_summary(reader, root, expected)?;
    let institution_relations = reader.relations_to(InstitutionAccount::reference(), root)?;
    let institution = exactly_one(&institution_relations, "InstitutionAccount")?.from();
    let institution = required(
        reader.field(institution, InstitutionIdentityField::reference()),
        "InstitutionIdentityField",
    )?;
    let kind = summary.kind();
    let personal = reader.relations_to(PersonalOwner::reference(), root)?;
    let business = reader.relations_to(BusinessAccount::reference(), root)?;
    let (personal_owner, business_owner) = match kind {
        AccountKind::Personal => {
            let owner = exactly_one(&personal, "PersonalOwner")?.from();
            ensure_empty(&business, "BusinessAccount")?;
            (
                Some(required(
                    reader.field(owner, PrincipalIdentityField::reference()),
                    "PrincipalIdentityField",
                )?),
                None,
            )
        }
        AccountKind::Business => {
            let owner = exactly_one(&business, "BusinessAccount")?.from();
            ensure_empty(&personal, "PersonalOwner")?;
            (
                None,
                Some(required(
                    reader.field(owner, BusinessIdentityField::reference()),
                    "BusinessIdentityField",
                )?),
            )
        }
        AccountKind::InstitutionCash | AccountKind::InstitutionSettlement => {
            ensure_empty(&personal, "PersonalOwner")?;
            ensure_empty(&business, "BusinessAccount")?;
            (None, None)
        }
    };
    Ok(BankReadProjectedBatch::complete(
        AccountDetail::from_projection(summary, institution, personal_owner, business_owner),
        1,
    ))
}

pub(crate) fn project_account_users_read(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        ReadAccountAuthorizedUsersOperation,
    >,
    root: &AccountEntity,
    expected: AccountId,
    maximum_results: usize,
) -> Result<BankReadProjectedBatch<Vec<AuthorizedAccountUser>>, BankProjectionDenial> {
    validate_account(reader, root, expected)?;
    let relations = reader.relations_to(AuthorizationAccount::reference(), root)?;
    let mut users = Vec::with_capacity(relations.len().min(maximum_results));
    for relation in relations.iter().take(maximum_results) {
        let authorization = relation.from();
        let owners = reader.relations_to(AccountAuthorizedUser::reference(), authorization)?;
        let owner = exactly_one(&owners, "AccountAuthorizedUser")?.from();
        users.push(AuthorizedAccountUser::from_projection(
            required(
                reader.field(authorization, AccountAuthorizationIdentity::reference()),
                "AccountAuthorizationIdentity",
            )?,
            required(
                reader.field(owner, PrincipalIdentityField::reference()),
                "PrincipalIdentityField",
            )?,
            required(
                reader.field(authorization, AuthorizationRole::reference()),
                "AuthorizationRole",
            )?,
        ));
    }
    users.sort_by_key(|user| user.authorization());
    let count = users.len();
    Ok(if relations.len() > maximum_results {
        BankReadProjectedBatch::truncated(users, count)
    } else {
        BankReadProjectedBatch::complete(users, count)
    })
}

fn project_summary<Operation>(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        Operation,
    >,
    root: &AccountEntity,
    expected: AccountId,
) -> Result<AccountSummary, BankProjectionDenial>
where
    AccountIdentity: worth_query_host::facade::domain::OperationReads<Operation>,
    AccountDisplayName: worth_query_host::facade::domain::OperationReads<Operation>,
    AccountingRevision: worth_query_host::facade::domain::OperationReads<Operation>,
    Kind: worth_query_host::facade::domain::OperationReads<Operation>,
    Status: worth_query_host::facade::domain::OperationReads<Operation>,
    PostingAccount: worth_query_host::facade::domain::OperationReads<Operation>,
    JournalPosting: worth_query_host::facade::domain::OperationReads<Operation>,
    JournalIdentityField: worth_query_host::facade::domain::OperationReads<Operation>,
    JournalPurpose: worth_query_host::facade::domain::OperationReads<Operation>,
    PostingAmount: worth_query_host::facade::domain::OperationReads<Operation>,
    PostingAccountSequence: worth_query_host::facade::domain::OperationReads<Operation>,
    Purpose: worth_query_host::facade::domain::OperationReads<Operation>,
    JournalReversal: worth_query_host::facade::domain::OperationReads<Operation>,
{
    validate_account(reader, root, expected)?;
    let (balance, _) = account_postings(reader, root)?;
    Ok(AccountSummary::from_projection(
        expected,
        required(
            reader.field(root, AccountDisplayName::reference()),
            "AccountDisplayName",
        )?,
        required(reader.field(root, Kind::reference()), "Kind")?,
        required(reader.field(root, Status::reference()), "Status")?,
        balance,
        balance,
    ))
}

pub(super) fn validate_account<Operation>(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        Operation,
    >,
    root: &AccountEntity,
    expected: AccountId,
) -> Result<(), BankProjectionDenial>
where
    AccountIdentity: worth_query_host::facade::domain::OperationReads<Operation>,
{
    match reader.field(root, AccountIdentity::reference()) {
        Some(actual) if actual == expected => Ok(()),
        Some(_) => Err(BankProjectionDenial::AmbiguousRelation("AccountIdentity")),
        None => Err(BankProjectionDenial::MissingField("AccountIdentity")),
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

fn ensure_empty<Row>(rows: &[Row], relation: &'static str) -> Result<(), BankProjectionDenial> {
    if rows.is_empty() {
        Ok(())
    } else {
        Err(BankProjectionDenial::AmbiguousRelation(relation))
    }
}

fn required<Value>(
    value: Option<Value>,
    field: &'static str,
) -> Result<Value, BankProjectionDenial> {
    value.ok_or(BankProjectionDenial::MissingField(field))
}
