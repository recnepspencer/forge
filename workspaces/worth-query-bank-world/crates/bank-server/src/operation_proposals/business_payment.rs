use bank_domain::model::{BusinessId, PaymentId};
use bank_domain::proposals::{
    BankIdempotencyKey, BankProposalDenial, BankProposalEngine, BankProposedEffect,
};
use bank_domain::schema::{
    ApprovePayment, ApprovePaymentOperation, Business, InitiateBusinessPayment,
    InitiateBusinessPaymentOperation, PaymentIntent, RejectPayment, RejectPaymentOperation,
};

use crate::bank_projection::{
    project_business_payment_initiation, project_payment_approval, project_payment_rejection,
};
use crate::{
    BankAdmittedOperation, BankAuthorizedProposal, BankIdentityRuntime, BankOperationProposalError,
    BankOperationProposals,
};

impl
    BankAuthorizedProposal<
        InitiateBusinessPaymentOperation,
        InitiateBusinessPayment,
        Business,
        BusinessId,
    >
{
    pub(crate) fn initiated_payment_id(&self) -> Option<PaymentId> {
        let [BankProposedEffect::CreatePayment(payment)] = self.invariant().effects() else {
            return None;
        };
        Some(payment.id())
    }
}

impl BankOperationProposals {
    pub fn prepare_initiate_business_payment(
        runtime: &BankIdentityRuntime,
        admission: BankAdmittedOperation<
            InitiateBusinessPaymentOperation,
            InitiateBusinessPayment,
            Business,
            BusinessId,
        >,
        key: &BankIdempotencyKey,
        input: &InitiateBusinessPayment,
    ) -> Result<
        BankAuthorizedProposal<
            InitiateBusinessPaymentOperation,
            InitiateBusinessPayment,
            Business,
            BusinessId,
        >,
        BankOperationProposalError,
    > {
        if admission.scope() != input.business {
            return Err(BankProposalDenial::ScopeInputMismatch.into());
        }
        let completed = runtime.invariant_projection().project_admitted_operation(
            admission.query(),
            |reader, business| {
                project_business_payment_initiation(reader, business, admission.actor(), input)
            },
        )?;
        let (snapshot, projection, work) = completed.into_parts();
        let snapshot = snapshot?;
        let invariant = BankProposalEngine::prepare_initiate_business_payment(
            &snapshot,
            admission.idempotency_binding(),
            key,
            admission.actor(),
            input,
        )?;
        Ok(BankAuthorizedProposal::new_bounded(
            admission, invariant, projection, work,
        ))
    }

    pub fn prepare_approve_payment(
        runtime: &BankIdentityRuntime,
        admission: BankAdmittedOperation<
            ApprovePaymentOperation,
            ApprovePayment,
            PaymentIntent,
            PaymentId,
        >,
        key: &BankIdempotencyKey,
        input: &ApprovePayment,
    ) -> Result<
        BankAuthorizedProposal<ApprovePaymentOperation, ApprovePayment, PaymentIntent, PaymentId>,
        BankOperationProposalError,
    > {
        validate_decision_binding(
            admission.scope(),
            admission.actor(),
            input.payment,
            input.approver,
        )?;
        let completed = runtime
            .invariant_projection()
            .project_admitted_operation(admission.query(), |reader, payment| {
                project_payment_approval(reader, payment, input)
            })?;
        let (snapshot, projection, work) = completed.into_parts();
        let snapshot = snapshot?;
        let invariant = BankProposalEngine::prepare_approve_payment_from_decision(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new_bounded(
            admission, invariant, projection, work,
        ))
    }

    pub fn prepare_reject_payment(
        runtime: &BankIdentityRuntime,
        admission: BankAdmittedOperation<
            RejectPaymentOperation,
            RejectPayment,
            PaymentIntent,
            PaymentId,
        >,
        key: &BankIdempotencyKey,
        input: &RejectPayment,
    ) -> Result<
        BankAuthorizedProposal<RejectPaymentOperation, RejectPayment, PaymentIntent, PaymentId>,
        BankOperationProposalError,
    > {
        validate_decision_binding(
            admission.scope(),
            admission.actor(),
            input.payment,
            input.rejecting_principal,
        )?;
        let completed = runtime
            .invariant_projection()
            .project_admitted_operation(admission.query(), |reader, payment| {
                project_payment_rejection(reader, payment, input)
            })?;
        let (snapshot, projection, work) = completed.into_parts();
        let snapshot = snapshot?;
        let invariant = BankProposalEngine::prepare_reject_payment_from_decision(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new_bounded(
            admission, invariant, projection, work,
        ))
    }
}

fn validate_decision_binding(
    admitted_payment: PaymentId,
    authenticated_actor: bank_domain::model::BankPrincipalId,
    input_payment: PaymentId,
    input_actor: bank_domain::model::BankPrincipalId,
) -> Result<(), BankProposalDenial> {
    if admitted_payment != input_payment {
        Err(BankProposalDenial::ScopeInputMismatch)
    } else if authenticated_actor != input_actor {
        Err(BankProposalDenial::AuthenticatedActorMismatch)
    } else {
        Ok(())
    }
}
