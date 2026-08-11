use bank_domain::{
    estate::EstateAction,
    schema::{BankSchema, EstateCase},
};
use worth_query_host::facade::primary_graph::{
    WorthQueryAdmittedApplicationOperation, WorthQueryApplicationIdempotencyBinding,
    WorthQueryApplicationIdempotencyResolution,
};

use super::BankEstateProgressionDenial;
use crate::operation_commit::commit_receipt;
use crate::{
    BankCommitDenialKind, BankCommitDenialStage, BankIdentityRuntime, BankMutationCommitOutcome,
};

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
        .map_err(BankEstateProgressionDenial::from_idempotency)?
        .into_resolution()
    {
        WorthQueryApplicationIdempotencyResolution::Unseen => Ok(None),
        WorthQueryApplicationIdempotencyResolution::AlreadyCommitted(receipt) => Ok(Some(
            BankMutationCommitOutcome::AlreadyCommitted(commit_receipt(receipt)),
        )),
        WorthQueryApplicationIdempotencyResolution::IntentDrift => {
            Ok(Some(BankMutationCommitOutcome::Denied {
                kind: BankCommitDenialKind::IdempotencyIntentDrift,
                stage: BankCommitDenialStage::Idempotency,
            }))
        }
    }
}
