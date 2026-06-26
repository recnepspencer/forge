use crate::capability::RuntimeOutcomeProjectionId;

use super::{
    RuntimeOutcomeDenialPosture, RuntimeOutcomeFamily, RuntimeOutcomePresentation,
    RuntimeOutcomeRecoveryPosture, RuntimeOutcomeSourceReference,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeOutcomeProjectionDescriptor {
    id: RuntimeOutcomeProjectionId,
    family: RuntimeOutcomeFamily,
    source: Option<RuntimeOutcomeSourceReference>,
    presentation: Option<RuntimeOutcomePresentation>,
    denial_posture: Option<RuntimeOutcomeDenialPosture>,
    recovery_posture: Option<RuntimeOutcomeRecoveryPosture>,
    local_status_enum_claim: Option<String>,
}

impl RuntimeOutcomeProjectionDescriptor {
    pub fn new(
        id: RuntimeOutcomeProjectionId,
        family: RuntimeOutcomeFamily,
        source: RuntimeOutcomeSourceReference,
    ) -> Self {
        Self {
            id,
            family,
            source: Some(source),
            presentation: None,
            denial_posture: None,
            recovery_posture: None,
            local_status_enum_claim: None,
        }
    }

    pub fn local_status_enum_for_diagnostics(
        id: RuntimeOutcomeProjectionId,
        claim: impl Into<String>,
    ) -> Self {
        Self {
            id,
            family: RuntimeOutcomeFamily::unknown_for_diagnostics("local_status_enum"),
            source: None,
            presentation: None,
            denial_posture: None,
            recovery_posture: None,
            local_status_enum_claim: Some(claim.into()),
        }
    }

    pub fn with_presentation(mut self, presentation: RuntimeOutcomePresentation) -> Self {
        self.presentation = Some(presentation);
        self
    }

    pub fn with_denial_posture(mut self, denial_posture: RuntimeOutcomeDenialPosture) -> Self {
        self.denial_posture = Some(denial_posture);
        self
    }

    pub fn with_recovery_posture(
        mut self,
        recovery_posture: RuntimeOutcomeRecoveryPosture,
    ) -> Self {
        self.recovery_posture = Some(recovery_posture);
        self
    }

    pub fn id(&self) -> &RuntimeOutcomeProjectionId {
        &self.id
    }

    pub fn family(&self) -> &RuntimeOutcomeFamily {
        &self.family
    }

    pub fn source(&self) -> Option<&RuntimeOutcomeSourceReference> {
        self.source.as_ref()
    }

    pub fn presentation(&self) -> Option<&RuntimeOutcomePresentation> {
        self.presentation.as_ref()
    }

    pub fn denial_posture(&self) -> Option<&RuntimeOutcomeDenialPosture> {
        self.denial_posture.as_ref()
    }

    pub fn recovery_posture(&self) -> Option<&RuntimeOutcomeRecoveryPosture> {
        self.recovery_posture.as_ref()
    }

    pub(crate) fn has_local_status_enum_claim(&self) -> bool {
        self.local_status_enum_claim.is_some()
    }
}
