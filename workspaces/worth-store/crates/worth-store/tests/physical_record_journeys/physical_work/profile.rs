use tempfile::tempdir;
use worth_foundational::{aspects, AspectMask, MutationMask, ProjectionMask, ScalarAspectType};
use worth_signal::facade::PartitionSubscription;
use worth_store::aspect_native::{StoreAspectContractAdmission, StoreAspectIdentity};
use worth_store::physical_runtime::{
    PhysicalSignalAspectBindingSet, PhysicalSignalAspectDeclaration, PhysicalSignalAspectRole,
    PhysicalWorkCapacity, PhysicalWorkProfileDeclaration, PhysicalWorkSignalFamily,
    PhysicalWorkSignalFamilySet,
};

use super::fixture::{
    admitted_contract, alternative_physical_witness, security_scope, security_scope_from_authority,
    serving_from_initialization_with_work_profile,
};

#[test]
fn inconsistent_capacity_profiles_have_no_construction_path() {
    assert!(PhysicalWorkCapacity::new(1, 2, 1, 1, 1).is_none());
    assert!(PhysicalWorkCapacity::new(1, 1, 1, 2, 1).is_none());
}

#[test]
fn duplicate_profile_contracts_are_rejected_before_signal_construction() {
    let (_, _, admission, witness) = admitted_contract(1);
    assert!(matches!(
        PhysicalWorkProfileDeclaration::new(
            security_scope(witness),
            [admission.clone(), admission]
        ),
        Err(worth_store::physical_runtime::PhysicalWorkProfileDenial::DuplicateAspectContract)
    ));
}

#[test]
fn profile_identity_is_canonical_across_contract_input_order() {
    let (_, _, first, witness) = admitted_contract(1);
    let second = contract_admission(
        "store.physical.work.secondary",
        72,
        ScalarAspectType::String,
        witness,
    );
    let security = security_scope(witness);
    let forward =
        PhysicalWorkProfileDeclaration::new(security, [first.clone(), second.clone()]).unwrap();
    let reversed = PhysicalWorkProfileDeclaration::new(security, [second, first]).unwrap();
    assert_eq!(forward.identity(), reversed.identity());
}

#[test]
fn profile_identity_includes_the_admitted_security_authority() {
    let (_, _, admission, witness) = admitted_contract(1);
    let local =
        PhysicalWorkProfileDeclaration::new(security_scope(witness), [admission.clone()]).unwrap();
    let foreign = PhysicalWorkProfileDeclaration::new(
        security_scope_from_authority("store.physical.foreign_profile_authority", witness),
        [admission],
    )
    .unwrap();

    assert_ne!(local.identity(), foreign.identity());
}

#[test]
fn binding_role_and_partition_are_canonical_profile_truth() {
    let (_, _, admission, witness) = admitted_contract(1);
    let partitioned_admission = admission
        .clone()
        .admit_mutation_mask(AspectMask::<MutationMask>::whole_aspect())
        .unwrap();
    let security = security_scope(witness);
    let dependency = PhysicalWorkProfileDeclaration::from_signal_aspects(
        security,
        [PhysicalSignalAspectDeclaration::new(
            admission.clone(),
            PhysicalSignalAspectRole::Dependency,
        )],
    )
    .unwrap();
    let partitioned = PhysicalWorkProfileDeclaration::from_signal_aspects(
        security,
        [PhysicalSignalAspectDeclaration::new(
            partitioned_admission,
            PhysicalSignalAspectRole::DependencyAndOutput,
        )
        .with_partition(PartitionSubscription::whole_partition("artifact-7"))],
    )
    .unwrap();
    assert_ne!(dependency.identity(), partitioned.identity());

    let dependency = PhysicalSignalAspectBindingSet::from_profile(dependency);
    let partitioned = PhysicalSignalAspectBindingSet::from_profile(partitioned);
    let left = dependency.binding_for_slot(0).unwrap();
    let right = partitioned.binding_for_slot(0).unwrap();
    assert_ne!(left.digest(), right.digest());
    assert!(left.partition().is_none());
    assert!(right.partition().is_some());
    assert_eq!(
        dependency
            .binding_for_identity(left.identity())
            .unwrap()
            .digest(),
        left.digest()
    );
}

