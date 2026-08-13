//! Bank-owned typed undo continuations derived from Query admission.

use bank_domain::schema::ReverseJournal;
use worth_query_host::facade::provisional_aftermath::{
    WorthQueryRetainedPreImage, WorthQueryUndoAdmission, WorthQueryUndoDerivedRequest,
};

/// Bank-owned typed continuation for a compensation derived at undo admission.
///
/// A recorded-inverse continuation cannot enter this lane:
///
/// ```compile_fail
/// use bank_server::{BankAuthenticatedPrincipal, BankIdentityRuntime, BankRecordedInverseUndoAdmission};
/// use bank_domain::proposals::BankIdempotencyKey;
/// use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
/// fn substitute(
///     runtime: &BankIdentityRuntime,
///     inverse: BankRecordedInverseUndoAdmission,
///     principal: &BankAuthenticatedPrincipal,
///     key: &BankIdempotencyKey,
///     request: &WorthQueryRequestScope,
/// ) {
///     let _ = runtime.progress_undo_commit_recovery(inverse, principal, key, request);
/// }
/// ```
///
/// A caller-authored journal target is not accepted either:
///
/// ```compile_fail
/// use bank_domain::{proposals::BankIdempotencyKey, schema::ReverseJournal};
/// use bank_server::{BankAuthenticatedPrincipal, BankCompensationUndoAdmission, BankIdentityRuntime};
/// use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
/// fn substitute(
///     runtime: &BankIdentityRuntime,
///     admission: BankCompensationUndoAdmission,
///     principal: &BankAuthenticatedPrincipal,
///     journal: ReverseJournal,
///     key: &BankIdempotencyKey,
///     request: &WorthQueryRequestScope,
/// ) {
///     let _ = runtime.progress_undo_commit_recovery(
///         admission, principal, journal, key, request,
///     );
/// }
/// ```
#[derive(Debug)]
pub struct BankCompensationUndoAdmission {
    pub(super) query: WorthQueryUndoAdmission,
    pub(super) reverse_journal: ReverseJournal,
}

impl BankCompensationUndoAdmission {
    pub(crate) const fn new(
        query: WorthQueryUndoAdmission,
        reverse_journal: ReverseJournal,
    ) -> Self {
        Self {
            query,
            reverse_journal,
        }
    }

    pub const fn derived_request(&self) -> WorthQueryUndoDerivedRequest {
        self.query.derived_request()
    }

    pub const fn undo_admission_work(
        &self,
    ) -> worth_query_host::facade::domain::WorthQueryCanonicalWorkEvidence {
        self.query.undo_admission_work()
    }
}

/// Bank-owned typed continuation for an exact recorded-inverse target.
///
/// Progression has no caller-authored action slot:
///
/// ```compile_fail
/// use bank_domain::estate::EstateAction;
/// use bank_server::{BankAuthenticatedPrincipal, BankIdentityRuntime, BankRecordedInverseUndoAdmission};
/// use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
/// use worth_query_host::facade::primary_graph::WorthQueryApplicationIdempotencyBinding;
/// fn substitute(
///     runtime: &BankIdentityRuntime,
///     inverse: BankRecordedInverseUndoAdmission,
///     principal: &BankAuthenticatedPrincipal,
///     action: EstateAction,
///     idempotency: WorthQueryApplicationIdempotencyBinding,
///     request: &WorthQueryRequestScope,
/// ) {
///     let _ = runtime.progress_undo_recorded_inverse(
///         inverse, principal, action, idempotency, request,
///     );
/// }
/// ```
#[derive(Debug)]
pub struct BankRecordedInverseUndoAdmission {
    pub(super) query: WorthQueryUndoAdmission,
}

impl BankRecordedInverseUndoAdmission {
    pub(crate) const fn new(query: WorthQueryUndoAdmission) -> Self {
        Self { query }
    }

    pub const fn derived_request(&self) -> WorthQueryUndoDerivedRequest {
        self.query.derived_request()
    }

    pub const fn retained_preimage(&self) -> Option<&WorthQueryRetainedPreImage> {
        self.query.retained_preimage()
    }

    pub const fn undo_admission_work(
        &self,
    ) -> worth_query_host::facade::domain::WorthQueryCanonicalWorkEvidence {
        self.query.undo_admission_work()
    }

    pub fn canonical_work_phases(
        &self,
    ) -> worth_query_host::facade::domain::WorthQueryCanonicalWorkPhases {
        self.query.canonical_work_phases()
    }
}
