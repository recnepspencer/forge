use crate::{CompactionVisibilityAction, ModeledOutcome};

use super::{CanonicalProtocolAction, ProtocolTraceValidationDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactionTraceDenial {
    RewriteLoweringRequired,
    PublicationRequired,
    PublicationRequiredBeforeRecoveryVisibility,
    ReclaimWasNotDeferred,
    ActionAfterTerminalDenial,
}

pub(super) fn validate(
    actions: &[CanonicalProtocolAction],
) -> Result<(), ProtocolTraceValidationDenial> {
    let mut model = CompactionTraceModel::default();
    for (action_index, action) in actions.iter().copied().enumerate() {
        let CanonicalProtocolAction::CompactionVisibility(action) = action else {
            return Err(ProtocolTraceValidationDenial::ActionFamilyMismatch { action_index });
        };
        model.apply(action).map_err(|denial| {
            ProtocolTraceValidationDenial::CompactionVisibility {
                action_index,
                denial,
            }
        })?;
    }
    Ok(())
}

#[derive(Default)]
struct CompactionTraceModel {
    lowered: bool,
    published: bool,
    recovery_visibility: bool,
    reclaim_deferred: bool,
    terminal_denial: bool,
}

impl CompactionTraceModel {
    fn apply(&mut self, action: CompactionVisibilityAction) -> Result<(), CompactionTraceDenial> {
        if self.terminal_denial {
            return Err(CompactionTraceDenial::ActionAfterTerminalDenial);
        }
        match action {
            CompactionVisibilityAction::AdmitRecoveryVisibility => {
                if !self.published {
                    return Err(CompactionTraceDenial::PublicationRequiredBeforeRecoveryVisibility);
                }
                self.recovery_visibility = true;
            }
            CompactionVisibilityAction::LowerRewrite => {
                self.lowered = true;
            }
            CompactionVisibilityAction::PublishRewrite => {
                if !self.lowered {
                    return Err(CompactionTraceDenial::RewriteLoweringRequired);
                }
                self.published = true;
            }
            CompactionVisibilityAction::DeferReclaim => {
                if !self.published {
                    return Err(CompactionTraceDenial::PublicationRequired);
                }
                self.reclaim_deferred = true;
            }
            CompactionVisibilityAction::DrainReclaimAfterReadRelease => {
                if !self.reclaim_deferred {
                    return Err(CompactionTraceDenial::ReclaimWasNotDeferred);
                }
            }
            CompactionVisibilityAction::LsmMembership {
                outcome: ModeledOutcome::Denied(_),
                ..
            }
            | CompactionVisibilityAction::LsmExecution {
                outcome: ModeledOutcome::Denied(_),
                ..
            }
            | CompactionVisibilityAction::LsmMaintenance {
                outcome: ModeledOutcome::Denied(_),
                ..
            }
            | CompactionVisibilityAction::DenyInPlaceOverwrite
            | CompactionVisibilityAction::DenyEarlyReclaim
            | CompactionVisibilityAction::DenyStaleEpochReuse
            | CompactionVisibilityAction::DenyBackendResidueCandidateSelection
            | CompactionVisibilityAction::DenyLatchHierarchyInversion
            | CompactionVisibilityAction::DenyMixedRootRead => {
                self.terminal_denial = true;
            }
            CompactionVisibilityAction::LsmMembership { .. }
            | CompactionVisibilityAction::LsmExecution { .. }
            | CompactionVisibilityAction::LsmMaintenance { .. } => {}
        }
        Ok(())
    }
}
