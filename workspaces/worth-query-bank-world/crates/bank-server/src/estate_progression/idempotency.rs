use bank_domain::{
    estate::EstateAction,
    schema::{BankSchema, EstateCase},
};
use worth_query_host::facade::primary_graph::{
    WorthQueryAdmittedApplicationOperation, WorthQueryApplicationCommitDenialKind,
    WorthQueryApplicationCommitDenialStage, WorthQueryApplicationIdempotencyBinding,
    WorthQueryApplicationIdempotencyResolution,
};

use super::BankEstateProgressionDenial;
use crate::operation_commit::commit_receipt;
use crate::{BankIdentityRuntime, BankMutationCommitOutcome};

pub(super) fn resolve_admitted_idempotency<Operation>(
    runtime: &BankIdentityRuntime,
    admission: &WorthQueryAdmittedApplicationOperation<
        BankSchema,
        Operation,
        EstateAction,
        EstateCase,
    >,
    idempotency: WorthQueryApplicationIdempotencyBinding,
) -> Result<Option<BankMutationCommitOutcome>, BankEstateProgressionDenial> {
    match runtime
        .application_runtime()
        .resolve_admitted_application_idempotency(admission, idempotency)
        .map_err(BankEstateProgressionDenial::Idempotency)?
    {
        WorthQueryApplicationIdempotencyResolution::Unseen => Ok(None),
        WorthQueryApplicationIdempotencyResolution::AlreadyCommitted(receipt) => Ok(Some(
            BankMutationCommitOutcome::AlreadyCommitted(commit_receipt(receipt)),
        )),
        WorthQueryApplicationIdempotencyResolution::IntentDrift => {
            Ok(Some(BankMutationCommitOutcome::Denied {
                kind: WorthQueryApplicationCommitDenialKind::IdempotencyIntentDrift,
                stage: WorthQueryApplicationCommitDenialStage::Idempotency,
            }))
        }
    }
}
