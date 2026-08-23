use std::sync::{Arc, Mutex};

use crate::runtime::RelationalRuntime;

mod branch_basis;
mod branch_head_bindings;
mod branch_heads;
mod committed_patches;
mod continuity_lineage;
mod observation_bindings;
mod snapshot_reads;
mod source_profile;

pub use branch_head_bindings::{
    RelationalBridgeBranchHeadLease, RelationalBridgeBranchHeadReleaseReceipt,
};
pub use observation_bindings::{
    RelationalBridgeObservationLease, RelationalBridgeObservationReleaseReceipt,
};

#[derive(Debug, Clone)]
pub struct RuntimeBridgeRelationalSource {
    runtime: crate::visibility::runtime_authority::RelationalVisibilityRuntimeAuthority,
    observation_bindings: Arc<observation_bindings::RelationalBridgeObservationBindings>,
    branch_head_bindings: Arc<branch_head_bindings::RelationalBridgeBranchHeadBindings>,
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
            runtime: crate::visibility::runtime_authority::RelationalVisibilityRuntimeAuthority::immutable(runtime),
            observation_bindings: observation_bindings::RelationalBridgeObservationBindings::new(),
            branch_head_bindings: branch_head_bindings::RelationalBridgeBranchHeadBindings::new(),
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
            runtime:
                crate::visibility::runtime_authority::RelationalVisibilityRuntimeAuthority::shared(
                    runtime,
                ),
            observation_bindings: observation_bindings::RelationalBridgeObservationBindings::new(),
            branch_head_bindings: branch_head_bindings::RelationalBridgeBranchHeadBindings::new(),
            graph_role,
            partition: None,
        })
    }

    pub fn for_shared_graph_partition(
        runtime: Arc<Mutex<RelationalRuntime>>,
        graph_role: impl Into<Arc<str>>,
        relational_partition: crate::identity::data::PartitionId,
        truth_partition: worth_foundational::facade::TruthPartitionRole,
    ) -> Result<Self, RelationalBridgeSourceConfigurationError> {
        let mut source = Self::for_shared_graph_role(runtime, graph_role)?;
        source.partition = Some(RelationalBridgePartitionBinding {
            relational: relational_partition,
            truth: truth_partition,
        });
        Ok(source)
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
    ) -> Result<
        super::RelationalBridgePublicationOutcome,
        worth_runtime_bridge::facade::RelationalBridgeSourceError,
    > {
        let snapshot_identity = self
            .observation_bindings
            .snapshot_identity_for_commit(commit_id)?;
        Ok(self.publish_commit_at_snapshot(commit_id, snapshot_identity))
    }

    fn publish_commit_at_snapshot(
        &self,
        commit_id: crate::history::data::CommitId,
        snapshot_identity: worth_runtime_bridge::facade::TruthSnapshotIdentity,
    ) -> super::RelationalBridgePublicationOutcome {
        self.runtime.with_runtime(|runtime| match &self.partition {
            Some(partition) => runtime.publish_commit_for_bridge_graph_partition_at_snapshot(
                commit_id,
                self.graph_role.clone(),
                partition.relational,
                partition.truth.clone(),
                snapshot_identity,
            ),
            None => runtime.publish_commit_for_bridge_graph_role_at_snapshot(
                commit_id,
                self.graph_role.clone(),
                snapshot_identity,
            ),
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
