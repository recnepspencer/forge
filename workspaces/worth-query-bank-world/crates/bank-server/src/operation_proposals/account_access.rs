use bank_domain::model::AccountId;
use bank_domain::proposals::{
    BankIdempotencyKey, BankProposalDenial, BankProposalEngine, BankSnapshot,
};
use bank_domain::schema::{
    Account, GrantAccountAuthorization, GrantAccountAuthorizationOperation,
    RevokeAccountAuthorization, RevokeAccountAuthorizationOperation,
};

use crate::{BankAdmittedOperation, BankAuthorizedProposal, BankOperationProposals};

impl BankOperationProposals {
    pub fn prepare_grant_account_access(
        snapshot: &BankSnapshot,
        admission: BankAdmittedOperation<
            GrantAccountAuthorizationOperation,
            GrantAccountAuthorization,
            Account,
            AccountId,
        >,
        key: &BankIdempotencyKey,
        input: &GrantAccountAuthorization,
    ) -> Result<
        BankAuthorizedProposal<
            GrantAccountAuthorizationOperation,
            GrantAccountAuthorization,
            Account,
            AccountId,
        >,
        BankProposalDenial,
    > {
        if admission.scope() != input.account {
            return Err(BankProposalDenial::ScopeInputMismatch);
        }
        let invariant = BankProposalEngine::prepare_grant_account_authorization(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new(admission, invariant))
    }

    pub fn prepare_revoke_account_access(
        snapshot: &BankSnapshot,
        admission: BankAdmittedOperation<
            RevokeAccountAuthorizationOperation,
            RevokeAccountAuthorization,
            Account,
            AccountId,
        >,
        key: &BankIdempotencyKey,
        input: &RevokeAccountAuthorization,
    ) -> Result<
        BankAuthorizedProposal<
            RevokeAccountAuthorizationOperation,
            RevokeAccountAuthorization,
            Account,
            AccountId,
        >,
        BankProposalDenial,
    > {
        let authorization = snapshot
            .authorization(input.authorization)
            .ok_or(BankProposalDenial::UnknownAuthorization)?;
        if admission.scope() != authorization.account() {
            return Err(BankProposalDenial::ScopeInputMismatch);
        }
        let invariant = BankProposalEngine::prepare_revoke_account_authorization(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new(admission, invariant))
    }
}
