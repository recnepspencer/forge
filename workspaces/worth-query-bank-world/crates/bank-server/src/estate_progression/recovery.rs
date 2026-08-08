//! Production recovery-handle assembly for estate commits (Gate 8.3 / R8.31).
//!
//! Mint and transitions go through Query admission — the same path test worlds
//! must use (§10.4). Callers cannot assert current capability or disclosure.

use bank_domain::estate::{EstateAction, EstateDisbursement};
use bank_domain::proposals::BankIdempotencyClaim;
use bank_domain::schema::{
    NotifyDeathEstateCapability, NotifyDeathEstateOperation, ReversalReason, ReverseJournal,
};
use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
use worth_query_host::facade::primary_graph::{
    compensate_recovery_handle, dispose_recovery_handle, inspect_recovery_handle,
    reconcile_recovery_handle, resolve_recovery_handle, safe_retry_recovery_handle,
    WorthQueryApplicationIdempotencyResolution, WorthQueryRecoveryCompensateAdmission,
    WorthQueryRecoveryDisposalReceipt, WorthQueryRecoveryEffectAuthority, WorthQueryRecoveryHandle,
    WorthQueryRecoveryHandleDenial, WorthQueryRecoveryHandleDenialKind,
    WorthQueryRecoveryInspectAuthority, WorthQueryRecoveryInspectionView,
    WorthQueryRecoveryReconcileAdmission, WorthQueryRecoverySafeRetryAdmission,
};
use worth_query_host::facade::provisional_aftermath::{
    WorthQueryUndoAdmission, WorthQueryUndoDenial,
};

