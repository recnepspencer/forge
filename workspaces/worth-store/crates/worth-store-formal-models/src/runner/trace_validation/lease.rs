use crate::LeaseReclaimAction;

use super::{CanonicalProtocolAction, ProtocolTraceValidationDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseTraceDenial {
    LeaseAlreadyActive,
    ActiveLeaseRequired,
    LeaseIdentityMismatch,
    LiveOrExpiredLeaseBlocksReclaim,
    ReclaimRequiredBeforeReuse,
    GenerationDidNotAdvance,
}

pub(super) fn validate(
    actions: &[CanonicalProtocolAction],
) -> Result<(), ProtocolTraceValidationDenial> {
    let mut model = LeaseTraceModel::default();
    for (action_index, action) in actions.iter().copied().enumerate() {
        let CanonicalProtocolAction::LeaseReclaim(action) = action else {
            return Err(ProtocolTraceValidationDenial::ActionFamilyMismatch { action_index });
        };
        model
            .apply(action)
            .map_err(|denial| ProtocolTraceValidationDenial::LeaseReclaim {
                action_index,
                denial,
            })?;
    }
    Ok(())
}

#[derive(Default)]
struct LeaseTraceModel {
    active: Option<(u32, u64)>,
    expired_without_authority: bool,
    reclaimed: bool,
}

impl LeaseTraceModel {
    fn apply(&mut self, action: LeaseReclaimAction) -> Result<(), LeaseTraceDenial> {
        match action {
            LeaseReclaimAction::LeaseAcquired { slot, generation } => {
                if self.active.replace((slot, generation)).is_some() {
                    return Err(LeaseTraceDenial::LeaseAlreadyActive);
                }
            }
            LeaseReclaimAction::LeaseReleased { slot, generation }
            | LeaseReclaimAction::LeaseRevoked { slot, generation }
            | LeaseReclaimAction::OwnedCopyStabilized { slot, generation } => {
                self.release_matching(slot, generation)?;
            }
            LeaseReclaimAction::LeaseExpiredWithoutAuthority { slot, generation } => {
                self.require_matching(slot, generation)?;
                self.active = None;
                self.expired_without_authority = true;
            }
            LeaseReclaimAction::ReclaimDeniedByLiveLease => {
                if self.active.is_none() && !self.expired_without_authority {
                    return Err(LeaseTraceDenial::ActiveLeaseRequired);
                }
            }
            LeaseReclaimAction::ReclaimAdmitted => {
                if self.active.is_some() || self.expired_without_authority {
                    return Err(LeaseTraceDenial::LiveOrExpiredLeaseBlocksReclaim);
                }
                self.reclaimed = true;
            }
            LeaseReclaimAction::IdentityReuseDenied => {}
            LeaseReclaimAction::IdentityReuseAdmitted {
                old_generation,
                new_generation,
            } => {
                if !self.reclaimed {
                    return Err(LeaseTraceDenial::ReclaimRequiredBeforeReuse);
                }
                if new_generation <= old_generation {
                    return Err(LeaseTraceDenial::GenerationDidNotAdvance);
                }
            }
        }
        Ok(())
    }

    fn release_matching(&mut self, slot: u32, generation: u64) -> Result<(), LeaseTraceDenial> {
        self.require_matching(slot, generation)?;
        self.active = None;
        self.expired_without_authority = false;
        Ok(())
    }

    fn require_matching(&self, slot: u32, generation: u64) -> Result<(), LeaseTraceDenial> {
        match self.active {
            Some(identity) if identity == (slot, generation) => Ok(()),
            Some(_) => Err(LeaseTraceDenial::LeaseIdentityMismatch),
            None => Err(LeaseTraceDenial::ActiveLeaseRequired),
        }
    }
}
