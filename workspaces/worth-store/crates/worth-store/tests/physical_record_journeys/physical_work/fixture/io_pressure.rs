use worth_foundational::aspects;
use worth_proof::TransitionOutcome;
use worth_store::aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectPatchAuthorityInput,
    StoreAspectPatchBoundaryFact,
};
use worth_store::physical_runtime::{
    PhysicalMutationWorkRequest, PhysicalReadWorkRequest, PhysicalWorkProfileDeclaration,
    PhysicalWorkScope, PhysicalWorkSemanticBasis,
};
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

pub(in super::super) fn disjoint_io_pressure_fixture() -> (
    PhysicalWorkProfileDeclaration,
    [PhysicalReadWorkRequest; 2],
    [PhysicalMutationWorkRequest; 2],
) {
    let (contract, identity, contract_admission, witness) = super::admitted_contract(1);
    let security = super::security_scope(witness);
    let profile =
        PhysicalWorkProfileDeclaration::new(security, [contract_admission.clone()]).unwrap();
    let read_state = match aspects()
        .authoritative_state()
        .admit([super::validated_value(&contract, "pressure-read")])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("pressure read state should admit: {outcome:?}"),
    };
    let read_fact = StoreAspectBoundaryFact::from_admitted_state(
        identity.clone(),
        StoreAspectAuthorityInput::new(read_state, witness),
    )
    .unwrap();
    let read_basis =
        PhysicalWorkSemanticBasis::projection(read_fact, contract_admission.clone()).unwrap();
    let patch = match aspects()
        .patch()
        .whole_aspect()
        .set(super::validated_value(&contract, "pressure-write"))
        .finish()
    {
        TransitionOutcome::Success(patch) => patch,
        outcome => panic!("pressure mutation patch should admit: {outcome:?}"),
    };
    let patch_fact = StoreAspectPatchBoundaryFact::from_authoritative_patch(
        identity,
        StoreAspectPatchAuthorityInput::new(patch, witness),
    )
    .unwrap();
    let write_basis = PhysicalWorkSemanticBasis::mutation(patch_fact, contract_admission).unwrap();
    let bootstrap_read =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 8).unwrap();
    let root_read =
        RecordFrameCoordinate::new(RecordArtifactFile::RootManifest { generation: 1 }, 0, 8)
            .unwrap();
    let bootstrap_write =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap();
    let root_write =
        RecordFrameCoordinate::new(RecordArtifactFile::RootManifest { generation: 1 }, 8, 8)
            .unwrap();
    (
        profile,
        [
            read_request(bootstrap_read, read_basis.clone(), security),
            read_request(root_read, read_basis, security),
        ],
        [
            write_request(bootstrap_write, write_basis.clone(), security),
            write_request(root_write, write_basis, security),
        ],
    )
}

fn read_request(
    coordinate: RecordFrameCoordinate,
    basis: PhysicalWorkSemanticBasis,
    security: worth_store_security::StoreAuthorityBoundSecurityScopeReceipt,
) -> PhysicalReadWorkRequest {
    PhysicalReadWorkRequest::new(PhysicalWorkScope::one(coordinate), basis, security).unwrap()
}

fn write_request(
    coordinate: RecordFrameCoordinate,
    basis: PhysicalWorkSemanticBasis,
    security: worth_store_security::StoreAuthorityBoundSecurityScopeReceipt,
) -> PhysicalMutationWorkRequest {
    PhysicalMutationWorkRequest::exact_write(
        PhysicalWorkScope::one(coordinate),
        basis,
        security,
        ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
    )
    .unwrap()
}
