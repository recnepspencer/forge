use crate::capability::CapabilitySnapshotDigest;
use crate::runtime::admission::WorthUiQuerySupportReceipt;
use crate::runtime::candidate::WorthUiCandidateDependencyMetadata;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCandidateLoweringBasis {
    snapshot_digest: u64,
    query_support_receipt: WorthUiQuerySupportReceipt,
}

impl WorthUiCandidateLoweringBasis {
    pub(crate) fn from_snapshot_and_dependency_metadata(
        snapshot_digest: CapabilitySnapshotDigest,
        dependency_metadata: &WorthUiCandidateDependencyMetadata,
    ) -> Self {
        Self {
            snapshot_digest: snapshot_digest.as_u64(),
            query_support_receipt: WorthUiQuerySupportReceipt::from_dependency_metadata(
                dependency_metadata,
            ),
        }
    }

    pub fn snapshot_digest(self) -> u64 {
        self.snapshot_digest
    }

    pub fn query_support_receipt(self) -> WorthUiQuerySupportReceipt {
        self.query_support_receipt
    }

    pub(crate) fn basis_digest(self) -> u64 {
        0x6d33_39e2_1010_0003
            ^ self.snapshot_digest.rotate_left(7)
            ^ self
                .query_support_receipt
                .contract_identity()
                .as_u64()
                .rotate_left(31)
    }

    #[cfg(test)]
    pub(crate) fn from_raw_parts_for_test(
        snapshot_digest: u64,
        query_support_receipt: WorthUiQuerySupportReceipt,
    ) -> Self {
        Self {
            snapshot_digest,
            query_support_receipt,
        }
    }
}
