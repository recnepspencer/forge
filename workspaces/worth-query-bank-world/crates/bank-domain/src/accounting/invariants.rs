use std::collections::BTreeSet;

use crate::proposals::{BankProposalDenial, BankSnapshot};
use crate::schema::AccountKind;

use super::account_balance;

pub(crate) struct BankInvariantWitness {
    _private: (),
}

pub(crate) fn validate_proposed_snapshot(
    basis: &BankSnapshot,
    proposed: &BankSnapshot,
) -> Result<BankInvariantWitness, BankProposalDenial> {
    if !basis.has_valid_topology()
        || !proposed.has_valid_topology()
        || !proposed.journal().starts_with(basis.journal())
    {
        return Err(BankProposalDenial::SnapshotInvariantViolated);
    }

    let mut journal_ids = BTreeSet::new();
    let mut posting_ids = BTreeSet::new();
    for entry in proposed.journal() {
        if entry.postings().len() < 2 {
            return Err(BankProposalDenial::JournalHasTooFewPostings);
        }
        if !journal_ids.insert(entry.id()) {
            return Err(BankProposalDenial::SnapshotInvariantViolated);
        }
        let sum = entry.postings().iter().try_fold(0_i64, |sum, posting| {
            if proposed.account(posting.account()).is_none() || !posting_ids.insert(posting.id()) {
                return None;
            }
            sum.checked_add(posting.amount().minor_units())
        });
        if sum != Some(0) {
            return Err(match sum {
                None => BankProposalDenial::ArithmeticOverflow,
                Some(_) => BankProposalDenial::JournalIsUnbalanced,
            });
        }
    }

    for account in proposed.accounts() {
        if matches!(
            account.kind(),
            AccountKind::Personal | AccountKind::Business
        ) && account_balance(proposed.journal(), account.id())
            .map_err(|_| BankProposalDenial::ArithmeticOverflow)?
            .minor_units()
            < 0
        {
            return Err(BankProposalDenial::InsufficientFunds(account.id()));
        }
    }

    Ok(BankInvariantWitness { _private: () })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accounting::{BankJournalEntry, BankPosting};
    use crate::model::{
        AccountId, AccountName, BankPrincipalId, BankSnapshotVersion, InstitutionId, SignedMoney,
    };
    use crate::proposals::BankSnapshotBuilder;
    use crate::schema::{AccountStatus, PostingPurpose};

    #[test]
    fn malformed_journal_shapes_fail_the_invariant_oracle() {
        let basis = BankSnapshotBuilder::new(BankSnapshotVersion::new(1).unwrap())
            .institution(InstitutionId::new(1).unwrap())
            .principal(BankPrincipalId::new(1).unwrap())
            .personal_account(
                AccountId::new(1).unwrap(),
                InstitutionId::new(1).unwrap(),
                BankPrincipalId::new(1).unwrap(),
                AccountName::new("Invariant target").unwrap(),
                AccountStatus::Open,
            )
            .build()
            .unwrap();

        let mut partial = basis.clone();
        let partial_journal = partial.allocate_journal_id().unwrap();
        let partial_posting = partial.allocate_posting_id().unwrap();
        partial.append_journal(BankJournalEntry::new(
            partial_journal,
            PostingPurpose::Deposit,
            vec![BankPosting::new(
                partial_posting,
                AccountId::new(1).unwrap(),
                SignedMoney::from_minor(10),
            )],
            None,
        ));
        assert!(matches!(
            validate_proposed_snapshot(&basis, &partial),
            Err(BankProposalDenial::JournalHasTooFewPostings)
        ));

        let mut unbalanced = basis.clone();
        let unbalanced_journal = unbalanced.allocate_journal_id().unwrap();
        let first_posting = unbalanced.allocate_posting_id().unwrap();
        let second_posting = unbalanced.allocate_posting_id().unwrap();
        unbalanced.append_journal(BankJournalEntry::new(
            unbalanced_journal,
            PostingPurpose::Deposit,
            vec![
                BankPosting::new(
                    first_posting,
                    AccountId::new(1).unwrap(),
                    SignedMoney::from_minor(10),
                ),
                BankPosting::new(
                    second_posting,
                    AccountId::new(1).unwrap(),
                    SignedMoney::from_minor(-9),
                ),
            ],
            None,
        ));
        assert!(matches!(
            validate_proposed_snapshot(&basis, &unbalanced),
            Err(BankProposalDenial::JournalIsUnbalanced)
        ));
    }
}
