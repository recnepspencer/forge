use crate::QuarantineReadmissionState;

use super::{CanonicalProtocolAction, ProtocolTraceValidationDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuarantineTraceDenial {
    InitialProposalRequired,
    SealRequired,
    VerificationRequired,
    TerminalStateAlreadyReached,
}

pub(super) fn validate(
    actions: &[CanonicalProtocolAction],
) -> Result<(), ProtocolTraceValidationDenial> {
    let mut previous = None;
    for (action_index, action) in actions.iter().copied().enumerate() {
        let CanonicalProtocolAction::QuarantineReadmission(state) = action else {
            return Err(ProtocolTraceValidationDenial::ActionFamilyMismatch { action_index });
        };
        validate_transition(previous, state).map_err(|denial| {
            ProtocolTraceValidationDenial::QuarantineReadmission {
                action_index,
                denial,
            }
        })?;
        previous = Some(state);
    }
    Ok(())
}

fn validate_transition(
    previous: Option<QuarantineReadmissionState>,
    next: QuarantineReadmissionState,
) -> Result<(), QuarantineTraceDenial> {
    use QuarantineReadmissionState as State;
    match (previous, next) {
        (None, State::Proposed) => Ok(()),
        (None, _) => Err(QuarantineTraceDenial::InitialProposalRequired),
        (Some(State::Proposed), State::Sealed) => Ok(()),
        (Some(State::Proposed), _) => Err(QuarantineTraceDenial::SealRequired),
        (Some(State::Sealed), State::RecoveryVerificationPending) => Ok(()),
        (Some(State::RecoveryVerificationPending), State::Readmitted | State::Denied) => Ok(()),
        (Some(State::RecoveryVerificationPending), State::RetainedForAudit) => Ok(()),
        (Some(State::Sealed), State::RetainedForAudit) => Ok(()),
        (Some(State::Sealed), _) => Err(QuarantineTraceDenial::VerificationRequired),
        (Some(State::Readmitted | State::RetainedForAudit | State::Denied), _) => {
            Err(QuarantineTraceDenial::TerminalStateAlreadyReached)
        }
        (Some(State::RecoveryVerificationPending), _) => {
            Err(QuarantineTraceDenial::VerificationRequired)
        }
    }
}
