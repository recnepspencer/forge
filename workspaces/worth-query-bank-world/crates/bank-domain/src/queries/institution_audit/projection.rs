use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationProjection, WorthQueryApplicationProjectionDenial,
    WorthQueryApplicationProjectionRow,
};

use crate::model::AccountId;
use crate::reads::{AccountActivityItem, InstitutionAuditAccount, InstitutionAuditView};
use crate::schema::BankSchema;

use super::fields::*;
use super::relations::*;
use super::InstitutionAuditQuery;

impl WorthQueryApplicationProjection<BankSchema, InstitutionAuditQuery> for InstitutionAuditView {
    fn project(
        row: &WorthQueryApplicationProjectionRow<'_, BankSchema, InstitutionAuditQuery>,
    ) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        let institution = row.field(institution_identity())?;
        let accounts = row
            .many(institution_accounts())?
            .iter()
            .map(|account| project_account(&account))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::from_projection(institution, accounts))
    }
}

fn project_account(
    row: &WorthQueryApplicationProjectionRow<'_, BankSchema, InstitutionAuditQuery>,
) -> Result<InstitutionAuditAccount, WorthQueryApplicationProjectionDenial> {
    let account = row.field(account_identity())?;
    let entries = row
        .many(account_postings())?
        .iter()
        .map(|posting| project_posting(account, &posting))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(InstitutionAuditAccount::from_projection(account, entries))
}

fn project_posting(
    account: AccountId,
    row: &WorthQueryApplicationProjectionRow<'_, BankSchema, InstitutionAuditQuery>,
) -> Result<AccountActivityItem, WorthQueryApplicationProjectionDenial> {
    let _posting = row.field(posting_identity())?;
    let sequence = row.field(posting_sequence())?;
    let amount = row.field(posting_amount())?;
    let purpose = row.field(posting_purpose())?;
    let journal = row.one(posting_journal())?;
    let journal_id = journal.field(journal_identity::<JournalIdentitySlot>("journal"))?;
    if journal.field(journal_purpose())? != purpose {
        return Err(WorthQueryApplicationProjectionDenial::reject(
            "posting and journal purpose disagree",
        ));
    }
    let reversal_of = journal
        .optional(journal_reversal())?
        .map(|reversal| reversal.field(journal_identity::<ReversalIdentitySlot>("reversal_of")))
        .transpose()?;
    Ok(AccountActivityItem::from_projection(
        account,
        sequence,
        journal_id,
        purpose,
        amount,
        reversal_of,
    ))
}
