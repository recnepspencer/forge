use std::path::Path;

use worth_foundational::{aspects, AspectContract};
use worth_proof::TransitionOutcome;
use worth_signal::facade::PartitionSubscription;
use worth_store::aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectContractAdmission,
    StoreAspectIdentity, StoreAspectPatchAuthorityInput, StoreAspectPatchBoundaryFact,
    StorePhysicalBoundaryWitness,
};
use worth_store::physical_runtime::{
    FilesystemMediaAdmission, PhysicalMutationWorkRequest, PhysicalReadWorkRequest,
    PhysicalRecordOpen, PhysicalRuntimeAdmission, PhysicalSignalAspectBindingSet,
    PhysicalSignalAspectDeclaration, PhysicalSignalAspectRole, PhysicalStore,
    PhysicalWorkAspectDelta, PhysicalWorkCapacity, PhysicalWorkProfileDeclaration,
    PhysicalWorkScope, PhysicalWorkSemanticBasis, PhysicalWorkSignalFamily,
    PhysicalWorkSignalFamilySet, ServingPhysicalRuntime,
};
use worth_store_physical_backend::{
    ArtifactRangeWriteDurabilityRequirement, CertificationMediaFaultActivation,
    CertificationMediaFaultAuthority, FilesystemAccessPosture, MediaFaultDirective, MediaFaultRule,
    MediaOperationRole, MediaPauseGate,
};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};
use worth_store_security::StoreAuthorityBoundSecurityScopeReceipt;

pub(super) struct MaelstromFixture {
    pub profile: PhysicalWorkProfileDeclaration,
    pub reads: [PhysicalReadWorkRequest; 2],
    pub write: PhysicalMutationWorkRequest,
    pub read_delta: PhysicalWorkAspectDelta,
}

pub(super) struct MaelstromPauseGates {
    pub first_read: MediaPauseGate,
    pub second_read: MediaPauseGate,
    pub close_read: MediaPauseGate,
    pub close_read_activation: CertificationMediaFaultActivation,
    pub first_append: MediaPauseGate,
    pub second_append: MediaPauseGate,
}

struct AdmittedBinding {
    contract: AspectContract,
    identity: StoreAspectIdentity,
    admission: StoreAspectContractAdmission,
}

struct MaelstromBindings {
    first_read: AdmittedBinding,
    second_read: AdmittedBinding,
    write: AdmittedBinding,
    witness: StorePhysicalBoundaryWitness,
    security: StoreAuthorityBoundSecurityScopeReceipt,
}

struct MaelstromSemanticBases {
    first_read: PhysicalWorkSemanticBasis,
    second_read: PhysicalWorkSemanticBasis,
    write: PhysicalWorkSemanticBasis,
    first_read_fact: StoreAspectBoundaryFact,
}

pub(super) fn maelstrom_fixture() -> MaelstromFixture {
    let admitted = admitted_bindings();
    let profile = physical_work_profile(&admitted);
    let bases = semantic_bases(&admitted);
    let first_scope = PhysicalWorkScope::one(
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 8).unwrap(),
    );
    let second_scope = PhysicalWorkScope::one(
        RecordFrameCoordinate::new(RecordArtifactFile::RootManifest { generation: 1 }, 0, 8)
            .unwrap(),
    );
    let profile_bindings = PhysicalSignalAspectBindingSet::from_profile(profile.clone());
    let read_delta = PhysicalWorkAspectDelta::from_boundary_fact(
        profile_bindings
            .binding_for_identity(&admitted.first_read.identity)
            .unwrap(),
        &bases.first_read_fact,
        first_scope.clone(),
    )
    .unwrap();
    let reads = [
        PhysicalReadWorkRequest::new(first_scope, bases.first_read, admitted.security).unwrap(),
        PhysicalReadWorkRequest::new(second_scope, bases.second_read, admitted.security).unwrap(),
    ];
    let write = PhysicalMutationWorkRequest::exact_write(
        PhysicalWorkScope::one(
            RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap(),
        ),
        bases.write,
        admitted.security,
        ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
    )
    .unwrap();
    MaelstromFixture {
        profile,
        reads,
        write,
        read_delta,
    }
}

