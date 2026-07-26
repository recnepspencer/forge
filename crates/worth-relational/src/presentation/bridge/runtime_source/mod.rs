use std::sync::Arc;

use crate::logic::runtime::RelationalRuntime;

mod branch_heads;
mod committed_patches;
mod continuity_lineage;
mod execution_basis;
mod snapshot_authority;
mod snapshot_reads;
mod source_profile;

#[derive(Debug, Clone)]
pub struct RuntimeBridgeRelationalSource {
    runtime: Arc<RelationalRuntime>,
    graph_role: Arc<str>,
    partition: Option<RelationalBridgePartitionBinding>,
}

#[derive(Debug, Clone)]
struct RelationalBridgePartitionBinding {
    relational: crate::identity::data::PartitionId,
    truth: worth_foundational::facade::TruthPartitionRole,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationalBridgeSourceConfigurationError {
    InvalidGraphRole,
}

impl RuntimeBridgeRelationalSource {
    pub fn for_graph_role(
        runtime: Arc<RelationalRuntime>,
        graph_role: impl Into<Arc<str>>,
    ) -> Result<Self, RelationalBridgeSourceConfigurationError> {
        let graph_role = graph_role.into();
        if graph_role.trim().is_empty() || graph_role.trim() != graph_role.as_ref() {
            return Err(RelationalBridgeSourceConfigurationError::InvalidGraphRole);
        }
        Ok(Self {
            runtime,
            graph_role,
            partition: None,
        })
    }

    pub fn for_graph_partition(
        runtime: Arc<RelationalRuntime>,
        graph_role: impl Into<Arc<str>>,
        relational_partition: crate::identity::data::PartitionId,
        truth_partition: worth_foundational::facade::TruthPartitionRole,
    ) -> Result<Self, RelationalBridgeSourceConfigurationError> {
        let mut source = Self::for_graph_role(runtime, graph_role)?;
        source.partition = Some(RelationalBridgePartitionBinding {
            relational: relational_partition,
            truth: truth_partition,
        });
        Ok(source)
    }

    fn publish_commit(
        &self,
        commit_id: crate::history::data::CommitId,
    ) -> super::RelationalBridgePublicationOutcome {
        match &self.partition {
            Some(partition) => self.runtime.publish_commit_for_bridge_graph_partition(
                commit_id,
                self.graph_role.clone(),
                partition.relational,
                partition.truth.clone(),
            ),
            None => self
                .runtime
                .publish_commit_for_bridge_graph_role(commit_id, self.graph_role.clone()),
        }
    }

    fn admits_relational_partition(&self, partition_id: u32) -> bool {
        self.partition
            .as_ref()
            .is_none_or(|partition| partition.relational.as_u32() == partition_id)
    }
}
