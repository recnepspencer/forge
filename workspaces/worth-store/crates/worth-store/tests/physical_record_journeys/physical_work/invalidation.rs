use tempfile::tempdir;
use worth_foundational::{
    aspects, AbsenceLaw, AspectEquivalenceBasis, AspectEvolutionPolicy, AspectMask, AspectValue,
    ProjectionMask, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_signal::facade::PartitionSubscription;
use worth_store::aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectContractAdmission,
    StoreAspectPatchAuthorityInput, StoreAspectPatchBoundaryFact,
};
use worth_store::physical_runtime::{
    PhysicalSignalAspectBindingSet, PhysicalSignalAspectDeclaration, PhysicalSignalAspectRole,
    PhysicalSignalDeltaApplicationFailure, PhysicalWorkAspectDelta, PhysicalWorkAspectDeltaDenial,
    PhysicalWorkProfileDeclaration,
};

use super::fixture::{
    admitted_contract, security_scope, serving_from_initialization_with_work_profile,
    validated_value,
};

#[test]
fn an_installed_native_delta_routes_without_media_or_temporal_clock_effects() {
    let root = tempdir().unwrap();
    let (contract, identity, admission, witness) = admitted_contract(1);
    let profile = PhysicalWorkProfileDeclaration::new(security_scope(witness), [admission]).unwrap();
    let bindings = PhysicalSignalAspectBindingSet::from_profile(profile.clone());
    let delta = PhysicalWorkAspectDelta::from_boundary_fact(
        bindings.binding_for_slot(0).unwrap(),
        &boundary_fact(&contract, identity, witness, "changed"),
    )
    .unwrap();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let before_clock = serving.physical_signal_clock_observation().unwrap();
    let before_media = serving.media_counters();

    serving.apply_physical_aspect_delta(delta).unwrap();

    assert_eq!(
        serving.physical_signal_clock_observation().unwrap(),
        before_clock,
        "dependency invalidation must not fabricate temporal-clock progress"
    );
    assert_eq!(serving.media_counters(), before_media);
    serving.close();
}

#[test]
fn a_valid_delta_from_another_binding_cannot_alias_the_same_signal_slot() {
    let root = tempdir().unwrap();
    let (_, _, installed, installed_witness) = admitted_contract(1);
    let installed_profile =
        PhysicalWorkProfileDeclaration::new(security_scope(installed_witness), [installed])
            .unwrap();
    let (foreign_contract, foreign_identity, foreign, foreign_witness) = admitted_contract(2);
    let foreign_bindings = PhysicalSignalAspectBindingSet::from_profile(
        PhysicalWorkProfileDeclaration::new(security_scope(foreign_witness), [foreign]).unwrap(),
    );
    let foreign_delta = PhysicalWorkAspectDelta::from_boundary_fact(
        foreign_bindings.binding_for_slot(0).unwrap(),
        &boundary_fact(
            &foreign_contract,
            foreign_identity,
            foreign_witness,
            "foreign",
        ),
    )
    .unwrap();
    let serving = serving_from_initialization_with_work_profile(root.path(), installed_profile);
    let before_clock = serving.physical_signal_clock_observation().unwrap();
    let before_media = serving.media_counters();

    assert_eq!(
        serving.apply_physical_aspect_delta(foreign_delta),
        Err(PhysicalSignalDeltaApplicationFailure::BindingNotInstalled)
    );
    assert_eq!(
        serving.physical_signal_clock_observation().unwrap(),
        before_clock
    );
    assert_eq!(serving.media_counters(), before_media);
    serving.close();
}

#[test]
fn wrong_revision_and_projection_only_facts_cannot_mint_deltas() {
    let (installed_contract, installed_identity, installed, installed_witness) =
        admitted_contract(1);
    let bindings = PhysicalSignalAspectBindingSet::from_profile(
        PhysicalWorkProfileDeclaration::new(security_scope(installed_witness), [installed])
            .unwrap(),
    );
    let (foreign_contract, foreign_identity, _, foreign_witness) = admitted_contract(2);
    assert_eq!(
        PhysicalWorkAspectDelta::from_boundary_fact(
            bindings.binding_for_slot(0).unwrap(),
            &boundary_fact(
                &foreign_contract,
                foreign_identity,
                foreign_witness,
                "wrong-revision",
            ),
        ),
        Err(PhysicalWorkAspectDeltaDenial::ContractRevisionMismatch)
    );

    let projection_only = StoreAspectContractAdmission::new(
        installed_identity.clone(),
        installed_contract.clone(),
        installed_witness,
    )
    .unwrap()
    .admit_projection_mask(AspectMask::<ProjectionMask>::whole_aspect())
    .unwrap();
    let projection_bindings = PhysicalSignalAspectBindingSet::from_profile(
        PhysicalWorkProfileDeclaration::new(
            security_scope(installed_witness),
            [projection_only],
        )
        .unwrap(),
    );
    assert_eq!(
        PhysicalWorkAspectDelta::from_boundary_fact(
            projection_bindings.binding_for_slot(0).unwrap(),
            &boundary_fact(
                &installed_contract,
                installed_identity,
                installed_witness,
                "projection-only",
            ),
        ),
        Err(PhysicalWorkAspectDeltaDenial::MutationMaskAbsent)
    );
}

