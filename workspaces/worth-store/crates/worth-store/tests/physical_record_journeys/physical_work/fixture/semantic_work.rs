use worth_foundational::aspects;
use worth_proof::TransitionOutcome;
use worth_store::aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectPatchAuthorityInput,
    StoreAspectPatchBoundaryFact,
};
use worth_store::physical_runtime::{
    PhysicalMutationWorkRequest, PhysicalReadWorkRequest, PhysicalSignalAspectBindingSet,
    PhysicalSignalAspectDeclaration, PhysicalSignalAspectRole, PhysicalWorkAspectDelta,
    PhysicalWorkProfileDeclaration, PhysicalWorkScope, PhysicalWorkSemanticBasis,
    PhysicalWorkSignalFamily, PhysicalWorkSignalFamilySet,
};
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::authority::{
    admitted_contract, admitted_named_contract, security_scope, validated_value,
};

pub(in crate::physical_work) const EXPECTED_NATIVE_RECORD_BINDING_COUNT: u16 = 12;

pub(crate) fn work_fixture() -> (
    PhysicalWorkProfileDeclaration,
    PhysicalReadWorkRequest,
    PhysicalMutationWorkRequest,
) {
    let (contract, identity, contract_admission, physical_witness) = admitted_contract(1);
    let security = security_scope(physical_witness);
    let profile =
        PhysicalWorkProfileDeclaration::new(security, [contract_admission.clone()]).unwrap();
    let read_state = match aspects()
        .authoritative_state()
        .admit([validated_value(&contract, "read")])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state should admit: {outcome:?}"),
    };
    let read_fact = StoreAspectBoundaryFact::from_admitted_state(
        identity.clone(),
        StoreAspectAuthorityInput::new(read_state, physical_witness),
    )
    .unwrap();
    let read_basis =
        PhysicalWorkSemanticBasis::projection(read_fact, contract_admission.clone()).unwrap();
    let patch = match aspects()
        .patch()
        .whole_aspect()
        .set(validated_value(&contract, "mutation"))
        .finish()
    {
        TransitionOutcome::Success(patch) => patch,
        outcome => panic!("patch should construct: {outcome:?}"),
    };
    let patch_fact = StoreAspectPatchBoundaryFact::from_authoritative_patch(
        identity,
        StoreAspectPatchAuthorityInput::new(patch, physical_witness),
    )
    .unwrap();
    let mutation_basis =
        PhysicalWorkSemanticBasis::mutation(patch_fact, contract_admission).unwrap();
    let read_scope = PhysicalWorkScope::one(
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 8).unwrap(),
    );
    let mutation_scope = PhysicalWorkScope::one(
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap(),
    );
    (
        profile,
        PhysicalReadWorkRequest::new(read_scope, read_basis, security).unwrap(),
        PhysicalMutationWorkRequest::exact_write(
            mutation_scope,
            mutation_basis,
            security,
            ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
        )
        .unwrap(),
    )
}

pub(in crate::physical_work) fn matching_aspect_delta(
    revision: u64,
    value: &str,
) -> PhysicalWorkAspectDelta {
    let (contract, identity, admission, witness) = admitted_contract(revision);
    let bindings = PhysicalSignalAspectBindingSet::from_profile(
        PhysicalWorkProfileDeclaration::new(security_scope(witness), [admission]).unwrap(),
    );
    let state = match aspects()
        .authoritative_state()
        .admit([validated_value(&contract, value)])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state should admit: {outcome:?}"),
    };
    let fact = StoreAspectBoundaryFact::from_admitted_state(
        identity,
        StoreAspectAuthorityInput::new(state, witness),
    )
    .unwrap();
    PhysicalWorkAspectDelta::from_boundary_fact(
        bindings.binding_for_slot(0).unwrap(),
        &fact,
        PhysicalWorkScope::one(
            RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 8).unwrap(),
        ),
    )
    .unwrap()
}

pub(in crate::physical_work) fn family_locality_fixture() -> (
    PhysicalWorkProfileDeclaration,
    PhysicalReadWorkRequest,
    PhysicalMutationWorkRequest,
    PhysicalWorkAspectDelta,
) {
    let (read_contract, read_identity, read_admission, witness) =
        admitted_named_contract("store.physical.work.read-availability", 81, 1);
    let (write_contract, write_identity, write_admission, _) =
        admitted_named_contract("store.physical.work.write-eligibility", 82, 1);
    let profile = PhysicalWorkProfileDeclaration::from_signal_aspects(
        security_scope(witness),
        [
            PhysicalSignalAspectDeclaration::new(
                read_admission.clone(),
                PhysicalSignalAspectRole::Dependency,
            )
            .for_families(PhysicalWorkSignalFamilySet::only(
                PhysicalWorkSignalFamily::ReadFault,
            )),
            PhysicalSignalAspectDeclaration::new(
                write_admission.clone(),
                PhysicalSignalAspectRole::DependencyAndOutput,
            )
            .for_families(PhysicalWorkSignalFamilySet::only(
                PhysicalWorkSignalFamily::ExactWriteback,
            )),
        ],
    )
    .unwrap();
    let read_state = match aspects()
        .authoritative_state()
        .admit([validated_value(&read_contract, "available")])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state should admit: {outcome:?}"),
    };
    let read_fact = StoreAspectBoundaryFact::from_admitted_state(
        read_identity.clone(),
        StoreAspectAuthorityInput::new(read_state, witness),
    )
    .unwrap();
    let read_basis =
        PhysicalWorkSemanticBasis::projection(read_fact.clone(), read_admission).unwrap();
    let write_patch = match aspects()
        .patch()
        .whole_aspect()
        .set(validated_value(&write_contract, "eligible"))
        .finish()
    {
        TransitionOutcome::Success(patch) => patch,
        outcome => panic!("patch should construct: {outcome:?}"),
    };
    let write_fact = StoreAspectPatchBoundaryFact::from_authoritative_patch(
        write_identity,
        StoreAspectPatchAuthorityInput::new(write_patch, witness),
    )
    .unwrap();
    let write_basis = PhysicalWorkSemanticBasis::mutation(write_fact, write_admission).unwrap();
    let bindings = PhysicalSignalAspectBindingSet::from_profile(profile.clone());
    let read_binding = bindings.binding_for_identity(&read_identity).unwrap();
    let read_scope = PhysicalWorkScope::one(
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 8).unwrap(),
    );
    let delta =
        PhysicalWorkAspectDelta::from_boundary_fact(read_binding, &read_fact, read_scope.clone())
            .unwrap();
    let security = security_scope(witness);
    let read = PhysicalReadWorkRequest::new(read_scope, read_basis, security).unwrap();
    let write = PhysicalMutationWorkRequest::exact_write(
        PhysicalWorkScope::one(
            RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap(),
        ),
        write_basis,
        security,
        ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
    )
    .unwrap();
    (profile, read, write, delta)
}
