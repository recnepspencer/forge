use sha2::{Digest, Sha256};
use worth_store_physical_backend::{
    ArtifactTreeFile, BackendTargetProfile, PhysicalDurabilityAdmissionIdentity,
    WalDurabilityBarrier,
};

use crate::physical_runtime::{
    PhysicalDurabilityGroupBasis, PhysicalDurabilityObservation, PhysicalDurabilityPolicyIdentity,
    PhysicalWalBarrierScope, SealedPhysicalDurabilityGroupMembers,
};

const GROUP_BARRIER_BINDING_DOMAIN: &[u8] = b"worth.store.physical.wal-group-barrier-binding.v1";

pub struct PhysicalWalGroupBarrierDeclaration {
    basis: PhysicalDurabilityGroupBasis,
    artifact: ArtifactTreeFile,
    segment: u64,
    generation: u64,
    lsn_start: u64,
    lsn_end_exclusive: u64,
    append_offset: u64,
    append_byte_count: u64,
    policy: PhysicalDurabilityPolicyIdentity,
    admission_basis: PhysicalDurabilityAdmissionIdentity,
    profile: BackendTargetProfile,
    required_barrier: WalDurabilityBarrier,
    binding_digest: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWalGroupBarrierDeclarationDenial {
    PolicyOrRuntimeMismatch,
    UnsupportedProfile,
    MixedWalArtifact,
    MixedWalSegment,
    MixedWalGeneration,
    WalRangeOverflow,
}

struct PhysicalWalGroupBarrierRange {
    segment: u64,
    generation: u64,
    lsn_start: u64,
    lsn_end_exclusive: u64,
    append_offset: u64,
    append_end_exclusive: u64,
}

impl PhysicalWalGroupBarrierDeclaration {
    pub(in crate::physical_runtime) fn for_appended_group(
        group: &SealedPhysicalDurabilityGroupMembers,
        durability: PhysicalDurabilityObservation,
    ) -> Result<Self, PhysicalWalGroupBarrierDeclarationDenial> {
        require_current_group(group, durability)?;
        let required_barrier = required_barrier(durability.profile())?;
        let first = group.members().first().expect("sealed groups are nonempty");
        let artifact = first.mutation().reserved().artifact().clone();
        let range = require_one_barrier_artifact(group, &artifact)?;
        let basis = group.basis();
        Ok(Self {
            basis,
            artifact,
            segment: range.segment,
            generation: range.generation,
            lsn_start: range.lsn_start,
            lsn_end_exclusive: range.lsn_end_exclusive,
            append_offset: range.append_offset,
            append_byte_count: range
                .append_end_exclusive
                .checked_sub(range.append_offset)
                .ok_or(PhysicalWalGroupBarrierDeclarationDenial::WalRangeOverflow)?,
            policy: durability.policy_identity(),
            admission_basis: durability.admission_basis_identity(),
            profile: durability.profile(),
            required_barrier,
            binding_digest: binding_digest(group, durability, required_barrier),
        })
    }

    pub const fn basis(&self) -> PhysicalDurabilityGroupBasis {
        self.basis
    }

