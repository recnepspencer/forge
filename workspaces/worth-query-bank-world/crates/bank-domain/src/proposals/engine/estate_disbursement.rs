use crate::{
    estate::{EstateDisbursement, EstatePosting},
    model::SignedMoney,
    schema::PostingPurpose,
};

use super::{money_movement::prepare_claimed_transfer_from_decision, BankProposalEngine};
use crate::proposals::{
    BankDecisionSnapshot, BankIdempotencyClaim, BankInvariantApprovedProposal, BankProposalDenial,
};

impl BankProposalEngine {
    pub fn prepare_estate_disbursement_from_decision(
        decision: BankDecisionSnapshot,
        idempotency: BankIdempotencyClaim,
        input: &EstateDisbursement,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        validate_posting_shape(input)?;
        prepare_claimed_transfer_from_decision(
            decision,
            idempotency,
            input.source_account,
            input.destination_account,
            input.amount,
            PostingPurpose::EstateDisbursement,
        )
    }
}

fn validate_posting_shape(input: &EstateDisbursement) -> Result<(), BankProposalDenial> {
    let debit_minor_units = input
        .amount
        .minor_units()
        .checked_neg()
        .ok_or(BankProposalDenial::ArithmeticOverflow)?;
    let expected = [
        EstatePosting {
            account: input.source_account,
            amount: SignedMoney::from_minor(debit_minor_units),
        },
        EstatePosting {
            account: input.destination_account,
            amount: SignedMoney::from(input.amount),
        },
    ];
    if input.postings != expected {
        return Err(BankProposalDenial::DisbursementPostingMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use super::*;
    use crate::{
        model::{
            AccountId, AccountName, BankPrincipalId, BankSnapshotVersion, InstitutionId, Money,
        },
        proposals::{BankProposedEffect, BankSnapshotBuilder},
        schema::AccountStatus,
    };

    #[test]
    fn estate_disbursement_reuses_balanced_journal_with_distinct_purpose() {
        let input = input();
        let proposal = BankProposalEngine::prepare_estate_disbursement_from_decision(
            decision(100),
            BankIdempotencyClaim::from_application_binding([7; 32], [8; 32]),
            &input,
        )
        .unwrap();
        let [BankProposedEffect::AppendJournal(journal)] = proposal.effects() else {
            panic!("estate disbursement must append exactly one journal");
        };

        assert_eq!(journal.purpose(), PostingPurpose::EstateDisbursement);
        assert_eq!(journal.postings().len(), 2);
        assert_eq!(journal.postings()[0].account(), input.source_account);
        assert_eq!(journal.postings()[0].amount(), input.postings[0].amount);
        assert_eq!(journal.postings()[1].account(), input.destination_account);
        assert_eq!(journal.postings()[1].amount(), input.postings[1].amount);
    }

    #[test]
    fn malformed_disbursement_postings_and_insufficient_funds_deny() {
        let mut malformed = input();
        malformed.postings.swap(0, 1);
        assert_eq!(
            BankProposalEngine::prepare_estate_disbursement_from_decision(
                decision(100),
                BankIdempotencyClaim::from_application_binding([9; 32], [10; 32]),
                &malformed,
            )
            .err(),
            Some(BankProposalDenial::DisbursementPostingMismatch)
        );
        assert_eq!(
            BankProposalEngine::prepare_estate_disbursement_from_decision(
                decision(4),
                BankIdempotencyClaim::from_application_binding([11; 32], [12; 32]),
                &input(),
            )
            .err(),
            Some(BankProposalDenial::InsufficientFunds(account(1)))
        );
    }

    fn input() -> EstateDisbursement {
        EstateDisbursement {
            estate: crate::estate::EstateCaseId::new(1).unwrap(),
            source_account: account(1),
            destination_account: account(2),
            beneficiary: BankPrincipalId::new(2).unwrap(),
            amount: Money::from_minor(5).unwrap(),
            postings: [
                EstatePosting {
                    account: account(1),
                    amount: SignedMoney::from_minor(-5),
                },
                EstatePosting {
                    account: account(2),
                    amount: SignedMoney::from_minor(5),
                },
            ],
        }
    }

    fn decision(source_balance: i64) -> BankDecisionSnapshot {
        let institution = InstitutionId::new(1).unwrap();
        let source_owner = BankPrincipalId::new(1).unwrap();
        let destination_owner = BankPrincipalId::new(2).unwrap();
        let snapshot = BankSnapshotBuilder::new(BankSnapshotVersion::new(1).unwrap())
            .institution(institution)
            .principal(source_owner)
            .principal(destination_owner)
            .personal_account(
                account(1),
                institution,
                source_owner,
                AccountName::new("Estate source").unwrap(),
                AccountStatus::Open,
            )
            .personal_account(
                account(2),
                institution,
                destination_owner,
                AccountName::new("Beneficiary destination").unwrap(),
                AccountStatus::Open,
            )
            .build()
            .unwrap();
        BankDecisionSnapshot::new(
            snapshot,
            BTreeSet::from([account(1)]),
            BTreeMap::from([(account(1), SignedMoney::from_minor(source_balance))]),
        )
    }

    fn account(value: u64) -> AccountId {
        AccountId::new(value).unwrap()
    }
}
