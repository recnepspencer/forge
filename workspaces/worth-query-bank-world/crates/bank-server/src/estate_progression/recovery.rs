//! Production recovery-handle assembly for estate commits (Gate 8.3 / R8.31).
//!
//! Mint and transitions go through Query admission — the same path test worlds
//! must use (§10.4). Callers cannot assert current capability or disclosure.

use bank_domain::estate::{EstateAction, EstateDisbursement};
use bank_domain::proposals::BankIdempotencyClaim;
use bank_domain::schema::{ReversalReason, ReverseJournal};
use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
use worth_query_host::facade::primary_graph::{
    resolve_recovery_handle, safe_retry_recovery_handle, WorthQueryRecoveryHandle,
};
use worth_query_host::facade::provisional_aftermath::WorthQueryUndoDenial;

use super::{
    recovery_types::map_idempotency, BankCommitRecoveryHandle, BankCompensationUndoAdmission,
    BankEstateProgressionDenial, BankRecordedInverseUndoAdmission, BankRecoveryDenialKind,
    BankRecoveryIdempotencyResolution, BankRecoverySafeRetryReceipt,
};
use crate::bank_projection::project_estate_disbursement;
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

mod handle_lifecycle;

fn original_estate_action(
    handle: &WorthQueryRecoveryHandle,
) -> Result<EstateAction, BankEstateProgressionDenial> {
    handle
        .binding()
        .original_input::<EstateAction>()
        .copied()
        .ok_or(BankEstateProgressionDenial::CommandInput(
            "retained estate operation input",
        ))
}

fn disbursement_input(
    action: EstateAction,
) -> Result<EstateDisbursement, BankEstateProgressionDenial> {
    match action {
        EstateAction::DisburseEstate(input) => Ok(input),
        _ => Err(BankEstateProgressionDenial::CommandInput(
            "retained DisburseEstateOperation input",
        )),
    }
}

impl BankIdentityRuntime {
    /// Fresh undo admission through current authority (R8.36 / R8.37).
    ///
    /// Re-admits capability/operation first — the receipt authorizes nothing about
    /// the current world. Then derives the undo request from installed axes.
    pub fn admit_undo_commit_recovery(
        &self,
        handle: BankCommitRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        request: &WorthQueryRequestScope,
    ) -> Result<BankRecordedInverseUndoAdmission, BankEstateProgressionDenial> {
        self.admit_undo_commit_recovery_retaining(handle, principal, request)
            .map_err(super::BankEstateProgressionFailure::into_denial)
    }

