#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationFootprintReceipt {
    metadata_digest: String,
    footprint_digest: String,
    canonical_digest: String,
}

impl WorthServerOperationFootprintReceipt {
    pub(crate) fn new(
        metadata_digest: impl Into<String>,
        footprint_digest: impl Into<String>,
    ) -> Self {
        let metadata_digest = metadata_digest.into();
        let footprint_digest = footprint_digest.into();
        let canonical_digest = format!(
            "worth-server-operation-footprint-receipt-v1|metadata={metadata_digest}|footprint={footprint_digest}"
        );
        Self {
            metadata_digest,
            footprint_digest,
            canonical_digest,
        }
    }

    pub fn metadata_digest(&self) -> &str {
        &self.metadata_digest
    }

    pub fn footprint_digest(&self) -> &str {
        &self.footprint_digest
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
