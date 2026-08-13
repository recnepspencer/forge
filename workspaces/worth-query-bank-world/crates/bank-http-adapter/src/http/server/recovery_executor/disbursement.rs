use bank_server::BankMutationCommitOutcome;
use worth_query_host::facade::admission::authenticated_principal::{
    WorthQueryCancellationSource, WorthQueryRequestScope,
};

use super::super::super::protocol::{BankHttpCommitDisposition, BankHttpEstateDisbursementOutcome};
use super::super::authenticated_owner::BankHttpAuthenticatedOwner;
use super::super::authentication::BankHttpApplicationAuthenticator;
use super::super::recovery_registry::{
    BankHttpCommitReplay, BankHttpRecoveryRegistration, BankHttpRecoveryRegistry,
};
use super::{outcome::*, AdmittedBankHttpDisbursementRequest};

pub(super) async fn execute_disbursement<A>(
    application: &A,
    registry: &mut BankHttpRecoveryRegistry,
    request: AdmittedBankHttpDisbursementRequest,
    cancellation: WorthQueryCancellationSource,
) -> BankHttpEstateDisbursementOutcome
where
    A: BankHttpApplicationAuthenticator,
{
    let request_id = request.request_id;
    let scope = WorthQueryRequestScope::new(request.deadline, cancellation.token());
    let principal = match application.authenticate(request.credential, &scope).await {
        Ok(principal) => principal,
        Err(denial) => return disbursement_denied(Some(request_id), denial),
    };
    let owner = BankHttpAuthenticatedOwner::from_principal(&principal);
    match registry.disbursement_replay(&owner, &request.idempotency_key, request.action) {
        BankHttpCommitReplay::Applied { commit, recovery } => {
            return BankHttpEstateDisbursementOutcome::Applied {
                request_id,
                disposition: BankHttpCommitDisposition::AlreadyCommitted,
                commit,
                recovery,
            };
        }
        BankHttpCommitReplay::Conflicting => {
            return disbursement_denied(Some(request_id), conflicting_idempotency_key());
        }
        BankHttpCommitReplay::Missing => {}
    }
    let Some(reservation) =
        registry.reserve_disbursement(owner, request.idempotency_key.clone(), request.action)
    else {
        return disbursement_denied(Some(request_id), saturated());
    };
    let outcome = match application.runtime().disburse_estate_with_key(
        &principal,
        request.action,
        &request.idempotency_key,
        &scope,
    ) {
        Ok(outcome) => outcome,
        Err(denial) => return disbursement_denied(Some(request_id), estate_denial(denial)),
    };
    let (disposition, receipt) = match outcome {
        BankMutationCommitOutcome::Committed(receipt) => {
            (BankHttpCommitDisposition::Committed, receipt)
        }
        BankMutationCommitOutcome::AlreadyCommitted(receipt) => {
            (BankHttpCommitDisposition::AlreadyCommitted, receipt)
        }
        other => return disbursement_denied(Some(request_id), commit_denial(other)),
    };
    let handle = match application.runtime().open_commit_recovery(&receipt) {
        Ok(handle) => handle,
        Err(_) => return disbursement_denied(Some(request_id), unavailable()),
    };
    let commit = commit_description(&receipt);
    let recovery = reservation.register(BankHttpRecoveryRegistration { commit, handle });
    BankHttpEstateDisbursementOutcome::Applied {
        request_id,
        disposition,
        commit,
        recovery,
    }
}
