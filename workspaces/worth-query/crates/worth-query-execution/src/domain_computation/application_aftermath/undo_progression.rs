//! Ordinary progression handoff for an admitted undo (R8.37 / A11).
//!
//! Undo is an ordinary operation with an unusual input. Admission derives the
//! request; this module hands that request into the same compare-and-commit
//! entry used by every other mutation. It does not mutate and does not call the
//! provider to repair state.

use super::recovery_handle::RelinquishOnDenial;
use super::undo_admission::{WorthQueryUndoAdmission, WorthQueryUndoDerivedRequest};
use super::undo_denial::{WorthQueryUndoDenial, WorthQueryUndoDenialKind};
use super::undo_preimage::WorthQueryRetainedPreImage;

/// Sealed handoff from undo admission into ordinary mutation progression.
#[derive(Debug)]
pub struct WorthQueryUndoProgressionHandoff {
    admission: WorthQueryUndoAdmission,
    retained_preimage: Option<WorthQueryRetainedPreImage>,
    _private: (),
}

impl WorthQueryUndoProgressionHandoff {
    pub const fn admission(&self) -> &WorthQueryUndoAdmission {
        &self.admission
    }

    pub const fn derived_request(&self) -> WorthQueryUndoDerivedRequest {
        self.admission.derived_request()
    }

    /// Retained pre-image required for recorded-inverse restoration (R8.2).
    pub const fn retained_preimage(&self) -> Option<&WorthQueryRetainedPreImage> {
        self.retained_preimage.as_ref()
    }

    pub fn into_parts(self) -> (WorthQueryUndoAdmission, Option<WorthQueryRetainedPreImage>) {
        (self.admission, self.retained_preimage)
    }

    pub(crate) fn into_recovery_handle(self) -> super::WorthQueryRecoveryHandle {
        self.admission.into_recovery_handle()
    }

    pub(crate) fn pending_causality(&self) -> super::WorthQueryPendingAftermathCausality {
        super::WorthQueryPendingAftermathCausality::undo_of(
            self.admission.original_commit().clone(),
        )
    }
}

/// Progress an admitted undo into the ordinary mutation lane.
///
/// RecordedInverse requires the receipt's retained pre-image already consumed
/// into admission. Compensation and reconciliation do not require pre-image bytes.
pub fn progress_admitted_undo(
    admission: WorthQueryUndoAdmission,
) -> Result<WorthQueryUndoProgressionHandoff, WorthQueryUndoDenial> {
    // A missing pre-image is a non-event: nothing progressed, so the admission
    // relinquishes its handle instead of dropping it and the commit stays
    // recoverable (Q8.21-L11).
    let (admission, retained_preimage) =
        admission.admit_deriving(|admission| match admission.derived_request() {
            WorthQueryUndoDerivedRequest::RecordedInverse => admission
                .retained_preimage()
                .cloned()
                .map(Some)
                .ok_or_else(|| {
                    WorthQueryUndoDenial::new(WorthQueryUndoDenialKind::RetainedPreImageRequired)
                }),
            WorthQueryUndoDerivedRequest::Compensation
            | WorthQueryUndoDerivedRequest::Reconciliation => Ok(None),
        })?;
    Ok(WorthQueryUndoProgressionHandoff {
        admission,
        retained_preimage,
        _private: (),
    })
}

/// Map an ordinary compare-and-commit denial into an undo denial cause.
pub fn map_ordinary_commit_conflict() -> WorthQueryUndoDenial {
    WorthQueryUndoDenial::new(WorthQueryUndoDenialKind::Conflicted)
}
