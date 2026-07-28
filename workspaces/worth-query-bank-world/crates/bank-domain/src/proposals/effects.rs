use crate::accounting::{BankAccount, BankJournalEntry};
use crate::model::{AccountAuthorizationId, PaymentId};
use crate::payments::BusinessPayment;
use crate::schema::ActivityEvent;

use super::BankAccountAuthorization;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankProposedEffect {
    CreateAccount(BankAccount),
    AppendJournal(BankJournalEntry),
    ReverseJournal {
        original: crate::model::JournalEntryId,
        reversal: BankJournalEntry,
    },
    EmitAccountActivity(ActivityEvent),
    CreatePayment(BusinessPayment),
    UpdatePayment {
        payment: PaymentId,
        replacement: BusinessPayment,
    },
    GrantAuthorization(BankAccountAuthorization),
    RevokeAuthorization(AccountAuthorizationId),
}
