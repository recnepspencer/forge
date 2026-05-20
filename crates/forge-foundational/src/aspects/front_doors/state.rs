use super::super::{
    admit_authoritative_record_aspect_state, AuthoritativePatchApplicationDenial,
    AuthoritativeRecordAspectPatch, AuthoritativeRecordAspectStateArtifact,
    AuthoritativeStateAdmissionDenial, ContractValidatedAspectArtifact,
};
use forge_proof::TransitionOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AuthoritativeStateFrontDoor;

impl AuthoritativeStateFrontDoor {
    pub fn admit(
        self,
        entries: impl IntoIterator<Item = ContractValidatedAspectArtifact>,
    ) -> TransitionOutcome<AuthoritativeRecordAspectStateArtifact, AuthoritativeStateAdmissionDenial>
    {
        admit_authoritative_record_aspect_state(entries)
    }

    pub fn apply_patch(
        self,
        state: &super::super::AuthoritativeRecordAspectState,
        patch: &AuthoritativeRecordAspectPatch,
    ) -> TransitionOutcome<
        AuthoritativeRecordAspectStateArtifact,
        AuthoritativePatchApplicationDenial,
    > {
        patch.apply_to(state)
    }
}
