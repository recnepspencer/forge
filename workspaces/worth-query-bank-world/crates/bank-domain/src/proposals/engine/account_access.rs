use crate::proposals::{
    complete_proposal, BankAccountAuthorization, BankIdempotencyClaim, BankIdempotencyKey,
    BankInvariantApprovedProposal, BankOperationScopeBinding, BankProposalDenial,
    BankProposedEffect, BankSnapshot, CanonicalProposalPayload,
};
use crate::schema::{GrantAccountAuthorization, RevokeAccountAuthorization};

use super::BankProposalEngine;

impl BankProposalEngine {
    pub fn prepare_grant_account_authorization(
        snapshot: &BankSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &GrantAccountAuthorization,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        if snapshot.account(input.account).is_none() {
            return Err(BankProposalDenial::UnknownAccount(input.account));
        }
        if !snapshot.is_known_principal(input.principal) {
            return Err(BankProposalDenial::UnknownPrincipal);
        }
        if snapshot.has_authorization(input.account, input.principal) {
            return Err(BankProposalDenial::DuplicateAuthorization);
        }

        let payload = CanonicalProposalPayload::new()
            .text(&input.account.canonical_text())
            .u64(input.principal.get())
            .byte(role_tag(input.role));
        let intent = BankIdempotencyClaim::derive(
            binding,
            key,
            "grant-account-authorization",
            payload.as_bytes(),
        );
        let mut proposed = snapshot.clone();
        let authorization = BankAccountAuthorization::new(
            crate::model::AccountAuthorizationId::from_operation(intent.key().bytes(), 0),
            input.account,
            input.principal,
            input.role,
        );
        proposed.insert_authorization(authorization);
        complete_proposal(
            snapshot,
            proposed,
            intent,
            vec![BankProposedEffect::GrantAuthorization(authorization)],
        )
    }

    pub fn prepare_revoke_account_authorization(
        snapshot: &BankSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &RevokeAccountAuthorization,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        let authorization = *snapshot
            .authorization(input.authorization)
            .ok_or(BankProposalDenial::UnknownAuthorization)?;
        if authorization.account() != input.account {
            return Err(BankProposalDenial::ScopeInputMismatch);
        }
        let payload = CanonicalProposalPayload::new()
            .text(&input.account.canonical_text())
            .text(&input.authorization.canonical_text())
            .text(&authorization.account().canonical_text())
            .u64(authorization.principal().get());
        let intent = BankIdempotencyClaim::derive(
            binding,
            key,
            "revoke-account-authorization",
            payload.as_bytes(),
        );
        let mut proposed = snapshot.clone();
        proposed.remove_authorization(input.authorization);
        complete_proposal(
            snapshot,
            proposed,
            intent,
            vec![BankProposedEffect::RevokeAuthorization(authorization)],
        )
    }
}

const fn role_tag(role: crate::model::CustomerRole) -> u8 {
    match role {
        crate::model::CustomerRole::PersonalOwner => 1,
        crate::model::CustomerRole::BusinessOwner => 2,
        crate::model::CustomerRole::Initiator => 3,
        crate::model::CustomerRole::Approver => 4,
        crate::model::CustomerRole::Viewer => 5,
    }
}
