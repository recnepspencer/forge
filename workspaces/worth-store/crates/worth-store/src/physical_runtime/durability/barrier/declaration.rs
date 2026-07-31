use sha2::{Digest, Sha256};
use worth_store_physical_backend::{
    ArtifactTreeFile, BackendTargetProfile, PhysicalDurabilityAdmissionIdentity,
    WalDurabilityBarrier,
};

use crate::physical_runtime::{
    PhysicalDurabilityObservation, PhysicalDurabilityPolicyIdentity, PhysicalWalBarrierScope,
    PhysicalWalMemberBasis, PhysicalWorkIdentity, WalAppendedPhysicalMutation,
};

const BARRIER_BINDING_DOMAIN: &[u8] = b"worth.store.physical.wal-barrier-binding.v1";

pub struct PhysicalWalBarrierDeclaration {
    member: PhysicalWalMemberBasis,
    artifact: ArtifactTreeFile,
    policy: PhysicalDurabilityPolicyIdentity,
    admission_basis: PhysicalDurabilityAdmissionIdentity,
    profile: BackendTargetProfile,
    required_barrier: WalDurabilityBarrier,
    binding_digest: [u8; 32],
}

impl PhysicalWalBarrierDeclaration {
    pub(in crate::physical_runtime) fn for_appended(
        appended: &WalAppendedPhysicalMutation,
        durability: PhysicalDurabilityObservation,
    ) -> Option<Self> {
        let mutation = appended.mutation_identity();
        if mutation.store_identity() != durability.store_identity()
            || mutation.runtime_identity() != durability.runtime_identity()
        {
            return None;
        }
        let required_barrier = match durability.profile() {
            BackendTargetProfile::PosixFileFsyncDirSync => WalDurabilityBarrier::WalFileFsync,
            BackendTargetProfile::WindowsFlushFileBuffers => {
                WalDurabilityBarrier::WindowsFlushFileBuffers
            }
            _ => return None,
        };
        let member = appended.reserved().member_basis();
        let append_work = appended.settlement().work_identity();
        let append_payload_digest = appended.settlement().payload_digest();
        let policy = durability.policy_identity();
        let admission_basis = durability.admission_basis_identity();
        let profile = durability.profile();
        let binding_digest = binding_digest(
            member,
            append_work,
            append_payload_digest,
            policy,
            admission_basis,
            profile,
            required_barrier,
        );
        Some(Self {
            member,
            artifact: appended.reserved().artifact().clone(),
            policy,
            admission_basis,
            profile,
            required_barrier,
            binding_digest,
        })
    }

    pub const fn member_basis(&self) -> PhysicalWalMemberBasis {
        self.member
    }

    pub const fn policy_identity(&self) -> PhysicalDurabilityPolicyIdentity {
        self.policy
    }

    pub const fn admission_basis_identity(&self) -> PhysicalDurabilityAdmissionIdentity {
        self.admission_basis
    }

    pub const fn profile(&self) -> BackendTargetProfile {
        self.profile
    }

    pub const fn required_barrier(&self) -> WalDurabilityBarrier {
        self.required_barrier
    }

    pub const fn binding_digest(&self) -> [u8; 32] {
        self.binding_digest
    }

    pub(in crate::physical_runtime) const fn artifact(&self) -> &ArtifactTreeFile {
        &self.artifact
    }

    pub(in crate::physical_runtime) fn scope(
        &self,
        appended: &WalAppendedPhysicalMutation,
    ) -> PhysicalWalBarrierScope {
        let lsn = self.member.lsn_range();
        let append = appended.reserved().declaration();
        let artifact_range = append.artifact_range();
        PhysicalWalBarrierScope::new(
            self.member.member_identity().bytes(),
            append.segment().get(),
            append.generation().get(),
            lsn.start().get(),
            lsn.end_exclusive().get(),
            artifact_range.offset(),
            artifact_range.byte_count(),
        )
        .expect("an appended WAL member carries one valid barrier scope")
    }
}

#[allow(clippy::too_many_arguments)]
fn binding_digest(
    member: PhysicalWalMemberBasis,
    append_work: PhysicalWorkIdentity,
    append_payload_digest: [u8; 32],
    policy: PhysicalDurabilityPolicyIdentity,
    admission_basis: PhysicalDurabilityAdmissionIdentity,
    profile: BackendTargetProfile,
    required_barrier: WalDurabilityBarrier,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(BARRIER_BINDING_DOMAIN);
    digest.update(member.member_identity().bytes());
    digest.update(member.lsn_range().start().get().to_le_bytes());
    digest.update(member.lsn_range().end_exclusive().get().to_le_bytes());
    digest.update(append_work.store().bytes());
    digest.update(append_work.runtime().get().to_le_bytes());
    digest.update(append_work.generation().lifecycle().get().to_le_bytes());
    digest.update(append_work.operation().get().to_le_bytes());
    digest.update(append_payload_digest);
    digest.update(policy.bytes());
    digest.update(admission_basis.bytes());
    digest.update([profile_tag(profile)]);
    digest.update([barrier_tag(required_barrier)]);
    digest.finalize().into()
}

const fn profile_tag(profile: BackendTargetProfile) -> u8 {
    match profile {
        BackendTargetProfile::SimulatedStrictDurable => 1,
        BackendTargetProfile::PosixFileFsyncDirSync => 2,
        BackendTargetProfile::WindowsFlushFileBuffers => 3,
        BackendTargetProfile::MmapFlushNotDurabilityCertified => 4,
        BackendTargetProfile::AdversarialLostFlush => 5,
        BackendTargetProfile::AdversarialReorderedFlush => 6,
    }
}

const fn barrier_tag(barrier: WalDurabilityBarrier) -> u8 {
    match barrier {
        WalDurabilityBarrier::SimulatedDurableCommit => 1,
        WalDurabilityBarrier::WalFileFsync => 2,
        WalDurabilityBarrier::WalDirectoryFsync => 3,
        WalDurabilityBarrier::WindowsFlushFileBuffers => 4,
        WalDurabilityBarrier::WindowsDirectorySync => 5,
        WalDurabilityBarrier::OrderedPersistenceFence => 6,
    }
}
