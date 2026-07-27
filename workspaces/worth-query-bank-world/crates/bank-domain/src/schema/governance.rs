use worth_query_decl::facade::{worth_query_currency, worth_query_effect, worth_query_policy};

use crate::model::{AccountId, USD};

use super::BankSchema;

worth_query_policy!(pub AccountVisibilityPolicy in BankSchema);
worth_query_policy!(pub AccountMutationScopePolicy in BankSchema);
worth_query_policy!(pub EmployeeScopePolicy in BankSchema);
worth_query_policy!(pub DistinctApproverPolicy in BankSchema);
worth_query_currency!(pub UsdCurrency(USD) in BankSchema);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivityEvent {
    pub account: AccountId,
    pub journal_sequence: u64,
}

worth_query_effect!(pub AccountActivityEffect(ActivityEvent) in BankSchema);
