use forge_foundational::{
    admit_authoritative_record_aspect_state, AbsenceLaw, AspectContract, AspectValue,
    FieldDeclaration, FieldRequirement, ScalarAspectType, StructAspectShape, StructAspectValue,
};
use forge_proof::TransitionOutcome;

use crate::foundational_vocabulary::{field, identity, key, revision};

pub(super) fn admitted_state<const N: usize>(
    entries: [forge_foundational::ContractValidatedAspectArtifact; N],
) -> forge_foundational::AuthoritativeRecordAspectStateArtifact {
    let TransitionOutcome::Success(state) = admit_authoritative_record_aspect_state(entries) else {
        panic!("expected authoritative state admission");
    };
    state
}

pub(super) fn task_summary_contract() -> AspectContract {
    let shape = StructAspectShape::new([
        FieldDeclaration::new(
            field("title"),
            ScalarAspectType::String,
            FieldRequirement::Required,
            AbsenceLaw::Required,
            forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired,
        )
        .expect("coherent field law"),
        FieldDeclaration::new(
            field("done"),
            ScalarAspectType::Bool,
            FieldRequirement::Required,
            AbsenceLaw::Required,
            forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired,
        )
        .expect("coherent field law"),
        FieldDeclaration::new(
            field("note"),
            ScalarAspectType::String,
            FieldRequirement::Optional,
            AbsenceLaw::Optional,
            forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired,
        )
        .expect("coherent field law"),
    ])
    .expect("unique fields");

    AspectContract::struct_aspect(key("task.summary"), identity(20), revision(1), shape)
}

pub(super) fn validated_task_summary(
    contract: &AspectContract,
    title: &str,
    done: bool,
    note: Option<&str>,
) -> forge_foundational::ContractValidatedAspectArtifact {
    let mut fields = vec![
        (field("title"), AspectValue::String(title.into())),
        (field("done"), AspectValue::Bool(done)),
    ];
    if let Some(note) = note {
        fields.push((field("note"), AspectValue::String(note.into())));
    }
    let value = StructAspectValue::new(fields).expect("unique fields");
    let TransitionOutcome::Success(artifact) =
        forge_foundational::validate_aspect_value(contract, value.into())
    else {
        panic!("expected validated task summary");
    };
    artifact
}
