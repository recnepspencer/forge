use crate::model::{AccountId, InstitutionId, Money, USD};
use crate::schema::{ApplyOpeningFunding, Deposit, PostingPurpose, SendMoney, Withdraw};

use super::BankProposalEngine;
use crate::proposals::{
    append_balanced_transfer, complete_decision_proposal, complete_proposal, BankDecisionSnapshot,
    BankIdempotencyClaim, BankIdempotencyKey, BankInvariantApprovedProposal,
    BankOperationScopeBinding, BankProposalDenial, BankProposedEffect, BankSnapshot,
    CanonicalProposalPayload,
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

    pub fn prepare_deposit_from_decision(
        decision: BankDecisionSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &Deposit,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        prepare_institution_movement_from_decision(
            decision,
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

    pub fn prepare_withdrawal_from_decision(
        decision: BankDecisionSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &Withdraw,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        let cash = institution_cash(decision.snapshot(), input.institution, input.account)?;
        prepare_transfer_from_decision(
            decision,
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
        let destination = send_money_destination(snapshot, input)?;
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

    pub fn send_money_idempotency(
        snapshot: &BankSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &SendMoney,
    ) -> Result<BankIdempotencyClaim, BankProposalDenial> {
        let destination = send_money_destination(snapshot, input)?;
        Ok(transfer_intent(
            binding,
            key,
            "send-money",
            input.from,
            destination,
            input.amount,
        ))
    }

    pub fn prepare_send_money_from_decision(
        decision: BankDecisionSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &SendMoney,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        let destination = send_money_destination(decision.snapshot(), input)?;
        prepare_transfer_from_decision(
            decision,
            binding,
            key,
            "send-money",
            input.from,
            destination,
            input.amount,
            PostingPurpose::Transfer,
        )
    }

    pub fn prepare_opening_funding_from_decision(
        decision: BankDecisionSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &ApplyOpeningFunding,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        if decision
            .starting_balance(input.account)
            .ok_or(BankProposalDenial::SnapshotInvariantViolated)?
            .minor_units()
            != 0
        {
            return Err(BankProposalDenial::AccountAlreadyFunded(input.account));
        }
        prepare_institution_movement_from_decision(
            decision,
            binding,
            key,
            input.institution,
            input.account,
            input.amount,
            InstitutionMovement::OpeningFunding,
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

fn prepare_institution_movement_from_decision(
    decision: BankDecisionSnapshot,
    binding: BankOperationScopeBinding,
    key: &BankIdempotencyKey,
    institution: InstitutionId,
    account: AccountId,
    amount: Money<USD>,
    movement: InstitutionMovement,
) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
    let cash = institution_cash(decision.snapshot(), institution, account)?;
    let (operation, purpose) = match movement {
        InstitutionMovement::OpeningFunding => {
            ("apply-opening-funding", PostingPurpose::OpeningFunding)
        }
        InstitutionMovement::Deposit => ("deposit", PostingPurpose::Deposit),
    };
    prepare_transfer_from_decision(
        decision, binding, key, operation, cash, account, amount, purpose,
    )
}

fn prepare_transfer_from_decision(
    decision: BankDecisionSnapshot,
    binding: BankOperationScopeBinding,
    key: &BankIdempotencyKey,
    operation: &'static str,
    debit: AccountId,
    credit: AccountId,
    amount: Money<USD>,
    purpose: PostingPurpose,
) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
    let intent = transfer_intent(binding, key, operation, debit, credit, amount);
    let (basis, required_balance_accounts, starting_balances) = decision.into_parts();
    let mut proposed = basis.clone();
    let journal = append_balanced_transfer(
        &mut proposed,
        debit,
        credit,
        amount,
        purpose,
        None,
        intent.key(),
    )?;
    complete_decision_proposal(
        basis,
        required_balance_accounts,
        starting_balances,
        proposed,
        intent,
        vec![BankProposedEffect::AppendJournal(journal)],
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
    let intent = transfer_intent(binding, key, operation, debit, credit, amount);
    let mut proposed = snapshot.clone();
    let journal = append_balanced_transfer(
        &mut proposed,
        debit,
        credit,
        amount,
        purpose,
        None,
        intent.key(),
    )?;
    let effects = vec![BankProposedEffect::AppendJournal(journal)];
    complete_proposal(snapshot, proposed, intent, effects)
}

fn send_money_destination(
    snapshot: &BankSnapshot,
    input: &SendMoney,
) -> Result<AccountId, BankProposalDenial> {
    snapshot
        .primary_account(input.recipient)
        .ok_or(BankProposalDenial::UnknownRecipient)
}

fn transfer_intent(
    binding: BankOperationScopeBinding,
    key: &BankIdempotencyKey,
    operation: &'static str,
    debit: AccountId,
    credit: AccountId,
    amount: Money<USD>,
) -> BankIdempotencyClaim {
    let payload = CanonicalProposalPayload::new(operation)
        .text("debit-account", &debit.canonical_text())
        .text("credit-account", &credit.canonical_text())
        .i64("amount-minor-units", amount.minor_units());
    BankIdempotencyClaim::derive(binding, key, payload)
}
