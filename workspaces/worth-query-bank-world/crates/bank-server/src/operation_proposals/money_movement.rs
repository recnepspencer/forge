use bank_domain::proposals::{
    BankIdempotencyKey, BankProposalDenial, BankProposalEngine, BankSnapshot,
};
use bank_domain::schema::{
    Account, ApplyOpeningFunding, ApplyOpeningFundingOperation, Deposit, DepositOperation,
    Institution, SendMoney, SendMoneyOperation, Withdraw, WithdrawOperation,
};

use crate::{BankAdmittedOperation, BankAuthorizedProposal, BankOperationProposals};

impl BankOperationProposals {
    pub fn prepare_opening_funding(
        snapshot: &BankSnapshot,
        admission: BankAdmittedOperation<
            ApplyOpeningFundingOperation,
            ApplyOpeningFunding,
            Institution,
            bank_domain::model::InstitutionId,
        >,
        key: &BankIdempotencyKey,
        input: &ApplyOpeningFunding,
    ) -> Result<
        BankAuthorizedProposal<
            ApplyOpeningFundingOperation,
            ApplyOpeningFunding,
            Institution,
            bank_domain::model::InstitutionId,
        >,
        BankProposalDenial,
    > {
        if admission.scope() != input.institution {
            return Err(BankProposalDenial::ScopeInputMismatch);
        }
        let invariant = BankProposalEngine::prepare_opening_funding(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new(admission, invariant))
    }

    pub fn prepare_deposit(
        snapshot: &BankSnapshot,
        admission: BankAdmittedOperation<
            DepositOperation,
            Deposit,
            Institution,
            bank_domain::model::InstitutionId,
        >,
        key: &BankIdempotencyKey,
        input: &Deposit,
    ) -> Result<
        BankAuthorizedProposal<
            DepositOperation,
            Deposit,
            Institution,
            bank_domain::model::InstitutionId,
        >,
        BankProposalDenial,
    > {
        if admission.scope() != input.institution {
            return Err(BankProposalDenial::ScopeInputMismatch);
        }
        let invariant = BankProposalEngine::prepare_deposit(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new(admission, invariant))
    }

    pub fn prepare_withdrawal(
        snapshot: &BankSnapshot,
        admission: BankAdmittedOperation<
            WithdrawOperation,
            Withdraw,
            Institution,
            bank_domain::model::InstitutionId,
        >,
        key: &BankIdempotencyKey,
        input: &Withdraw,
    ) -> Result<
        BankAuthorizedProposal<
            WithdrawOperation,
            Withdraw,
            Institution,
            bank_domain::model::InstitutionId,
        >,
        BankProposalDenial,
    > {
        if admission.scope() != input.institution {
            return Err(BankProposalDenial::ScopeInputMismatch);
        }
        let invariant = BankProposalEngine::prepare_withdrawal(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new(admission, invariant))
    }

    pub fn prepare_send_money(
        snapshot: &BankSnapshot,
        admission: BankAdmittedOperation<
            SendMoneyOperation,
            SendMoney,
            Account,
            bank_domain::model::AccountId,
        >,
        key: &BankIdempotencyKey,
        input: &SendMoney,
    ) -> Result<
        BankAuthorizedProposal<
            SendMoneyOperation,
            SendMoney,
            Account,
            bank_domain::model::AccountId,
        >,
        BankProposalDenial,
    > {
        if admission.scope() != input.from {
            return Err(BankProposalDenial::ScopeInputMismatch);
        }
        let invariant = BankProposalEngine::prepare_send_money(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new(admission, invariant))
    }
}
