use worth_query_host::facade::admission::authenticated_principal::{
    WorthQueryCancellationSource, WorthQueryRequestScope,
};

use super::super::super::protocol::{BankHttpUndoAdmissionOutcome, BankHttpUndoCorrection};
use super::super::authenticated_owner::BankHttpAuthenticatedOwner;
use super::super::authentication::BankHttpApplicationAuthenticator;
use super::super::recovery_registry::{BankHttpRecoveryAuthority, BankHttpRecoveryRegistry};
use super::{outcome::*, AdmittedBankHttpRecoveryRequest};

pub(super) async fn execute_undo_admission<A>(
    application: &A,
    registry: &mut BankHttpRecoveryRegistry,
    request: AdmittedBankHttpRecoveryRequest,
    cancellation: WorthQueryCancellationSource,
) -> BankHttpUndoAdmissionOutcome
where
    A: BankHttpApplicationAuthenticator,
{
    let AdmittedBankHttpRecoveryRequest {
        request_id,
        credential,
        token,
        deadline,
    } = request;
    let scope = WorthQueryRequestScope::new(deadline, cancellation.token());
    let principal = match application.authenticate(credential, &scope).await {
        Ok(principal) => principal,
        Err(denial) => return undo_denied(Some(request_id), denial),
    };
    let owner = BankHttpAuthenticatedOwner::from_principal(&principal);
    if let Some(correction) = registry.undo_admission_replay(&owner, &token) {
        return admitted(request_id, token, correction);
    }
    let Some(recovery) = registry.take_recovery(&owner, &token) else {
        return undo_denied(Some(request_id), stale());
    };
    match recovery {
        BankHttpRecoveryAuthority::Notification(handle) => match application
            .runtime()
            .admit_undo_commit_recovery_retaining(handle, &principal, &scope)
        {
            Ok(admission) => {
                let correction = undo_correction(admission.correction());
                registry.install_recorded_inverse_undo(&token, admission, correction);
                admitted(request_id, token, correction)
            }
            Err(failure) => {
                let (denial, retry) = failure.into_parts();
                if let Some(handle) = retry {
                    registry
                        .restore_recovery(&token, BankHttpRecoveryAuthority::Notification(handle));
                }
                undo_denied(Some(request_id), estate_denial(denial))
            }
        },
        BankHttpRecoveryAuthority::Disbursement(handle) => match application
            .runtime()
            .admit_undo_disbursement_recovery_retaining(handle, &principal, &scope)
        {
            Ok(admission) => {
                registry.install_compensation_undo(&token, admission);
                admitted(request_id, token, BankHttpUndoCorrection::Compensation)
            }
            Err(failure) => {
                let (denial, retry) = failure.into_parts();
                if let Some(handle) = retry {
                    registry
                        .restore_recovery(&token, BankHttpRecoveryAuthority::Disbursement(handle));
                }
                undo_denied(Some(request_id), estate_denial(denial))
            }
        },
    }
}

fn admitted(
    request_id: String,
    token: String,
    correction: BankHttpUndoCorrection,
) -> BankHttpUndoAdmissionOutcome {
    BankHttpUndoAdmissionOutcome::Admitted {
        request_id,
        undo: token,
        correction,
    }
}