fn admitted_bindings() -> MaelstromBindings {
    let (contract, identity, admission, witness) = super::super::fixture::admitted_named_contract(
        "store.physical.work.phase-16-read-left",
        961,
        1,
    );
    let first_read = AdmittedBinding {
        contract,
        identity,
        admission,
    };
    let (contract, identity, admission, _) = super::super::fixture::admitted_named_contract(
        "store.physical.work.phase-16-read-right",
        962,
        1,
    );
    let second_read = AdmittedBinding {
        contract,
        identity,
        admission,
    };
    let (contract, identity, admission, _) = super::super::fixture::admitted_named_contract(
        "store.physical.work.phase-16-write",
        963,
        1,
    );
    MaelstromBindings {
        first_read,
        second_read,
        write: AdmittedBinding {
            contract,
            identity,
            admission,
        },
        witness,
        security: super::super::fixture::security_scope(witness),
    }
}

fn physical_work_profile(bindings: &MaelstromBindings) -> PhysicalWorkProfileDeclaration {
    PhysicalWorkProfileDeclaration::from_signal_aspects(
        bindings.security,
        [
            read_aspect(&bindings.first_read, "bootstrap-catalog", "frame-0"),
            read_aspect(
                &bindings.second_read,
                "root-manifest",
                "generation-1-frame-0",
            ),
            PhysicalSignalAspectDeclaration::new(
                bindings.write.admission.clone(),
                PhysicalSignalAspectRole::DependencyAndOutput,
            )
            .with_partition(PartitionSubscription::partition_and_detail(
                "bootstrap-catalog",
                "frame-0",
            )),
        ],
    )
    .unwrap()
    .with_capacity(
        PhysicalWorkCapacity::new(32, 4, 32, 16_384, 32_768)
            .unwrap()
            .with_terminal_evidence_capacity(64)
            .unwrap(),
    )
}

fn read_aspect(
    binding: &AdmittedBinding,
    partition: &str,
    detail: &str,
) -> PhysicalSignalAspectDeclaration {
    PhysicalSignalAspectDeclaration::new(
        binding.admission.clone(),
        PhysicalSignalAspectRole::Dependency,
    )
    .with_partition(PartitionSubscription::partition_and_detail(
        partition, detail,
    ))
    .for_families(PhysicalWorkSignalFamilySet::only(
        PhysicalWorkSignalFamily::ReadFault,
    ))
}

fn semantic_bases(bindings: &MaelstromBindings) -> MaelstromSemanticBases {
    let (first_read, first_read_fact) =
        read_basis(&bindings.first_read, bindings.witness, "available");
    let (second_read, _) = read_basis(&bindings.second_read, bindings.witness, "available");
    let write = write_basis(&bindings.write, bindings.witness);
    MaelstromSemanticBases {
        first_read,
        second_read,
        write,
        first_read_fact,
    }
}

fn read_basis(
    binding: &AdmittedBinding,
    witness: StorePhysicalBoundaryWitness,
    value: &str,
) -> (PhysicalWorkSemanticBasis, StoreAspectBoundaryFact) {
    let state =
        match aspects()
            .authoritative_state()
            .admit([super::super::fixture::validated_value(
                &binding.contract,
                value,
            )]) {
            TransitionOutcome::Success(state) => state,
            outcome => panic!("Phase 16 read state should admit: {outcome:?}"),
        };
    let fact = StoreAspectBoundaryFact::from_admitted_state(
        binding.identity.clone(),
        StoreAspectAuthorityInput::new(state, witness),
    )
    .unwrap();
    let basis =
        PhysicalWorkSemanticBasis::projection(fact.clone(), binding.admission.clone()).unwrap();
    (basis, fact)
}

