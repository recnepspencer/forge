use crate::model::{AccountId, SignedMoney, USD};

use super::BankJournalEntry;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountBalanceDenial {
    ArithmeticOverflow,
}

pub fn account_balance(
    journal: &[BankJournalEntry],
    account: AccountId,
) -> Result<SignedMoney<USD>, AccountBalanceDenial> {
    let balance = journal
        .iter()
        .flat_map(BankJournalEntry::postings)
        .filter(|posting| posting.account() == account)
        .try_fold(0_i64, |balance, posting| {
            balance.checked_add(posting.amount().minor_units())
        })
        .ok_or(AccountBalanceDenial::ArithmeticOverflow)?;
    Ok(SignedMoney::from_minor(balance))
}