#[test]
fn native_patch_delta_preserves_the_bound_partition_route() {
    let root = tempdir().unwrap();
    let (contract, identity, admission, witness) = admitted_contract(1);
    let profile = PhysicalWorkProfileDeclaration::from_signal_aspects(
        security_scope(witness),
        [PhysicalSignalAspectDeclaration::new(
            admission,
            PhysicalSignalAspectRole::DependencyAndOutput,
        )
        .with_partition(PartitionSubscription::partition_and_detail(
            "artifact-7",
            "frame-2",
        ))],
    )
    .unwrap();
    let bindings = PhysicalSignalAspectBindingSet::from_profile(profile.clone());
    let patch = match aspects()
        .patch()
        .whole_aspect()
        .set(validated_value(&contract, "partition-change"))
        .finish()
    {
        TransitionOutcome::Success(patch) => patch,
        outcome => panic!("patch should construct: {outcome:?}"),
    };
    let fact = StoreAspectPatchBoundaryFact::from_authoritative_patch(
        identity,
        StoreAspectPatchAuthorityInput::new(patch, witness),
    )
    .unwrap();
    let delta = PhysicalWorkAspectDelta::from_patch_boundary_fact(
        bindings.binding_for_slot(0).unwrap(),
        &fact,
    )
    .unwrap();
    assert!(delta.is_partitioned());
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let before = serving.media_counters();

    serving.apply_physical_aspect_delta(delta).unwrap();

    assert_eq!(serving.media_counters(), before);
    serving.close();
}

#[test]
fn native_patch_delta_rejects_fields_outside_the_admitted_mutation_mask() {
    let vocabulary = aspects().vocabulary();
    let key = vocabulary
        .key("store.physical.work.delta-mask")
        .unwrap();
    let shape = aspects()
        .struct_fields()
        .optional("admitted", ScalarAspectType::String)
        .optional("foreign", ScalarAspectType::String)
        .finish()
        .unwrap();
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(vocabulary.identity(901))
        .at_revision(vocabulary.revision(1))
        .struct_with(
            shape,
            aspects().mask_contract().struct_fields(),
            AbsenceLaw::Optional,
            AspectEquivalenceBasis::DeclaredStructFields,
            AspectEvolutionPolicy::Frozen,
        );
    let (_, _, _, witness) = admitted_contract(1);
    let admission = StoreAspectContractAdmission::new(
        worth_store::aspect_native::StoreAspectIdentity::from_aspect_key(key.clone()),
        contract.clone(),
        witness,
    )
    .unwrap()
    .admit_projection_mask(AspectMask::<ProjectionMask>::whole_aspect())
    .unwrap()
    .with_mutation_mask(aspects().mutation_mask().fields(["admitted"]).unwrap())
    .unwrap();
    let bindings = PhysicalSignalAspectBindingSet::from_profile(
        PhysicalWorkProfileDeclaration::new(security_scope(witness), [admission]).unwrap(),
    );
    let patch_mask = aspects().mutation_mask().fields(["foreign"]).unwrap();
    let patch = match aspects()
        .patch()
        .field_level(&contract, &patch_mask)
        .set_field(
            vocabulary.field_key("foreign").unwrap(),
            AspectValue::String("outside-mask".into()),
        )
        .finish()
    {
        TransitionOutcome::Success(patch) => patch,
        outcome => panic!("hostile patch should construct: {outcome:?}"),
    };
    let fact = StoreAspectPatchBoundaryFact::from_authoritative_patch(
        worth_store::aspect_native::StoreAspectIdentity::from_aspect_key(key),
        StoreAspectPatchAuthorityInput::new(patch, witness),
    )
    .unwrap();

    assert_eq!(
        PhysicalWorkAspectDelta::from_patch_boundary_fact(
            bindings.binding_for_slot(0).unwrap(),
            &fact,
        ),
        Err(PhysicalWorkAspectDeltaDenial::MutationMaskMismatch)
    );
}

fn boundary_fact(
    contract: &worth_foundational::AspectContract,
    identity: worth_store::aspect_native::StoreAspectIdentity,
    witness: worth_store::aspect_native::StorePhysicalBoundaryWitness,
    value: &str,
) -> StoreAspectBoundaryFact {
    let state = match aspects()
        .authoritative_state()
        .admit([validated_value(contract, value)])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state should admit: {outcome:?}"),
    };
    StoreAspectBoundaryFact::from_admitted_state(
        identity,
        StoreAspectAuthorityInput::new(state, witness),
    )
    .unwrap()
}
