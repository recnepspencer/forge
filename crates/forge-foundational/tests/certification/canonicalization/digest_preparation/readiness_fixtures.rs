use forge_foundational::{
    admit_authoritative_record_aspect_state, prepare_aspect_contract_for_digest,
    prepare_aspect_mask_for_digest, prepare_aspect_patch_for_digest,
    prepare_aspect_state_for_digest, AbsenceLaw, AspectContract, AspectMask, FieldDeclaration,
    FieldRequirement, ScalarAspectType, StructAspectShape,
};
use forge_proof::TransitionOutcome;

use crate::foundational_vocabulary::{field, identity, key, revision};

pub(super) fn ready_contract(
    contract: AspectContract,
) -> forge_foundational::DigestPreparationReadyAspectContractArtifact {
    let ready = match prepare_aspect_contract_for_digest(contract) {
        TransitionOutcome::Success(ready) => ready,
        _ => unreachable!("digest preparation uses only success"),
    };
    ready
}

pub(super) fn ready_mask<Mode>(
    aspect_key: forge_foundational::AspectKey,
    mask: AspectMask<Mode>,
) -> forge_foundational::DigestPreparationReadyAspectMaskArtifact<Mode>
where
    Mode: forge_foundational::DigestPreparationMaskMode,
{
    let ready = match prepare_aspect_mask_for_digest(aspect_key, mask) {
        TransitionOutcome::Success(ready) => ready,
        _ => unreachable!("digest preparation uses only success"),
    };
    ready
}

pub(super) fn ready_patch(
    patch: &forge_foundational::AuthoritativeRecordAspectPatch,
) -> forge_foundational::DigestPreparationReadyAspectPatchArtifact {
    let ready = match prepare_aspect_patch_for_digest(patch) {
        TransitionOutcome::Success(ready) => ready,
        _ => unreachable!("digest preparation uses only success"),
    };
    ready
}

pub(super) fn admitted_state<const N: usize>(
    entries: [forge_foundational::ContractValidatedAspectArtifact; N],
) -> forge_foundational::AuthoritativeRecordAspectStateArtifact {
    let TransitionOutcome::Success(state) = admit_authoritative_record_aspect_state(entries) else {
        panic!("expected authoritative state admission");
    };
    state
}

pub(super) fn ready_state(
    state: forge_foundational::AuthoritativeRecordAspectStateArtifact,
) -> forge_foundational::DigestPreparationReadyAspectStateArtifact {
    let ready = match prepare_aspect_state_for_digest(state) {
        TransitionOutcome::Success(ready) => ready,
        _ => unreachable!("digest preparation uses only success"),
    };
    ready
}

pub(super) fn task_summary_contract() -> AspectContract {
    AspectContract::struct_aspect(
        key("task.summary"),
        identity(20),
        revision(1),
        task_summary_shape([
            ("title", ScalarAspectType::String),
            ("done", ScalarAspectType::Bool),
        ]),
    )
}

pub(super) fn task_summary_contract_with_reversed_declaration_order() -> AspectContract {
    AspectContract::struct_aspect(
        key("task.summary"),
        identity(20),
        revision(1),
        task_summary_shape([
            ("done", ScalarAspectType::Bool),
            ("title", ScalarAspectType::String),
        ]),
    )
}

fn task_summary_shape<const N: usize>(fields: [(&str, ScalarAspectType); N]) -> StructAspectShape {
    StructAspectShape::new(fields.map(|(name, value_type)| {
        FieldDeclaration::new(
            field(name),
            value_type,
            FieldRequirement::Required,
            AbsenceLaw::Required,
            forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired,
        )
        .expect("coherent field law")
    }))
    .expect("unique fields")
}