    /// Fresh undo admission that returns the same recovery handle when no
    /// Query correction preparation has consumed it.
    pub fn admit_undo_commit_recovery_retaining(
        &self,
        handle: BankCommitRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankRecordedInverseUndoAdmission,
        super::BankEstateProgressionFailure<BankCommitRecoveryHandle>,
    > {
        let action = match original_estate_action(handle.query()) {
            Ok(action) => action,
            Err(denial) => {
                return Err(super::BankEstateProgressionFailure::retained(
                    denial, handle,
                ));
            }
        };
        let authority = match self
            .admit_commit_recovery_effect(handle.query(), principal, action, request)
            .map_err(map_undo_path_recovery_denial)
        {
            Ok(authority) => authority,
            Err(denial) => {
                return Err(super::BankEstateProgressionFailure::retained(
                    denial, handle,
                ));
            }
        };
        self.application_runtime()
            .admit_undo(handle.query, &authority)
            .map(BankRecordedInverseUndoAdmission::new)
            .map_err(BankEstateProgressionDenial::Undo)
            .map_err(super::BankEstateProgressionFailure::consumed)
    }

    /// Fresh undo admission for a compensatable estate disbursement (R8.38).
    pub fn admit_undo_disbursement_recovery(
        &self,
        handle: BankCommitRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        request: &WorthQueryRequestScope,
    ) -> Result<BankCompensationUndoAdmission, BankEstateProgressionDenial> {
        self.admit_undo_disbursement_recovery_retaining(handle, principal, request)
            .map_err(super::BankEstateProgressionFailure::into_denial)
    }

    /// Compensation admission preserving the exact handle on any safe
    /// pre-Query denial.
    pub fn admit_undo_disbursement_recovery_retaining(
        &self,
        handle: BankCommitRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankCompensationUndoAdmission,
        super::BankEstateProgressionFailure<BankCommitRecoveryHandle>,
    > {
        let action = match original_estate_action(handle.query()) {
            Ok(action) => action,
            Err(denial) => {
                return Err(super::BankEstateProgressionFailure::retained(
                    denial, handle,
                ));
            }
        };
        let input = match disbursement_input(action) {
            Ok(input) => input,
            Err(denial) => {
                return Err(super::BankEstateProgressionFailure::retained(
                    denial, handle,
                ));
            }
        };
        let original_idempotency = handle.query().binding().idempotency();
        let admission = match self.admit_estate_disbursement(principal, action, request) {
            Ok(admission) => admission,
            Err(denial) => {
                return Err(super::BankEstateProgressionFailure::retained(
                    denial, handle,
                ));
            }
        };
        let projected = match self
            .invariant_projection()
            .project_admitted_operation(&admission, |reader, estate| {
                project_estate_disbursement(reader, estate, &input)
            })
            .map_err(BankEstateProgressionDenial::from_projection)
        {
            Ok(projected) => projected,
            Err(denial) => {
                return Err(super::BankEstateProgressionFailure::retained(
                    denial, handle,
                ));
            }
        };
        let (decision, _projection, _) = projected.into_parts();
        let decision =
            match decision.map_err(BankEstateProgressionDenial::EstateDisbursementProjection) {
                Ok(decision) => decision,
                Err(denial) => {
                    return Err(super::BankEstateProgressionFailure::retained(
                        denial, handle,
                    ));
                }
            };
        let institution = match decision
            .into_parts()
            .0
            .snapshot()
            .account(input.source_account)
            .ok_or(BankEstateProgressionDenial::CommandInput(
                "retained disbursement source account",
            )) {
            Ok(account) => account.institution(),
            Err(denial) => {
                return Err(super::BankEstateProgressionFailure::retained(
                    denial, handle,
                ));
            }
        };
        let claim = BankIdempotencyClaim::from_application_binding(
            *original_idempotency.key_identity(),
            *original_idempotency.intent_identity(),
        );
        let reverse_journal = ReverseJournal {
            institution,
            journal: claim.journal_identity(0),
            reason: ReversalReason::OperatorCorrection,
        };
        let authority = match self
            .application_runtime()
            .admit_recovery_effect_authority(handle.query(), &admission)
            .map_err(|denial| {
                map_undo_path_recovery_denial(BankEstateProgressionDenial::from_recovery(denial))
            }) {
            Ok(authority) => authority,
            Err(denial) => {
                return Err(super::BankEstateProgressionFailure::retained(
                    denial, handle,
                ));
            }
        };
        let query = self
            .application_runtime()
            .admit_undo(handle.query, &authority)
            .map_err(BankEstateProgressionDenial::Undo)
            .map_err(super::BankEstateProgressionFailure::consumed)?;
        Ok(BankCompensationUndoAdmission::new(query, reverse_journal))
    }

    /// Fresh undo admission for a RecordedInverse estate freeze (R8.2 / R8.36).
    pub fn admit_undo_freeze_recovery(
        &self,
        handle: BankCommitRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        request: &WorthQueryRequestScope,
    ) -> Result<BankRecordedInverseUndoAdmission, BankEstateProgressionDenial> {
        let action = original_estate_action(handle.query())?;
        let admission = self.admit_freeze_operation(principal, action, request)?;
        let authority = self
            .application_runtime()
            .admit_recovery_effect_authority(handle.query(), &admission)
            .map_err(|denial| {
                map_undo_path_recovery_denial(BankEstateProgressionDenial::from_recovery(denial))
            })?;
        let query = self
            .application_runtime()
            .admit_undo(handle.query, &authority)
            .map_err(BankEstateProgressionDenial::Undo)?;
        Ok(BankRecordedInverseUndoAdmission::new(query))
    }

    /// Resolve through an admitted graph idempotency read (R8.32).
    pub fn resolve_commit_recovery(
        &self,
        handle: BankCommitRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        request: &WorthQueryRequestScope,
    ) -> Result<BankRecoveryIdempotencyResolution, BankEstateProgressionDenial> {
        let admission = self.admit_notification_operation(principal, action, request)?;
        let authority = self
            .application_runtime()
            .admit_recovery_effect_authority(handle.query(), &admission)
            .map_err(BankEstateProgressionDenial::from_recovery)?;
        let resolution = self
            .application_runtime()
            .resolve_admitted_application_idempotency(
                &admission,
                handle.query().binding().idempotency(),
            )
            .map_err(BankEstateProgressionDenial::from_idempotency)?;
        resolve_recovery_handle(handle.query, &authority, resolution)
            .map(map_idempotency)
            .map_err(BankEstateProgressionDenial::from_recovery)
    }

    /// Safe-retry through fresh authority then runtime re-dispatch (R8.66–R8.69).
    ///
    /// Authority is established before any transport call. Re-dispatch requires
    /// the live handle and effect authority, reads the outbox from the binding,
    /// and remains the sole classification site beside post-commit dispatch.
    pub fn safe_retry_commit_recovery(
        &self,
        handle: BankCommitRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        request: &WorthQueryRequestScope,
    ) -> Result<BankRecoverySafeRetryReceipt, BankEstateProgressionDenial> {
        let admission = self.admit_notification_operation(principal, action, request)?;
        let authority = self
            .application_runtime()
            .admit_recovery_effect_authority(handle.query(), &admission)
            .map_err(BankEstateProgressionDenial::from_recovery)?;
        let redispatch = self
            .application_runtime()
            .redispatch_admitted_external_effect(handle.query(), &authority, &admission)
            .map_err(|denial| {
                let denial: worth_query_host::facade::primary_graph::WorthQueryRecoveryHandleDenial =
                    denial.into();
                BankEstateProgressionDenial::from_recovery(denial)
            })?;
        safe_retry_recovery_handle(handle.query, &authority, redispatch)
            .map(BankRecoverySafeRetryReceipt::from_query)
            .map_err(BankEstateProgressionDenial::from_recovery)
    }
}

/// On the undo entry, expiry and terminal handles are R8.39 undo causes — not
/// generic recovery denials. Other recovery denials stay Recovery-shaped.
fn map_undo_path_recovery_denial(
    denial: BankEstateProgressionDenial,
) -> BankEstateProgressionDenial {
    match denial {
        BankEstateProgressionDenial::Recovery(inner) => match inner.kind() {
            BankRecoveryDenialKind::Expired => {
                BankEstateProgressionDenial::Undo(WorthQueryUndoDenial::stale())
            }
            BankRecoveryDenialKind::AlreadyTerminal => {
                BankEstateProgressionDenial::Undo(WorthQueryUndoDenial::already_consumed())
            }
            _ => BankEstateProgressionDenial::Recovery(inner),
        },
        other => other,
    }
}
