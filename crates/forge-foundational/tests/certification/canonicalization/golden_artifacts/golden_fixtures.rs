use forge_foundational::{
    prepare_aspect_contract_for_digest, prepare_aspect_mask_for_digest,
    prepare_aspect_patch_for_digest, prepare_aspect_state_for_digest, AspectContract, AspectMask,
    AspectValue, CanonicalDigestAspectShapeKind, CanonicalDigestMaskMode,
    CanonicalDigestPreparationEntry, DigestPreparationMaskMode, FieldDeclaration, FieldRequirement,
    ScalarAspectType, StructAspectShape,
};
use forge_proof::TransitionOutcome;

use crate::foundational_vocabulary::{field, identity, key, revision};

pub(super) fn admitted_state<const N: usize>(
    entries: [forge_foundational::ContractValidatedAspectArtifact; N],
) -> forge_foundational::AuthoritativeRecordAspectStateArtifact {
    let TransitionOutcome::Success(state) =
        forge_foundational::admit_authoritative_record_aspect_state(entries)
    else {
        panic!("expected authoritative state admission");
    };
    state
}

pub(super) fn ready_state(
    state: forge_foundational::AuthoritativeRecordAspectStateArtifact,
) -> forge_foundational::DigestPreparationReadyAspectStateArtifact {
    match prepare_aspect_state_for_digest(state) {
        TransitionOutcome::Success(ready) => ready,
        _ => unreachable!("digest preparation uses only success"),
    }
}

pub(super) fn ready_patch(
    patch: &forge_foundational::AuthoritativeRecordAspectPatch,
) -> forge_foundational::DigestPreparationReadyAspectPatchArtifact {
    match prepare_aspect_patch_for_digest(patch) {
        TransitionOutcome::Success(ready) => ready,
        _ => unreachable!("digest preparation uses only success"),
    }
}

pub(super) fn ready_contract(
    contract: AspectContract,
) -> forge_foundational::DigestPreparationReadyAspectContractArtifact {
    match prepare_aspect_contract_for_digest(contract) {
        TransitionOutcome::Success(ready) => ready,
        _ => unreachable!("digest preparation uses only success"),
    }
}

pub(super) fn ready_mask<Mode>(
    aspect_key: forge_foundational::AspectKey,
    mask: AspectMask<Mode>,
) -> forge_foundational::DigestPreparationReadyAspectMaskArtifact<Mode>
where
    Mode: DigestPreparationMaskMode,
{
    match prepare_aspect_mask_for_digest(aspect_key, mask) {
        TransitionOutcome::Success(ready) => ready,
        _ => unreachable!("digest preparation uses only success"),
    }
}

pub(super) fn golden_contract() -> AspectContract {
    let shape = StructAspectShape::new([
        FieldDeclaration::new(
            field("title"),
            ScalarAspectType::String,
            FieldRequirement::Required,
            forge_foundational::AbsenceLaw::Required,
            forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired,
        )
        .expect("coherent title field"),
        FieldDeclaration::new(
            field("done"),
            ScalarAspectType::Bool,
            FieldRequirement::Required,
            forge_foundational::AbsenceLaw::Required,
            forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired,
        )
        .expect("coherent done field"),
    ])
    .expect("unique fields");

    AspectContract::struct_aspect(key("task.summary"), identity(20), revision(1), shape)
}

pub(super) fn golden_contract_basis() -> Vec<CanonicalDigestPreparationEntry> {
    vec![
        CanonicalDigestPreparationEntry::ContractHeader {
            key: key("task.summary"),
            identity: identity(20),
            revision: revision(1),
        },
        CanonicalDigestPreparationEntry::ContractShape {
            key: key("task.summary"),
            shape: CanonicalDigestAspectShapeKind::Struct,
        },
        CanonicalDigestPreparationEntry::ContractStructField {
            key: key("task.summary"),
            field: field("done"),
            value_type: ScalarAspectType::Bool,
            requirement: FieldRequirement::Required,
            absence: forge_foundational::AbsenceLaw::Required,
            evolution: forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired,
        },
        CanonicalDigestPreparationEntry::ContractStructField {
            key: key("task.summary"),
            field: field("title"),
            value_type: ScalarAspectType::String,
            requirement: FieldRequirement::Required,
            absence: forge_foundational::AbsenceLaw::Required,
            evolution: forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired,
        },
        CanonicalDigestPreparationEntry::ContractMaskMode {
            key: key("task.summary"),
            mode: CanonicalDigestMaskMode::Projection,
            allowed: true,
        },
        CanonicalDigestPreparationEntry::ContractMaskMode {
            key: key("task.summary"),
            mode: CanonicalDigestMaskMode::Mutation,
            allowed: true,
        },
        CanonicalDigestPreparationEntry::ContractMaskMode {
            key: key("task.summary"),
            mode: CanonicalDigestMaskMode::Diagnostic,
            allowed: true,
        },
        CanonicalDigestPreparationEntry::ContractAbsenceLaw {
            key: key("task.summary"),
            absence: forge_foundational::AbsenceLaw::Required,
        },
        CanonicalDigestPreparationEntry::ContractEquivalenceBasis {
            key: key("task.summary"),
            equivalence: forge_foundational::AspectEquivalenceBasis::DeclaredStructFields,
        },
        CanonicalDigestPreparationEntry::ContractEvolutionPolicy {
            key: key("task.summary"),
            evolution: forge_foundational::AspectEvolutionPolicy::AdditiveFieldsAllowed,
        },
    ]
}

pub(super) fn golden_mutation_mask_basis() -> Vec<CanonicalDigestPreparationEntry> {
    vec![
        CanonicalDigestPreparationEntry::MaskFieldPath {
            key: key("task.summary"),
            mode: CanonicalDigestMaskMode::Mutation,
            path: forge_foundational::CanonicalFieldPath::single(field("done")),
        },
        CanonicalDigestPreparationEntry::MaskFieldPath {
            key: key("task.summary"),
            mode: CanonicalDigestMaskMode::Mutation,
            path: forge_foundational::CanonicalFieldPath::single(field("title")),
        },
    ]
}

pub(super) fn golden_patch_basis() -> Vec<CanonicalDigestPreparationEntry> {
    vec![
        CanonicalDigestPreparationEntry::PatchWholeAspectSet {
            key: key("task.summary"),
            revision: revision(1),
        },
        CanonicalDigestPreparationEntry::PatchWholeAspectStructFieldValue {
            key: key("task.summary"),
            field: field("done"),
            value: AspectValue::Bool(true),
        },
        CanonicalDigestPreparationEntry::PatchWholeAspectStructFieldValue {
            key: key("task.summary"),
            field: field("title"),
            value: AspectValue::String("Ship it".into()),
        },
    ]
}
