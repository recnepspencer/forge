use tempfile::tempdir;
use worth_foundational::aspects;
use worth_proof::TransitionOutcome;
use worth_store::aspect_native::{StoreAspectAuthorityInput, StoreAspectBoundaryFact};
use worth_store::physical_runtime::{
    PhysicalReadWorkRequest, PhysicalSignalAspectBindingSet, PhysicalSignalAspectDeclaration,
    PhysicalSignalAspectRole, PhysicalWorkAspectDelta, PhysicalWorkProfileDeclaration,
    PhysicalWorkReadiness, PhysicalWorkScope, PhysicalWorkSemanticBasis,
    PhysicalWorkSignalFamily, PhysicalWorkSignalFamilySet,
};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::fixture::{
    admitted_named_contract, security_scope, serving_from_initialization_with_work_profile,
    validated_value,
};

#[test]
fn disjoint_read_bindings_progress_through_one_graph_and_independent_bounded_routes() {
    let root = tempdir().unwrap();
    let (left_contract, left_identity, left_admission, witness) =
        admitted_named_contract("store.physical.work.left-read", 911, 1);
    let (right_contract, right_identity, right_admission, _) =
        admitted_named_contract("store.physical.work.right-read", 912, 1);
    let profile = PhysicalWorkProfileDeclaration::from_signal_aspects(
        security_scope(witness),
        [
            read_dependency(left_admission.clone()),
            read_dependency(right_admission.clone()),
        ],
    )
    .unwrap();
    let left_fact = projection_fact(
        &left_contract,
        left_identity.clone(),
        witness,
        "left",
    );
    let right_fact = projection_fact(&right_contract, right_identity, witness, "right");
    let left_basis =
        PhysicalWorkSemanticBasis::projection(left_fact.clone(), left_admission).unwrap();
    let right_basis =
        PhysicalWorkSemanticBasis::projection(right_fact, right_admission).unwrap();
    let bindings = PhysicalSignalAspectBindingSet::from_profile(profile.clone());
    let delta = PhysicalWorkAspectDelta::from_boundary_fact(
        bindings.binding_for_identity(&left_identity).unwrap(),
        &left_fact,
    )
    .unwrap();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let topology = serving.physical_signal_observation().unwrap();
    assert_eq!(
        topology.graph_owner_count(),
        1,
        "physical dependency truth must remain in one derived Signal graph"
    );
    assert_eq!(topology.aspect_binding_count(), 2);
    assert_eq!(
        topology.locality_owner_count(),
        2,
        "each admitted locality binding must own a distinct bounded Signal route"
    );
    serving.apply_physical_aspect_delta(delta).unwrap();
    let left = admit_read(
        &serving,
        request(left_basis, witness, RecordArtifactFile::BootstrapCatalog, 0),
    );
    let right = admit_read(
        &serving,
        request(
            right_basis,
            witness,
            RecordArtifactFile::RootManifest { generation: 1 },
            0,
        ),
    );
    let before = serving.media_counters();

    std::thread::scope(|scope| {
        let left = scope.spawn(|| serving.request_physical_work(left));
        let right = scope.spawn(|| serving.request_physical_work(right));
        assert!(matches!(
            left.join().unwrap().unwrap(),
            PhysicalWorkReadiness::Ready(_)
        ));
        assert!(matches!(
            right.join().unwrap().unwrap(),
            PhysicalWorkReadiness::Ready(_)
        ));
    });

    assert_eq!(serving.media_counters(), before);
    serving.close();
}

fn read_dependency(
    admission: worth_store::aspect_native::StoreAspectContractAdmission,
) -> PhysicalSignalAspectDeclaration {
    PhysicalSignalAspectDeclaration::new(admission, PhysicalSignalAspectRole::Dependency)
        .for_families(PhysicalWorkSignalFamilySet::only(
            PhysicalWorkSignalFamily::ReadFault,
        ))
}

fn projection_fact(
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

fn request(
    basis: PhysicalWorkSemanticBasis,
    witness: worth_store::aspect_native::StorePhysicalBoundaryWitness,
    artifact: RecordArtifactFile,
    offset: u64,
) -> PhysicalReadWorkRequest {
    PhysicalReadWorkRequest::new(
        PhysicalWorkScope::one(RecordFrameCoordinate::new(artifact, offset, 8).unwrap()),
        basis,
        security_scope(witness),
    )
    .unwrap()
}

fn admit_read(
    serving: &worth_store::physical_runtime::ServingPhysicalRuntime,
    request: PhysicalReadWorkRequest,
) -> worth_store::physical_runtime::AdmittedPhysicalWork {
    let receipt = match serving
        .physical_read_submission()
        .submit(request)
        .into_raw()
    {
        TransitionOutcome::Success(receipt) => receipt,
        outcome => panic!("read should submit: {outcome:?}"),
    };
    serving.admit_physical_work(receipt).unwrap()
}
