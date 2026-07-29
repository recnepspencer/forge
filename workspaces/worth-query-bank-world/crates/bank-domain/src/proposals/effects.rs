use super::BankAccountAuthorization;
use crate::accounting::{BankAccount, BankJournalEntry};
use crate::model::PaymentId;
use crate::payments::BusinessPayment;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankProposedEffect {
    CreateAccount(BankAccount),
    AppendJournal(BankJournalEntry),
    ReverseJournal {
        original: crate::model::JournalEntryId,
        reversal: BankJournalEntry,
    },
    CreatePayment(BusinessPayment),
    UpdatePayment {
        payment: PaymentId,
        replacement: BusinessPayment,
    },
    GrantAuthorization(BankAccountAuthorization),
    RevokeAuthorization(BankAccountAuthorization),
}
