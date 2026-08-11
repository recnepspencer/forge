//! Ordinary progression handoff for an admitted redo (R8.43).
//!
//! Redo is an ordinary operation with an unusual input. Admission derives
//! nothing about current authority from the intent; this module only hands the
//! admitted request into ordinary mutation progression.

use super::redo_admission::WorthQueryRedoAdmission;
use super::redo_denial::WorthQueryRedoDenial;
use crate::domain_computation::managed_run::WorthQueryRecoveryResourceTerminal;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationIdempotencyBinding, WorthQueryRetainedGovernedInput,
};
use worth_query_installation::facade::WorthQueryCanonicalWorkEvidence;

use super::redo_intent::WorthQueryRedoIntent;

/// Sealed handoff from redo admission into ordinary mutation progression.
#[derive(Debug)]
pub struct WorthQueryRedoProgressionHandoff {
    intent: WorthQueryRedoIntent,
    retained_governed_input: WorthQueryRetainedGovernedInput,
    idempotency: WorthQueryApplicationIdempotencyBinding,
    redo_admission_work: WorthQueryCanonicalWorkEvidence,
    _private: (),
}

impl WorthQueryRedoProgressionHandoff {
    pub const fn intent(&self) -> &WorthQueryRedoIntent {
        &self.intent
    }

    pub const fn redo_admission_work(&self) -> WorthQueryCanonicalWorkEvidence {
        self.redo_admission_work
    }

    pub fn original_input<Input: 'static>(&self) -> Option<&Input> {
        self.retained_governed_input.downcast_ref()
    }

    pub const fn idempotency_binding(&self) -> WorthQueryApplicationIdempotencyBinding {
        self.idempotency
    }

    pub(crate) fn pending_causality(&self) -> super::WorthQueryPendingAftermathCausality {
        super::WorthQueryPendingAftermathCausality::redo_of(self.intent.undo_commit().clone())
    }
}

/// Progress an admitted redo into the ordinary mutation lane.
pub fn progress_admitted_redo(
    admission: WorthQueryRedoAdmission,
) -> Result<WorthQueryRedoProgressionHandoff, WorthQueryRedoDenial> {
    let (handle, intent, retained_governed_input, idempotency, redo_admission_work) =
        admission.into_progression_parts();
    handle
        .consume(WorthQueryRecoveryResourceTerminal::Consumed)
        .map_err(|denial| super::redo_admission::map_recovery_denial(denial.kind()))?;
    Ok(WorthQueryRedoProgressionHandoff {
        intent,
        retained_governed_input,
        idempotency,
        redo_admission_work,
        _private: (),
    })
}

/// Map an ordinary compare-and-commit denial into a redo denial cause.
pub fn map_ordinary_commit_conflict_to_redo() -> WorthQueryRedoDenial {
    WorthQueryRedoDenial::conflicted()
}
