use serde::{Deserialize, Serialize};

use crate::data::comparator::VersionComparatorPolicy;
use crate::data::handle::NodeId;
use crate::data::node::ContextRequirement;
use crate::data::output::PartitionSubscription;
use crate::data::performance::AuthorityPolicy;

/// Compact runtime evidence needed to certify artifact reuse across semantic boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReuseBoundaryContext {
    pub topology_regime: u32,
    pub tolerance_regime: VersionComparatorPolicy,
    pub semantic_region: ReuseSemanticRegionIdentity,
    pub authority_policy: AuthorityPolicy,
}

/// Stable node-local semantic region identity for one artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReuseSemanticRegionIdentity {
    pub node: NodeId,
    pub partitioned_output: bool,
    #[serde(default)]
    pub partition_scope: Vec<PartitionSubscription>,
    #[serde(default)]
    pub required_context: ContextRequirement,
}

impl ReuseSemanticRegionIdentity {
    pub fn new(
        node: NodeId,
        partitioned_output: bool,
        partition_scope: impl Into<Vec<PartitionSubscription>>,
        required_context: ContextRequirement,
    ) -> Self {
        let mut partition_scope = partition_scope.into();
        if partition_scope.len() > 1 {
            partition_scope.sort_unstable();
            partition_scope.dedup();
        }
        Self {
            node,
            partitioned_output,
            partition_scope,
            required_context,
        }
    }
}
