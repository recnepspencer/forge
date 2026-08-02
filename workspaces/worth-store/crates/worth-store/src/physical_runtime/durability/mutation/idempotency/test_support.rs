use std::num::{NonZeroU32, NonZeroU64};

use worth_proof::TransitionOutcome;
use worth_store_physical_backend::FilesystemAccessPosture;
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::registry::PhysicalMutationIdempotencyRegistry;
use crate::physical_runtime::{
    durability::mutation::{
        request::PhysicalMutationDurabilityRequest, PhysicalMutationFingerprintInput,
        PhysicalMutationOperationFamily, PhysicalMutationPayloadDigest,
        PhysicalMutationRequestScope, PhysicalMutationSecurityBasis,
    },
    CheckpointMemoryLimit, FilesystemMediaAdmission, GroupCommitDelay, GroupCommitLimit,
    IdempotencyRetentionGenerations, LifecycleGeneration, LiveIdempotencyBindingLimit,
    PendingUnresolvedMutationLimit, PhysicalCheckpointPolicy, PhysicalDurabilityDeclaration,
    PhysicalDurabilityPolicyIdentity, PhysicalIdempotencyPolicy,
    PhysicalMutationRequestFingerprint, PhysicalOperationIdentity, PhysicalRuntimeAdmission,
    PhysicalStore, PhysicalWalPolicy, PhysicalWorkGeneration, PhysicalWorkIdentity,
    RetainedWalTailLimit, RuntimeIdentity, WalSegmentByteLimit, WalSegmentInventoryLimit,
};

pub(super) struct RegistryFixture {
    pub(super) _root: tempfile::TempDir,
    pub(super) media: crate::physical_runtime::MediaOwnedPhysicalRuntime,
    pub(super) registry: PhysicalMutationIdempotencyRegistry,
    pub(super) store: StableStoreIdentity,
    pub(super) runtime: RuntimeIdentity,
    pub(super) generation: LifecycleGeneration,
    pub(super) policy: PhysicalDurabilityPolicyIdentity,
    pub(super) foreign_policy: PhysicalDurabilityPolicyIdentity,
    pub(super) idempotency: PhysicalIdempotencyPolicy,
}

pub(super) fn fixture(pending: u32) -> RegistryFixture {
    fixture_with_limits(pending, 16)
}

pub(super) fn fixture_with_limits(pending: u32, live: u32) -> RegistryFixture {
    let root = tempfile::tempdir().unwrap();
    let runtime =
        PhysicalStore::admit(PhysicalRuntimeAdmission::new(root.path()).unwrap()).unwrap();
    let media = match runtime
        .try_admit_filesystem_media(FilesystemMediaAdmission::production(
            FilesystemAccessPosture::CoordinatedServiceAccount,
        ))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("test media must admit"),
    };
    let store = media.store_identity();
    let runtime = media.runtime_identity();
    let generation = media.observer().snapshot().unwrap().generation();
    let policy = admitted_policy_identity(&media, 32);
    let foreign_policy = admitted_policy_identity(&media, 64);
    let idempotency = idempotency_policy(pending, live);
    let registry =
        PhysicalMutationIdempotencyRegistry::generation_zero(store, runtime, policy, idempotency);
    RegistryFixture {
        _root: root,
        media,
        registry,
        store,
        runtime,
        generation,
        policy,
        foreign_policy,
        idempotency,
    }
}

pub(super) fn mutation(
    fixture: &RegistryFixture,
    operation: u64,
) -> crate::physical_runtime::PhysicalMutationIdentity {
    mutation_with(fixture, fixture.store, fixture.runtime, operation)
}

pub(super) fn mutation_with(
    fixture: &RegistryFixture,
    store: StableStoreIdentity,
    runtime: RuntimeIdentity,
    operation: u64,
) -> crate::physical_runtime::PhysicalMutationIdentity {
    crate::physical_runtime::PhysicalMutationIdentity::from_reserved_operation(
        PhysicalWorkIdentity::from_instance_owner(
            store,
            runtime,
            PhysicalWorkGeneration::from_lifecycle(fixture.generation),
            PhysicalOperationIdentity::from_owner_sequence(NonZeroU64::new(operation).unwrap()),
        ),
    )
}

pub(super) fn fingerprint(
    fixture: &RegistryFixture,
    payload: u8,
) -> PhysicalMutationRequestFingerprint {
    PhysicalMutationRequestFingerprint::derive(PhysicalMutationFingerprintInput {
        store: fixture.store,
        durability_policy: fixture.policy,
        scope: PhysicalMutationRequestScope::record_append([3; 32]),
        payload: PhysicalMutationPayloadDigest::from_validated_payload([payload; 32]),
        durability_request: PhysicalMutationDurabilityRequest::PlatformDurable,
        operation_family: PhysicalMutationOperationFamily::RecordAppend,
        security_bases: &[PhysicalMutationSecurityBasis::from_admitted_security(
            [5; 32],
        )],
    })
    .unwrap()
}

pub(super) fn idempotency_policy(pending: u32, live: u32) -> PhysicalIdempotencyPolicy {
    PhysicalIdempotencyPolicy::new(
        IdempotencyRetentionGenerations::new(NonZeroU64::new(4).unwrap()),
        PendingUnresolvedMutationLimit::new(NonZeroU32::new(pending).unwrap()),
        LiveIdempotencyBindingLimit::new(NonZeroU32::new(live).unwrap()),
    )
}

fn admitted_policy_identity(
    media: &crate::physical_runtime::MediaOwnedPhysicalRuntime,
    group_limit: u32,
) -> PhysicalDurabilityPolicyIdentity {
    let basis = media.physical_durability_admission_basis().unwrap();
    match PhysicalDurabilityDeclaration::builder()
        .group_commit(
            GroupCommitLimit::new(NonZeroU32::new(group_limit).unwrap()),
            GroupCommitDelay::new(NonZeroU64::new(1).unwrap()),
        )
        .wal(PhysicalWalPolicy::segmented(
            WalSegmentByteLimit::new(NonZeroU64::new(1024).unwrap()),
            WalSegmentInventoryLimit::new(NonZeroU32::new(64).unwrap()),
        ))
        .idempotency(idempotency_policy(2, 16))
        .checkpoint(PhysicalCheckpointPolicy::fuzzy(
            CheckpointMemoryLimit::new(NonZeroU64::new(1024).unwrap()),
            RetainedWalTailLimit::new(NonZeroU64::new(4096).unwrap()),
        ))
        .admit(basis)
        .into_raw()
    {
        TransitionOutcome::Success(policy) => policy.identity(),
        _ => panic!("test durability policy must admit"),
    }
}
