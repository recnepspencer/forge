use crate::ReplicationAdmissionAction;

use super::{CanonicalProtocolAction, ProtocolTraceValidationDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicationTraceDenial {
    SourceAdmissionRequired,
    ProgressRequired,
    PublicationReadinessRequired,
    DeliveryKindMismatch,
    TerminalStateAlreadyReached,
}

pub(super) fn validate(
    actions: &[CanonicalProtocolAction],
) -> Result<(), ProtocolTraceValidationDenial> {
    let mut model = ReplicationTraceModel::Raw;
    for (action_index, action) in actions.iter().copied().enumerate() {
        let CanonicalProtocolAction::ReplicationAdmission(action) = action else {
            return Err(ProtocolTraceValidationDenial::ActionFamilyMismatch { action_index });
        };
        model = model.apply(action).map_err(|denial| {
            ProtocolTraceValidationDenial::ReplicationAdmission {
                action_index,
                denial,
            }
        })?;
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum Delivery {
    Fresh,
    Resumed,
}

#[derive(Clone, Copy)]
enum ReplicationTraceModel {
    Raw,
    SourceAdmitted,
    Progress(Delivery),
    Pending(Delivery),
    Terminal,
}

impl ReplicationTraceModel {
    fn apply(self, action: ReplicationAdmissionAction) -> Result<Self, ReplicationTraceDenial> {
        use ReplicationAdmissionAction as Action;
        match (self, action) {
            (Self::Raw, Action::SourceAdmitted) => Ok(Self::SourceAdmitted),
            (Self::Raw, action) if source_denial(action) => Ok(Self::Terminal),
            (Self::Raw, _) => Err(ReplicationTraceDenial::SourceAdmissionRequired),
            (Self::SourceAdmitted, Action::FreshProgressObserved) => {
                Ok(Self::Progress(Delivery::Fresh))
            }
            (Self::SourceAdmitted, Action::ResumeProgressObserved) => {
                Ok(Self::Progress(Delivery::Resumed))
            }
            (Self::SourceAdmitted, Action::DuplicateObserved) => Ok(Self::Terminal),
            (Self::SourceAdmitted, action) if progress_denial(action) => Ok(Self::Terminal),
            (Self::SourceAdmitted, _) => Err(ReplicationTraceDenial::ProgressRequired),
            (Self::Progress(Delivery::Fresh), Action::FreshPublicationPending) => {
                Ok(Self::Pending(Delivery::Fresh))
            }
            (Self::Progress(Delivery::Resumed), Action::ResumePublicationPending) => {
                Ok(Self::Pending(Delivery::Resumed))
            }
            (Self::Progress(_), _) => Err(ReplicationTraceDenial::DeliveryKindMismatch),
            (Self::Pending(Delivery::Fresh), Action::FreshPublicationDurable)
            | (Self::Pending(Delivery::Resumed), Action::ResumePublicationDurable) => {
                Ok(Self::Terminal)
            }
            (Self::Pending(_), action) if publication_denial(action) => Ok(Self::Terminal),
            (
                Self::Pending(_),
                Action::FreshPublicationDurable | Action::ResumePublicationDurable,
            ) => Err(ReplicationTraceDenial::DeliveryKindMismatch),
            (Self::Pending(_), _) => Err(ReplicationTraceDenial::PublicationReadinessRequired),
            (Self::Terminal, _) => Err(ReplicationTraceDenial::TerminalStateAlreadyReached),
        }
    }
}

const fn source_denial(action: ReplicationAdmissionAction) -> bool {
    matches!(
        action,
        ReplicationAdmissionAction::SourcePeerIdentityDenied
            | ReplicationAdmissionAction::SourceEpochRequiredDenied
            | ReplicationAdmissionAction::SourceLineageIdentityDenied
            | ReplicationAdmissionAction::SourceCurrentAuthorityDenied
            | ReplicationAdmissionAction::SourceReplayIdentityDenied
    )
}

const fn progress_denial(action: ReplicationAdmissionAction) -> bool {
    matches!(
        action,
        ReplicationAdmissionAction::ResumeCurrentAuthorityDenied
            | ReplicationAdmissionAction::SourceEpochDivergenceDetected
            | ReplicationAdmissionAction::LineageDivergenceDetected
            | ReplicationAdmissionAction::ReplayOverlapDivergenceDetected
            | ReplicationAdmissionAction::ResumeProgressGapDenied
    )
}

const fn publication_denial(action: ReplicationAdmissionAction) -> bool {
    matches!(
        action,
        ReplicationAdmissionAction::PublicationCurrentAuthorityDenied
            | ReplicationAdmissionAction::PublicationPeerProgressChangedDenied
            | ReplicationAdmissionAction::PublicationPeerCapacityDenied
            | ReplicationAdmissionAction::PublicationProgressStoreDenied
    )
}