fn write_basis(
    binding: &AdmittedBinding,
    witness: StorePhysicalBoundaryWitness,
) -> PhysicalWorkSemanticBasis {
    let patch = match aspects()
        .patch()
        .whole_aspect()
        .set(super::super::fixture::validated_value(
            &binding.contract,
            "eligible",
        ))
        .finish()
    {
        TransitionOutcome::Success(patch) => patch,
        outcome => panic!("Phase 16 write patch should admit: {outcome:?}"),
    };
    let fact = StoreAspectPatchBoundaryFact::from_authoritative_patch(
        binding.identity.clone(),
        StoreAspectPatchAuthorityInput::new(patch, witness),
    )
    .unwrap();
    PhysicalWorkSemanticBasis::mutation(fact, binding.admission.clone()).unwrap()
}

pub(super) fn open_with_maelstrom_faults(
    root: &Path,
    profile: PhysicalWorkProfileDeclaration,
    identified_reads_after_open: u64,
    identified_writes_after_open: u64,
) -> (ServingPhysicalRuntime, MaelstromPauseGates) {
    let (admission, gates) =
        faulted_admission(identified_reads_after_open, identified_writes_after_open);
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    let media = match runtime.try_admit_filesystem_media(admission).into_raw() {
        TransitionOutcome::Success(media) => media,
        _ => panic!("Phase 16 faulted media should admit"),
    };
    let (format, _, access) = super::super::configuration();
    let serving = super::super::success(open_record_store!(media, |durability| {
        PhysicalRecordOpen::new(format, access, durability).with_physical_work_profile(profile)
    },));
    (serving, gates)
}

fn faulted_admission(
    identified_reads_after_open: u64,
    identified_writes_after_open: u64,
) -> (FilesystemMediaAdmission, MaelstromPauseGates) {
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let gates = MaelstromPauseGates {
        first_read: authority.pause_gate(),
        second_read: authority.pause_gate(),
        close_read: authority.pause_gate(),
        close_read_activation: authority.one_shot_activation(),
        first_append: authority.pause_gate(),
        second_append: authority.pause_gate(),
    };
    let mut rules = read_fault_rules(&authority, identified_reads_after_open, &gates).to_vec();
    rules.extend(write_fault_rules(
        &authority,
        identified_writes_after_open,
        &gates,
    ));
    let schedule = authority.schedule(rules).unwrap();
    (admission.with_fault_schedule(schedule), gates)
}

fn read_fault_rules(
    authority: &CertificationMediaFaultAuthority,
    after_open: u64,
    gates: &MaelstromPauseGates,
) -> [MediaFaultRule; 3] {
    [
        authority
            .rule(
                MediaOperationRole::PositionedRead,
                after_open + 1,
                MediaFaultDirective::PauseBefore(gates.first_read.clone()),
            )
            .for_identified_operation_ordinal(),
        authority
            .rule(
                MediaOperationRole::PositionedRead,
                after_open + 2,
                MediaFaultDirective::PauseBefore(gates.second_read.clone()),
            )
            .for_identified_operation_ordinal(),
        authority
            .rule(
                MediaOperationRole::PositionedRead,
                1,
                MediaFaultDirective::PauseBefore(gates.close_read.clone()),
            )
            .for_next_identified_operation_after_activation(gates.close_read_activation.clone()),
    ]
}

fn write_fault_rules(
    authority: &CertificationMediaFaultAuthority,
    after_open: u64,
    gates: &MaelstromPauseGates,
) -> [MediaFaultRule; 3] {
    [
        authority
            .rule(
                MediaOperationRole::PositionedWrite,
                after_open + 1,
                MediaFaultDirective::FailBefore {
                    kind: std::io::ErrorKind::Other,
                    raw_os_error: None,
                },
            )
            .for_identified_operation_ordinal(),
        authority
            .rule(
                MediaOperationRole::PositionedWrite,
                after_open + 4,
                MediaFaultDirective::PauseBefore(gates.first_append.clone()),
            )
            .for_identified_operation_ordinal(),
        authority
            .rule(
                MediaOperationRole::PositionedWrite,
                after_open + 5,
                MediaFaultDirective::PauseBefore(gates.second_append.clone()),
            )
            .for_identified_operation_ordinal(),
    ]
}
