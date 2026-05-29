use super::AuthoritativeStateFrontDoor;
use crate::{
    AuthoritativePatchApplicationDenial, AuthoritativeRecordAspectPatch,
    AuthoritativeRecordAspectState, AuthoritativeRecordAspectStateArtifact,
};
use forge_proof::TransitionOutcome;

impl AuthoritativeStateFrontDoor {
    pub fn apply_patch(
        self,
        state: &AuthoritativeRecordAspectState,
        patch: &AuthoritativeRecordAspectPatch,
    ) -> TransitionOutcome<
        AuthoritativeRecordAspectStateArtifact,
        AuthoritativePatchApplicationDenial,
    > {
        patch.apply_to(state)
    }
}
