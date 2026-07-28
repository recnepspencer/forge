use crate::accounting::BankAccount;
use crate::schema::{CreateBusinessAccount, CreatePersonalAccount};

use super::BankProposalEngine;
use crate::proposals::{
    complete_proposal, BankIdempotencyIntent, BankIdempotencyKey, BankInvariantApprovedProposal,
    BankOperationScopeBinding, BankProposalDenial, BankProposedEffect, BankSnapshot,
    CanonicalProposalPayload,
};

impl BankProposalEngine {
    pub fn prepare_create_personal_account(
        snapshot: &BankSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &CreatePersonalAccount,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        if !snapshot.is_known_institution(input.institution) {
            return Err(BankProposalDenial::UnknownInstitution);
        }
        if !snapshot.is_known_principal(input.owner) {
            return Err(BankProposalDenial::UnknownPrincipal);
        }
        if snapshot.primary_account(input.owner).is_some() {
            return Err(BankProposalDenial::DuplicatePersonalAccount);
        }

        let mut proposed = snapshot.clone();
        let account = BankAccount::personal(
            proposed.allocate_account_id()?,
            input.institution,
            input.owner,
            input.display_name.clone(),
        );
        proposed.insert_account(account.clone());
        let intent = account_creation_intent(
            binding,
            key,
            "create-personal-account",
            input.institution.get(),
            input.owner.get(),
            input.display_name.as_str(),
        );
        complete_proposal(
            snapshot,
            proposed,
            intent,
            vec![BankProposedEffect::CreateAccount(account)],
        )
    }

    pub fn prepare_create_business_account(
        snapshot: &BankSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &CreateBusinessAccount,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        if !snapshot.is_known_institution(input.institution) {
            return Err(BankProposalDenial::UnknownInstitution);
        }
        if !snapshot.is_known_business(input.business) {
            return Err(BankProposalDenial::UnknownBusiness);
        }
        if snapshot.business_account(input.business).is_some() {
            return Err(BankProposalDenial::DuplicateBusinessAccount);
        }

        let mut proposed = snapshot.clone();
        let account = BankAccount::business(
            proposed.allocate_account_id()?,
            input.institution,
            input.business,
            input.display_name.clone(),
        );
        proposed.insert_account(account.clone());
        let intent = account_creation_intent(
            binding,
            key,
            "create-business-account",
            input.institution.get(),
            input.business.get(),
            input.display_name.as_str(),
        );
        complete_proposal(
            snapshot,
            proposed,
            intent,
            vec![BankProposedEffect::CreateAccount(account)],
        )
    }
}

fn account_creation_intent(
    binding: BankOperationScopeBinding,
    key: &BankIdempotencyKey,
    operation: &'static str,
    institution: u64,
    owner: u64,
    display_name: &str,
) -> BankIdempotencyIntent {
    let payload = CanonicalProposalPayload::new()
        .u64(institution)
        .u64(owner)
        .text(display_name);
    BankIdempotencyIntent::derive(binding, key, operation, payload.as_bytes())
}
