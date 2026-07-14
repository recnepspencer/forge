use worth_foundational::facade::{
    aspects, AuthoritativeRecordAspectPatch, ContractValidatedAspectArtifact,
};
use worth_proof::TransitionOutcome;

use crate::transactions::data::EntityAuthoritativeAspectStateDenial;

pub(super) fn construct_creation_patch(
    validated_artifacts: Vec<ContractValidatedAspectArtifact>,
) -> Result<Option<AuthoritativeRecordAspectPatch>, EntityAuthoritativeAspectStateDenial> {
    let mut builder = aspects().patch().whole_aspect();
    for artifact in validated_artifacts {
        builder = builder.set(artifact);
    }
    match builder.finish() {
        TransitionOutcome::Success(patch) if patch.is_empty() => Ok(None),
        TransitionOutcome::Success(patch) => Ok(Some(patch)),
        TransitionOutcome::Denied(denial) => {
            Err(EntityAuthoritativeAspectStateDenial::PatchConstructionDenied { denial })
        }
    }
}
