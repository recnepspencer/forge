use std::num::{NonZeroU32, NonZeroU64};

use sha2::{Digest, Sha256};
use worth_proof::{ProofOutcome, TransitionOutcome};
use worth_store_physical_backend::{
    BackendCapabilityKind, BackendTargetProfile, CapabilityEvidenceClass,
    PhysicalDurabilityAdmissionBasis, PhysicalDurabilityAdmissionIdentity,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::{
    PhysicalDurabilityPolicyDeferred, PhysicalDurabilityPolicyDenial,
    PhysicalDurabilityPolicyFailure, PhysicalDurabilityPolicyRebindRequired,
    PhysicalDurabilityPolicyStale,
};

const POLICY_IDENTITY_DOMAIN: &[u8] = b"worth.store.physical.durability.policy.v1";

pub type PhysicalDurabilityPolicyAdmissionOutcome = ProofOutcome<
    AdmittedPhysicalDurabilityPolicy,
    PhysicalDurabilityPolicyDenial,
    PhysicalDurabilityPolicyDeferred,
    PhysicalDurabilityPolicyStale,
    PhysicalDurabilityPolicyRebindRequired,
    PhysicalDurabilityPolicyFailure,
>;

macro_rules! nonzero_limit {
    ($name:ident, $inner:ty) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name($inner);

        impl $name {
            pub const fn new(value: $inner) -> Self {
                Self(value)
            }

            pub const fn get(self) -> $inner {
                self.0
            }
        }
    };
}

