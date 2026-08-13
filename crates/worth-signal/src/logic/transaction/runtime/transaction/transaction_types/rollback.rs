use crate::data::handle::NodeId;

use super::super::super::super::patch_buffer::SparsePatchBuffer;
use super::super::super::config::SignalRuntimeConfig;
use super::super::super::state::{ResourceRuntimeState, TemporalRuntimeState};

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct ConfigRollbackDelta<T: Copy + Ord> {
    pub baseline: SignalRuntimeConfig<T>,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct DiagnosticsRollbackDelta {
    pub baseline: crate::diagnostics::state::DiagnosticsState,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct GraphPatchRollbackDelta {
    pub patches: SparsePatchBuffer,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct GraphCauseAuthorityRollbackDelta {
    pub baseline: crate::data::graph::storage::invalidation_causes::CanonicalCauseSetStore,
    pub readmission_required: bool,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct CreatedNodeRollbackDelta {
    pub created_nodes: Vec<NodeId>,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct SubscriberRepairRollbackDelta {
    pub sources: Vec<NodeId>,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct ResourceRollbackDelta {
    pub baseline: ResourceRuntimeState,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct TemporalRollbackDelta {
    pub baseline: TemporalRuntimeState,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) enum TransactionRollbackPacket<T: Copy + Ord> {
    Config(ConfigRollbackDelta<T>),
    DiagnosticsRequired(DiagnosticsRollbackDelta),
    GraphPatches(GraphPatchRollbackDelta),
    GraphCauseAuthority(GraphCauseAuthorityRollbackDelta),
    CreatedNodes(CreatedNodeRollbackDelta),
    SubscriberRepair(SubscriberRepairRollbackDelta),
    Resource(ResourceRollbackDelta),
    Temporal(TemporalRollbackDelta),
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct TransactionRollbackPacketSet<T: Copy + Ord> {
    config: Option<ConfigRollbackDelta<T>>,
    diagnostics: Option<DiagnosticsRollbackDelta>,
    graph_patches: Option<GraphPatchRollbackDelta>,
    graph_cause_authority: Option<GraphCauseAuthorityRollbackDelta>,
    created_nodes: Option<CreatedNodeRollbackDelta>,
    subscriber_repair: Option<SubscriberRepairRollbackDelta>,
    resource: Option<ResourceRollbackDelta>,
    temporal: Option<TemporalRollbackDelta>,
}

impl<T: Copy + Ord> Default for TransactionRollbackPacketSet<T> {
    fn default() -> Self {
        Self {
            config: None,
            diagnostics: None,
            graph_patches: None,
            graph_cause_authority: None,
            created_nodes: None,
            subscriber_repair: None,
            resource: None,
            temporal: None,
        }
    }
}

impl<T: Copy + Ord> TransactionRollbackPacketSet<T> {
    pub fn capture_runtime_baseline_if_needed(
        &mut self,
        config: &SignalRuntimeConfig<T>,
        diagnostics_state: &crate::diagnostics::state::DiagnosticsState,
        graph: &crate::data::graph::SignalGraph,
    ) {
        if self.config.is_none() {
            self.config = Some(ConfigRollbackDelta {
                baseline: config.clone(),
            });
        }
        if self.diagnostics.is_none() {
            self.diagnostics = Some(DiagnosticsRollbackDelta {
                baseline: diagnostics_state.clone(),
            });
        }
        if self.graph_cause_authority.is_none() {
            self.graph_cause_authority = Some(GraphCauseAuthorityRollbackDelta {
                baseline: graph.cause_sets.clone(),
                readmission_required: graph.cause_readmission_required,
            });
        }
    }

    pub fn stage_graph_patches(
        &mut self,
        delta: GraphPatchRollbackDelta,
    ) -> Result<(), crate::data::error::SignalError> {
        if self.graph_patches.is_some() {
            return Err(crate::data::error::SignalError::internal(
                "graph patch rollback packet was staged more than once",
            ));
        }
        self.graph_patches = Some(delta);
        Ok(())
    }

    pub fn stage_created_nodes(
        &mut self,
        delta: CreatedNodeRollbackDelta,
    ) -> Result<(), crate::data::error::SignalError> {
        if self.created_nodes.is_some() {
            return Err(crate::data::error::SignalError::internal(
                "created-node rollback packet was staged more than once",
            ));
        }
        self.created_nodes = Some(delta);
        Ok(())
    }

    pub fn stage_subscriber_repair(
        &mut self,
        delta: SubscriberRepairRollbackDelta,
    ) -> Result<(), crate::data::error::SignalError> {
        if self.subscriber_repair.is_some() {
            return Err(crate::data::error::SignalError::internal(
                "subscriber-repair rollback packet was staged more than once",
            ));
        }
        self.subscriber_repair = Some(delta);
        Ok(())
    }

    pub fn capture_resource_baseline_if_needed(&mut self, resource: &ResourceRuntimeState) {
        if self.resource.is_none() {
            self.resource = Some(ResourceRollbackDelta {
                baseline: resource.clone(),
            });
        }
    }

    pub fn capture_temporal_baseline_if_needed(&mut self, temporal: &TemporalRuntimeState) {
        if self.temporal.is_none() {
            self.temporal = Some(TemporalRollbackDelta {
                baseline: temporal.clone(),
            });
        }
    }

    pub fn drain_ordered(&mut self) -> Vec<TransactionRollbackPacket<T>> {
        let mut packets = Vec::with_capacity(8);
        if let Some(delta) = self.graph_patches.take() {
            packets.push(TransactionRollbackPacket::GraphPatches(delta));
        }
        if let Some(delta) = self.created_nodes.take() {
            packets.push(TransactionRollbackPacket::CreatedNodes(delta));
        }
        if let Some(delta) = self.graph_cause_authority.take() {
            packets.push(TransactionRollbackPacket::GraphCauseAuthority(delta));
        }
        if let Some(delta) = self.subscriber_repair.take() {
            packets.push(TransactionRollbackPacket::SubscriberRepair(delta));
        }
        if let Some(delta) = self.resource.take() {
            packets.push(TransactionRollbackPacket::Resource(delta));
        }
        if let Some(delta) = self.temporal.take() {
            packets.push(TransactionRollbackPacket::Temporal(delta));
        }
        if let Some(delta) = self.config.take() {
            packets.push(TransactionRollbackPacket::Config(delta));
        }
        if let Some(delta) = self.diagnostics.take() {
            packets.push(TransactionRollbackPacket::DiagnosticsRequired(delta));
        }
        packets
    }
}
