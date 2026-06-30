use crate::replay_undo_inventory::{
    ReplayUndoInventoryError, ReplayUndoInventorySourceIdentity, ReplayUndoSourceFirewallViolation,
};
use crate::workload_composition::WorkloadCompositionError;

use super::forbidden_surface_denial::ReplayUndoForbiddenConsumerSurfaceKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplayUndoConsumerCutoverErrorKind {
    WorkloadCompositionBoundary,
    SourceFirewallViolation,
    UndeclaredReceiptConsumer,
    UnownedResidue,
    MissingResidueRemovalTrigger,
    MissingForbiddenSurfaceDenial,
    ForbiddenSurfaceFirewallViolation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoConsumerCutoverError {
    kind: ReplayUndoConsumerCutoverErrorKind,
    detail: String,
}

impl ReplayUndoConsumerCutoverError {
    pub(crate) fn new(kind: ReplayUndoConsumerCutoverErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> &ReplayUndoConsumerCutoverErrorKind {
        &self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl From<WorkloadCompositionError> for ReplayUndoConsumerCutoverError {
    fn from(error: WorkloadCompositionError) -> Self {
        Self::new(
            ReplayUndoConsumerCutoverErrorKind::WorkloadCompositionBoundary,
            error.human_reason(),
        )
    }
}

impl From<ReplayUndoInventoryError> for ReplayUndoConsumerCutoverError {
    fn from(error: ReplayUndoInventoryError) -> Self {
        Self::new(
            ReplayUndoConsumerCutoverErrorKind::UnownedResidue,
            error.detail().to_string(),
        )
    }
}

impl From<ReplayUndoSourceFirewallViolation> for ReplayUndoConsumerCutoverError {
    fn from(error: ReplayUndoSourceFirewallViolation) -> Self {
        Self::new(
            ReplayUndoConsumerCutoverErrorKind::SourceFirewallViolation,
            format!(
                "replay/undo source `{}` is missing declared role {:?}",
                error.source_identity().as_str(),
                error.missing_role()
            ),
        )
    }
}

pub(crate) fn missing_residue_trigger(
    source: ReplayUndoInventorySourceIdentity,
) -> ReplayUndoConsumerCutoverError {
    ReplayUndoConsumerCutoverError::new(
        ReplayUndoConsumerCutoverErrorKind::MissingResidueRemovalTrigger,
        format!(
            "non-ordinary replay/undo source `{}` is missing removal trigger",
            source.as_str()
        ),
    )
}

pub(crate) fn missing_forbidden_surface_denial(
    kind: ReplayUndoForbiddenConsumerSurfaceKind,
) -> ReplayUndoConsumerCutoverError {
    ReplayUndoConsumerCutoverError::new(
        ReplayUndoConsumerCutoverErrorKind::MissingForbiddenSurfaceDenial,
        format!(
            "replay/undo forbidden surface `{}` is missing Phase 11 denial proof",
            kind.as_str()
        ),
    )
}

pub(crate) fn forbidden_surface_firewall_violation(
    kind: ReplayUndoForbiddenConsumerSurfaceKind,
    surface: &str,
    occurrence_count: usize,
) -> ReplayUndoConsumerCutoverError {
    ReplayUndoConsumerCutoverError::new(
        ReplayUndoConsumerCutoverErrorKind::ForbiddenSurfaceFirewallViolation,
        format!(
            "replay/undo forbidden surface `{}` has {occurrence_count} ordinary occurrence(s) in `{surface}`",
            kind.as_str()
        ),
    )
}
