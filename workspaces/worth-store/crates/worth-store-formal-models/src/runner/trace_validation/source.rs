use std::collections::{BTreeMap, BTreeSet};

use worth_store_recovery_physics::RecoverySourceApplicationRole;

use crate::SourcePrecedenceAction;

use super::{CanonicalProtocolAction, ProtocolTraceValidationDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceTraceDenial {
    DuplicateDiscovery,
    CandidateNotDiscovered,
    CandidateRoleMismatch,
    ContradictionNotPresent,
    NoAdmittedSource,
    NoQuarantinedSource,
    DenialHasSelectableSource,
}

pub(super) fn validate(
    actions: &[CanonicalProtocolAction],
) -> Result<(), ProtocolTraceValidationDenial> {
    let mut model = SourceTraceModel::default();
    for (action_index, action) in actions.iter().copied().enumerate() {
        let CanonicalProtocolAction::RecoverySourcePrecedence(action) = action else {
            return Err(ProtocolTraceValidationDenial::ActionFamilyMismatch { action_index });
        };
        model
            .apply(action)
            .map_err(|denial| ProtocolTraceValidationDenial::SourcePrecedence {
                action_index,
                denial,
            })?;
    }
    Ok(())
}

#[derive(Default)]
struct SourceTraceModel {
    discovered: BTreeMap<u64, RecoverySourceApplicationRole>,
    admitted: BTreeSet<u64>,
    quarantined: bool,
}

impl SourceTraceModel {
    fn apply(&mut self, action: SourcePrecedenceAction) -> Result<(), SourceTraceDenial> {
        match action {
            SourcePrecedenceAction::CandidateDiscovered {
                discovery_order,
                role,
            } => {
                if self.discovered.insert(discovery_order, role).is_some() {
                    return Err(SourceTraceDenial::DuplicateDiscovery);
                }
            }
            SourcePrecedenceAction::CandidateAdmitted { discovery_order } => {
                self.require_role(discovery_order, |role| {
                    matches!(
                        role,
                        RecoverySourceApplicationRole::CheckpointBase
                            | RecoverySourceApplicationRole::WalTailRedo
                    )
                })?;
                self.admitted.insert(discovery_order);
            }
            SourcePrecedenceAction::CandidateAdvisoryOnly { discovery_order } => {
                self.require_role(discovery_order, |role| {
                    matches!(
                        role,
                        RecoverySourceApplicationRole::PageSkipApply
                            | RecoverySourceApplicationRole::CompactionVisibility
                    )
                })?;
            }
            SourcePrecedenceAction::CandidateRejected { discovery_order } => {
                self.require_role(discovery_order, |role| {
                    matches!(
                        role,
                        RecoverySourceApplicationRole::ResidueDiscoveryOnly
                            | RecoverySourceApplicationRole::CompactionVisibility
                    )
                })?;
            }
            SourcePrecedenceAction::ContradictionPreserved => {
                if self.discovered.len() < 2 {
                    return Err(SourceTraceDenial::ContradictionNotPresent);
                }
            }
            SourcePrecedenceAction::SourceSelected => {
                if self.admitted.is_empty() {
                    return Err(SourceTraceDenial::NoAdmittedSource);
                }
            }
            SourcePrecedenceAction::SourceQuarantined => {
                if !self
                    .discovered
                    .values()
                    .any(|role| *role == RecoverySourceApplicationRole::RecoveryBlocked)
                {
                    return Err(SourceTraceDenial::NoQuarantinedSource);
                }
                self.quarantined = true;
            }
            SourcePrecedenceAction::SourceDenied => {
                if !self.admitted.is_empty() && !self.quarantined {
                    return Err(SourceTraceDenial::DenialHasSelectableSource);
                }
            }
        }
        Ok(())
    }

    fn require_role(
        &self,
        discovery_order: u64,
        predicate: impl FnOnce(RecoverySourceApplicationRole) -> bool,
    ) -> Result<(), SourceTraceDenial> {
        let role = self
            .discovered
            .get(&discovery_order)
            .copied()
            .ok_or(SourceTraceDenial::CandidateNotDiscovered)?;
        if predicate(role) {
            Ok(())
        } else {
            Err(SourceTraceDenial::CandidateRoleMismatch)
        }
    }
}
