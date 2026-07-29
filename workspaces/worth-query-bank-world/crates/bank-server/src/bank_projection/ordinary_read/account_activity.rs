use bank_domain::model::{AccountId, AccountJournalRevision, JournalEntryId, SignedMoney, USD};
use bank_domain::proposals::BankProposalDenial;
use bank_domain::reads::AccountActivityItem;
use bank_domain::schema::*;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationOperationInvariantProjectionReader, WorthQueryInvariantEntityIdentity,
};

use crate::ordinary::{BankProjectedActivityPage, BankReadProjectedBatch};
use crate::BankProjectionDenial;

use super::account::{validate_account, AccountEntity};

pub(crate) fn project_account_activity_read(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        ReadAccountActivityOperation,
    >,
    root: &AccountEntity,
    expected: AccountId,
    maximum_results: usize,
) -> Result<BankReadProjectedBatch<Vec<AccountActivityItem>>, BankProjectionDenial> {
    validate_account(reader, root, expected)?;
    let (_, mut activity) = account_postings(reader, root)?;
    activity.sort_by_key(|item| item.account_sequence());
    let truncated = activity.len() > maximum_results;
    activity.truncate(maximum_results);
    let count = activity.len();
    Ok(if truncated {
        BankReadProjectedBatch::truncated(activity, count)
    } else {
        BankReadProjectedBatch::complete(activity, count)
    })
}

pub(crate) fn project_account_activity_cause_read(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        ReadAccountActivityOperation,
    >,
    root: &AccountEntity,
    expected: AccountId,
    journal: JournalEntryId,
    journal_sequence: u64,
) -> Result<BankReadProjectedBatch<Option<AccountActivityItem>>, BankProjectionDenial> {
    validate_account(reader, root, expected)?;
    let revision = required(
        reader.field(root, AccountingRevision::reference()),
        "AccountingRevision",
    )?;
    if revision.get() < journal_sequence {
        return Ok(BankReadProjectedBatch::complete(None, 0));
    }
    let journal = reader.resolve_optional_entity(JournalIdentityField::reference(), journal)?;
    let Some(journal) = journal else {
        return Ok(BankReadProjectedBatch::complete(None, 0));
    };
    let expected_sequence = AccountJournalRevision::from_posting_count(journal_sequence);
    let mut activity = None;
    for relation in reader.relations_from(JournalPosting::reference(), &journal)? {
        let posting = relation.to();
        if reader.field(posting, PostingAccountSequence::reference()) != Some(expected_sequence) {
            continue;
        }
        let accounts = reader.relations_from(PostingAccount::reference(), posting)?;
        if exactly_one(&accounts, "PostingAccount")?.to() != root {
            continue;
        }
        if activity.is_some() {
            return Err(BankProjectionDenial::AmbiguousRelation(
                "PostingAccountSequence",
            ));
        }
        activity = Some(project_activity_item(reader, expected, posting, &journal)?);
    }
    Ok(BankReadProjectedBatch::complete(
        activity,
        usize::from(activity.is_some()),
    ))
}

pub(crate) fn project_account_activity_page_read(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        ReadAccountActivityOperation,
    >,
    root: &AccountEntity,
    expected: AccountId,
    offset: usize,
    maximum_results: usize,
) -> Result<BankReadProjectedBatch<BankProjectedActivityPage>, BankProjectionDenial> {
    validate_account(reader, root, expected)?;
    let (_, mut activity) = account_postings(reader, root)?;
    activity.sort_by_key(|item| item.account_sequence());
    let total = activity.len();
    let entries = activity
        .into_iter()
        .skip(offset)
        .take(maximum_results)
        .collect::<Vec<_>>();
    let next_offset = offset
        .checked_add(entries.len())
        .filter(|next| *next < total);
    let count = entries.len();
    let page = BankProjectedActivityPage {
        entries,
        next_offset,
    };
    Ok(if next_offset.is_some() {
        BankReadProjectedBatch::truncated(page, count)
    } else {
        BankReadProjectedBatch::complete(page, count)
    })
}

