use std::sync::{Arc, Mutex};

use crate::logic::runtime::RelationalRuntime;

mod branch_heads;
mod committed_patches;
mod continuity_lineage;
mod execution_basis;
pub(super) mod runtime_authority;
mod snapshot_authority;
mod snapshot_reads;
mod source_profile;

#[derive(Debug, Clone)]
pub struct RuntimeBridgeRelationalSource {
    runtime: runtime_authority::RelationalBridgeRuntimeAuthority,
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
            runtime: runtime_authority::RelationalBridgeRuntimeAuthority::immutable(runtime),
            graph_role,
            partition: None,
        })
    }

    pub fn for_shared_graph_role(
        runtime: Arc<Mutex<RelationalRuntime>>,
        graph_role: impl Into<Arc<str>>,
    ) -> Result<Self, RelationalBridgeSourceConfigurationError> {
        let graph_role = graph_role.into();
        validate_graph_role(&graph_role)?;
        Ok(Self {
            runtime: runtime_authority::RelationalBridgeRuntimeAuthority::shared(runtime),
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
        self.runtime.with_runtime(|runtime| match &self.partition {
            Some(partition) => runtime.publish_commit_for_bridge_graph_partition(
                commit_id,
                self.graph_role.clone(),
                partition.relational,
                partition.truth.clone(),
            ),
            None => {
                runtime.publish_commit_for_bridge_graph_role(commit_id, self.graph_role.clone())
            }
        })
    }

    fn admits_relational_partition(&self, partition_id: u32) -> bool {
        self.partition
            .as_ref()
            .is_none_or(|partition| partition.relational.as_u32() == partition_id)
    }
}

fn validate_graph_role(
    graph_role: &Arc<str>,
) -> Result<(), RelationalBridgeSourceConfigurationError> {
    if graph_role.trim().is_empty() || graph_role.trim() != graph_role.as_ref() {
        Err(RelationalBridgeSourceConfigurationError::InvalidGraphRole)
    } else {
        Ok(())
    }
}
