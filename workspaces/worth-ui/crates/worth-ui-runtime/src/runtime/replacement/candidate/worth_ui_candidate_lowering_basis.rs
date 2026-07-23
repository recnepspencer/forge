use crate::capability::CapabilitySnapshotDigest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCandidateLoweringBasis {
    snapshot_digest: u64,
}

impl WorthUiCandidateLoweringBasis {
    pub(crate) fn from_snapshot_and_dependency_metadata(
        snapshot_digest: CapabilitySnapshotDigest,
        _dependency_metadata: &super::WorthUiCandidateDependencyMetadata,
    ) -> Self {
        Self {
            snapshot_digest: snapshot_digest.as_u64(),
        }
    }

    pub fn snapshot_digest(self) -> u64 {
        self.snapshot_digest
    }

    pub(crate) fn basis_digest(self) -> u64 {
        0x6d33_39e2_1010_0004 ^ self.snapshot_digest.rotate_left(7)
    }

    #[cfg(test)]
    pub(crate) fn from_raw_parts_for_test(snapshot_digest: u64) -> Self {
        Self { snapshot_digest }
    }
}
