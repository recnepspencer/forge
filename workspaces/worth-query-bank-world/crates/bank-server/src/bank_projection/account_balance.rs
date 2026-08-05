use bank_domain::model::{AccountId, AccountJournalRevision, SignedMoney, USD};
use worth_query_host::facade::primary_graph::WorthQueryInvariantAggregate;

use super::BankProjectionDenial;

pub(super) fn validated_account_balance(
    account: AccountId,
    revision: AccountJournalRevision,
    aggregate: WorthQueryInvariantAggregate<SignedMoney<USD>>,
) -> Result<SignedMoney<USD>, BankProjectionDenial> {
    if aggregate.source_count() != revision.get() {
        return Err(BankProjectionDenial::AccountingRevisionMismatch(account));
    }
    Ok(aggregate.into_value())
}
