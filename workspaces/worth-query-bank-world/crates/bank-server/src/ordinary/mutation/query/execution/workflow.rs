use bank_domain::schema::{
    ApprovePayment, ApprovePaymentOperation, BankSchema, Business, InitiateBusinessPayment,
    InitiateBusinessPaymentOperation, PaymentIntent, RejectPayment, RejectPaymentOperation,
};
use worth_query_host::facade::declaration::application_schema::TypedMutationPreconditions;

use super::{denied, execute_standard, interrupted, map_admission_denial};
use crate::ordinary::mutation::{
    mutations, BankApprovePendingPayment, BankMutationDenial, BankMutationOutcome,
    BankPaymentInitiationOutcome, BankPendingPaymentContinuation, BankRejectPendingPayment,
};
use crate::{
    BankCommitPreparationDenial, BankIdentityRuntime, BankOperationProposals, BankReadyMutation,
};

impl
    BankReadyMutation<
        '_,
        '_,
        mutations::InitiateBusinessPaymentMutation,
        InitiateBusinessPaymentOperation,
        Business,
    >
{
    pub fn execute(self) -> BankPaymentInitiationOutcome {
        execute_initiation(
            self.runtime,
            self.principal,
            self.preconditions,
            &self.controls,
            &self.mutation.input,
        )
    }
}

impl BankReadyMutation<'_, '_, BankApprovePendingPayment, ApprovePaymentOperation, PaymentIntent> {
    pub fn execute(self) -> BankMutationOutcome {
        let input = ApprovePayment {
            payment: self.mutation.payment,
            approver: self.principal.principal_id(),
        };
        execute_standard(
            &self.controls,
            || {
                self.runtime.authorize_approve_payment(
                    self.principal,
                    input.payment,
                    self.preconditions,
                    self.controls.request(),
                )
            },
            |admission, key| {
                BankOperationProposals::prepare_approve_payment(
                    self.runtime,
                    admission,
                    key,
                    &input,
                )
            },
            |proposal| self.runtime.commit_approve_payment(proposal),
        )
    }
}

impl BankReadyMutation<'_, '_, BankRejectPendingPayment, RejectPaymentOperation, PaymentIntent> {
    pub fn execute(self) -> BankMutationOutcome {
        let input = RejectPayment {
            payment: self.mutation.payment,
            rejecting_principal: self.principal.principal_id(),
        };
        execute_standard(
            &self.controls,
            || {
                self.runtime.authorize_reject_payment(
                    self.principal,
                    input.payment,
                    self.preconditions,
                    self.controls.request(),
                )
            },
            |admission, key| {
                BankOperationProposals::prepare_reject_payment(self.runtime, admission, key, &input)
            },
            |proposal| self.runtime.commit_reject_payment(proposal),
        )
    }
}

fn execute_initiation(
    runtime: &BankIdentityRuntime,
    principal: &crate::BankAuthenticatedPrincipal,
    preconditions: TypedMutationPreconditions<
        BankSchema,
        InitiateBusinessPaymentOperation,
        Business,
    >,
    controls: &crate::BankMutationControls,
    input: &InitiateBusinessPayment,
) -> BankPaymentInitiationOutcome {
    if let Some(outcome) = interrupted(controls) {
        return BankPaymentInitiationOutcome::new(outcome, None);
    }
    let admission = match runtime.authorize_initiate_business_payment(
        principal,
        input.business,
        preconditions,
        controls.request(),
    ) {
        Ok(admission) => admission,
        Err(denial) => {
            return BankPaymentInitiationOutcome::new(
                denied(map_admission_denial(denial), None),
                None,
            );
        }
    };
    let proposal = match BankOperationProposals::prepare_initiate_business_payment(
        runtime,
        admission,
        controls.idempotency_key(),
        input,
    ) {
        Ok(proposal) => proposal,
        Err(denial) => {
            return BankPaymentInitiationOutcome::new(
                denied(BankMutationDenial::Proposal(denial), None),
                None,
            );
        }
    };
    let work = proposal.projection_work();
    let Some(payment) = proposal.initiated_payment_id() else {
        return BankPaymentInitiationOutcome::new(
            denied(
                BankMutationDenial::Preparation(BankCommitPreparationDenial::InvalidProposalShape),
                Some(work),
            ),
            None,
        );
    };
    let outcome = match runtime.commit_initiate_business_payment(proposal) {
        Ok(outcome) => super::committed(outcome, Some(work)),
        Err(denial) => denied(BankMutationDenial::Preparation(denial), Some(work)),
    };
    BankPaymentInitiationOutcome::new(
        outcome,
        Some(BankPendingPaymentContinuation::from_payment_id(payment)),
    )
}