pub(super) fn account_postings<Operation>(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        Operation,
    >,
    account: &AccountEntity,
) -> Result<(SignedMoney<USD>, Vec<AccountActivityItem>), BankProjectionDenial>
where
    AccountIdentity: worth_query_host::facade::domain::OperationReads<Operation>,
    AccountingRevision: worth_query_host::facade::domain::OperationReads<Operation>,
    PostingAccount: worth_query_host::facade::domain::OperationReads<Operation>,
    JournalPosting: worth_query_host::facade::domain::OperationReads<Operation>,
    JournalIdentityField: worth_query_host::facade::domain::OperationReads<Operation>,
    JournalPurpose: worth_query_host::facade::domain::OperationReads<Operation>,
    PostingAmount: worth_query_host::facade::domain::OperationReads<Operation>,
    PostingAccountSequence: worth_query_host::facade::domain::OperationReads<Operation>,
    Purpose: worth_query_host::facade::domain::OperationReads<Operation>,
    JournalReversal: worth_query_host::facade::domain::OperationReads<Operation>,
{
    let account_id = required(
        reader.field(account, AccountIdentity::reference()),
        "AccountIdentity",
    )?;
    let revision = required(
        reader.field(account, AccountingRevision::reference()),
        "AccountingRevision",
    )?;
    let postings = reader.relations_to(PostingAccount::reference(), account)?;
    if u64::try_from(postings.len()).ok() != Some(revision.get()) {
        return Err(BankProjectionDenial::AccountingRevisionMismatch(account_id));
    }
    let mut balance = 0_i64;
    let mut activity = Vec::with_capacity(postings.len());
    for relation in postings {
        let posting = relation.from();
        let owners = reader.relations_to(JournalPosting::reference(), posting)?;
        let journal = exactly_one(&owners, "JournalPosting")?.from();
        let item = project_activity_item(reader, account_id, posting, journal)?;
        balance = balance.checked_add(item.amount().minor_units()).ok_or(
            BankProjectionDenial::InvalidDomainState(BankProposalDenial::SnapshotInvariantViolated),
        )?;
        activity.push(item);
    }
    Ok((SignedMoney::from_minor(balance), activity))
}

fn project_activity_item<Operation>(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        '_,
        '_,
        BankSchema,
        Operation,
    >,
    account: AccountId,
    posting: &WorthQueryInvariantEntityIdentity<BankSchema, Posting>,
    journal: &WorthQueryInvariantEntityIdentity<BankSchema, JournalEntry>,
) -> Result<AccountActivityItem, BankProjectionDenial>
where
    JournalIdentityField: worth_query_host::facade::domain::OperationReads<Operation>,
    JournalPurpose: worth_query_host::facade::domain::OperationReads<Operation>,
    PostingAmount: worth_query_host::facade::domain::OperationReads<Operation>,
    PostingAccountSequence: worth_query_host::facade::domain::OperationReads<Operation>,
    Purpose: worth_query_host::facade::domain::OperationReads<Operation>,
    JournalReversal: worth_query_host::facade::domain::OperationReads<Operation>,
{
    let journal_purpose = required(
        reader.field(journal, JournalPurpose::reference()),
        "JournalPurpose",
    )?;
    let posting_purpose = required(reader.field(posting, Purpose::reference()), "Purpose")?;
    if journal_purpose != posting_purpose {
        return Err(BankProjectionDenial::InvalidDomainState(
            BankProposalDenial::SnapshotInvariantViolated,
        ));
    }
    let reversals = reader.relations_from(JournalReversal::reference(), journal)?;
    let reversal_of = match reversals.as_slice() {
        [] => None,
        [reversal] => Some(required(
            reader.field(reversal.to(), JournalIdentityField::reference()),
            "JournalIdentityField",
        )?),
        _ => return Err(BankProjectionDenial::AmbiguousRelation("JournalReversal")),
    };
    Ok(AccountActivityItem::from_projection(
        account,
        required(
            reader.field(posting, PostingAccountSequence::reference()),
            "PostingAccountSequence",
        )?,
        required(
            reader.field(journal, JournalIdentityField::reference()),
            "JournalIdentityField",
        )?,
        journal_purpose,
        required(
            reader.field(posting, PostingAmount::reference()),
            "PostingAmount",
        )?,
        reversal_of,
    ))
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