nonzero_limit!(GroupCommitLimit, NonZeroU32);
nonzero_limit!(GroupCommitDelay, NonZeroU64);
nonzero_limit!(IdempotencyRetentionGenerations, NonZeroU64);
nonzero_limit!(PendingUnresolvedMutationLimit, NonZeroU32);
nonzero_limit!(CheckpointMemoryLimit, NonZeroU64);
nonzero_limit!(RetainedWalTailLimit, NonZeroU64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PhysicalIdempotencyPolicy {
    retention: IdempotencyRetentionGenerations,
    pending_unresolved: PendingUnresolvedMutationLimit,
}

impl PhysicalIdempotencyPolicy {
    pub const fn new(
        retention: IdempotencyRetentionGenerations,
        pending_unresolved: PendingUnresolvedMutationLimit,
    ) -> Self {
        Self {
            retention,
            pending_unresolved,
        }
    }

    pub const fn retention(self) -> IdempotencyRetentionGenerations {
        self.retention
    }

    pub const fn pending_unresolved_limit(self) -> PendingUnresolvedMutationLimit {
        self.pending_unresolved
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PhysicalCheckpointPolicy {
    memory: CheckpointMemoryLimit,
    retained_wal_tail: RetainedWalTailLimit,
}

impl PhysicalCheckpointPolicy {
    pub const fn fuzzy(
        memory: CheckpointMemoryLimit,
        retained_wal_tail: RetainedWalTailLimit,
    ) -> Self {
        Self {
            memory,
            retained_wal_tail,
        }
    }

    pub const fn memory_limit(self) -> CheckpointMemoryLimit {
        self.memory
    }

    pub const fn retained_wal_tail_limit(self) -> RetainedWalTailLimit {
        self.retained_wal_tail
    }
}

#[derive(Debug)]
pub struct PhysicalDurabilityDeclaration;

impl PhysicalDurabilityDeclaration {
    pub const fn builder(
    ) -> PhysicalDurabilityDeclarationBuilder<GroupMissing, IdempotencyMissing, CheckpointMissing>
    {
        PhysicalDurabilityDeclarationBuilder {
            group: GroupMissing,
            idempotency: IdempotencyMissing,
            checkpoint: CheckpointMissing,
        }
    }
}

#[doc(hidden)]
pub struct GroupMissing;
#[doc(hidden)]
pub struct IdempotencyMissing;
#[doc(hidden)]
pub struct CheckpointMissing;

#[derive(Clone, Copy)]
struct GroupPolicy {
    limit: GroupCommitLimit,
    delay: GroupCommitDelay,
}

#[doc(hidden)]
pub struct GroupConfigured(GroupPolicy);
#[doc(hidden)]
pub struct IdempotencyConfigured(PhysicalIdempotencyPolicy);
#[doc(hidden)]
pub struct CheckpointConfigured(PhysicalCheckpointPolicy);

pub struct PhysicalDurabilityDeclarationBuilder<Group, Idempotency, Checkpoint> {
    group: Group,
    idempotency: Idempotency,
    checkpoint: Checkpoint,
}

impl<Idempotency, Checkpoint>
    PhysicalDurabilityDeclarationBuilder<GroupMissing, Idempotency, Checkpoint>
{
    pub fn group_commit(
        self,
        limit: GroupCommitLimit,
        delay: GroupCommitDelay,
    ) -> PhysicalDurabilityDeclarationBuilder<GroupConfigured, Idempotency, Checkpoint> {
        PhysicalDurabilityDeclarationBuilder {
            group: GroupConfigured(GroupPolicy { limit, delay }),
            idempotency: self.idempotency,
            checkpoint: self.checkpoint,
        }
    }
}

impl<Group, Checkpoint>
    PhysicalDurabilityDeclarationBuilder<Group, IdempotencyMissing, Checkpoint>
{
    pub fn idempotency(
        self,
        policy: PhysicalIdempotencyPolicy,
    ) -> PhysicalDurabilityDeclarationBuilder<Group, IdempotencyConfigured, Checkpoint> {
        PhysicalDurabilityDeclarationBuilder {
            group: self.group,
            idempotency: IdempotencyConfigured(policy),
            checkpoint: self.checkpoint,
        }
    }
}

impl<Group, Idempotency>
    PhysicalDurabilityDeclarationBuilder<Group, Idempotency, CheckpointMissing>
{
    pub fn checkpoint(
        self,
        policy: PhysicalCheckpointPolicy,
    ) -> PhysicalDurabilityDeclarationBuilder<Group, Idempotency, CheckpointConfigured> {
        PhysicalDurabilityDeclarationBuilder {
            group: self.group,
            idempotency: self.idempotency,
            checkpoint: CheckpointConfigured(policy),
        }
    }
}

impl
    PhysicalDurabilityDeclarationBuilder<
        GroupConfigured,
        IdempotencyConfigured,
        CheckpointConfigured,
    >
{
    pub fn admit(
        self,
        basis: PhysicalDurabilityAdmissionBasis,
    ) -> PhysicalDurabilityPolicyAdmissionOutcome {
        if !matches!(
            basis.target_profile(),
            BackendTargetProfile::PosixFileFsyncDirSync
                | BackendTargetProfile::WindowsFlushFileBuffers
        ) {
            return TransitionOutcome::denied(
                PhysicalDurabilityPolicyDenial::UnsupportedBackendProfile {
                    profile: basis.target_profile(),
                },
            )
            .into();
        }
        for (claim, capability) in [
            (basis.file_sync_claim(), BackendCapabilityKind::Fsync),
            (
                basis.directory_sync_claim(),
                BackendCapabilityKind::DirectorySync,
            ),
            (
                basis.durable_rename_claim(),
                BackendCapabilityKind::DurableRename,
            ),
        ] {
            if claim.kind() != capability
                || claim.profile() != basis.target_profile()
                || claim.evidence_class()
                    != CapabilityEvidenceClass::EstablishedByFilesystemAdmission
            {
                return TransitionOutcome::denied(
                    PhysicalDurabilityPolicyDenial::InvalidCapabilityBinding { capability },
                )
                .into();
            }
        }
        TransitionOutcome::success(AdmittedPhysicalDurabilityPolicy::new(
            basis,
            self.group.0,
            self.idempotency.0,
            self.checkpoint.0,
        ))
        .into()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhysicalDurabilityPolicyIdentity([u8; 32]);

impl PhysicalDurabilityPolicyIdentity {
    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }
}

pub struct AdmittedPhysicalDurabilityPolicy {
    identity: PhysicalDurabilityPolicyIdentity,
    basis: PhysicalDurabilityAdmissionIdentity,
    store: StableStoreIdentity,
    profile: BackendTargetProfile,
    group: GroupPolicy,
    idempotency: PhysicalIdempotencyPolicy,
    checkpoint: PhysicalCheckpointPolicy,
}

impl AdmittedPhysicalDurabilityPolicy {
    fn new(
        basis: PhysicalDurabilityAdmissionBasis,
        group: GroupPolicy,
        idempotency: PhysicalIdempotencyPolicy,
        checkpoint: PhysicalCheckpointPolicy,
    ) -> Self {
        let identity = policy_identity(basis.identity(), group, idempotency, checkpoint);
        Self {
            identity,
            basis: basis.identity(),
            store: basis.store_identity(),
            profile: basis.target_profile(),
            group,
            idempotency,
            checkpoint,
        }
    }

    pub const fn identity(&self) -> PhysicalDurabilityPolicyIdentity {
        self.identity
    }

    pub const fn admission_basis_identity(&self) -> PhysicalDurabilityAdmissionIdentity {
        self.basis
    }

    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.store
    }

    pub const fn profile(&self) -> BackendTargetProfile {
        self.profile
    }

    pub const fn group_commit_limit(&self) -> GroupCommitLimit {
        self.group.limit
    }

    pub const fn group_commit_delay(&self) -> GroupCommitDelay {
        self.group.delay
    }

    pub const fn idempotency_policy(&self) -> PhysicalIdempotencyPolicy {
        self.idempotency
    }

    pub const fn checkpoint_policy(&self) -> PhysicalCheckpointPolicy {
        self.checkpoint
    }
}

fn policy_identity(
    basis: PhysicalDurabilityAdmissionIdentity,
    group: GroupPolicy,
    idempotency: PhysicalIdempotencyPolicy,
    checkpoint: PhysicalCheckpointPolicy,
) -> PhysicalDurabilityPolicyIdentity {
    let mut digest = Sha256::new();
    digest.update((POLICY_IDENTITY_DOMAIN.len() as u64).to_le_bytes());
    digest.update(POLICY_IDENTITY_DOMAIN);
    digest.update(basis.bytes());
    digest.update(group.limit.get().get().to_le_bytes());
    digest.update(group.delay.get().get().to_le_bytes());
    digest.update(idempotency.retention.get().get().to_le_bytes());
    digest.update(idempotency.pending_unresolved.get().get().to_le_bytes());
    digest.update(checkpoint.memory.get().get().to_le_bytes());
    digest.update(checkpoint.retained_wal_tail.get().get().to_le_bytes());
    PhysicalDurabilityPolicyIdentity(digest.finalize().into())
}
