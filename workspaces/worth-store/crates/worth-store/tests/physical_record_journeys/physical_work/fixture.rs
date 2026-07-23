use std::path::Path;

use worth_foundational::{
    aspects, AspectMask, AspectValue, ContractValidationInput, InternedString, MutationMask,
    ProjectionMask, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store::aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectContractAdmission,
    StoreAspectIdentity, StoreAspectPatchAuthorityInput, StoreAspectPatchBoundaryFact,
    StorePhysicalAuthorityWitness, StorePhysicalBoundaryWitness,
    ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
};
use worth_store::physical_runtime::{
    PhysicalMutationWorkRequest, PhysicalReadWorkRequest, PhysicalSignalAspectBindingSet,
    PhysicalSignalAspectDeclaration, PhysicalSignalAspectRole, PhysicalWorkAspectDelta,
    PhysicalWorkProfileDeclaration, PhysicalWorkScope,
    PhysicalWorkSemanticBasis, PhysicalWorkSignalFamily, PhysicalWorkSignalFamilySet,
    ServingPhysicalRuntime,
};
use worth_store_contracts::ROADMAP_2_REPLAY_PHYSICAL_BOUNDARY;
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};
use worth_store_security::{
    StoreAuthorityBoundSecurityScopeReceipt,
};

pub(super) fn serving_from_initialization_with_work_profile(
    root: &Path,
    profile: PhysicalWorkProfileDeclaration,
) -> ServingPhysicalRuntime {
    let (format, placement, access) = super::configuration();
    super::success(
        super::media(root).initialize_record_store(
            worth_store::physical_runtime::PhysicalRecordInitialization::new(
                format, placement, access,
            )
            .with_physical_work_profile(profile),
        ),
    )
}

pub(super) fn serving_from_open_with_work_profile(
    root: &Path,
    profile: PhysicalWorkProfileDeclaration,
) -> ServingPhysicalRuntime {
    let (format, _, access) = super::configuration();
    super::success(
        super::media(root).open_record_store(
            worth_store::physical_runtime::PhysicalRecordOpen::new(format, access)
                .with_physical_work_profile(profile),
        ),
    )
}

pub(super) fn work_fixture() -> (
    PhysicalWorkProfileDeclaration,
    PhysicalReadWorkRequest,
    PhysicalMutationWorkRequest,
) {
    let (contract, identity, contract_admission, physical_witness) = admitted_contract(1);
    let profile = PhysicalWorkProfileDeclaration::new([contract_admission.clone()]).unwrap();
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
    let security = security_scope(physical_witness);
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

pub(super) fn matching_aspect_delta(revision: u64, value: &str) -> PhysicalWorkAspectDelta {
    let (contract, identity, admission, witness) = admitted_contract(revision);
    let bindings = PhysicalSignalAspectBindingSet::from_profile(
        PhysicalWorkProfileDeclaration::new([admission]).unwrap(),
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
    PhysicalWorkAspectDelta::from_boundary_fact(bindings.binding_for_slot(0).unwrap(), &fact)
        .unwrap()
}

pub(super) fn family_locality_fixture() -> (
    PhysicalWorkProfileDeclaration,
    PhysicalReadWorkRequest,
    PhysicalMutationWorkRequest,
    PhysicalWorkAspectDelta,
) {
    let (read_contract, read_identity, read_admission, witness) =
        admitted_named_contract("store.physical.work.read-availability", 81, 1);
    let (write_contract, write_identity, write_admission, _) =
        admitted_named_contract("store.physical.work.write-eligibility", 82, 1);
    let profile = PhysicalWorkProfileDeclaration::from_signal_aspects([
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
    ])
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
    let delta = PhysicalWorkAspectDelta::from_boundary_fact(read_binding, &read_fact).unwrap();
    let security = security_scope(witness);
    let read = PhysicalReadWorkRequest::new(
        PhysicalWorkScope::one(
            RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 8).unwrap(),
        ),
        read_basis,
        security,
    )
    .unwrap();
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

pub(super) fn disjoint_mutation_fixture() -> (
    PhysicalWorkProfileDeclaration,
    PhysicalMutationWorkRequest,
    PhysicalMutationWorkRequest,
) {
    let (contract, identity, contract_admission, physical_witness) = admitted_contract(1);
    let patch = match aspects()
        .patch()
        .whole_aspect()
        .set(validated_value(&contract, "disjoint-mutations"))
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
    let basis =
        PhysicalWorkSemanticBasis::mutation(patch_fact, contract_admission.clone()).unwrap();
    let request = |offset, basis| {
        PhysicalMutationWorkRequest::exact_write(
            PhysicalWorkScope::one(
                RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, offset, 8)
                    .unwrap(),
            ),
            basis,
            security_scope(physical_witness),
            ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
        )
        .unwrap()
    };
    (
        PhysicalWorkProfileDeclaration::new([contract_admission]).unwrap(),
        request(8, basis.clone()),
        request(16, basis),
    )
}

pub(super) fn admitted_contract(
    revision: u64,
) -> (
    worth_foundational::AspectContract,
    StoreAspectIdentity,
    StoreAspectContractAdmission,
    StorePhysicalBoundaryWitness,
) {
    admitted_named_contract("store.physical.work.lifecycle", 71, revision)
}

pub(super) fn admitted_named_contract(
    key: &str,
    identity_value: u64,
    revision: u64,
) -> (
    worth_foundational::AspectContract,
    StoreAspectIdentity,
    StoreAspectContractAdmission,
    StorePhysicalBoundaryWitness,
) {
    let key = aspects().vocabulary().key(key).unwrap();
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(identity_value))
        .at_revision(aspects().vocabulary().revision(revision))
        .scalar(ScalarAspectType::String);
    let witness = StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap();
    let identity = StoreAspectIdentity::from_aspect_key(key);
    let admission = StoreAspectContractAdmission::new(identity.clone(), contract.clone(), witness)
        .unwrap()
        .admit_projection_mask(AspectMask::<ProjectionMask>::whole_aspect())
        .unwrap()
        .admit_mutation_mask(AspectMask::<MutationMask>::whole_aspect())
        .unwrap();
    (contract, identity, admission, witness)
}

pub(super) fn alternative_physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary_instance(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
            ROADMAP_2_REPLAY_PHYSICAL_BOUNDARY,
        )
        .unwrap(),
    )
    .unwrap()
}

pub(super) fn security_scope(
    witness: StorePhysicalBoundaryWitness,
) -> StoreAuthorityBoundSecurityScopeReceipt {
    worth_store_security::admitted_store_internal_security_scope_for_physical_witness_test(witness)
        .authority_bound_receipt()
}

pub(super) fn validated_value(
    contract: &worth_foundational::AspectContract,
    value: &str,
) -> worth_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(ContractValidationInput::from(AspectValue::String(
            InternedString::from(value),
        ))) {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("value should validate: {outcome:?}"),
    }
}
