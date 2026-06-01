use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AuthoritativePatchApplicationDenial,
    AuthoritativeRecordAspectPatch, AuthoritativeRecordAspectState, CanonicalFieldPath,
    LocatorAuthority,
};
use forge_proof::TransitionOutcome;

use crate::transactions::data::EntityFieldAspectPatchDenial;

pub(super) fn apply_entity_authoritative_patch(
    current_state: Option<&AuthoritativeRecordAspectState>,
    patch: &AuthoritativeRecordAspectPatch,
) -> Result<AuthoritativeRecordAspectState, EntityFieldAspectPatchDenial> {
    let patch_aspect_key = first_patch_aspect_key(patch);
    let current_state = current_state.ok_or_else(|| {
        EntityFieldAspectPatchDenial::MissingAuthoritativeAspectState {
            aspect_key: patch_aspect_key.clone(),
        }
    })?;
    let Some(patch_aspect_key) = patch_aspect_key else {
        return Err(EntityFieldAspectPatchDenial::EmptyAuthoritativePatchPlan);
    };

    match patch.apply_to(current_state) {
        TransitionOutcome::Success(artifact) => {
            let (state, _proofs, _basis) = artifact.into_parts().into_parts();
            Ok(state)
        }
        TransitionOutcome::Denied(denial) => Err(application_denial_with_patch_target(
            patch,
            patch_aspect_key,
            denial,
        )),
    }
}

pub(super) fn first_patch_aspect_key(patch: &AuthoritativeRecordAspectPatch) -> Option<AspectKey> {
    patch
        .whole_aspect_sets()
        .map(|(aspect_key, _)| aspect_key.clone())
        .chain(patch.whole_aspect_clears().cloned())
        .chain(
            patch
                .field_patches()
                .map(|(aspect_key, _)| aspect_key.clone()),
        )
        .next()
}

fn first_field_patch_locator(patch: &AuthoritativeRecordAspectPatch) -> Option<AspectFieldLocator> {
    patch.field_patches().find_map(|(aspect_key, field_patch)| {
        first_field_patch_path(field_patch).map(|field_path| {
            AspectFieldLocator::new(LocatorAuthority::Planned, aspect_key.clone(), field_path)
        })
    })
}

fn first_field_patch_path(
    field_patch: &forge_foundational::facade::FieldLevelAspectPatch,
) -> Option<CanonicalFieldPath> {
    field_patch
        .field_sets()
        .map(|(field_key, _)| CanonicalFieldPath::single(field_key.clone()))
        .chain(
            field_patch
                .field_clears()
                .map(|field_key| CanonicalFieldPath::single(field_key.clone())),
        )
        .next()
}

fn application_denial_with_patch_target(
    patch: &AuthoritativeRecordAspectPatch,
    patch_aspect_key: AspectKey,
    denial: AuthoritativePatchApplicationDenial,
) -> EntityFieldAspectPatchDenial {
    match first_field_patch_locator(patch) {
        Some(field_locator) => EntityFieldAspectPatchDenial::FieldPatchApplicationDenied {
            field_locator,
            denial,
        },
        None => EntityFieldAspectPatchDenial::WholeAspectPatchApplicationDenied {
            aspect_key: patch_aspect_key,
            denial,
        },
    }
}
