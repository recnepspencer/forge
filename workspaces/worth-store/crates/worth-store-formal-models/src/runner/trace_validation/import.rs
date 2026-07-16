use crate::ImportPublicationAction;

use super::{CanonicalProtocolAction, ProtocolTraceValidationDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportTraceDenial {
    RawDeclarationRequired,
    CurrentScopeReadmissionRequired,
    RecoveredArtifactAdmissionRequired,
    LayoutMaterializationRequired,
    PublicationReadinessRequired,
    TerminalStateAlreadyReached,
}

pub(super) fn validate(
    actions: &[CanonicalProtocolAction],
) -> Result<(), ProtocolTraceValidationDenial> {
    let mut model = ImportTraceModel::Initial;
    for (action_index, action) in actions.iter().copied().enumerate() {
        let CanonicalProtocolAction::ImportPublication(action) = action else {
            return Err(ProtocolTraceValidationDenial::ActionFamilyMismatch { action_index });
        };
        model = model.apply(action).map_err(|denial| {
            ProtocolTraceValidationDenial::ImportPublication {
                action_index,
                denial,
            }
        })?;
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum ImportTraceModel {
    Initial,
    Raw,
    Readmitted,
    ArtifactAdmitted,
    Materialized,
    Pending,
    Terminal,
}

impl ImportTraceModel {
    fn apply(self, action: ImportPublicationAction) -> Result<Self, ImportTraceDenial> {
        use ImportPublicationAction as Action;
        match (self, action) {
            (Self::Initial, Action::RawDeclarationObserved) => Ok(Self::Raw),
            (Self::Initial, _) => Err(ImportTraceDenial::RawDeclarationRequired),
            (Self::Raw, Action::CurrentScopeReadmitted) => Ok(Self::Readmitted),
            (Self::Raw, _) => Err(ImportTraceDenial::CurrentScopeReadmissionRequired),
            (Self::Readmitted, Action::RecoveredArtifactAdmitted) => Ok(Self::ArtifactAdmitted),
            (Self::Readmitted, _) => Err(ImportTraceDenial::RecoveredArtifactAdmissionRequired),
            (Self::ArtifactAdmitted, Action::LayoutMaterializationAdmitted) => {
                Ok(Self::Materialized)
            }
            (Self::ArtifactAdmitted, _) => Err(ImportTraceDenial::LayoutMaterializationRequired),
            (Self::Materialized, Action::PublicationPending) => Ok(Self::Pending),
            (Self::Materialized, _) => Err(ImportTraceDenial::PublicationReadinessRequired),
            (
                Self::Pending,
                Action::PublicationDurable
                | Action::CrashBeforePublication
                | Action::PublicationDenied,
            ) => Ok(Self::Terminal),
            (Self::Pending, _) => Err(ImportTraceDenial::PublicationReadinessRequired),
            (Self::Terminal, _) => Err(ImportTraceDenial::TerminalStateAlreadyReached),
        }
    }
}