#[test]
fn work_family_scope_is_canonical_and_empty_scope_is_rejected() {
    let (_, _, admission, witness) = admitted_contract(1);
    let security = security_scope(witness);
    let all = PhysicalWorkProfileDeclaration::from_signal_aspects(
        security,
        [PhysicalSignalAspectDeclaration::new(
            admission.clone(),
            PhysicalSignalAspectRole::Dependency,
        )],
    )
    .unwrap();
    let read_only = PhysicalWorkProfileDeclaration::from_signal_aspects(
        security,
        [PhysicalSignalAspectDeclaration::new(
            admission.clone(),
            PhysicalSignalAspectRole::Dependency,
        )
        .for_families(PhysicalWorkSignalFamilySet::only(
            PhysicalWorkSignalFamily::ReadFault,
        ))],
    )
    .unwrap();
    assert_ne!(all.identity(), read_only.identity());
    assert_ne!(
        PhysicalSignalAspectBindingSet::from_profile(all)
            .binding_for_slot(0)
            .unwrap()
            .digest(),
        PhysicalSignalAspectBindingSet::from_profile(read_only)
            .binding_for_slot(0)
            .unwrap()
            .digest()
    );
    assert!(matches!(
        PhysicalWorkProfileDeclaration::from_signal_aspects(
            security,
            [PhysicalSignalAspectDeclaration::new(
                admission,
                PhysicalSignalAspectRole::Dependency,
            )
            .for_families(PhysicalWorkSignalFamilySet::none())],
        ),
        Err(worth_store::physical_runtime::PhysicalWorkProfileDenial::WorkFamilySetEmpty)
    ));
}

#[test]
fn partitioned_dependency_profile_initializes_the_real_signal_owner() {
    let root = tempdir().unwrap();
    let (_, _, admission, witness) = admitted_contract(1);
    let profile = PhysicalWorkProfileDeclaration::from_signal_aspects(
        security_scope(witness),
        [PhysicalSignalAspectDeclaration::new(
            admission,
            PhysicalSignalAspectRole::Dependency,
        )
        .with_partition(PartitionSubscription::whole_partition("artifact-7"))],
    )
    .unwrap();

    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    assert_eq!(
        serving
            .physical_signal_observation()
            .unwrap()
            .aspect_binding_count(),
        1
    );
    serving.close();
}

#[test]
fn profile_identity_changes_with_exact_contract_shape_and_physical_witness() {
    let (_, _, string, witness) = admitted_contract(1);
    let integer = contract_admission(
        "store.physical.work.lifecycle",
        71,
        ScalarAspectType::UInt64,
        witness,
    );
    let alternate_witness = StoreAspectContractAdmission::new(
        integer.identity().clone(),
        integer.contract().clone(),
        alternative_physical_witness(),
    )
    .unwrap()
    .admit_projection_mask(AspectMask::<ProjectionMask>::whole_aspect())
    .unwrap();
    assert_ne!(
        PhysicalWorkProfileDeclaration::new(security_scope(witness), [string])
            .unwrap()
            .identity(),
        PhysicalWorkProfileDeclaration::new(security_scope(witness), [integer])
            .unwrap()
            .identity()
    );
    assert_ne!(
        PhysicalWorkProfileDeclaration::new(
            security_scope(witness),
            [contract_admission(
                "store.physical.work.lifecycle",
                71,
                ScalarAspectType::UInt64,
                witness,
            )],
        )
        .unwrap()
        .identity(),
        PhysicalWorkProfileDeclaration::new(security_scope(witness), [alternate_witness])
            .unwrap()
            .identity()
    );
}

#[test]
fn signal_aspect_width_is_rejected_before_worker_construction() {
    let (_, _, _, witness) = admitted_contract(1);
    let contracts = (0..=worth_signal::facade::MAX_ASPECTS)
        .map(|index| {
            contract_admission(
                &format!("store.physical.work.profile.{index}"),
                1_000 + index as u64,
                ScalarAspectType::String,
                witness,
            )
        })
        .collect::<Vec<_>>();
    assert!(matches!(
        PhysicalWorkProfileDeclaration::new(security_scope(witness), contracts),
        Err(worth_store::physical_runtime::PhysicalWorkProfileDenial::SignalAspectCapacityExceeded)
    ));
}

#[test]
fn signal_aspect_capacity_denial_stops_consuming_the_profile_source() {
    let (_, _, admission, witness) = admitted_contract(1);
    let hostile_unbounded_source =
        std::iter::repeat(admission)
            .enumerate()
            .map(|(index, admission)| {
                assert!(
                    index <= worth_signal::facade::MAX_ASPECTS,
                    "profile construction consumed beyond its declared Signal capacity"
                );
                admission
            });

    assert!(matches!(
        PhysicalWorkProfileDeclaration::new(
            security_scope(witness),
            hostile_unbounded_source,
        ),
        Err(worth_store::physical_runtime::PhysicalWorkProfileDenial::SignalAspectCapacityExceeded)
    ));
}

fn contract_admission(
    key: &str,
    identity: u64,
    scalar: ScalarAspectType,
    witness: worth_store::aspect_native::StorePhysicalBoundaryWitness,
) -> StoreAspectContractAdmission {
    let key = aspects().vocabulary().key(key).unwrap();
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(identity))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(scalar);
    StoreAspectContractAdmission::new(StoreAspectIdentity::from_aspect_key(key), contract, witness)
        .unwrap()
        .admit_projection_mask(AspectMask::<ProjectionMask>::whole_aspect())
        .unwrap()
}
