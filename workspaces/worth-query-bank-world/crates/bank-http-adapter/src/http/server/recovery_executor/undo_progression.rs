use bank_server::{BankMutationCommitOutcome, BankUndoRetry};
use worth_query_host::facade::admission::authenticated_principal::{
    WorthQueryCancellationSource, WorthQueryRequestScope,
};

use super::super::super::protocol::{
    BankHttpCommitDisposition, BankHttpUndoCorrection, BankHttpUndoProgressionOutcome,
};
use super::super::authenticated_owner::BankHttpAuthenticatedOwner;
use super::super::authentication::BankHttpApplicationAuthenticator;
use super::super::recovery_registry::{
    BankHttpRecoveryRegistry, BankHttpUndoAuthority, BankHttpUndoReplay,
};
use super::{outcome::*, AdmittedBankHttpUndoProgressionRequest};

pub(super) async fn execute_undo_progression<A>(
    application: &A,
    registry: &mut BankHttpRecoveryRegistry,
    request: AdmittedBankHttpUndoProgressionRequest,
    cancellation: WorthQueryCancellationSource,
) -> BankHttpUndoProgressionOutcome
where
    A: BankHttpApplicationAuthenticator,
{
    let request_id = request.request_id;
    let scope = WorthQueryRequestScope::new(request.deadline, cancellation.token());
    let principal = match application.authenticate(request.credential, &scope).await {
        Ok(principal) => principal,
        Err(denial) => return undo_progression_denied(Some(request_id), denial),
    };
    let owner = BankHttpAuthenticatedOwner::from_principal(&principal);
    match registry.undo_replay(&owner, &request.token, &request.idempotency_key) {
        BankHttpUndoReplay::Applied {
            disposition,
            commit,
            redo,
        } => {
            return BankHttpUndoProgressionOutcome::Applied {
                request_id,
                disposition,
                commit,
                redo,
            };
        }
        BankHttpUndoReplay::Conflicting => {
            return undo_progression_denied(Some(request_id), conflicting_idempotency_key());
        }
        BankHttpUndoReplay::Reconciled => {
            return BankHttpUndoProgressionOutcome::Reconciled { request_id };
        }
        BankHttpUndoReplay::Missing => {}
    }
    let Some(admission) = registry.take_undo(&owner, &request.token) else {
        return undo_progression_denied(Some(request_id), stale());
    };
    let admission = match admission {
        BankHttpUndoAuthority::Compensation(admission) => admission,
        BankHttpUndoAuthority::RecordedInverse {
            admission,
            correction,
        } => {
            if correction != BankHttpUndoCorrection::Reconciliation {
                return undo_progression_denied(Some(request_id), unavailable());
            }
            if let Err(failure) = application
                .runtime()
                .progress_undo_reconciliation_retaining(admission, &principal, &scope)
            {
                let (denial, retry) = failure.into_parts();
                if let Some(admission) = retry {
                    registry.restore_undo(
                        &request.token,
                        BankHttpUndoAuthority::RecordedInverse {
                            admission,
                            correction,
                        },
                    );
                }
                return undo_progression_denied(Some(request_id), estate_denial(denial));
            }
            registry.install_reconciled(&request.token, request.idempotency_key);
            return BankHttpUndoProgressionOutcome::Reconciled { request_id };
        }
    };
    let outcome = match application.runtime().progress_undo_commit_recovery(
        admission,
        &principal,
        &request.idempotency_key,
        &scope,
    ) {
        Ok(outcome) => outcome,
        Err(failure) => {
            let (denial, retry) = failure.into_parts();
            if let Some(retry) = retry {
                restore_retry(registry, &request.token, retry);
            }
            return undo_progression_denied(Some(request_id), estate_denial(denial));
        }
    };
    let (mutation, recovery, retry) = outcome.into_parts();
    let (disposition, receipt) = match mutation {
        BankMutationCommitOutcome::Committed(receipt) => {
            (BankHttpCommitDisposition::Committed, receipt)
        }
        BankMutationCommitOutcome::AlreadyCommitted(receipt) => {
            (BankHttpCommitDisposition::AlreadyCommitted, receipt)
        }
        other => {
            if let Some(retry) = retry {
                restore_retry(registry, &request.token, retry);
            }
            return undo_progression_denied(Some(request_id), commit_denial(other));
        }
    };
    let Some(recovery) = recovery else {
        return undo_progression_denied(Some(request_id), unavailable());
    };
    let commit = commit_description(&receipt);
    registry.install_redo(
        &request.token,
        request.idempotency_key,
        disposition,
        commit,
        recovery,
    );
    BankHttpUndoProgressionOutcome::Applied {
        request_id,
        disposition,
        commit,
        redo: request.token,
    }
}

fn restore_retry(registry: &mut BankHttpRecoveryRegistry, token: &str, retry: BankUndoRetry) {
    let authority = match retry {
        BankUndoRetry::Compensation(admission) => BankHttpUndoAuthority::Compensation(admission),
        BankUndoRetry::RecordedInverse(admission) => {
            let correction = undo_correction(admission.correction());
            BankHttpUndoAuthority::RecordedInverse {
                admission,
                correction,
            }
        }
    };
    registry.restore_undo(token, authority);
}
