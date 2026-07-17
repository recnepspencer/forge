use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnerPlanNodeIdentity([u8; 32]);

impl OwnerPlanNodeIdentity {
    pub const fn fingerprint(self) -> [u8; 32] {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StoreOwnerKind {
    PhysicalBackend,
    PhysicalIntegrity,
    RecoveryPhysics,
    PhysicalIsolation,
    LayoutIndexes,
    BlobChunks,
    Authority,
    Replication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OwnerPlanEffect {
    AllocateNonCurrentStaging,
    CopyBackupComponent,
    ReplayWalToExactFrontier,
    ValidatePhysicalIntegrity,
    RebuildDerivedLayout,
    RebuildBlobReachability,
    VerifyLayoutArtifacts,
    VerifyBlobArtifacts,
    ReplaceQuarantinedLayout,
    ReplaceQuarantinedBlob,
    ClassifyQuarantine,
    ChangeReachability,
    EstablishAuthorityPosture,
    EstablishWriteFence,
    PublishNonCurrentRoot,
    ReadmitCurrentAuthority,
    BootstrapReplica,
    FenceOldPrimary,
    PromoteReplica,
    ResolveOldPrimaryRejoin,
    HoldBootstrapSourceLease,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OwnerPlanExecutionStage {
    Staging,
    PostVerification,
    Cutover,
    Readmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OwnerPlanAccess {
    Observe,
    Mutate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnerPlanFootprint {
    start: u64,
    end_exclusive: u64,
}

impl OwnerPlanFootprint {
    pub const fn bounded(start: u64, end_exclusive: u64) -> Option<Self> {
        if start < end_exclusive {
            Some(Self {
                start,
                end_exclusive,
            })
        } else {
            None
        }
    }

    pub const fn start(self) -> u64 {
        self.start
    }

    pub const fn end_exclusive(self) -> u64 {
        self.end_exclusive
    }

    pub const fn overlaps(self, other: Self) -> bool {
        self.start < other.end_exclusive && other.start < self.end_exclusive
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct OwnerPlanNode {
    identity: OwnerPlanNodeIdentity,
    owner: StoreOwnerKind,
    effect: OwnerPlanEffect,
    stage: OwnerPlanExecutionStage,
    footprint: OwnerPlanFootprint,
    estimated_work_units: u64,
    irreversible: bool,
    access: OwnerPlanAccess,
    expected_receipt_fingerprint: [u8; 32],
}

impl OwnerPlanNode {
    pub(crate) fn from_owner_binding(
        owner: StoreOwnerKind,
        effect: OwnerPlanEffect,
        footprint: OwnerPlanFootprint,
        estimated_work_units: u64,
        irreversible: bool,
        owner_plan_fingerprint: [u8; 32],
        expected_receipt_fingerprint: [u8; 32],
    ) -> Self {
        Self::from_owner_binding_at_stage(
            owner,
            effect,
            OwnerPlanExecutionStage::Staging,
            footprint,
            estimated_work_units,
            irreversible,
            owner_plan_fingerprint,
            expected_receipt_fingerprint,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_owner_observation_binding(
        owner: StoreOwnerKind,
        effect: OwnerPlanEffect,
        stage: OwnerPlanExecutionStage,
        footprint: OwnerPlanFootprint,
        estimated_work_units: u64,
        owner_plan_fingerprint: [u8; 32],
        expected_receipt_fingerprint: [u8; 32],
    ) -> Self {
        Self::from_owner_binding_with_access(
            owner,
            effect,
            stage,
            footprint,
            estimated_work_units,
            false,
            OwnerPlanAccess::Observe,
            owner_plan_fingerprint,
            expected_receipt_fingerprint,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_owner_binding_at_stage(
        owner: StoreOwnerKind,
        effect: OwnerPlanEffect,
        stage: OwnerPlanExecutionStage,
        footprint: OwnerPlanFootprint,
        estimated_work_units: u64,
        irreversible: bool,
        owner_plan_fingerprint: [u8; 32],
        expected_receipt_fingerprint: [u8; 32],
    ) -> Self {
        Self::from_owner_binding_with_access(
            owner,
            effect,
            stage,
            footprint,
            estimated_work_units,
            irreversible,
            OwnerPlanAccess::Mutate,
            owner_plan_fingerprint,
            expected_receipt_fingerprint,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn from_owner_binding_with_access(
        owner: StoreOwnerKind,
        effect: OwnerPlanEffect,
        stage: OwnerPlanExecutionStage,
        footprint: OwnerPlanFootprint,
        estimated_work_units: u64,
        irreversible: bool,
        access: OwnerPlanAccess,
        owner_plan_fingerprint: [u8; 32],
        expected_receipt_fingerprint: [u8; 32],
    ) -> Self {
        let mut digest = Sha256::new();
        digest.update(b"worth-store-owner-plan-node-v3");
        digest.update([
            owner_tag(owner),
            effect_tag(effect),
            stage_tag(stage),
            u8::from(irreversible),
            match access {
                OwnerPlanAccess::Observe => 1,
                OwnerPlanAccess::Mutate => 2,
            },
        ]);
        digest.update(footprint.start.to_be_bytes());
        digest.update(footprint.end_exclusive.to_be_bytes());
        digest.update(estimated_work_units.to_be_bytes());
        digest.update(owner_plan_fingerprint);
        digest.update(expected_receipt_fingerprint);
        Self {
            identity: OwnerPlanNodeIdentity(digest.finalize().into()),
            owner,
            effect,
            stage,
            footprint,
            estimated_work_units,
            irreversible,
            access,
            expected_receipt_fingerprint,
        }
    }

    pub(crate) const fn identity(&self) -> OwnerPlanNodeIdentity {
        self.identity
    }
    pub(crate) const fn owner(&self) -> StoreOwnerKind {
        self.owner
    }
    pub(crate) const fn effect(&self) -> OwnerPlanEffect {
        self.effect
    }
    pub(crate) const fn stage(&self) -> OwnerPlanExecutionStage {
        self.stage
    }
    pub(crate) const fn footprint(&self) -> OwnerPlanFootprint {
        self.footprint
    }
    pub(crate) const fn estimated_work_units(&self) -> u64 {
        self.estimated_work_units
    }
    pub(crate) const fn irreversible(&self) -> bool {
        self.irreversible
    }
    pub(crate) const fn access(&self) -> OwnerPlanAccess {
        self.access
    }
    pub(crate) const fn expected_receipt_fingerprint(&self) -> [u8; 32] {
        self.expected_receipt_fingerprint
    }
}

const fn stage_tag(stage: OwnerPlanExecutionStage) -> u8 {
    match stage {
        OwnerPlanExecutionStage::Staging => 1,
        OwnerPlanExecutionStage::PostVerification => 2,
        OwnerPlanExecutionStage::Cutover => 3,
        OwnerPlanExecutionStage::Readmission => 4,
    }
}

pub(crate) const fn owner_tag(owner: StoreOwnerKind) -> u8 {
    match owner {
        StoreOwnerKind::PhysicalBackend => 1,
        StoreOwnerKind::PhysicalIntegrity => 2,
        StoreOwnerKind::RecoveryPhysics => 3,
        StoreOwnerKind::PhysicalIsolation => 4,
        StoreOwnerKind::LayoutIndexes => 5,
        StoreOwnerKind::BlobChunks => 6,
        StoreOwnerKind::Authority => 7,
        StoreOwnerKind::Replication => 8,
    }
}

pub(crate) const fn effect_tag(effect: OwnerPlanEffect) -> u8 {
    match effect {
        OwnerPlanEffect::AllocateNonCurrentStaging => 1,
        OwnerPlanEffect::CopyBackupComponent => 2,
        OwnerPlanEffect::ReplayWalToExactFrontier => 3,
        OwnerPlanEffect::ValidatePhysicalIntegrity => 4,
        OwnerPlanEffect::RebuildDerivedLayout => 5,
        OwnerPlanEffect::RebuildBlobReachability => 6,
        OwnerPlanEffect::VerifyLayoutArtifacts => 7,
        OwnerPlanEffect::VerifyBlobArtifacts => 8,
        OwnerPlanEffect::ReplaceQuarantinedLayout => 9,
        OwnerPlanEffect::ReplaceQuarantinedBlob => 10,
        OwnerPlanEffect::ClassifyQuarantine => 11,
        OwnerPlanEffect::ChangeReachability => 12,
        OwnerPlanEffect::EstablishAuthorityPosture => 13,
        OwnerPlanEffect::EstablishWriteFence => 14,
        OwnerPlanEffect::PublishNonCurrentRoot => 15,
        OwnerPlanEffect::ReadmitCurrentAuthority => 16,
        OwnerPlanEffect::BootstrapReplica => 17,
        OwnerPlanEffect::FenceOldPrimary => 18,
        OwnerPlanEffect::PromoteReplica => 19,
        OwnerPlanEffect::ResolveOldPrimaryRejoin => 20,
        OwnerPlanEffect::HoldBootstrapSourceLease => 21,
    }
}
