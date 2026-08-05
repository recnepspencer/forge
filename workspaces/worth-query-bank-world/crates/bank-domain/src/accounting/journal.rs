use crate::model::{AccountId, JournalEntryId, PostingId, SignedMoney, USD};
use crate::schema::PostingPurpose;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankPosting {
    id: PostingId,
    account: AccountId,
    amount: SignedMoney<USD>,
}

impl BankPosting {
    pub const fn from_projection(
        id: PostingId,
        account: AccountId,
        amount: SignedMoney<USD>,
    ) -> Self {
        Self {
            id,
            account,
            amount,
        }
    }

    pub(crate) const fn new(id: PostingId, account: AccountId, amount: SignedMoney<USD>) -> Self {
        Self {
            id,
            account,
            amount,
        }
    }

    pub const fn id(&self) -> PostingId {
        self.id
    }

    pub const fn account(&self) -> AccountId {
        self.account
    }

    pub const fn amount(&self) -> SignedMoney<USD> {
        self.amount
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankJournalEntry {
    id: JournalEntryId,
    purpose: PostingPurpose,
    postings: Vec<BankPosting>,
    reversal_of: Option<JournalEntryId>,
}

impl BankJournalEntry {
    pub fn from_projection(
        id: JournalEntryId,
        purpose: PostingPurpose,
        postings: Vec<BankPosting>,
        reversal_of: Option<JournalEntryId>,
    ) -> Self {
        Self {
            id,
            purpose,
            postings,
            reversal_of,
        }
    }

    pub(crate) fn new(
        id: JournalEntryId,
        purpose: PostingPurpose,
        postings: Vec<BankPosting>,
        reversal_of: Option<JournalEntryId>,
    ) -> Self {
        Self {
            id,
            purpose,
            postings,
            reversal_of,
        }
    }

    pub const fn id(&self) -> JournalEntryId {
        self.id
    }

    pub const fn purpose(&self) -> PostingPurpose {
        self.purpose
    }

    pub fn postings(&self) -> &[BankPosting] {
        &self.postings
    }

    pub const fn reversal_of(&self) -> Option<JournalEntryId> {
        self.reversal_of
    }
}
