use crate::model::{AccountId, InstitutionId, Money, USD};
use crate::schema::{ApplyOpeningFunding, Deposit, PostingPurpose, SendMoney, Withdraw};

use super::BankProposalEngine;
use crate::proposals::{
    account_activity_effects, append_balanced_transfer, complete_proposal, BankIdempotencyIntent,
    BankIdempotencyKey, BankInvariantApprovedProposal, BankOperationScopeBinding,
    BankProposalDenial, BankProposedEffect, BankSnapshot, CanonicalProposalPayload,
};

impl BankProposalEngine {
    pub fn prepare_opening_funding(
        snapshot: &BankSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &ApplyOpeningFunding,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        let balance = crate::accounting::account_balance(snapshot.journal(), input.account)
            .map_err(|_| BankProposalDenial::ArithmeticOverflow)?;
        if balance.minor_units() != 0 {
            return Err(BankProposalDenial::AccountAlreadyFunded(input.account));
        }
        prepare_institution_movement(
            snapshot,
            binding,
            key,
            input.institution,
            input.account,
            input.amount,
            InstitutionMovement::OpeningFunding,
        )
    }

    pub fn prepare_deposit(
        snapshot: &BankSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &Deposit,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        prepare_institution_movement(
            snapshot,
            binding,
            key,
            input.institution,
            input.account,
            input.amount,
            InstitutionMovement::Deposit,
        )
    }

    pub fn prepare_withdrawal(
        snapshot: &BankSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &Withdraw,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        let cash = institution_cash(snapshot, input.institution, input.account)?;
        prepare_transfer(
            snapshot,
            binding,
            key,
            "withdraw",
            input.account,
            cash,
            input.amount,
            PostingPurpose::Withdrawal,
        )
    }

    pub fn prepare_send_money(
        snapshot: &BankSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &SendMoney,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        let destination = snapshot
            .primary_account(input.recipient)
            .ok_or(BankProposalDenial::UnknownRecipient)?;
        prepare_transfer(
            snapshot,
            binding,
            key,
            "send-money",
            input.from,
            destination,
            input.amount,
            PostingPurpose::Transfer,
        )
    }
}

enum InstitutionMovement {
    OpeningFunding,
    Deposit,
}

fn prepare_institution_movement(
    snapshot: &BankSnapshot,
    binding: BankOperationScopeBinding,
    key: &BankIdempotencyKey,
    institution: InstitutionId,
    account: AccountId,
    amount: Money<USD>,
    movement: InstitutionMovement,
) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
    let cash = institution_cash(snapshot, institution, account)?;
    let (operation, purpose) = match movement {
        InstitutionMovement::OpeningFunding => {
            ("apply-opening-funding", PostingPurpose::OpeningFunding)
        }
        InstitutionMovement::Deposit => ("deposit", PostingPurpose::Deposit),
    };
    prepare_transfer(
        snapshot, binding, key, operation, cash, account, amount, purpose,
    )
}

fn institution_cash(
    snapshot: &BankSnapshot,
    institution: InstitutionId,
    account: AccountId,
) -> Result<AccountId, BankProposalDenial> {
    if !snapshot.is_known_institution(institution) {
        return Err(BankProposalDenial::UnknownInstitution);
    }
    let account_institution = snapshot
        .account(account)
        .ok_or(BankProposalDenial::UnknownAccount(account))?
        .institution();
    if account_institution != institution {
        return Err(BankProposalDenial::AccountInstitutionMismatch);
    }
    snapshot
        .institution_cash_account(institution)
        .ok_or(BankProposalDenial::MissingInstitutionCashAccount)
}

pub(super) fn prepare_transfer(
    snapshot: &BankSnapshot,
    binding: BankOperationScopeBinding,
    key: &BankIdempotencyKey,
    operation: &'static str,
    debit: AccountId,
    credit: AccountId,
    amount: Money<USD>,
    purpose: PostingPurpose,
) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
    let payload = CanonicalProposalPayload::new()
        .u64(debit.get())
        .u64(credit.get())
        .i64(amount.minor_units());
    let intent = BankIdempotencyIntent::derive(binding, key, operation, payload.as_bytes());
    let mut proposed = snapshot.clone();
    let journal = append_balanced_transfer(&mut proposed, debit, credit, amount, purpose, None)?;
    let mut effects = vec![BankProposedEffect::AppendJournal(journal.clone())];
    effects.extend(account_activity_effects(&journal));
    complete_proposal(snapshot, proposed, intent, effects)
}
