use bank_server::BankMutationCommitOutcome;
use worth_query_host::facade::admission::authenticated_principal::{
    WorthQueryCancellationSource, WorthQueryRequestScope,
};

use super::super::super::protocol::{BankHttpCommitDisposition, BankHttpRedoProgressionOutcome};
use super::super::authenticated_owner::BankHttpAuthenticatedOwner;
use super::super::authentication::BankHttpApplicationAuthenticator;
use super::super::recovery_registry::{
    BankHttpRecoveryRegistry, BankHttpRedoBinding, BankHttpRedoReplay,
};
use super::{outcome::*, AdmittedBankHttpRecoveryRequest};

pub(super) async fn execute_redo_progression<A>(
    application: &A,
    registry: &mut BankHttpRecoveryRegistry,
    request: AdmittedBankHttpRecoveryRequest,
    cancellation: WorthQueryCancellationSource,
) -> BankHttpRedoProgressionOutcome
where
    A: BankHttpApplicationAuthenticator,
{
    let request_id = request.request_id;
    let scope = WorthQueryRequestScope::new(request.deadline, cancellation.token());
    let principal = match application.authenticate(request.credential, &scope).await {
        Ok(principal) => principal,
        Err(denial) => return redo_progression_denied(Some(request_id), denial),
    };
    let owner = BankHttpAuthenticatedOwner::from_principal(&principal);
    match registry.redo_replay(&owner, &request.token) {
        BankHttpRedoReplay::Applied {
            disposition,
            commit,
        } => {
            return BankHttpRedoProgressionOutcome::Applied {
                request_id,
                disposition,
                commit,
            };
        }
        BankHttpRedoReplay::Missing => {}
    }
    let Some(authority) = registry.take_redo(&owner, &request.token) else {
        return redo_progression_denied(Some(request_id), stale());
    };
    let (recovery, binding) = authority.into_parts();
    let intent = match application.runtime().derive_redo_intent(&recovery) {
        Ok(intent) => intent,
        Err(denial) => {
            restore_redo(registry, &request.token, recovery, binding);
            return redo_progression_denied(Some(request_id), estate_denial(denial));
        }
    };
    let admission = match application
        .runtime()
        .admit_redo_disbursement_recovery_retaining(recovery, &principal, &scope, &intent)
    {
        Ok(admission) => admission,
        Err(failure) => {
            let (denial, retry) = failure.into_parts();
            if let Some(recovery) = retry {
                restore_redo(registry, &request.token, recovery, binding);
            }
            return redo_progression_denied(Some(request_id), estate_denial(denial));
        }
    };
    let outcome = match application.runtime().progress_redo_disbursement(admission) {
        Ok(outcome) => outcome,
        Err(failure) => {
            let (denial, retry) = failure.into_parts();
            if let Some(recovery) = retry {
                restore_redo(registry, &request.token, recovery, binding);
            }
            return redo_progression_denied(Some(request_id), estate_denial(denial));
        }
    };
    let (mutation, retry) = outcome.into_parts();
    let (disposition, receipt) = match mutation {
        BankMutationCommitOutcome::Committed(receipt) => {
            (BankHttpCommitDisposition::Committed, receipt)
        }
        BankMutationCommitOutcome::AlreadyCommitted(receipt) => {
            (BankHttpCommitDisposition::AlreadyCommitted, receipt)
        }
        other => {
            if let Some(recovery) = retry {
                restore_redo(registry, &request.token, recovery, binding);
            }
            return redo_progression_denied(Some(request_id), commit_denial(other));
        }
    };
    let commit = commit_description(&receipt);
    registry.install_redo_commit(&request.token, disposition, commit);
    BankHttpRedoProgressionOutcome::Applied {
        request_id,
        disposition,
        commit,
    }
}

fn restore_redo(
    registry: &mut BankHttpRecoveryRegistry,
    token: &str,
    recovery: bank_server::BankRedoRecovery,
    binding: BankHttpRedoBinding,
) {
    registry.restore_redo(token, binding.bind(recovery));
}
