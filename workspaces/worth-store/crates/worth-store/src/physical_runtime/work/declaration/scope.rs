use sha2::{Digest, Sha256};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::{PhysicalWalAppendScope, PhysicalWalBarrierScope, PhysicalWorkDeclarationDenial};

const MAX_PHYSICAL_SCOPE_MEMBERS: usize = 256;

/// Exact, non-overlapping artifact ranges belonging to one work identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkScope {
    members: PhysicalWorkScopeMembers,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PhysicalWorkScopeMembers {
    Artifact(RecordArtifactFile),
    One(RecordFrameCoordinate),
    Batch(Box<[RecordFrameCoordinate]>),
    WalAppend(PhysicalWalAppendScope),
    WalBarrier(PhysicalWalBarrierScope),
}

impl PhysicalWorkScope {
    pub fn artifact(artifact: RecordArtifactFile) -> Self {
        Self {
            members: PhysicalWorkScopeMembers::Artifact(artifact),
        }
    }

    pub fn one(coordinate: RecordFrameCoordinate) -> Self {
        Self {
            members: PhysicalWorkScopeMembers::One(coordinate),
        }
    }

    pub fn batch(
        coordinates: impl IntoIterator<Item = RecordFrameCoordinate>,
    ) -> Result<Self, PhysicalWorkDeclarationDenial> {
        let mut exact = Vec::new();
        for coordinate in coordinates {
            if exact.len() == MAX_PHYSICAL_SCOPE_MEMBERS {
                return Err(PhysicalWorkDeclarationDenial::ScopeCapacityExceeded);
            }
            exact.push(coordinate);
        }
        if exact.is_empty() {
            return Err(PhysicalWorkDeclarationDenial::EmptyScope);
        }
        if exact.len() == 1 {
            return Err(PhysicalWorkDeclarationDenial::BatchRequiresMultipleMembers);
        }
        exact.sort_unstable();
        require_disjoint_members(&exact)?;
        Ok(Self {
            members: PhysicalWorkScopeMembers::Batch(exact.into_boxed_slice()),
        })
    }

    pub(in crate::physical_runtime) const fn wal_append(scope: PhysicalWalAppendScope) -> Self {
        Self {
            members: PhysicalWorkScopeMembers::WalAppend(scope),
        }
    }

    pub(in crate::physical_runtime) const fn wal_barrier(scope: PhysicalWalBarrierScope) -> Self {
        Self {
            members: PhysicalWorkScopeMembers::WalBarrier(scope),
        }
    }

    pub fn coordinates(&self) -> &[RecordFrameCoordinate] {
        match &self.members {
            PhysicalWorkScopeMembers::Artifact(_) => &[],
            PhysicalWorkScopeMembers::WalAppend(_) | PhysicalWorkScopeMembers::WalBarrier(_) => &[],
            PhysicalWorkScopeMembers::One(coordinate) => std::slice::from_ref(coordinate),
            PhysicalWorkScopeMembers::Batch(coordinates) => coordinates,
        }
    }

    pub const fn artifact_target(&self) -> Option<RecordArtifactFile> {
        match &self.members {
            PhysicalWorkScopeMembers::Artifact(artifact) => Some(*artifact),
            PhysicalWorkScopeMembers::One(_) | PhysicalWorkScopeMembers::Batch(_) => None,
            PhysicalWorkScopeMembers::WalAppend(_) | PhysicalWorkScopeMembers::WalBarrier(_) => {
                None
            }
        }
    }

    pub const fn wal_append_target(&self) -> Option<PhysicalWalAppendScope> {
        match &self.members {
            PhysicalWorkScopeMembers::WalAppend(scope) => Some(*scope),
            _ => None,
        }
    }

    pub const fn wal_barrier_target(&self) -> Option<PhysicalWalBarrierScope> {
        match &self.members {
            PhysicalWorkScopeMembers::WalBarrier(scope) => Some(*scope),
            _ => None,
        }
    }

    pub const fn member_count(&self) -> usize {
        match &self.members {
            PhysicalWorkScopeMembers::Artifact(_)
            | PhysicalWorkScopeMembers::One(_)
            | PhysicalWorkScopeMembers::WalAppend(_)
            | PhysicalWorkScopeMembers::WalBarrier(_) => 1,
            PhysicalWorkScopeMembers::Batch(coordinates) => coordinates.len(),
        }
    }

    pub(in crate::physical_runtime::work) fn stable_digest(&self) -> [u8; 32] {
        let mut digest = Sha256::new();
        digest.update(b"worth-store.physical-work-scope.v1");
        digest.update((self.member_count() as u64).to_le_bytes());
        if let PhysicalWorkScopeMembers::Artifact(artifact) = &self.members {
            digest.update(b"artifact");
            let name = artifact.file_name();
            digest.update((name.len() as u64).to_le_bytes());
            digest.update(name.as_bytes());
            return digest.finalize().into();
        }
        if let PhysicalWorkScopeMembers::WalAppend(scope) = &self.members {
            digest.update(b"wal-append");
            digest.update(scope.segment().to_le_bytes());
            digest.update(scope.generation().to_le_bytes());
            digest.update(scope.offset().to_le_bytes());
            digest.update(scope.byte_count().to_le_bytes());
            return digest.finalize().into();
        }
        if let PhysicalWorkScopeMembers::WalBarrier(scope) = &self.members {
            digest.update(b"wal-barrier");
            digest.update(scope.member());
            digest.update(scope.segment().to_le_bytes());
            digest.update(scope.generation().to_le_bytes());
            digest.update(scope.lsn_start().to_le_bytes());
            digest.update(scope.lsn_end_exclusive().to_le_bytes());
            digest.update(scope.append_offset().to_le_bytes());
            digest.update(scope.append_byte_count().to_le_bytes());
            return digest.finalize().into();
        }
        digest.update(b"ranges");
        for coordinate in self.coordinates() {
            let artifact = coordinate.artifact().file_name();
            digest.update((artifact.len() as u64).to_le_bytes());
            digest.update(artifact.as_bytes());
            digest.update(coordinate.offset().to_le_bytes());
            digest.update(coordinate.length().to_le_bytes());
        }
        digest.finalize().into()
    }
}

fn require_disjoint_members(
    members: &[RecordFrameCoordinate],
) -> Result<(), PhysicalWorkDeclarationDenial> {
    for pair in members.windows(2) {
        let left = pair[0];
        let right = pair[1];
        if left == right {
            return Err(PhysicalWorkDeclarationDenial::DuplicateScopeMember);
        }
        if left.artifact() == right.artifact()
            && left.offset().saturating_add(u64::from(left.length())) > right.offset()
        {
            return Err(PhysicalWorkDeclarationDenial::OverlappingScopeMembers);
        }
    }
    Ok(())
}
