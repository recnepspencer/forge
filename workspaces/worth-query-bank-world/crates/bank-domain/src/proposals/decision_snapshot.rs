use std::collections::{BTreeMap, BTreeSet};

use crate::model::{AccountId, SignedMoney, USD};

use super::BankSnapshot;

/// A causally complete operation projection, intentionally narrower than a
/// complete bank world.
///
/// It grants no commit authority. The required balance accounts name the
/// exact monetary invariants that proposal approval must establish.
pub struct BankDecisionSnapshot {
    snapshot: BankSnapshot,
    required_balance_accounts: BTreeSet<AccountId>,
    starting_balances: BTreeMap<AccountId, SignedMoney<USD>>,
}

impl BankDecisionSnapshot {
    pub fn snapshot(&self) -> &BankSnapshot {
        &self.snapshot
    }

    /// Descriptive provider-projected starting balance for this decision basis.
    ///
    /// The value grants no mutation authority; Query's retained projection and
    /// exact accounting-revision decision fact govern any later commit.
    pub fn starting_balance(&self, account: AccountId) -> Option<SignedMoney<USD>> {
        self.starting_balances.get(&account).copied()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        BankSnapshot,
        BTreeSet<AccountId>,
        BTreeMap<AccountId, SignedMoney<USD>>,
    ) {
        (
            self.snapshot,
            self.required_balance_accounts,
            self.starting_balances,
        )
    }

    pub(crate) fn new(
        snapshot: BankSnapshot,
        required_balance_accounts: BTreeSet<AccountId>,
        starting_balances: BTreeMap<AccountId, SignedMoney<USD>>,
    ) -> Self {
        Self {
            snapshot,
            required_balance_accounts,
            starting_balances,
        }
    }
}