use super::{
    BankCompensationUndoAdmission, BankEstateProgressionDenial, BankRecordedInverseUndoAdmission,
};
use crate::bank_projection::project_estate_disbursement;
use crate::{BankAuthenticatedPrincipal, BankCommitReceipt, BankIdentityRuntime};

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
    /// Mint a recovery handle from a committed receipt through production mint.
    pub fn open_commit_recovery(
        &self,
        receipt: &BankCommitReceipt,
    ) -> Result<WorthQueryRecoveryHandle, WorthQueryRecoveryHandleDenial> {
        self.application_runtime()
            .mint_recovery_handle(receipt.application())
    }

    /// Re-admit notify-death and establish effect authority against current truth.
    pub fn admit_commit_recovery_effect(
        &self,
        handle: &WorthQueryRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryRecoveryEffectAuthority, BankEstateProgressionDenial> {
        let admission = self.admit_notification_operation(principal, action, request)?;
        self.application_runtime()
            .admit_recovery_effect_authority(handle, &admission)
            .map_err(BankEstateProgressionDenial::Recovery)
    }

    /// Admit disclosure + inspect authority for recovery inspection.
    pub fn admit_commit_recovery_inspect(
        &self,
        handle: &WorthQueryRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryRecoveryInspectAuthority, BankEstateProgressionDenial> {
        let admission = self.admit_notification_operation(principal, action, request)?;
        let capability = self
            .application_runtime()
            .installed_schema()
            .capability(
                NotifyDeathEstateCapability::reference(),
                NotifyDeathEstateOperation::reference(),
            )
            .map_err(BankEstateProgressionDenial::CapabilityInstallation)?;
        let access = self
            .application_runtime()
            .admit_capability_access(principal.query(), &capability, action, request)
            .map_err(BankEstateProgressionDenial::Authorization)?;
        let disclosure = self
            .application_runtime()
            .admit_recovery_inspection_disclosure(&access)
            .map_err(BankEstateProgressionDenial::Recovery)?;
        self.application_runtime()
            .admit_recovery_inspect_authority(handle, &admission, &disclosure)
            .map_err(BankEstateProgressionDenial::Recovery)
    }

    /// Production dispose: re-admit, establish effect authority, dispose.
    pub fn dispose_commit_recovery(
        &self,
        handle: WorthQueryRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryRecoveryDisposalReceipt, BankEstateProgressionDenial> {
        let authority = self.admit_commit_recovery_effect(&handle, principal, action, request)?;
        dispose_recovery_handle(handle, &authority).map_err(BankEstateProgressionDenial::Recovery)
    }

    /// Production reconcile through current admission.
    pub fn reconcile_commit_recovery(
        &self,
        handle: WorthQueryRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryRecoveryReconcileAdmission, BankEstateProgressionDenial> {
        let authority = self.admit_commit_recovery_effect(&handle, principal, action, request)?;
        reconcile_recovery_handle(handle, &authority).map_err(BankEstateProgressionDenial::Recovery)
    }

    /// Production compensate through current admission.
    pub fn compensate_commit_recovery(
        &self,
        handle: WorthQueryRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryRecoveryCompensateAdmission, BankEstateProgressionDenial> {
        let authority = self.admit_commit_recovery_effect(&handle, principal, action, request)?;
        compensate_recovery_handle(handle, &authority)
            .map_err(BankEstateProgressionDenial::Recovery)
    }

    /// Production inspect through disclosure-backed inspect authority.
    pub fn inspect_commit_recovery(
        &self,
        handle: &WorthQueryRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryRecoveryInspectionView, BankEstateProgressionDenial> {
        let authority = self.admit_commit_recovery_inspect(handle, principal, action, request)?;
        inspect_recovery_handle(handle, &authority).map_err(BankEstateProgressionDenial::Recovery)
    }

    /// Evaluate recovery expiry from the runtime clock (R8.7 M2/M3).
    pub fn evaluate_commit_recovery_expiry(
        &self,
        handle: &WorthQueryRecoveryHandle,
    ) -> Result<
        worth_query_host::facade::primary_graph::WorthQueryRecoveryExpiryEvaluation,
        WorthQueryRecoveryHandleDenial,
    > {
        self.application_runtime().evaluate_recovery_expiry(handle)
    }

    /// Fresh undo admission through current authority (R8.36 / R8.37).
    ///
    /// Re-admits capability/operation first — the receipt authorizes nothing about
    /// the current world. Then derives the undo request from installed axes.
    pub fn admit_undo_commit_recovery(
        &self,
        handle: WorthQueryRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryUndoAdmission, BankEstateProgressionDenial> {
        let action = original_estate_action(&handle)?;
        let authority = self
            .admit_commit_recovery_effect(&handle, principal, action, request)
            .map_err(map_undo_path_recovery_denial)?;
        self.application_runtime()
            .admit_undo(handle, &authority)
            .map_err(BankEstateProgressionDenial::Undo)
    }

    /// Fresh undo admission for a compensatable estate disbursement (R8.38).
    pub fn admit_undo_disbursement_recovery(
        &self,
        handle: WorthQueryRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        request: &WorthQueryRequestScope,
    ) -> Result<BankCompensationUndoAdmission, BankEstateProgressionDenial> {
        let action = original_estate_action(&handle)?;
        let input = disbursement_input(action)?;
        let original_idempotency = handle.binding().idempotency();
        let admission = self.admit_estate_disbursement(principal, action, request)?;
        let projected = self
            .invariant_projection()
            .project_admitted_operation(&admission, |reader, estate| {
                project_estate_disbursement(reader, estate, &input)
            })
            .map_err(BankEstateProgressionDenial::Projection)?;
        let (decision, _projection, _) = projected.into_parts();
        let decision =
            decision.map_err(BankEstateProgressionDenial::EstateDisbursementProjection)?;
        let institution = decision
            .into_parts()
            .0
            .snapshot()
            .account(input.source_account)
            .ok_or(BankEstateProgressionDenial::CommandInput(
                "retained disbursement source account",
            ))?
            .institution();
        let claim = BankIdempotencyClaim::from_application_binding(
            *original_idempotency.key_identity(),
            *original_idempotency.intent_identity(),
        );
        let reverse_journal = ReverseJournal {
            institution,
            journal: claim.journal_identity(0),
            reason: ReversalReason::OperatorCorrection,
        };
        let authority = self
            .application_runtime()
            .admit_recovery_effect_authority(&handle, &admission)
            .map_err(|denial| {
                map_undo_path_recovery_denial(BankEstateProgressionDenial::Recovery(denial))
            })?;
        let query = self
            .application_runtime()
            .admit_undo(handle, &authority)
            .map_err(BankEstateProgressionDenial::Undo)?;
        Ok(BankCompensationUndoAdmission::new(query, reverse_journal))
    }

    /// Fresh undo admission for a RecordedInverse estate freeze (R8.2 / R8.36).
    pub fn admit_undo_freeze_recovery(
        &self,
        handle: WorthQueryRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        request: &WorthQueryRequestScope,
    ) -> Result<BankRecordedInverseUndoAdmission, BankEstateProgressionDenial> {
        let action = original_estate_action(&handle)?;
        let admission = self.admit_freeze_operation(principal, action, request)?;
        let authority = self
            .application_runtime()
            .admit_recovery_effect_authority(&handle, &admission)
            .map_err(|denial| {
                map_undo_path_recovery_denial(BankEstateProgressionDenial::Recovery(denial))
            })?;
        let query = self
            .application_runtime()
            .admit_undo(handle, &authority)
            .map_err(BankEstateProgressionDenial::Undo)?;
        Ok(BankRecordedInverseUndoAdmission::new(query))
    }

    /// Resolve through an admitted graph idempotency read (R8.32).
    pub fn resolve_commit_recovery(
        &self,
        handle: WorthQueryRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryApplicationIdempotencyResolution, BankEstateProgressionDenial> {
        let admission = self.admit_notification_operation(principal, action, request)?;
        let authority = self
            .application_runtime()
            .admit_recovery_effect_authority(&handle, &admission)
            .map_err(BankEstateProgressionDenial::Recovery)?;
        let resolution = self
            .application_runtime()
            .resolve_admitted_application_idempotency(&admission, handle.binding().idempotency())
            .map_err(BankEstateProgressionDenial::Idempotency)?;
        resolve_recovery_handle(handle, &authority, resolution)
            .map_err(BankEstateProgressionDenial::Recovery)
    }

    /// Safe-retry through fresh authority then runtime re-dispatch (R8.66–R8.69).
    ///
    /// Authority is established before any transport call. Re-dispatch requires
    /// the live handle and effect authority, reads the outbox from the binding,
    /// and remains the sole classification site beside post-commit dispatch.
    pub fn safe_retry_commit_recovery(
        &self,
        handle: WorthQueryRecoveryHandle,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryRecoverySafeRetryAdmission, BankEstateProgressionDenial> {
        let admission = self.admit_notification_operation(principal, action, request)?;
        let authority = self
            .application_runtime()
            .admit_recovery_effect_authority(&handle, &admission)
            .map_err(BankEstateProgressionDenial::Recovery)?;
        let redispatch = self
            .application_runtime()
            .redispatch_admitted_external_effect(&handle, &authority, &admission)
            .map_err(|denial| BankEstateProgressionDenial::Recovery(denial.into()))?;
        safe_retry_recovery_handle(handle, &authority, redispatch)
            .map_err(BankEstateProgressionDenial::Recovery)
    }
}

/// On the undo entry, expiry and terminal handles are R8.39 undo causes — not
/// generic recovery denials. Other recovery denials stay Recovery-shaped.
fn map_undo_path_recovery_denial(
    denial: BankEstateProgressionDenial,
) -> BankEstateProgressionDenial {
    match denial {
        BankEstateProgressionDenial::Recovery(inner) => match inner.kind() {
            WorthQueryRecoveryHandleDenialKind::Expired => {
                BankEstateProgressionDenial::Undo(WorthQueryUndoDenial::stale())
            }
            WorthQueryRecoveryHandleDenialKind::AlreadyTerminal => {
                BankEstateProgressionDenial::Undo(WorthQueryUndoDenial::already_consumed())
            }
            _ => BankEstateProgressionDenial::Recovery(inner),
        },
        other => other,
    }
}
