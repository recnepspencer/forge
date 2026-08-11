use super::super::classification_error;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportPortabilityManifestBudget {
    max_manifest_entries: u64,
    max_manifest_header_bytes: u64,
}

impl SupportPortabilityManifestBudget {
    pub fn new(
        max_manifest_entries: u64,
        max_manifest_header_bytes: u64,
    ) -> Result<Self, StoreError> {
        if max_manifest_entries == 0 || max_manifest_header_bytes == 0 {
            return Err(classification_error(
                "subscription-support portability manifest budgets must be non-zero",
            ));
        }
        Ok(Self {
            max_manifest_entries,
            max_manifest_header_bytes,
        })
    }

    pub fn admits(&self, manifest_entries: u64, manifest_header_bytes: u64) -> bool {
        manifest_entries <= self.max_manifest_entries
            && manifest_header_bytes <= self.max_manifest_header_bytes
    }

    pub fn max_manifest_entries(&self) -> u64 {
        self.max_manifest_entries
    }

    pub fn max_manifest_header_bytes(&self) -> u64 {
        self.max_manifest_header_bytes
    }
}
