use bank_domain::reads::AccountSummary;
use bank_domain::schema::{AccountKind, AccountStatus};
use serde::{Deserialize, Serialize};

use super::{
    BankHttpCredential, BankHttpDenial, BankHttpProtocolVersion, BankHttpQueryPublication,
    BankHttpRequestControls,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BankHttpAccountSummaryRequest {
    pub protocol: BankHttpProtocolVersion,
    pub request_id: String,
    pub credential: BankHttpCredential,
    pub controls: BankHttpRequestControls,
    pub account: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BankHttpAccountKind {
    Personal,
    Business,
    InstitutionCash,
    InstitutionSettlement,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BankHttpAccountStatus {
    Open,
    Frozen,
    Closed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BankHttpAccountSummary {
    pub account: String,
    pub display_name: String,
    pub kind: BankHttpAccountKind,
    pub status: BankHttpAccountStatus,
    pub accounting_revision: u64,
    pub currency: String,
    pub current_balance_minor: i64,
    pub available_balance_minor: i64,
}

impl From<&AccountSummary> for BankHttpAccountSummary {
    fn from(summary: &AccountSummary) -> Self {
        Self {
            account: summary.id().canonical_text(),
            display_name: summary.display_name().as_str().to_owned(),
            kind: map_kind(summary.kind()),
            status: map_status(summary.status()),
            accounting_revision: summary.accounting_revision().get(),
            currency: "USD".to_owned(),
            current_balance_minor: summary.current_balance().minor_units(),
            available_balance_minor: summary.available_balance().minor_units(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum BankHttpAccountSummaryOutcome {
    Delivered {
        request_id: String,
        summary: BankHttpAccountSummary,
        publication: BankHttpQueryPublication,
    },
    Denied {
        request_id: Option<String>,
        denial: BankHttpDenial,
    },
}

const fn map_kind(kind: AccountKind) -> BankHttpAccountKind {
    match kind {
        AccountKind::Personal => BankHttpAccountKind::Personal,
        AccountKind::Business => BankHttpAccountKind::Business,
        AccountKind::InstitutionCash => BankHttpAccountKind::InstitutionCash,
        AccountKind::InstitutionSettlement => BankHttpAccountKind::InstitutionSettlement,
    }
}

const fn map_status(status: AccountStatus) -> BankHttpAccountStatus {
    match status {
        AccountStatus::Open => BankHttpAccountStatus::Open,
        AccountStatus::Frozen => BankHttpAccountStatus::Frozen,
        AccountStatus::Closed => BankHttpAccountStatus::Closed,
    }
}
