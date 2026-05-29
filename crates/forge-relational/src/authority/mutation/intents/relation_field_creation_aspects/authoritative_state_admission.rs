use forge_foundational::facade::{
    admit_authoritative_record_aspect_state, AuthoritativeRecordAspectState,
    ContractValidatedAspectArtifact,
};
use forge_proof::TransitionOutcome;

use crate::transactions::data::RelationAuthoritativeAspectStateDenial;

pub(super) fn admit_relation_creation_state(
    validated_artifacts: Vec<ContractValidatedAspectArtifact>,
) -> Result<Option<AuthoritativeRecordAspectState>, RelationAuthoritativeAspectStateDenial> {
    if validated_artifacts.is_empty() {
        return Ok(None);
    }
    match admit_authoritative_record_aspect_state(validated_artifacts) {
        TransitionOutcome::Success(artifact) => {
            let (state, _proofs, _basis) = artifact.into_parts().into_parts();
            Ok(Some(state))
        }
        TransitionOutcome::Denied(denial) => {
            Err(RelationAuthoritativeAspectStateDenial::StateAdmissionDenied { denial })
        }
    }
}
