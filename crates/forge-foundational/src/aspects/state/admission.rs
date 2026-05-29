use forge_proof::{Artifact, TransitionOutcome};
use serde::{Deserialize, Serialize};

use super::{
    AuthoritativeRecordAspectState, AuthoritativeRecordAspectStateAdmitted,
    AuthoritativeRecordAspectStateArtifact,
};
use crate::aspects::keys::AspectKey;
use crate::aspects::validation::ContractValidatedAspectArtifact;

pub fn admit_authoritative_record_aspect_state(
    entries: impl IntoIterator<Item = ContractValidatedAspectArtifact>,
) -> TransitionOutcome<AuthoritativeRecordAspectStateArtifact, AuthoritativeStateAdmissionDenial> {
    let mut validated_entries = Vec::new();

    for artifact in entries {
        let (entry, _proofs, _basis) = artifact.into_parts().into_parts();
        validated_entries.push(entry);
    }

    if validated_entries.is_empty() {
        return TransitionOutcome::denied(AuthoritativeStateAdmissionDenial::EmptyAdmission);
    }

    match AuthoritativeRecordAspectState::from_validated_entries(validated_entries) {
        Ok(state) => TransitionOutcome::success(Artifact::<
            AuthoritativeRecordAspectStateAdmitted,
            _,
        >::new(state)),
        Err(denial) => TransitionOutcome::denied(denial),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthoritativeStateAdmissionDenial {
    EmptyAdmission,
    DuplicateAspectKey(AspectKey),
}
