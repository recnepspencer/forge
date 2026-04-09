use std::sync::Arc;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSourceCapability {
    SnapshotRead,
    HistoricalRead,
    BranchRead,
    FacetRead,
    ReplayCompatibleRead,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSourceCapabilitySet {
    capabilities: Arc<[BridgeSourceCapability]>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSourceCapabilitySet {
    pub fn new(mut capabilities: Vec<BridgeSourceCapability>) -> Self {
        capabilities.sort_unstable();
        capabilities.dedup();

        let canonical_basis = Arc::<str>::from(format!(
            "source-capability-set|capabilities={}",
            capabilities
                .iter()
                .map(|capability| format!("{capability:?}"))
                .collect::<Vec<_>>()
                .join(",")
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            capabilities: Arc::from(capabilities),
            canonical_basis,
            digest: Arc::from(format!("source-capability-set:sha256:{digest:x}")),
        }
    }

    pub fn capabilities(&self) -> &[BridgeSourceCapability] {
        &self.capabilities
    }

    pub fn contains(&self, capability: BridgeSourceCapability) -> bool {
        self.capabilities.binary_search(&capability).is_ok()
    }

    pub fn contains_all(&self, required: &Self) -> bool {
        required
            .capabilities()
            .iter()
            .all(|capability| self.contains(*capability))
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{BridgeSourceCapability, BridgeSourceCapabilitySet};

    #[test]
    fn source_capability_set_is_canonical_for_same_inputs() {
        let left = BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::FacetRead,
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::SnapshotRead,
        ]);
        let right = BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::FacetRead,
        ]);

        assert_eq!(left, right);
        assert_eq!(
            left.capabilities(),
            &[
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::FacetRead
            ]
        );
    }
}
