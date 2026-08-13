//! Production undo progression through ordinary mutation entry (R8.37 / A11).
//!
//! Admission derives the request. Compensation re-enters reverse-journal.
//! RecordedInverse restores retained pre-image through the same
//! compare-and-commit lane — never a parallel mutator and never a live re-read.

use bank_domain::estate::EstateAction;
use bank_domain::model::AccountId;
use bank_domain::proposals::BankIdempotencyKey;
use bank_domain::schema::{
    AccountIdentity, AccountStatus, BankSchema, EstateAccount, EstateCase,
    FreezeEstateAccountOperation, Status,
};
use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
use worth_query_host::facade::declaration::application_schema::TypedApplicationReadableValue;
use worth_query_host::facade::primary_graph::{
    map_ordinary_commit_conflict, WorthQueryAdmittedApplicationOperation,
    WorthQueryApplicationEffectProgram, WorthQueryApplicationIdempotencyBinding,
    WorthQueryApplicationOperationInvariantProjectionReader, WorthQueryInvariantEntityIdentity,
};
use worth_query_host::facade::provisional_aftermath::{
    consume_unresolved_undo_progression, progress_admitted_undo, WorthQueryRedoRecovery,
    WorthQueryRetainedPreImage, WorthQueryUndoDenial, WorthQueryUndoDerivedRequest,
    WorthQueryUndoProgressionHandoff,
};

use super::{
    BankCompensationUndoAdmission, BankEstateFreezeProjectionDenial, BankEstateProgressionDenial,
    BankRecordedInverseUndoAdmission,
};
use crate::{
    BankAuthenticatedPrincipal, BankIdentityRuntime, BankMutationCommitOutcome,
    BankOperationProposals,
};

pub use compensation_input::compensating_reverse_journal;
use denial::{map_admission, map_proposal};
use retry::{compensation_retry, recorded_inverse_retry};

type AdmittedFreezeOperation = WorthQueryAdmittedApplicationOperation<
    BankSchema,
    FreezeEstateAccountOperation,
    EstateAction,
    EstateCase,
>;
type FreezeEffectProgram = WorthQueryApplicationEffectProgram<
    BankSchema,
    FreezeEstateAccountOperation,
    EstateAction,
    EstateCase,
>;

mod compensation_input;
mod denial;
mod reconciliation;
mod retry;

/// Ordinary undo outcome plus causal evidence when an undo actually committed.
///
/// The proved value is sealed from the consumed progression handoff and the
/// ordinary commit receipt. Callers cannot reconstruct it from receipt fields.
#[derive(Debug)]
pub struct BankUndoCommitOutcome {
    mutation: BankMutationCommitOutcome,
    redo_recovery: Option<BankRedoRecovery>,
    retry: Option<BankUndoRetry>,
}

#[derive(Debug)]
pub struct BankRedoRecovery {
    pub(super) query: WorthQueryRedoRecovery,
}

#[derive(Debug)]
pub enum BankUndoRetry {
    Compensation(BankCompensationUndoAdmission),
    RecordedInverse(BankRecordedInverseUndoAdmission),
}

impl BankUndoCommitOutcome {
    pub const fn mutation(&self) -> &BankMutationCommitOutcome {
        &self.mutation
    }

    pub const fn has_proved_undo(&self) -> bool {
        self.redo_recovery.is_some()
    }

    pub const fn retry(&self) -> Option<&BankUndoRetry> {
        self.retry.as_ref()
    }

    pub fn into_parts(
        self,
    ) -> (
        BankMutationCommitOutcome,
        Option<BankRedoRecovery>,
        Option<BankUndoRetry>,
    ) {
        (self.mutation, self.redo_recovery, self.retry)
    }
}

