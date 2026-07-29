use bank_domain::proposals::{BankIdempotencyKey, BankProposalDenial, BankProposalEngine};
use bank_domain::schema::{
    Account, ApplyOpeningFunding, ApplyOpeningFundingOperation, Deposit, DepositOperation,
    Institution, SendMoney, SendMoneyOperation, Withdraw, WithdrawOperation,
};

use crate::bank_projection::{project_institution_money_movement, project_send_money_decision};
use crate::{
    BankAdmittedOperation, BankAuthorizedProposal, BankIdentityRuntime, BankOperationProposalError,
    BankOperationProposals, BankSendMoneyPreparation,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationIdempotencyBinding, WorthQueryApplicationIdempotencyResolution,
};

impl BankOperationProposals {
    pub fn prepare_opening_funding(
        runtime: &BankIdentityRuntime,
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
        BankOperationProposalError,
    > {
        if admission.scope() != input.institution {
            return Err(BankProposalDenial::ScopeInputMismatch.into());
        }
        let completed = runtime.invariant_projection().project_admitted_operation(
            admission.query(),
            |reader, institution| {
                project_institution_money_movement(
                    reader,
                    institution,
                    input.institution,
                    input.account,
                    [],
                )
            },
        )?;
        let (snapshot, projection, work) = completed.into_parts();
        let snapshot = snapshot?;
        let invariant = BankProposalEngine::prepare_opening_funding_from_decision(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new_bounded(
            admission, invariant, projection, work,
        ))
    }

    pub fn prepare_deposit(
        runtime: &BankIdentityRuntime,
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
        BankOperationProposalError,
    > {
        if admission.scope() != input.institution {
            return Err(BankProposalDenial::ScopeInputMismatch.into());
        }
        let completed = runtime.invariant_projection().project_admitted_operation(
            admission.query(),
            |reader, institution| {
                project_institution_money_movement(
                    reader,
                    institution,
                    input.institution,
                    input.account,
                    [],
                )
            },
        )?;
        let (snapshot, projection, work) = completed.into_parts();
        let snapshot = snapshot?;
        let invariant = BankProposalEngine::prepare_deposit_from_decision(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new_bounded(
            admission, invariant, projection, work,
        ))
    }

    pub fn prepare_withdrawal(
        runtime: &BankIdentityRuntime,
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
        BankOperationProposalError,
    > {
        if admission.scope() != input.institution {
            return Err(BankProposalDenial::ScopeInputMismatch.into());
        }
        let completed = runtime.invariant_projection().project_admitted_operation(
            admission.query(),
            |reader, institution| {
                project_institution_money_movement(
                    reader,
                    institution,
                    input.institution,
                    input.account,
                    [input.account],
                )
            },
        )?;
        let (snapshot, projection, work) = completed.into_parts();
        let snapshot = snapshot?;
        let invariant = BankProposalEngine::prepare_withdrawal_from_decision(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new_bounded(
            admission, invariant, projection, work,
        ))
    }

    pub fn prepare_send_money(
        runtime: &BankIdentityRuntime,
        admission: BankAdmittedOperation<
            SendMoneyOperation,
            SendMoney,
            Account,
            bank_domain::model::AccountId,
        >,
        key: &BankIdempotencyKey,
        input: &SendMoney,
    ) -> Result<BankSendMoneyPreparation, BankOperationProposalError> {
        if admission.scope() != input.from {
            return Err(BankProposalDenial::ScopeInputMismatch.into());
        }
        let completed = runtime
            .invariant_projection()
            .project_admitted_operation(admission.query(), |reader, source| {
                project_send_money_decision(reader, source, input)
            })?;
        let (decision, projection, work) = completed.into_parts();
        let decision = decision?;
        let idempotency = BankProposalEngine::send_money_idempotency(
            decision.snapshot(),
            admission.idempotency_binding(),
            key,
            input,
        )?;
        let binding = WorthQueryApplicationIdempotencyBinding::new(
            idempotency.key().bytes(),
            idempotency.intent().bytes(),
        );
        match runtime
            .application_runtime()
            .resolve_admitted_application_idempotency(admission.query(), binding)
            .map_err(|denial| BankOperationProposalError::Idempotency(denial.kind()))?
        {
            WorthQueryApplicationIdempotencyResolution::AlreadyCommitted(receipt) => {
                return Ok(BankSendMoneyPreparation::AlreadyCommitted {
                    receipt: crate::operation_commit::commit_receipt(receipt),
                    projection_work: work,
                });
            }
            WorthQueryApplicationIdempotencyResolution::IntentDrift => {
                return Ok(BankSendMoneyPreparation::IntentDrift {
                    projection_work: work,
                });
            }
            WorthQueryApplicationIdempotencyResolution::Unseen => {}
        }
        let invariant = BankProposalEngine::prepare_send_money_from_decision(
            decision,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankSendMoneyPreparation::Proposal(
            BankAuthorizedProposal::new_bounded(admission, invariant, projection, work),
        ))
    }
}
