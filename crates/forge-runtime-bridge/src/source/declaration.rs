use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, SourceDeclarationIdentityTag};
use crate::snapshot::BridgeTruthViewSelector;

use super::BridgeSourceCapabilitySet;

pub type SourceDeclarationIdentity = BridgeIdentity<SourceDeclarationIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDeclaration {
    declaration_identity: SourceDeclarationIdentity,
    selector: BridgeTruthViewSelector,
    required_capabilities: BridgeSourceCapabilitySet,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl SourceDeclaration {
    pub fn new(
        declaration_identity: SourceDeclarationIdentity,
        selector: BridgeTruthViewSelector,
        required_capabilities: BridgeSourceCapabilitySet,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "source-declaration|id={}|selector={}|capabilities={}",
            declaration_identity.as_str(),
            selector.canonical_basis(),
            required_capabilities.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            declaration_identity,
            selector,
            required_capabilities,
            canonical_basis,
            digest: Arc::from(format!("source-declaration:sha256:{digest:x}")),
        }
    }

    pub fn declaration_identity(&self) -> &SourceDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn selector(&self) -> &BridgeTruthViewSelector {
        &self.selector
    }

    pub fn required_capabilities(&self) -> &BridgeSourceCapabilitySet {
        &self.required_capabilities
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
    use super::{SourceDeclaration, SourceDeclarationIdentity};
    use crate::input::envelope::TruthBranchIdentity;
    use crate::snapshot::{BridgeTruthViewSelector, TruthSnapshotIdentity};
    use crate::source::{BridgeSourceCapability, BridgeSourceCapabilitySet};

    #[test]
    fn source_declaration_is_canonical_for_same_inputs() {
        let left = SourceDeclaration::new(
            SourceDeclarationIdentity::new("source:profile"),
            BridgeTruthViewSelector::committed_snapshot(
                TruthBranchIdentity::new("main"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeSourceCapabilitySet::new(vec![BridgeSourceCapability::SnapshotRead]),
        );
        let right = SourceDeclaration::new(
            SourceDeclarationIdentity::new("source:profile"),
            BridgeTruthViewSelector::committed_snapshot(
                TruthBranchIdentity::new("main"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeSourceCapabilitySet::new(vec![BridgeSourceCapability::SnapshotRead]),
        );

        assert_eq!(left, right);
        assert!(left.canonical_basis().contains("id=source:profile"));
    }
}