impl BankIdentityRuntime {
    /// Hand an admitted compensation undo into ordinary reverse-journal progression.
    pub fn progress_undo_commit_recovery(
        &self,
        admission: BankCompensationUndoAdmission,
        principal: &BankAuthenticatedPrincipal,
        idempotency_key: &BankIdempotencyKey,
        request: &WorthQueryRequestScope,
    ) -> Result<BankUndoCommitOutcome, super::BankEstateProgressionFailure<BankUndoRetry>> {
        let (query, reverse_journal) = admission.into_parts();
        let retry_journal = reverse_journal.clone();
        let handoff = progress_admitted_undo(query)
            .map_err(BankEstateProgressionDenial::Undo)
            .map_err(super::BankEstateProgressionFailure::consumed)?;
        match handoff.derived_request() {
            WorthQueryUndoDerivedRequest::Compensation => {}
            // Routing RecordedInverse through the compensation journal lane is
            // visibly wrong (T4b / R8.38 divergence).
            WorthQueryUndoDerivedRequest::RecordedInverse
            | WorthQueryUndoDerivedRequest::Reconciliation => {
                return Err(super::BankEstateProgressionFailure::retained(
                    BankEstateProgressionDenial::Undo(
                        WorthQueryUndoDenial::correction_not_admitted(),
                    ),
                    compensation_retry(handoff, retry_journal),
                ));
            }
        }
        let authorized = match self.authorize_reverse_journal(
            principal,
            reverse_journal.institution,
            Default::default(),
            request,
        ) {
            Ok(authorized) => authorized,
            Err(denial) => {
                return Err(super::BankEstateProgressionFailure::retained(
                    map_admission(denial),
                    compensation_retry(handoff, retry_journal),
                ));
            }
        };
        let proposal = match BankOperationProposals::prepare_reverse_journal(
            self,
            authorized,
            idempotency_key,
            &reverse_journal,
        ) {
            Ok(proposal) => proposal,
            Err(denial) => {
                return Err(super::BankEstateProgressionFailure::retained(
                    map_proposal(denial),
                    compensation_retry(handoff, retry_journal),
                ));
            }
        };
        let (program, idempotency) = match self.materialize_reverse_journal(proposal) {
            Ok(materialized) => materialized,
            Err(_) => {
                return Err(super::BankEstateProgressionFailure::retained(
                    BankEstateProgressionDenial::Undo(map_ordinary_commit_conflict()),
                    compensation_retry(handoff, retry_journal),
                ));
            }
        };
        let compensated =
            self.commit_materialized_reverse_journal_as_undo(program, idempotency, &handoff);
        self.finish_undo_progression(handoff, compensated, |handoff| {
            compensation_retry(handoff, retry_journal)
        })
    }

    /// Progress RecordedInverse undo by restoring retained prior status (R8.2).
    pub fn progress_undo_recorded_inverse(
        &self,
        admission: BankRecordedInverseUndoAdmission,
        principal: &BankAuthenticatedPrincipal,
        idempotency: WorthQueryApplicationIdempotencyBinding,
        request: &WorthQueryRequestScope,
    ) -> Result<BankUndoCommitOutcome, super::BankEstateProgressionFailure<BankUndoRetry>> {
        let handoff = progress_admitted_undo(admission.query)
            .map_err(BankEstateProgressionDenial::Undo)
            .map_err(super::BankEstateProgressionFailure::consumed)?;
        if handoff.derived_request() != WorthQueryUndoDerivedRequest::RecordedInverse {
            return Err(super::BankEstateProgressionFailure::retained(
                BankEstateProgressionDenial::Undo(WorthQueryUndoDenial::correction_not_admitted()),
                recorded_inverse_retry(handoff),
            ));
        }
        let Some(preimage) = handoff.retained_preimage().cloned() else {
            return Err(super::BankEstateProgressionFailure::retained(
                BankEstateProgressionDenial::Undo(
                    WorthQueryUndoDenial::retained_preimage_required(),
                ),
                recorded_inverse_retry(handoff),
            ));
        };
        let prior = match prior_status_from_preimage(&preimage) {
            Ok(prior) => prior,
            Err(denial) => {
                return Err(super::BankEstateProgressionFailure::retained(
                    denial,
                    recorded_inverse_retry(handoff),
                ));
            }
        };
        let action = match handoff.admission().original_input::<EstateAction>() {
            Some(action) => *action,
            None => {
                return Err(super::BankEstateProgressionFailure::retained(
                    BankEstateProgressionDenial::CommandInput(
                        "retained FreezeEstateAccountOperation input",
                    ),
                    recorded_inverse_retry(handoff),
                ));
            }
        };
        let account = match freeze_command_account(action) {
            Ok(account) => account,
            Err(denial) => {
                return Err(super::BankEstateProgressionFailure::retained(
                    denial,
                    recorded_inverse_retry(handoff),
                ));
            }
        };
        let admitted = match self.admit_freeze_operation(principal, action, request) {
            Ok(admitted) => admitted,
            Err(denial) => {
                return Err(super::BankEstateProgressionFailure::retained(
                    denial,
                    recorded_inverse_retry(handoff),
                ));
            }
        };
        let program = match self.materialize_inverse_restore(admitted, account, prior, &preimage) {
            Ok(program) => program,
            Err(denial) => {
                return Err(super::BankEstateProgressionFailure::retained(
                    denial,
                    recorded_inverse_retry(handoff),
                ));
            }
        };
        let outcome = self
            .application_runtime()
            .compare_and_commit_undo_application(program, idempotency, &handoff)
            .into();
        self.finish_undo_progression(handoff, outcome, recorded_inverse_retry)
    }

