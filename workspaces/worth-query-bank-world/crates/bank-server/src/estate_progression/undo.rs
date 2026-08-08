//! Production undo progression through ordinary mutation entry (R8.37 / A11).
//!
//! Admission derives the request. Compensation re-enters reverse-journal.
//! RecordedInverse restores retained pre-image through the same
//! compare-and-commit lane — never a parallel mutator and never a live re-read.

use bank_domain::estate::EstateAction;
use bank_domain::model::{AccountId, InstitutionId};
use bank_domain::proposals::BankIdempotencyKey;
use bank_domain::schema::{
    AccountIdentity, AccountStatus, BankSchema, EstateAccount, EstateCase,
    FreezeEstateAccountOperation, ReversalReason, ReverseJournal, Status,
};
use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
use worth_query_host::facade::declaration::application_schema::TypedApplicationReadableValue;
use worth_query_host::facade::primary_graph::{
    map_ordinary_commit_conflict, WorthQueryAdmittedApplicationOperation,
    WorthQueryApplicationEffectProgram, WorthQueryApplicationIdempotencyBinding,
    WorthQueryApplicationOperationInvariantProjectionReader, WorthQueryInvariantEntityIdentity,
};
use worth_query_host::facade::provisional_aftermath::{
    progress_admitted_undo, WorthQueryProvedUndo, WorthQueryRedoRecovery,
    WorthQueryRetainedPreImage, WorthQueryUndoDenial, WorthQueryUndoDerivedRequest,
    WorthQueryUndoProgressionHandoff,
};

use super::{
    BankCompensationUndoAdmission, BankEstateFreezeProjectionDenial, BankEstateProgressionDenial,
    BankRecordedInverseUndoAdmission,
};
use crate::operation_admission::BankOperationAdmissionError;
use crate::operation_proposals::BankOperationProposalError;
use crate::{
    BankAuthenticatedPrincipal, BankIdentityRuntime, BankMutationCommitOutcome,
    BankOperationProposals,
};

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

/// Ordinary undo outcome plus causal evidence when an undo actually committed.
///
/// The proved value is sealed from the consumed progression handoff and the
/// ordinary commit receipt. Callers cannot reconstruct it from receipt fields.
#[derive(Debug)]
pub struct BankUndoCommitOutcome {
    mutation: BankMutationCommitOutcome,
    redo_recovery: Option<WorthQueryRedoRecovery>,
}

impl BankUndoCommitOutcome {
    pub const fn mutation(&self) -> &BankMutationCommitOutcome {
        &self.mutation
    }

    pub const fn proved_undo(&self) -> Option<&WorthQueryProvedUndo> {
        match self.redo_recovery.as_ref() {
            Some(recovery) => Some(recovery.proved()),
            None => None,
        }
    }

    pub fn into_parts(self) -> (BankMutationCommitOutcome, Option<WorthQueryRedoRecovery>) {
        (self.mutation, self.redo_recovery)
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
    ) -> Result<BankUndoCommitOutcome, BankEstateProgressionDenial> {
        let handoff =
            progress_admitted_undo(admission.query).map_err(BankEstateProgressionDenial::Undo)?;
        match handoff.derived_request() {
            WorthQueryUndoDerivedRequest::Compensation => {}
            // Routing RecordedInverse through the compensation journal lane is
            // visibly wrong (T4b / R8.38 divergence).
            WorthQueryUndoDerivedRequest::RecordedInverse
            | WorthQueryUndoDerivedRequest::Reconciliation => {
                return Err(BankEstateProgressionDenial::Undo(
                    WorthQueryUndoDenial::correction_not_admitted(),
                ));
            }
        }
        let compensated = self.progress_compensation_through_reverse_journal(
            principal,
            admission.reverse_journal,
            idempotency_key,
            request,
            &handoff,
        )?;
        self.finish_undo_progression(handoff, compensated)
    }

    /// Progress RecordedInverse undo by restoring retained prior status (R8.2).
    pub fn progress_undo_recorded_inverse(
        &self,
        admission: BankRecordedInverseUndoAdmission,
        principal: &BankAuthenticatedPrincipal,
        idempotency: WorthQueryApplicationIdempotencyBinding,
        request: &WorthQueryRequestScope,
    ) -> Result<BankUndoCommitOutcome, BankEstateProgressionDenial> {
        let handoff =
            progress_admitted_undo(admission.query).map_err(BankEstateProgressionDenial::Undo)?;
        if handoff.derived_request() != WorthQueryUndoDerivedRequest::RecordedInverse {
            return Err(BankEstateProgressionDenial::Undo(
                WorthQueryUndoDenial::correction_not_admitted(),
            ));
        }
        let Some(preimage) = handoff.retained_preimage().cloned() else {
            return Err(BankEstateProgressionDenial::Undo(
                WorthQueryUndoDenial::retained_preimage_required(),
            ));
        };
        let prior = prior_status_from_preimage(&preimage)?;
        let action = *handoff.admission().original_input::<EstateAction>().ok_or(
            BankEstateProgressionDenial::CommandInput(
                "retained FreezeEstateAccountOperation input",
            ),
        )?;
        let account = freeze_command_account(action)?;
        let admitted = self.admit_freeze_operation(principal, action, request)?;
        let program = self.materialize_inverse_restore(admitted, account, prior, &preimage)?;
        let outcome = self
            .application_runtime()
            .compare_and_commit_undo_application(program, idempotency, &handoff)
            .into();
        self.finish_undo_progression(handoff, outcome)
    }

