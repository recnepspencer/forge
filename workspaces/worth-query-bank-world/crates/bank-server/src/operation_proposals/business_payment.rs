use bank_domain::model::{BusinessId, PaymentId};
use bank_domain::proposals::{
    BankIdempotencyKey, BankProposalDenial, BankProposalEngine, BankSnapshot,
};
use bank_domain::schema::{
    ApprovePayment, ApprovePaymentOperation, Business, InitiateBusinessPayment,
    InitiateBusinessPaymentOperation, PaymentIntent, RejectPayment, RejectPaymentOperation,
};

use crate::{BankAdmittedOperation, BankAuthorizedProposal, BankOperationProposals};

impl BankOperationProposals {
    pub fn prepare_initiate_business_payment(
        snapshot: &BankSnapshot,
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
        BankProposalDenial,
    > {
        if admission.scope() != input.business {
            return Err(BankProposalDenial::ScopeInputMismatch);
        }
        let invariant = BankProposalEngine::prepare_initiate_business_payment(
            snapshot,
            admission.idempotency_binding(),
            key,
            admission.actor(),
            input,
        )?;
        Ok(BankAuthorizedProposal::new(admission, invariant))
    }

    pub fn prepare_approve_payment(
        snapshot: &BankSnapshot,
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
        BankProposalDenial,
    > {
        validate_decision_binding(
            admission.scope(),
            admission.actor(),
            input.payment,
            input.approver,
        )?;
        let invariant = BankProposalEngine::prepare_approve_payment(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new(admission, invariant))
    }

    pub fn prepare_reject_payment(
        snapshot: &BankSnapshot,
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
        BankProposalDenial,
    > {
        validate_decision_binding(
            admission.scope(),
            admission.actor(),
            input.payment,
            input.rejecting_principal,
        )?;
        let invariant = BankProposalEngine::prepare_reject_payment(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new(admission, invariant))
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