    pub(in crate::physical_runtime) const fn artifact(&self) -> &ArtifactTreeFile {
        &self.artifact
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

    pub(in crate::physical_runtime) fn scope(&self) -> PhysicalWalBarrierScope {
        PhysicalWalBarrierScope::new(
            self.basis.identity().bytes(),
            self.basis.membership_digest(),
            self.basis.member_count().get(),
            self.segment,
            self.generation,
            self.lsn_start,
            self.lsn_end_exclusive,
            self.append_offset,
            self.append_byte_count,
        )
        .expect("a group barrier declaration carries one valid exact scope")
    }
}

fn require_current_group(
    group: &SealedPhysicalDurabilityGroupMembers,
    durability: PhysicalDurabilityObservation,
) -> Result<(), PhysicalWalGroupBarrierDeclarationDenial> {
    let is_current = group.members().iter().all(|member| {
        let identity = member.mutation().mutation_identity();
        identity.store_identity() == durability.store_identity()
            && identity.runtime_identity() == durability.runtime_identity()
    });
    is_current
        .then_some(())
        .ok_or(PhysicalWalGroupBarrierDeclarationDenial::PolicyOrRuntimeMismatch)
}

fn require_one_barrier_artifact(
    group: &SealedPhysicalDurabilityGroupMembers,
    artifact: &ArtifactTreeFile,
) -> Result<PhysicalWalGroupBarrierRange, PhysicalWalGroupBarrierDeclarationDenial> {
    let first = group.members().first().expect("sealed groups are nonempty");
    let first_declaration = first.mutation().reserved().declaration();
    let first_lsn = first_declaration.lsn_range();
    let first_append = first_declaration.artifact_range();
    let mut range = PhysicalWalGroupBarrierRange {
        segment: first_declaration.segment().get(),
        generation: first_declaration.generation().get(),
        lsn_start: first_lsn.start().get(),
        lsn_end_exclusive: first_lsn.end_exclusive().get(),
        append_offset: first_append.offset(),
        append_end_exclusive: append_end(first_append)?,
    };
    for member in group.members().iter().skip(1) {
        let mutation = member.mutation();
        if mutation.reserved().artifact() != artifact {
            return Err(PhysicalWalGroupBarrierDeclarationDenial::MixedWalArtifact);
        }
        let declaration = mutation.reserved().declaration();
        if declaration.segment().get() != range.segment {
            return Err(PhysicalWalGroupBarrierDeclarationDenial::MixedWalSegment);
        }
        if declaration.generation().get() != range.generation {
            return Err(PhysicalWalGroupBarrierDeclarationDenial::MixedWalGeneration);
        }
        let lsn = declaration.lsn_range();
        range.lsn_start = range.lsn_start.min(lsn.start().get());
        range.lsn_end_exclusive = range.lsn_end_exclusive.max(lsn.end_exclusive().get());
        let append = declaration.artifact_range();
        range.append_offset = range.append_offset.min(append.offset());
        range.append_end_exclusive = range.append_end_exclusive.max(append_end(append)?);
    }
    Ok(range)
}

fn append_end(
    range: worth_store_physical_backend::ArtifactAppendRange,
) -> Result<u64, PhysicalWalGroupBarrierDeclarationDenial> {
    range
        .offset()
        .checked_add(range.byte_count())
        .ok_or(PhysicalWalGroupBarrierDeclarationDenial::WalRangeOverflow)
}

fn required_barrier(
    profile: BackendTargetProfile,
) -> Result<WalDurabilityBarrier, PhysicalWalGroupBarrierDeclarationDenial> {
    match profile {
        BackendTargetProfile::PosixFileFsyncDirSync => Ok(WalDurabilityBarrier::WalFileFsync),
        BackendTargetProfile::WindowsFlushFileBuffers => {
            Ok(WalDurabilityBarrier::WindowsFlushFileBuffers)
        }
        _ => Err(PhysicalWalGroupBarrierDeclarationDenial::UnsupportedProfile),
    }
}

fn binding_digest(
    group: &SealedPhysicalDurabilityGroupMembers,
    durability: PhysicalDurabilityObservation,
    barrier: WalDurabilityBarrier,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    let basis = group.basis();
    digest.update(GROUP_BARRIER_BINDING_DOMAIN);
    digest.update(basis.identity().bytes());
    digest.update(basis.membership_digest());
    digest.update(basis.member_count().get().to_le_bytes());
    digest.update(durability.policy_identity().bytes());
    digest.update(durability.admission_basis_identity().bytes());
    digest.update([profile_tag(durability.profile())]);
    digest.update([barrier_tag(barrier)]);
    for member in group.members() {
        let binding = member.binding();
        let mutation = member.mutation();
        let declaration = mutation.reserved().declaration();
        let lsn = declaration.lsn_range();
        let append = declaration.artifact_range();
        digest.update(binding.member_identity().bytes());
        digest.update(binding.ordinal().get().to_le_bytes());
        digest.update(
            mutation
                .mutation_identity()
                .operation_identity()
                .get()
                .to_le_bytes(),
        );
        digest.update(lsn.start().get().to_le_bytes());
        digest.update(lsn.end_exclusive().get().to_le_bytes());
        digest.update(append.offset().to_le_bytes());
        digest.update(append.byte_count().to_le_bytes());
        digest.update(
            mutation
                .settlement()
                .work_identity()
                .operation()
                .get()
                .to_le_bytes(),
        );
        digest.update(mutation.settlement().payload_digest());
    }
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
