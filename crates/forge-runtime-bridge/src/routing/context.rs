use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::clone_budget::CheapClone;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BridgeLineageContext;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMappingContext {
    canonical_basis: Arc<str>,
    digest: Arc<str>,
    lineage_context: Option<BridgeLineageContext>,
}

impl BridgeMappingContext {
    pub fn empty() -> Self {
        Self::from_parts("mapping-context|empty", None)
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn lineage_context(&self) -> Option<&BridgeLineageContext> {
        self.lineage_context.as_ref()
    }

    pub fn with_lineage_context(self, lineage_context: BridgeLineageContext) -> Self {
        let canonical_basis = format!("{}|lineage:present", self.canonical_basis());
        Self::from_parts(canonical_basis, Some(lineage_context))
    }

    fn from_parts(
        canonical_basis: impl Into<Arc<str>>,
        lineage_context: Option<BridgeLineageContext>,
    ) -> Self {
        let canonical_basis = canonical_basis.into();
        let digest = Sha256::digest(canonical_basis.as_ref().as_bytes());
        Self {
            digest: format!("mapping-context:sha256:{digest:x}").into(),
            canonical_basis,
            lineage_context,
        }
    }
}

impl Default for BridgeMappingContext {
    fn default() -> Self {
        Self::empty()
    }
}

impl CheapClone for BridgeMappingContext {}
