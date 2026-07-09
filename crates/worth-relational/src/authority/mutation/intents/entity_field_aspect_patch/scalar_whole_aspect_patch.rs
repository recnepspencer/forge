use worth_foundational::facade::{
    aspects, validate_aspect_value, AspectValue, AuthoritativeRecordAspectPatch,
    CanonicalFieldPath, ContractValidatedAspectArtifact, ContractValidationInput,
};
use worth_proof::TransitionOutcome;

use crate::schema::data::LoweredAspectContractBinding;
use crate::transactions::data::EntityFieldAspectPatchDenial;

pub(super) fn validate_entity_scalar_field_value(
    field_path: &CanonicalFieldPath,
    binding: &LoweredAspectContractBinding,
    value: AspectValue,
) -> Result<ContractValidatedAspectArtifact, EntityFieldAspectPatchDenial> {
    match validate_aspect_value(&binding.contract, ContractValidationInput::Scalar(value)) {
        TransitionOutcome::Success(artifact) => Ok(artifact),
        TransitionOutcome::Denied(denial) => {
            Err(EntityFieldAspectPatchDenial::ContractValidationDenied {
                field_locator: worth_foundational::facade::AspectFieldLocator::new(
                    worth_foundational::facade::LocatorAuthority::Planned,
                    binding.aspect_key().clone(),
                    field_path.clone(),
                ),
                denial,
            })
        }
    }
}

pub(super) fn construct_scalar_whole_aspect_patch(
    scalar_sets: Vec<ContractValidatedAspectArtifact>,
) -> Result<AuthoritativeRecordAspectPatch, EntityFieldAspectPatchDenial> {
    if scalar_sets.is_empty() {
        return Ok(AuthoritativeRecordAspectPatch::empty());
    }

    let mut builder = aspects().patch().whole_aspect();
    for scalar_set in scalar_sets {
        builder = builder.set(scalar_set);
    }

    match builder.finish() {
        TransitionOutcome::Success(patch) => Ok(patch),
        TransitionOutcome::Denied(denial) => {
            Err(EntityFieldAspectPatchDenial::PatchConstructionDenied {
                field_locator: None,
                denial,
            })
        }
    }
}
