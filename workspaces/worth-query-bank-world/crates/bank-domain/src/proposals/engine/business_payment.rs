use crate::model::BankPrincipalId;
use crate::payments::BusinessPayment;
use crate::schema::{
    ApprovePayment, InitiateBusinessPayment, PaymentStatus, PostingPurpose, RejectPayment,
};

use super::BankProposalEngine;
use crate::proposals::{
    account_activity_effects, append_balanced_transfer, complete_proposal, ensure_open,
    BankIdempotencyIntent, BankIdempotencyKey, BankInvariantApprovedProposal,
    BankOperationScopeBinding, BankProposalDenial, BankProposedEffect, BankSnapshot,
    CanonicalProposalPayload,
};

impl BankProposalEngine {
    pub fn prepare_initiate_business_payment(
        snapshot: &BankSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        initiator: BankPrincipalId,
        input: &InitiateBusinessPayment,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        if !snapshot.is_known_principal(initiator) {
            return Err(BankProposalDenial::UnknownPrincipal);
        }
        if snapshot.business_account(input.business) != Some(input.from) {
            return Err(BankProposalDenial::AccountOwnershipMismatch);
        }
        let destination = snapshot
            .primary_account(input.recipient)
            .ok_or(BankProposalDenial::UnknownRecipient)?;
        ensure_open(snapshot, input.from)?;
        ensure_open(snapshot, destination)?;
        if destination == input.from {
            return Err(BankProposalDenial::SelfTransfer);
        }

        let payload = CanonicalProposalPayload::new()
            .u64(initiator.get())
            .u64(input.business.get())
            .u64(input.from.get())
            .u64(destination.get())
            .i64(input.amount.minor_units());
        let intent = BankIdempotencyIntent::derive(
            binding,
            key,
            "initiate-business-payment",
            payload.as_bytes(),
        );
        let mut proposed = snapshot.clone();
        let payment = BusinessPayment::pending(
            proposed.allocate_payment_id()?,
            input.business,
            input.from,
            destination,
            initiator,
            input.amount,
        );
        proposed.insert_payment(payment.clone());
        complete_proposal(
            snapshot,
            proposed,
            intent,
            vec![BankProposedEffect::CreatePayment(payment)],
        )
    }

    pub fn prepare_approve_payment(
        snapshot: &BankSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &ApprovePayment,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        let payment = pending_payment(snapshot, input.payment)?;
        if payment.initiator() == input.approver {
            return Err(BankProposalDenial::SelfApproval);
        }
        if !snapshot.is_known_principal(input.approver) {
            return Err(BankProposalDenial::UnknownPrincipal);
        }

        let payload = CanonicalProposalPayload::new()
            .u64(input.payment.get())
            .u64(input.approver.get());
        let intent =
            BankIdempotencyIntent::derive(binding, key, "approve-payment", payload.as_bytes());
        let mut proposed = snapshot.clone();
        let journal = append_balanced_transfer(
            &mut proposed,
            payment.source(),
            payment.destination(),
            payment.amount(),
            PostingPurpose::Transfer,
            None,
        )?;
        let replacement = payment.with_decision(PaymentStatus::Committed, input.approver);
        proposed.replace_payment(replacement.clone());
        let mut effects = vec![
            BankProposedEffect::AppendJournal(journal.clone()),
            BankProposedEffect::UpdatePayment {
                payment: input.payment,
                replacement,
            },
        ];
        effects.extend(account_activity_effects(&journal));
        complete_proposal(snapshot, proposed, intent, effects)
    }

    pub fn prepare_reject_payment(
        snapshot: &BankSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &RejectPayment,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        let payment = pending_payment(snapshot, input.payment)?;
        if payment.initiator() == input.rejecting_principal {
            return Err(BankProposalDenial::SelfApproval);
        }
        if !snapshot.is_known_principal(input.rejecting_principal) {
            return Err(BankProposalDenial::UnknownPrincipal);
        }

        let payload = CanonicalProposalPayload::new()
            .u64(input.payment.get())
            .u64(input.rejecting_principal.get());
        let intent =
            BankIdempotencyIntent::derive(binding, key, "reject-payment", payload.as_bytes());
        let replacement = payment.with_decision(PaymentStatus::Rejected, input.rejecting_principal);
        let mut proposed = snapshot.clone();
        proposed.replace_payment(replacement.clone());
        complete_proposal(
            snapshot,
            proposed,
            intent,
            vec![BankProposedEffect::UpdatePayment {
                payment: input.payment,
                replacement,
            }],
        )
    }
}

fn pending_payment(
    snapshot: &BankSnapshot,
    payment_id: crate::model::PaymentId,
) -> Result<&BusinessPayment, BankProposalDenial> {
    let payment = snapshot
        .payment(payment_id)
        .ok_or(BankProposalDenial::UnknownPayment(payment_id))?;
    if payment.status() != PaymentStatus::ApprovalRequired {
        return Err(BankProposalDenial::PaymentAlreadyDecided(payment_id));
    }
    Ok(payment)
}
