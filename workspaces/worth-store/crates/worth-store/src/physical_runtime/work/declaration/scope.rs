use worth_store_physical_format::RecordFrameCoordinate;
use sha2::{Digest, Sha256};

use super::PhysicalWorkDeclarationDenial;

const MAX_PHYSICAL_SCOPE_MEMBERS: usize = 256;

/// Exact, non-overlapping artifact ranges belonging to one work identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkScope {
    members: PhysicalWorkScopeMembers,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PhysicalWorkScopeMembers {
    One(RecordFrameCoordinate),
    Batch(Box<[RecordFrameCoordinate]>),
}

impl PhysicalWorkScope {
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

    pub fn coordinates(&self) -> &[RecordFrameCoordinate] {
        match &self.members {
            PhysicalWorkScopeMembers::One(coordinate) => std::slice::from_ref(coordinate),
            PhysicalWorkScopeMembers::Batch(coordinates) => coordinates,
        }
    }

    pub(in crate::physical_runtime::work) fn stable_digest(&self) -> [u8; 32] {
        let mut digest = Sha256::new();
        digest.update(b"worth-store.physical-work-scope.v1");
        digest.update((self.coordinates().len() as u64).to_le_bytes());
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