    fn finish_undo_progression(
        &self,
        handoff: WorthQueryUndoProgressionHandoff,
        mutation: BankMutationCommitOutcome,
    ) -> Result<BankUndoCommitOutcome, BankEstateProgressionDenial> {
        let redo_recovery = match &mutation {
            BankMutationCommitOutcome::Committed(receipt)
            | BankMutationCommitOutcome::AlreadyCommitted(receipt) => Some(
                WorthQueryRedoRecovery::from_completed_undo(handoff, receipt.application())
                    .map_err(
                        |_| BankEstateProgressionDenial::Undo(WorthQueryUndoDenial::stale()),
                    )?,
            ),
            _ => None,
        };
        Ok(BankUndoCommitOutcome {
            mutation,
            redo_recovery,
        })
    }

    fn progress_compensation_through_reverse_journal(
        &self,
        principal: &BankAuthenticatedPrincipal,
        input: ReverseJournal,
        idempotency_key: &BankIdempotencyKey,
        request: &WorthQueryRequestScope,
        handoff: &WorthQueryUndoProgressionHandoff,
    ) -> Result<BankMutationCommitOutcome, BankEstateProgressionDenial> {
        let authorized = self
            .authorize_reverse_journal(principal, input.institution, Default::default(), request)
            .map_err(map_admission)?;
        let proposal = BankOperationProposals::prepare_reverse_journal(
            self,
            authorized,
            idempotency_key,
            &input,
        )
        .map_err(map_proposal)?;
        self.commit_reverse_journal_as_undo(proposal, handoff)
            .map_err(|_| BankEstateProgressionDenial::Undo(map_ordinary_commit_conflict()))
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
            .map_err(BankEstateProgressionDenial::Projection)?;
        let (projection_result, projection, _) = projected.into_parts();
        projection_result.map_err(BankEstateProgressionDenial::FreezeProjection)?;
        let reads = self
            .application_runtime()
            .begin_projected_application_read_attempt(admission, projection)
            .map_err(BankEstateProgressionDenial::Attempt)?;
        let account = reads
            .resolve_entity(AccountIdentity::reference(), account)
            .map_err(BankEstateProgressionDenial::Attempt)?;
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
            .map_err(BankEstateProgressionDenial::Attempt)?
            .begin_effect_program();
        let account = effects
            .existing_entity(&account)
            .map_err(BankEstateProgressionDenial::Attempt)?;
        // R8.2 — restore uses the retained pre-image value, not a live re-read.
        effects
            .write_field(&account, Status::reference(), prior)
            .map_err(BankEstateProgressionDenial::Attempt)?;
        effects
            .finish()
            .map_err(BankEstateProgressionDenial::Attempt)
    }
}

fn prior_status_from_preimage(
    preimage: &WorthQueryRetainedPreImage,
) -> Result<AccountStatus, BankEstateProgressionDenial> {
    let Some(field) = preimage.field("Status") else {
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
        return Err(BankEstateFreezeProjectionDenial::RelatedAccountMismatch {
            expected: expected_account,
            observed: observed_account,
        });
    }
    let status = reader
        .decision_field(&account, Status::reference())?
        .ok_or(BankEstateFreezeProjectionDenial::MissingAccountStatus)?;
    // Inverse restore requires a frozen account; reuse AccountNotOpen for wrong posture.
    if status != AccountStatus::Frozen {
        return Err(BankEstateFreezeProjectionDenial::AccountNotOpen(status));
    }
    Ok(())
}

fn map_admission(denial: BankOperationAdmissionError) -> BankEstateProgressionDenial {
    match denial {
        BankOperationAdmissionError::Authorization(denial) => {
            BankEstateProgressionDenial::Authorization(denial)
        }
        _ => BankEstateProgressionDenial::Undo(WorthQueryUndoDenial::current_policy_denied()),
    }
}

fn map_proposal(denial: BankOperationProposalError) -> BankEstateProgressionDenial {
    let _ = denial;
    BankEstateProgressionDenial::Undo(WorthQueryUndoDenial::conflicted())
}

/// Construct a compensating reverse-journal input for an original disbursement.
pub fn compensating_reverse_journal(
    institution: InstitutionId,
    original: bank_domain::model::JournalEntryId,
) -> ReverseJournal {
    ReverseJournal {
        institution,
        journal: original,
        reason: ReversalReason::OperatorCorrection,
    }
}
