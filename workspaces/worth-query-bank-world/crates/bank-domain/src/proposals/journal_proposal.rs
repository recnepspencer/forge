use crate::accounting::{account_balance, BankJournalEntry, BankPosting};
use crate::model::{AccountId, JournalEntryId, Money, SignedMoney, USD};
use crate::schema::{AccountKind, AccountStatus, PostingPurpose};

use super::{BankProposalDenial, BankProposedEffect, BankSnapshot};

pub(crate) fn append_balanced_transfer(
    snapshot: &mut BankSnapshot,
    debit: AccountId,
    credit: AccountId,
    amount: Money<USD>,
    purpose: PostingPurpose,
    reversal_of: Option<JournalEntryId>,
) -> Result<BankJournalEntry, BankProposalDenial> {
    if debit == credit {
        return Err(BankProposalDenial::SelfTransfer);
    }
    ensure_open(snapshot, debit)?;
    ensure_open(snapshot, credit)?;
    ensure_available_funds(snapshot, debit, amount)?;

    let journal_id = snapshot.allocate_journal_id()?;
    let debit_posting = BankPosting::new(
        snapshot.allocate_posting_id()?,
        debit,
        SignedMoney::from_minor(
            amount
                .minor_units()
                .checked_neg()
                .ok_or(BankProposalDenial::ArithmeticOverflow)?,
        ),
    );
    let credit_posting = BankPosting::new(
        snapshot.allocate_posting_id()?,
        credit,
        SignedMoney::from(amount),
    );
    let entry = BankJournalEntry::new(
        journal_id,
        purpose,
        vec![debit_posting, credit_posting],
        reversal_of,
    );
    snapshot.append_journal(entry.clone());
    Ok(entry)
}

pub(crate) fn account_activity_effects(
    entry: &BankJournalEntry,
) -> impl Iterator<Item = BankProposedEffect> + '_ {
    entry.postings().iter().map(|posting| {
        BankProposedEffect::EmitAccountActivity(crate::schema::ActivityEvent {
            account: posting.account(),
            journal_sequence: entry.id().get(),
        })
    })
}

pub(crate) fn ensure_open(
    snapshot: &BankSnapshot,
    account_id: AccountId,
) -> Result<(), BankProposalDenial> {
    let account = snapshot
        .account(account_id)
        .ok_or(BankProposalDenial::UnknownAccount(account_id))?;
    if account.status() != AccountStatus::Open {
        return Err(BankProposalDenial::AccountStatus {
            account: account_id,
            status: account.status(),
        });
    }
    Ok(())
}

fn ensure_available_funds(
    snapshot: &BankSnapshot,
    account_id: AccountId,
    amount: Money<USD>,
) -> Result<(), BankProposalDenial> {
    let account = snapshot
        .account(account_id)
        .ok_or(BankProposalDenial::UnknownAccount(account_id))?;
    if matches!(
        account.kind(),
        AccountKind::InstitutionCash | AccountKind::InstitutionSettlement
    ) {
        return Ok(());
    }
    let available = account_balance(snapshot.journal(), account_id)
        .map_err(|_| BankProposalDenial::ArithmeticOverflow)?;
    if available.minor_units() < amount.minor_units() {
        return Err(BankProposalDenial::InsufficientFunds(account_id));
    }
    Ok(())
}
