use bank_domain::reads::AccountActivityItem;
use worth_query_host::facade::primary_graph::{
    WorthQueryLiveDeliveryOpenDenialKind, WorthQueryLiveDeliveryOverflow,
};

use crate::{BankReadDenial, BankReadResult};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankLiveOpenDenial {
    Admission(BankReadDenial),
    Delivery(WorthQueryLiveDeliveryOpenDenialKind),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankActivityLiveUpdate {
    commit_id: u64,
    activity: BankReadResult<AccountActivityItem>,
}

impl BankActivityLiveUpdate {
    pub(super) const fn new(commit_id: u64, activity: BankReadResult<AccountActivityItem>) -> Self {
        Self {
            commit_id,
            activity,
        }
    }

    pub const fn commit_id(&self) -> u64 {
        self.commit_id
    }

    pub const fn activity(&self) -> &BankReadResult<AccountActivityItem> {
        &self.activity
    }

    pub fn into_activity(self) -> BankReadResult<AccountActivityItem> {
        self.activity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankActivityLiveOutcome {
    Delivered(BankActivityLiveUpdate),
    Pending,
    Overflow(WorthQueryLiveDeliveryOverflow),
    AuthorizationRevoked(BankReadDenial),
    Cancelled,
    DeadlineExceeded,
    Closed,
    Unavailable,
}
