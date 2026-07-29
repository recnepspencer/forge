use bank_domain::model::AccountId;
use bank_domain::proposals::{BankIdempotencyKey, BankProposalDenial, BankProposalEngine};
use bank_domain::schema::{
    Account, GrantAccountAuthorization, GrantAccountAuthorizationOperation,
    RevokeAccountAuthorization, RevokeAccountAuthorizationOperation,
};

use crate::bank_projection::{
    project_account_authorization_grant, project_account_authorization_revoke,
};
use crate::{
    BankAdmittedOperation, BankAuthorizedProposal, BankIdentityRuntime, BankOperationProposalError,
    BankOperationProposals,
};

impl BankOperationProposals {
    pub fn prepare_grant_account_access(
        runtime: &BankIdentityRuntime,
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
        BankOperationProposalError,
    > {
        if admission.scope() != input.account {
            return Err(BankProposalDenial::ScopeInputMismatch.into());
        }
        let completed = runtime
            .invariant_projection()
            .project_admitted_operation(admission.query(), |reader, account| {
                project_account_authorization_grant(reader, account, input)
            })?;
        let (snapshot, projection, work) = completed.into_parts();
        let snapshot = snapshot?;
        let invariant = BankProposalEngine::prepare_grant_account_authorization(
            &snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new_bounded(
            admission, invariant, projection, work,
        ))
    }

    pub fn prepare_revoke_account_access(
        runtime: &BankIdentityRuntime,
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
        BankOperationProposalError,
    > {
        if admission.scope() != input.account {
            return Err(BankProposalDenial::ScopeInputMismatch.into());
        }
        let completed = runtime.invariant_projection().project_admitted_operation(
            admission.query(),
            |reader, account| {
                project_account_authorization_revoke(reader, account, admission.scope(), input)
            },
        )?;
        let (snapshot, projection, work) = completed.into_parts();
        let snapshot = snapshot?;
        let authorization = snapshot
            .authorization(input.authorization)
            .ok_or(BankProposalDenial::UnknownAuthorization)?;
        if input.account != authorization.account() {
            return Err(BankProposalDenial::ScopeInputMismatch.into());
        }
        let invariant = BankProposalEngine::prepare_revoke_account_authorization(
            &snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new_bounded(
            admission, invariant, projection, work,
        ))
    }
}
