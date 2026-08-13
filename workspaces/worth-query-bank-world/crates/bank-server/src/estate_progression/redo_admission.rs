//! Bank-owned typed continuation for one exact estate-disbursement redo.

use worth_query_host::facade::provisional_aftermath::WorthQueryRedoAdmission;

use super::disburse_estate::AdmittedEstateDisbursement;

/// Move-only redo continuation retaining its fresh admission context.
///
/// Progression has no caller-authored action or idempotency slot:
///
/// ```compile_fail,E0061
/// use bank_domain::estate::EstateAction;
/// use bank_server::{BankDisbursementRedoAdmission, BankIdentityRuntime};
/// use worth_query_host::facade::primary_graph::WorthQueryApplicationIdempotencyBinding;
/// fn substitute(
///     runtime: &BankIdentityRuntime,
///     admission: BankDisbursementRedoAdmission,
///     action: EstateAction,
///     idempotency: WorthQueryApplicationIdempotencyBinding,
/// ) {
///     let _ = runtime.progress_redo_disbursement(admission, action, idempotency);
/// }
/// ```
///
/// Admission likewise accepts no caller replacement for the original action:
///
/// ```compile_fail,E0061
/// use bank_domain::estate::EstateAction;
/// use bank_server::{BankAuthenticatedPrincipal, BankIdentityRuntime};
/// use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
/// use worth_query_host::facade::provisional_aftermath::{WorthQueryRedoIntent, WorthQueryRedoRecovery};
/// fn substitute(
///     runtime: &BankIdentityRuntime,
///     recovery: WorthQueryRedoRecovery,
///     principal: &BankAuthenticatedPrincipal,
///     replacement: EstateAction,
///     request: &WorthQueryRequestScope,
///     intent: &WorthQueryRedoIntent,
/// ) {
///     let _ = runtime.admit_redo_disbursement_recovery(
///         recovery, principal, replacement, request, intent,
///     );
/// }
/// ```
pub struct BankDisbursementRedoAdmission {
    query: WorthQueryRedoAdmission,
    ordinary: AdmittedEstateDisbursement,
}

impl std::fmt::Debug for BankDisbursementRedoAdmission {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("BankDisbursementRedoAdmission")
            .field("redo_admission_work", &self.query.redo_admission_work())
            .finish_non_exhaustive()
    }
}

impl BankDisbursementRedoAdmission {
    pub(crate) const fn new(
        query: WorthQueryRedoAdmission,
        ordinary: AdmittedEstateDisbursement,
    ) -> Self {
        Self { query, ordinary }
    }

    pub const fn redo_admission_work(
        &self,
    ) -> worth_query_host::facade::domain::WorthQueryCanonicalWorkEvidence {
        self.query.redo_admission_work()
    }

    pub(crate) fn into_parts(self) -> (WorthQueryRedoAdmission, AdmittedEstateDisbursement) {
        (self.query, self.ordinary)
    }
}