    fn finish_undo_progression<Retry>(
        &self,
        handoff: WorthQueryUndoProgressionHandoff,
        mutation: BankMutationCommitOutcome,
        retry: Retry,
    ) -> Result<BankUndoCommitOutcome, super::BankEstateProgressionFailure<BankUndoRetry>>
    where
        Retry: FnOnce(WorthQueryUndoProgressionHandoff) -> BankUndoRetry,
    {
        let (redo_recovery, retry) = match &mutation {
            BankMutationCommitOutcome::Committed(receipt)
            | BankMutationCommitOutcome::AlreadyCommitted(receipt) => (
                Some(
                    receipt
                        .recovery_evidence()
                        .seal_redo_recovery(handoff)
                        .map_err(|_| {
                            BankEstateProgressionDenial::Undo(WorthQueryUndoDenial::stale())
                        })
                        .map_err(super::BankEstateProgressionFailure::consumed)?,
                ),
                None,
            ),
            BankMutationCommitOutcome::PartialEffect(_)
            | BankMutationCommitOutcome::Indeterminate(_) => {
                consume_unresolved_undo_progression(handoff)
                    .map_err(BankEstateProgressionDenial::Undo)
                    .map_err(super::BankEstateProgressionFailure::consumed)?;
                (None, None)
            }
            BankMutationCommitOutcome::Stale { .. }
            | BankMutationCommitOutcome::Cancelled
            | BankMutationCommitOutcome::Denied { .. }
            | BankMutationCommitOutcome::Aborted => (None, Some(retry(handoff))),
        };
        Ok(BankUndoCommitOutcome {
            mutation,
            redo_recovery: redo_recovery.map(|query| BankRedoRecovery { query }),
            retry,
        })
    }

    fn materialize_inverse_restore(
        &self,
        admission: AdmittedFreezeOperation,
        account: AccountId,
        prior: AccountStatus,
        preimage: &WorthQueryRetainedPreImage,
    ) -> Result<FreezeEffectProgram, BankEstateProgressionDenial> {
        let projected = self
            .invariant_projection()
            .project_admitted_operation(&admission, |reader, estate| {
                project_inverse_restore_account(reader, estate, account)
            })
            .map_err(BankEstateProgressionDenial::from_projection)?;
        let (projection_result, projection, _) = projected.into_parts();
        projection_result.map_err(BankEstateProgressionDenial::FreezeProjection)?;
        let reads = self
            .application_runtime()
            .begin_projected_application_read_attempt(admission, projection)
            .map_err(BankEstateProgressionDenial::from_attempt)?;
        let account = reads
            .resolve_entity(AccountIdentity::reference(), account)
            .map_err(BankEstateProgressionDenial::from_attempt)?;
        if !preimage
            .target_record()
            .is_some_and(|target| account.matches_record(target))
        {
            return Err(BankEstateProgressionDenial::Undo(
                WorthQueryUndoDenial::touched_records_required(),
            ));
        }
        let mut effects = reads
            .complete_projected_dependencies()
            .map_err(BankEstateProgressionDenial::from_attempt)?
            .begin_effect_program();
        let account = effects
            .existing_entity(&account)
            .map_err(BankEstateProgressionDenial::from_attempt)?;
        // R8.2 — restore uses the retained pre-image value, not a live re-read.
        effects
            .write_field(&account, Status::reference(), prior)
            .map_err(BankEstateProgressionDenial::from_attempt)?;
        effects
            .finish()
            .map_err(BankEstateProgressionDenial::from_attempt)
    }
}

fn prior_status_from_preimage(
    preimage: &WorthQueryRetainedPreImage,
) -> Result<AccountStatus, BankEstateProgressionDenial> {
    let Some(field) = preimage.field_for(Status::reference()) else {
        return Err(BankEstateProgressionDenial::Undo(
            WorthQueryUndoDenial::retained_preimage_required(),
        ));
    };
    AccountStatus::from_foundational_value(field.value()).ok_or_else(|| {
        BankEstateProgressionDenial::Undo(WorthQueryUndoDenial::retained_preimage_required())
    })
}

fn freeze_command_account(action: EstateAction) -> Result<AccountId, BankEstateProgressionDenial> {
    match action {
        EstateAction::FreezeAccount { account, .. } => Ok(account),
        _ => Err(BankEstateProgressionDenial::CommandInput(
            "FreezeEstateAccountOperation",
        )),
    }
}

fn project_inverse_restore_account(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        BankSchema,
        FreezeEstateAccountOperation,
    >,
    estate: &WorthQueryInvariantEntityIdentity<BankSchema, EstateCase>,
    expected_account: AccountId,
) -> Result<(), BankEstateFreezeProjectionDenial> {
    let relations = reader.decision_relations_from(EstateAccount::reference(), estate)?;
    let [relation] = relations.as_slice() else {
        return Err(BankEstateFreezeProjectionDenial::RelationCardinality {
            expected: 1,
            observed: relations.len(),
        });
    };
    let account = relation.to().clone();
    let observed_account = reader
        .decision_field(&account, AccountIdentity::reference())?
        .ok_or(BankEstateFreezeProjectionDenial::MissingAccountIdentity)?;
    if observed_account != expected_account {
        return Err(BankEstateFreezeProjectionDenial::RelatedAccountMismatch);
    }
    let status = reader
        .decision_field(&account, Status::reference())?
        .ok_or(BankEstateFreezeProjectionDenial::MissingAccountStatus)?;
    // Inverse restore requires a frozen account; reuse AccountNotOpen for wrong posture.
    if status != AccountStatus::Frozen {
        return Err(BankEstateFreezeProjectionDenial::AccountNotOpen);
    }
    Ok(())
}
