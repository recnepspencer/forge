use forge_foundational::facade::{
    aspects, AuthoritativeRecordAspectPatch, AuthoritativeRecordAspectState,
};
use forge_proof::TransitionOutcome;

use crate::transactions::data::EntityAuthoritativeAspectStateDenial;

pub(crate) fn plan_entity_authoritative_deletion_patch(
    authoritative_state: Option<&AuthoritativeRecordAspectState>,
) -> Result<Option<AuthoritativeRecordAspectPatch>, EntityAuthoritativeAspectStateDenial> {
    let Some(authoritative_state) = authoritative_state else {
        return Ok(None);
    };
    let cleared_aspects = authoritative_state
        .aspects()
        .entries()
        .map(|(aspect_key, _)| aspect_key.clone());
    construct_deletion_whole_aspect_patch(cleared_aspects)
}

fn construct_deletion_whole_aspect_patch(
    clears: impl IntoIterator<Item = forge_foundational::facade::AspectKey>,
) -> Result<Option<AuthoritativeRecordAspectPatch>, EntityAuthoritativeAspectStateDenial> {
    let mut builder = aspects().patch().whole_aspect();
    for clear in clears {
        builder = builder.clear(clear);
    }

    match builder.finish() {
        TransitionOutcome::Success(patch) if patch.is_empty() => Ok(None),
        TransitionOutcome::Success(patch) => Ok(Some(patch)),
        TransitionOutcome::Denied(denial) => {
            Err(EntityAuthoritativeAspectStateDenial::PatchConstructionDenied { denial })
        }
    }
}
