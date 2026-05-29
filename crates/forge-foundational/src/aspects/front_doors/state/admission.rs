use crate::{
    admit_authoritative_record_aspect_state, AuthoritativeRecordAspectStateArtifact,
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
}
