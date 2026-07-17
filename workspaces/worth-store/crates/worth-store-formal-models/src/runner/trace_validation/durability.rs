use crate::{DurabilityRecoveryAction, DurabilityRecoveryDenial, DurabilityRecoveryFrontier};

use super::{CanonicalProtocolAction, ProtocolTraceValidationDenial};

pub(super) fn validate(
    actions: &[CanonicalProtocolAction],
) -> Result<(), ProtocolTraceValidationDenial> {
    let mut frontier = DurabilityRecoveryFrontier::initial();
    for (action_index, action) in actions.iter().copied().enumerate() {
        let CanonicalProtocolAction::DurabilityRecovery(action) = action else {
            return Err(ProtocolTraceValidationDenial::ActionFamilyMismatch { action_index });
        };
        match frontier.apply(action) {
            Ok(()) => {}
            Err(DurabilityRecoveryDenial::RedoGenerationMismatch)
                if action == DurabilityRecoveryAction::RecoveryReplayRejectedGenerationMismatch
                    && action_index + 1 == actions.len() => {}
            Err(denial) => {
                return Err(ProtocolTraceValidationDenial::Durability {
                    action_index,
                    denial,
                });
            }
        }
    }
    Ok(())
}
