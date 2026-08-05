use crate::model::{
    AccountId, AccountJournalRevision, InstitutionId, JournalEntryId, SignedMoney, USD,
};
use crate::schema::PostingPurpose;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccountActivityItem {
    account: AccountId,
    account_sequence: AccountJournalRevision,
    journal: JournalEntryId,
    purpose: PostingPurpose,
    amount: SignedMoney<USD>,
    reversal_of: Option<JournalEntryId>,
}

impl AccountActivityItem {
    pub const fn from_projection(
        account: AccountId,
        account_sequence: AccountJournalRevision,
        journal: JournalEntryId,
        purpose: PostingPurpose,
        amount: SignedMoney<USD>,
        reversal_of: Option<JournalEntryId>,
    ) -> Self {
        Self {
            account,
            account_sequence,
            journal,
            purpose,
            amount,
            reversal_of,
        }
    }

    pub const fn account(self) -> AccountId {
        self.account
    }

    pub const fn journal(self) -> JournalEntryId {
        self.journal
    }

    pub const fn account_sequence(self) -> AccountJournalRevision {
        self.account_sequence
    }

    pub const fn purpose(self) -> PostingPurpose {
        self.purpose
    }

    pub const fn amount(self) -> SignedMoney<USD> {
        self.amount
    }

    pub const fn reversal_of(self) -> Option<JournalEntryId> {
        self.reversal_of
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstitutionAuditAccount {
    account: AccountId,
    entries: Vec<AccountActivityItem>,
}

impl InstitutionAuditAccount {
    pub(crate) fn from_projection(account: AccountId, entries: Vec<AccountActivityItem>) -> Self {
        Self { account, entries }
    }

    pub const fn account(&self) -> AccountId {
        self.account
    }

    pub fn entries(&self) -> &[AccountActivityItem] {
        &self.entries
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstitutionAuditView {
    institution: InstitutionId,
    accounts: Vec<InstitutionAuditAccount>,
}

impl InstitutionAuditView {
    pub(crate) fn from_projection(
        institution: InstitutionId,
        accounts: Vec<InstitutionAuditAccount>,
    ) -> Self {
        Self {
            institution,
            accounts,
        }
    }

    pub const fn institution(&self) -> InstitutionId {
        self.institution
    }

    pub fn accounts(&self) -> &[InstitutionAuditAccount] {
        &self.accounts
    }
}
