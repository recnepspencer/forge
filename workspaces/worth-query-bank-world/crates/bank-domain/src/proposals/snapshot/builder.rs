use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::accounting::BankAccount;
use crate::model::{AccountId, BankPrincipalId, BankSnapshotVersion, BusinessId, InstitutionId};

use super::{BankSnapshot, BankSnapshotAuthority};
use crate::proposals::BankProposalDenial;

pub struct BankSnapshotBuilder {
    snapshot: BankSnapshot,
    valid: bool,
}

impl BankSnapshotBuilder {
    pub fn new(version: BankSnapshotVersion) -> Self {
        Self {
            snapshot: BankSnapshot {
                version,
                authority: Arc::new(BankSnapshotAuthority { _private: () }),
                institutions: BTreeSet::new(),
                principals: BTreeSet::new(),
                businesses: BTreeSet::new(),
                accounts: BTreeMap::new(),
                primary_personal_accounts: BTreeMap::new(),
                business_accounts: BTreeMap::new(),
                institution_cash_accounts: BTreeMap::new(),
                journal: Vec::new(),
                payments: BTreeMap::new(),
                authorizations: BTreeMap::new(),
                reversed_journals: BTreeSet::new(),
                next_account_id: 1,
                next_journal_id: 1,
                next_posting_id: 1,
                next_payment_id: 1,
                next_authorization_id: 1,
            },
            valid: true,
        }
    }

    pub fn institution(mut self, institution: InstitutionId) -> Self {
        self.snapshot.institutions.insert(institution);
        self
    }

    pub fn principal(mut self, principal: BankPrincipalId) -> Self {
        self.snapshot.principals.insert(principal);
        self
    }

    pub fn business(mut self, business: BusinessId) -> Self {
        self.snapshot.businesses.insert(business);
        self
    }

    pub fn institution_cash_account(
        mut self,
        account: AccountId,
        institution: InstitutionId,
    ) -> Self {
        self.insert_fixture_account(BankAccount::institution_cash(account, institution));
        self
    }

    pub fn personal_account(
        mut self,
        account: AccountId,
        institution: InstitutionId,
        owner: BankPrincipalId,
        display_name: crate::model::AccountName,
        status: crate::schema::AccountStatus,
    ) -> Self {
        self.insert_fixture_account(BankAccount::personal_with_status(
            account,
            institution,
            owner,
            display_name,
            status,
        ));
        self
    }

    pub fn business_account_fixture(
        mut self,
        account: AccountId,
        institution: InstitutionId,
        business: BusinessId,
        display_name: crate::model::AccountName,
        status: crate::schema::AccountStatus,
    ) -> Self {
        self.insert_fixture_account(BankAccount::business_with_status(
            account,
            institution,
            business,
            display_name,
            status,
        ));
        self
    }

    pub fn build(self) -> Result<BankSnapshot, BankProposalDenial> {
        if self.valid && self.snapshot.has_valid_topology() {
            Ok(self.snapshot)
        } else {
            Err(BankProposalDenial::SnapshotInvariantViolated)
        }
    }

    fn insert_fixture_account(&mut self, account: BankAccount) {
        let collides = self.snapshot.accounts.contains_key(&account.id())
            || account
                .personal_owner()
                .is_some_and(|owner| self.snapshot.primary_personal_accounts.contains_key(&owner))
            || account
                .business_owner()
                .is_some_and(|business| self.snapshot.business_accounts.contains_key(&business))
            || (account.kind() == crate::schema::AccountKind::InstitutionCash
                && self
                    .snapshot
                    .institution_cash_accounts
                    .contains_key(&account.institution()));
        let next = account.id().get().checked_add(1);
        if collides || next.is_none() {
            self.valid = false;
            return;
        }
        self.snapshot.insert_account(account);
        self.snapshot.next_account_id = self
            .snapshot
            .next_account_id
            .max(next.expect("checked above"));
    }
}
