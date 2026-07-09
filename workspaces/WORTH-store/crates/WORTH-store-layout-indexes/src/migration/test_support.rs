use worth_foundational::{aspects, AspectContract, AspectValue, InternedString, ScalarAspectType};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};

use crate::{
    layout_declarations, ArtifactFamilyAuthorityWitness, ArtifactFamilyLifecycleAdmission,
    LayoutBindingWitness, LayoutCompatibilityWindow, LayoutEvolutionDeclaration,
    LayoutInterruptionPolicy, LayoutReadCompatibilityPosture, LayoutVersion,
    LayoutWriteCompatibilityPosture,
};

use worth_store_compatibility::{ArtifactFormatVersion, ArtifactSemanticVersion};

pub(crate) fn declared_family() -> ArtifactFamilyAuthorityWitness {
    let declaration = layout_declarations().seed_family();
    let classification = layout_declarations().classify_family(declaration);
    layout_declarations()
        .require_production_authority(classification)
        .expect("seed family should stay authoritative")
}

pub(crate) fn lifecycle_admission() -> ArtifactFamilyLifecycleAdmission {
    layout_declarations()
        .require_strategy_lifecycle(declared_family())
        .expect("seed family should stay strategy-admitted")
}

pub(crate) fn other_declared_family() -> ArtifactFamilyAuthorityWitness {
    let declaration = layout_declarations()
        .declaration(DurableArtifactFamilyId::PublicationSnapshotImage)
        .expect("publication snapshot image family should stay declared");
    let classification = layout_declarations().classify_family(declaration);
    layout_declarations()
        .require_production_authority(classification)
        .expect("publication snapshot image family should stay authoritative")
}

pub(crate) fn other_lifecycle_admission() -> ArtifactFamilyLifecycleAdmission {
    layout_declarations()
        .require_strategy_lifecycle(other_declared_family())
        .expect("publication snapshot image family should stay strategy-admitted")
}

pub(crate) fn version(format: u32, major: u16, minor: u16) -> LayoutVersion {
    LayoutVersion::new(
        ArtifactFormatVersion(format),
        ArtifactSemanticVersion::new(major, minor),
    )
}

pub(crate) fn declaration() -> LayoutEvolutionDeclaration {
    LayoutEvolutionDeclaration::new(
        declared_family(),
        version(7, 2, 1),
        LayoutCompatibilityWindow::new(
            ArtifactFormatVersion(5),
            ArtifactFormatVersion(7),
            ArtifactFormatVersion(7),
            LayoutReadCompatibilityPosture::ReadOldWriteNew,
            LayoutWriteCompatibilityPosture::WriteNewDuringRollingUpgrade,
        )
        .expect("test compatibility window should admit"),
        version(5, 1, 0),
        version(7, 2, 1),
        version(7, 2, 1),
        version(5, 1, 0),
        LayoutInterruptionPolicy::ResumeDeclaredMigration,
    )
}

pub(crate) fn binding(
    bound_version: LayoutVersion,
    observed_version: LayoutVersion,
    bound_authority: StoreCurrentAuthorityWitness,
) -> LayoutBindingWitness {
    LayoutBindingWitness::new(
        lifecycle_admission(),
        bound_version,
        observed_version,
        bound_authority,
    )
}

pub(crate) fn other_family_binding(
    bound_version: LayoutVersion,
    observed_version: LayoutVersion,
    bound_authority: StoreCurrentAuthorityWitness,
) -> LayoutBindingWitness {
    LayoutBindingWitness::new(
        other_lifecycle_admission(),
        bound_version,
        observed_version,
        bound_authority,
    )
}

pub(crate) fn current_authority(identity_key: &str, value: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(identity_key, value))
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspects().vocabulary().key(identity_key).unwrap();
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String);
    let admitted_state = match aspects()
        .authoritative_state()
        .admit([validated_scalar_value(&contract, value)])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };

    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(admitted_state, physical_witness()),
    )
    .expect("Store boundary fact should admit matching identity")
}

fn validated_scalar_value(
    contract: &AspectContract,
    raw_value: &str,
) -> worth_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(AspectValue::String(InternedString::from(raw_value)))
    {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
    }
}

fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .expect("test physical authority scope should be valid"),
    )
    .expect("test physical boundary witness should admit")
}
