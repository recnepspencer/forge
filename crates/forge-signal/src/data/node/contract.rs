use serde::{Deserialize, Serialize};

use crate::data::aspect::AspectMask;
use crate::data::output::{scopes_overlap, PartitionSubscription};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ContextRequirement {
    #[default]
    None,
    DomainContext,
    RelationalSnapshot,
}

/// Declarative contract for one node's evaluation semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeContract {
    pub reads: AspectMask,
    pub produces: AspectMask,
    #[serde(default)]
    pub partition_scope: Option<Vec<PartitionSubscription>>,
    #[serde(default)]
    pub required_context: ContextRequirement,
}

impl NodeContract {
    pub fn wildcard() -> Self {
        Self {
            reads: AspectMask::ALL,
            produces: AspectMask::ALL,
            partition_scope: None,
            required_context: ContextRequirement::None,
        }
    }

    pub fn reads(reads: impl Into<AspectMask>) -> Self {
        Self {
            reads: reads.into(),
            ..Self::wildcard()
        }
    }

    pub fn with_reads(mut self, reads: impl Into<AspectMask>) -> Self {
        self.reads = reads.into();
        self
    }

    pub fn with_produces(mut self, produces: impl Into<AspectMask>) -> Self {
        self.produces = produces.into();
        self
    }

    pub fn with_partition_scope(
        mut self,
        partition_scope: impl Into<PartitionSubscription>,
    ) -> Self {
        self.partition_scope = Some(vec![partition_scope.into()]);
        self
    }

    pub fn with_partition_scopes(
        mut self,
        partition_scopes: impl IntoIterator<Item = PartitionSubscription>,
    ) -> Self {
        self.partition_scope = Some(partition_scopes.into_iter().collect());
        self
    }

    pub fn with_required_context(mut self, required_context: ContextRequirement) -> Self {
        self.required_context = required_context;
        self
    }

    pub fn reads_dirty_aspects(&self, dirty_aspects: AspectMask) -> bool {
        self.reads.intersects(dirty_aspects)
    }

    pub fn cares_about_change(
        &self,
        changed_aspects: AspectMask,
        changed_scopes: &[PartitionSubscription],
    ) -> bool {
        if !self.reads.intersects(changed_aspects) {
            return false;
        }
        match &self.partition_scope {
            None => true,
            Some(contract_scopes) if changed_scopes.is_empty() => true,
            Some(contract_scopes) => contract_scopes.iter().any(|contract_scope| {
                changed_scopes
                    .iter()
                    .any(|changed_scope| scopes_overlap(contract_scope, changed_scope))
            }),
        }
    }
}

impl Default for NodeContract {
    fn default() -> Self {
        Self::wildcard()
    }
}
