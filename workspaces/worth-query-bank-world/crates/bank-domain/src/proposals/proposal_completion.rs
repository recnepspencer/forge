use crate::accounting::validate_proposed_snapshot;

use super::{
    BankIdempotencyIntent, BankInvariantApprovedProposal, BankProposalDenial, BankProposedEffect,
    BankSnapshot,
};

pub(crate) fn complete_proposal(
    basis: &BankSnapshot,
    proposed: BankSnapshot,
    intent: BankIdempotencyIntent,
    effects: Vec<BankProposedEffect>,
) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
    validate_effect_replay(basis, &proposed, &effects)?;
    let witness = validate_proposed_snapshot(basis, &proposed)?;
    Ok(BankInvariantApprovedProposal::new(
        basis.retain_basis(),
        intent,
        effects,
        proposed,
        witness,
    ))
}

fn validate_effect_replay(
    basis: &BankSnapshot,
    proposed: &BankSnapshot,
    effects: &[BankProposedEffect],
) -> Result<(), BankProposalDenial> {
    let mut replayed = basis.clone();
    for effect in effects {
        match effect {
            BankProposedEffect::CreateAccount(account) => {
                if replayed.allocate_account_id()? != account.id() {
                    return Err(BankProposalDenial::SnapshotInvariantViolated);
                }
                replayed.insert_account(account.clone());
            }
            BankProposedEffect::AppendJournal(entry) => {
                replay_journal_allocation(&mut replayed, entry)?;
                replayed.append_journal(entry.clone());
            }
            BankProposedEffect::ReverseJournal { original, reversal } => {
                replay_journal_allocation(&mut replayed, reversal)?;
                replayed.append_journal(reversal.clone());
                replayed.mark_reversed(*original);
            }
            BankProposedEffect::EmitAccountActivity(_) => {}
            BankProposedEffect::CreatePayment(payment) => {
                if replayed.allocate_payment_id()? != payment.id() {
                    return Err(BankProposalDenial::SnapshotInvariantViolated);
                }
                replayed.insert_payment(payment.clone());
            }
            BankProposedEffect::UpdatePayment {
                payment,
                replacement,
            } => {
                if replacement.id() != *payment || replayed.payment(*payment).is_none() {
                    return Err(BankProposalDenial::SnapshotInvariantViolated);
                }
                replayed.replace_payment(replacement.clone());
            }
            BankProposedEffect::GrantAuthorization(authorization) => {
                if replayed.allocate_authorization_id()? != authorization.id() {
                    return Err(BankProposalDenial::SnapshotInvariantViolated);
                }
                replayed.insert_authorization(*authorization);
            }
            BankProposedEffect::RevokeAuthorization(authorization) => {
                if replayed.remove_authorization(*authorization).is_none() {
                    return Err(BankProposalDenial::SnapshotInvariantViolated);
                }
            }
        }
    }
    if replayed == *proposed {
        Ok(())
    } else {
        Err(BankProposalDenial::SnapshotInvariantViolated)
    }
}

fn replay_journal_allocation(
    snapshot: &mut BankSnapshot,
    entry: &crate::accounting::BankJournalEntry,
) -> Result<(), BankProposalDenial> {
    if snapshot.allocate_journal_id()? != entry.id() {
        return Err(BankProposalDenial::SnapshotInvariantViolated);
    }
    for posting in entry.postings() {
        if snapshot.allocate_posting_id()? != posting.id() {
            return Err(BankProposalDenial::SnapshotInvariantViolated);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accounting::BankAccount;
    use crate::model::{AccountName, BankPrincipalId, BankSnapshotVersion, InstitutionId};
    use crate::proposals::BankSnapshotBuilder;

    #[test]
    fn omitted_effect_cannot_certify_a_changed_snapshot() {
        let basis = BankSnapshotBuilder::new(BankSnapshotVersion::new(1).unwrap())
            .institution(InstitutionId::new(1).unwrap())
            .principal(BankPrincipalId::new(1).unwrap())
            .build()
            .unwrap();
        let mut proposed = basis.clone();
        let account = BankAccount::personal(
            proposed.allocate_account_id().unwrap(),
            InstitutionId::new(1).unwrap(),
            BankPrincipalId::new(1).unwrap(),
            AccountName::new("Unreported account").unwrap(),
        );
        proposed.insert_account(account);

        assert!(matches!(
            complete_proposal(
                &basis,
                proposed,
                BankIdempotencyIntent::derive(
                    super::super::BankOperationScopeBinding::from_fingerprint_bytes([1; 32]),
                    &super::super::BankIdempotencyKey::new("missing-effect").unwrap(),
                    "test",
                    &[],
                ),
                Vec::new(),
            ),
            Err(BankProposalDenial::SnapshotInvariantViolated)
        ));
    }
}
