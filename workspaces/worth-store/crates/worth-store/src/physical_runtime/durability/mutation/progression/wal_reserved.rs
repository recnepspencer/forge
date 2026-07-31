use worth_store_physical_backend::{ArtifactAppendRange, ArtifactTreeFile};
use worth_store_wal::{PlannedWalFrameAppend, WalAppendFrontier};

use crate::physical_runtime::{
    durability::{
        AdmittedPhysicalMutation, AllocatedPhysicalMutationAttemptBinding, WalBoundPhysicalDataPlan,
    },
    AdmittedRecordPlacementPolicy, CanonicalRedoRecords, PhysicalMutationDeadline,
    PhysicalMutationIdentity, PhysicalMutationResourceShape, PhysicalSignalProfileIdentity,
    PhysicalWalAppendDeclaration, PhysicalWalMemberBasis, PhysicalWorkSemanticBasis,
    PreparedPhysicalMutation, PreparedRecordPublicationContinuation, RecordAppendBatch,
};

pub struct WalRangeReservedPhysicalMutation {
    binding: AllocatedPhysicalMutationAttemptBinding,
    member: PhysicalWalMemberBasis,
    redo: CanonicalRedoRecords,
    data: WalBoundPhysicalDataPlan,
    continuation: PreparedRecordPublicationContinuation,
    frame: PlannedWalFrameAppend,
    artifact: ArtifactTreeFile,
    declaration: PhysicalWalAppendDeclaration,
    placement: AdmittedRecordPlacementPolicy,
    deadline: PhysicalMutationDeadline,
    signal_profile: PhysicalSignalProfileIdentity,
    durability_policy_basis: PhysicalWorkSemanticBasis,
    resources: PhysicalMutationResourceShape,
}

impl WalRangeReservedPhysicalMutation {
    pub(in crate::physical_runtime) fn new(
        binding: AllocatedPhysicalMutationAttemptBinding,
        member: PhysicalWalMemberBasis,
        redo: CanonicalRedoRecords,
        data: WalBoundPhysicalDataPlan,
        continuation: PreparedRecordPublicationContinuation,
        frame: PlannedWalFrameAppend,
        artifact: ArtifactTreeFile,
        declaration: PhysicalWalAppendDeclaration,
        placement: AdmittedRecordPlacementPolicy,
        deadline: PhysicalMutationDeadline,
        signal_profile: PhysicalSignalProfileIdentity,
        durability_policy_basis: PhysicalWorkSemanticBasis,
        resources: PhysicalMutationResourceShape,
    ) -> Self {
        Self {
            binding,
            member,
            redo,
            data,
            continuation,
            frame,
            artifact,
            declaration,
            placement,
            deadline,
            signal_profile,
            durability_policy_basis,
            resources,
        }
    }

    pub const fn mutation_identity(&self) -> PhysicalMutationIdentity {
        self.member.mutation_identity()
    }

    pub const fn request_fingerprint(
        &self,
    ) -> crate::physical_runtime::PhysicalMutationRequestFingerprint {
        self.binding.fingerprint()
    }

    pub const fn bound_redo_digest(&self) -> [u8; 32] {
        self.binding.redo_digest()
    }

    pub const fn member_basis(&self) -> PhysicalWalMemberBasis {
        self.member
    }

    pub fn redo(&self) -> &CanonicalRedoRecords {
        &self.redo
    }

    pub(in crate::physical_runtime) const fn data(&self) -> &WalBoundPhysicalDataPlan {
        &self.data
    }

    pub fn artifact(&self) -> &ArtifactTreeFile {
        &self.artifact
    }

    pub const fn declaration(&self) -> PhysicalWalAppendDeclaration {
        self.declaration
    }

    pub const fn placement(&self) -> AdmittedRecordPlacementPolicy {
        self.placement
    }

    pub const fn deadline(&self) -> PhysicalMutationDeadline {
        self.deadline
    }

    pub const fn signal_profile(&self) -> PhysicalSignalProfileIdentity {
        self.signal_profile
    }

    pub fn durability_policy_basis(&self) -> PhysicalWorkSemanticBasis {
        self.durability_policy_basis.clone()
    }

    pub fn append_range(&self) -> ArtifactAppendRange {
        ArtifactAppendRange::new(
            self.frame.frame().valid_prefix_bytes(),
            self.frame.frame().encoded_frame().len() as u64,
        )
        .expect("planned WAL frames are nonempty and nonoverflowing")
    }

    pub fn encoded_frame(&self) -> &[u8] {
        self.frame.frame().encoded_frame()
    }

    pub const fn resulting_frontier(&self) -> WalAppendFrontier {
        self.frame.resulting_frontier()
    }

    pub(in crate::physical_runtime) fn into_prepared_after_no_effect(
        self,
    ) -> PreparedPhysicalMutation {
        let binding = self.binding.release_wal_allocation();
        let batch =
            RecordAppendBatch::from_prepared_record_bytes(self.redo.into_prepared_record_bytes());
        PreparedPhysicalMutation::from_planned_parts(
            AdmittedPhysicalMutation::Fresh(binding),
            batch,
            self.data.into_prepared(),
            self.continuation,
            self.placement,
            self.deadline,
            self.signal_profile,
            self.durability_policy_basis,
            self.resources,
        )
    }
}
