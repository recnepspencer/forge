use bank_domain::queries::AccountActivityQueryResult;
use bank_domain::reads::AccountActivityItem;
use bank_domain::schema::PostingPurpose;
use serde::{Deserialize, Serialize};

use super::{
    BankHttpCredential, BankHttpDenial, BankHttpProtocolVersion, BankHttpQueryPublication,
    BankHttpRequestControls,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BankHttpAccountActivityStreamRequest {
    pub protocol: BankHttpProtocolVersion,
    pub request_id: String,
    pub credential: BankHttpCredential,
    pub controls: BankHttpRequestControls,
    pub account: String,
    pub source_buffer_capacity: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BankHttpAccountActivityPageRequest {
    pub protocol: BankHttpProtocolVersion,
    pub request_id: String,
    pub credential: BankHttpCredential,
    pub controls: BankHttpRequestControls,
    pub account: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BankHttpAccountActivityResumeRequest {
    pub protocol: BankHttpProtocolVersion,
    pub request_id: String,
    pub credential: BankHttpCredential,
    pub controls: BankHttpRequestControls,
    pub account: String,
    pub continuation: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BankHttpAccountActivity {
    pub account: String,
    pub entries: Vec<BankHttpAccountActivityItem>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BankHttpAccountActivityItem {
    pub account: String,
    pub account_sequence: u64,
    pub journal: String,
    pub purpose: BankHttpPostingPurpose,
    pub amount_minor: i64,
    pub reversal_of: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BankHttpPostingPurpose {
    OpeningFunding,
    Deposit,
    Withdrawal,
    Transfer,
    EstateDisbursement,
    Reversal,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum BankHttpAccountActivityEvent {
    Opened {
        request_id: String,
    },
    Update {
        request_id: String,
        activity: BankHttpAccountActivity,
        publication: BankHttpQueryPublication,
    },
    Overflow {
        request_id: String,
        missed_commit_batches: u64,
    },
    Denied {
        request_id: String,
        denial: BankHttpDenial,
    },
    Cancelled {
        request_id: String,
    },
    DeadlineExceeded {
        request_id: String,
    },
    Closed {
        request_id: String,
    },
    Unavailable {
        request_id: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum BankHttpAccountActivityPageOutcome {
    Delivered {
        request_id: String,
        activity: BankHttpAccountActivity,
        continuation: Option<String>,
        publication: BankHttpQueryPublication,
    },
    Denied {
        request_id: Option<String>,
        denial: BankHttpDenial,
    },
}

impl From<&AccountActivityQueryResult> for BankHttpAccountActivity {
    fn from(result: &AccountActivityQueryResult) -> Self {
        Self {
            account: result.account().canonical_text(),
            entries: result
                .entries()
                .iter()
                .copied()
                .map(BankHttpAccountActivityItem::from)
                .collect(),
        }
    }
}

impl From<AccountActivityItem> for BankHttpAccountActivityItem {
    fn from(item: AccountActivityItem) -> Self {
        Self {
            account: item.account().canonical_text(),
            account_sequence: item.account_sequence().get(),
            journal: item.journal().canonical_text(),
            purpose: map_purpose(item.purpose()),
            amount_minor: item.amount().minor_units(),
            reversal_of: item.reversal_of().map(|journal| journal.canonical_text()),
        }
    }
}

const fn map_purpose(purpose: PostingPurpose) -> BankHttpPostingPurpose {
    match purpose {
        PostingPurpose::OpeningFunding => BankHttpPostingPurpose::OpeningFunding,
        PostingPurpose::Deposit => BankHttpPostingPurpose::Deposit,
        PostingPurpose::Withdrawal => BankHttpPostingPurpose::Withdrawal,
        PostingPurpose::Transfer => BankHttpPostingPurpose::Transfer,
        PostingPurpose::EstateDisbursement => BankHttpPostingPurpose::EstateDisbursement,
        PostingPurpose::Reversal => BankHttpPostingPurpose::Reversal,
    }
}
