use std::sync::Arc;

use crate::data::graph::runtime::scratch::ScratchLeaseKind;
use crate::data::handle::NodeId;
use crate::data::proof::invalidation::binding::DependencyRevision;
use crate::data::proof::invalidation::progression::InvalidationWorkBindingAxes;
use crate::data::proof::invalidation::progression::{
    InvalidationOriginBinding, InvalidationReadinessEpoch, InvalidationStageOrder,
};
use crate::data::telemetry::SignalInvalidationRealizedCounters;
use crate::logic::transaction::{SignalObservationCompletion, SignalObservationRequest};
use crate::schema::data::SignalSchemaRegistry;

use super::SignalGraph;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SignalGraphRetainedObservation {
    pub(crate) serialized_authority: serde_json::Value,
    pub(crate) schema_registry: SignalSchemaRegistry,
    pub(crate) cause_readmission_required: bool,
    pub(crate) traversal: SignalTraversalReplacementObservation,
    pub(crate) conditional_dependency_versions: Vec<(NodeId, Vec<u64>)>,
    pub(crate) authorization_policy_identities: Vec<[u8; 32]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SignalTraversalReplacementObservation {
    scratch_lease: Option<ScratchLeaseKind>,
    cycle_stack: Vec<(NodeId, bool)>,
    node_buffer_a: Vec<NodeId>,
    node_buffer_b: Vec<NodeId>,
    planner_targets: Vec<NodeId>,
    topology_node_buffer: Vec<NodeId>,
    topology_dependency_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SignalGraphCloneLocalObservation {
    pub(crate) lifecycle_token_identity: usize,
    pub(crate) graph_instance_id: u64,
    pub(crate) schema_registry_identity: usize,
    pub(crate) aspect_lowering_owner_present: bool,
    pub(crate) invalidation_readiness_epoch: u64,
    pub(crate) observation_active_generation: u64,
    pub(crate) observation_active_request: SignalObservationRequest,
    pub(crate) completed_execution_boundaries: u64,
    pub(crate) last_completion: Option<SignalObservationCompletion>,
    pub(crate) observation_cleanup_identity: Option<usize>,
    pub(crate) performed_counters: SignalInvalidationRealizedCounters,
    pub(crate) performed_work: Vec<InvalidationWorkBindingAxes>,
    pub(crate) pending_repeated_admissions: Vec<(NodeId, u64)>,
}

impl SignalGraph {
    pub(crate) fn replacement_retained_observation(&self) -> SignalGraphRetainedObservation {
        SignalGraphRetainedObservation {
            serialized_authority: serde_json::to_value(self)
                .expect("Signal graph retained authority serializes for test observation"),
            schema_registry: (*self.schema_registry).clone(),
            cause_readmission_required: self.cause_readmission_required,
            traversal: SignalTraversalReplacementObservation {
                scratch_lease: self.traversal.scratch_lease,
                cycle_stack: self.traversal.scratch.cycle_stack.clone(),
                node_buffer_a: self.traversal.scratch.node_buffer_a.clone(),
                node_buffer_b: self.traversal.scratch.node_buffer_b.clone(),
                planner_targets: self.traversal.scratch.planner_targets.clone(),
                topology_node_buffer: self.traversal.topology_node_buffer.clone(),
                topology_dependency_count: self.traversal.topology_dependency_buffer.len(),
            },
            conditional_dependency_versions: self
                .conditional_dependency_versions
                .iter()
                .map(|(node, versions)| (*node, versions.clone()))
                .collect(),
            authorization_policy_identities: self
                .authorization_policy_identities
                .iter()
                .copied()
                .collect(),
        }
    }

    pub(crate) fn replacement_clone_local_observation(&self) -> SignalGraphCloneLocalObservation {
        SignalGraphCloneLocalObservation {
            lifecycle_token_identity: Arc::as_ptr(&self.lifecycle_token) as usize,
            graph_instance_id: self.instance_id,
            schema_registry_identity: Arc::as_ptr(&self.schema_registry) as usize,
            aspect_lowering_owner_present: self.aspect_lowering_owner.is_some(),
            invalidation_readiness_epoch: self.invalidation_readiness_epoch,
            observation_active_generation: self.observation_sessions.active_generation(),
            observation_active_request: self.observation_sessions.active_request(),
            completed_execution_boundaries: self
                .observation_sessions
                .completed_execution_boundaries(),
            last_completion: self.observation_sessions.last_completion(),
            observation_cleanup_identity: self
                .observation_capture_cleanup
                .as_ref()
                .map(|cleanup| Arc::as_ptr(cleanup) as usize),
            performed_counters: self.invalidation_performed_counters.snapshot(),
            performed_work: self.invalidation_performed_work.snapshot(),
            pending_repeated_admissions: self
                .pending_repeated_invalidation_admissions
                .iter()
                .map(|(node, count)| (*node, *count))
                .collect(),
        }
    }

    pub(crate) fn populate_replacement_clone_contract(&mut self, node: NodeId) {
        self.traversal.scratch_lease = Some(ScratchLeaseKind::Churn);
        self.traversal.scratch.cycle_stack.push((node, true));
        self.traversal.scratch.node_buffer_a.push(node);
        self.traversal.scratch.node_buffer_b.push(node);
        self.traversal.scratch.planner_targets.push(node);
        self.traversal.topology_node_buffer.push(node);
        self.conditional_dependency_versions
            .insert(node, vec![3, 5, 8]);
        self.authorization_policy_identities.insert([0xA5; 32]);
        self.claim_aspect_lowering_owner(&crate::data::aspect::SignalAspectLoweringOwner::fresh())
            .expect("replacement fixture claims one lowering owner");
        self.invalidation_readiness_epoch = 41;
        let observation_generation = self
            .observation_sessions
            .begin(SignalObservationRequest::counters().with_performed_work());
        assert_ne!(
            observation_generation, 0,
            "replacement fixture begins a live observation generation"
        );
        self.observation_sessions
            .record_completed_execution_boundary();
        self.observation_sessions
            .record_completion(SignalObservationCompletion::Completed);
        self.invalidation_performed_counters.set(
            crate::data::telemetry::InvalidationPerformedCounter::NodesEvaluated,
            17,
        );
        self.invalidation_performed_work
            .record(InvalidationWorkBindingAxes {
                graph_instance: self.instance_id,
                target: node,
                dependency_revision: DependencyRevision(29),
                origin: InvalidationOriginBinding::StructuralMutation { ordinal: 31 },
                readiness_epoch: InvalidationReadinessEpoch(41),
                stage_order: InvalidationStageOrder { stage: 2, order: 7 },
            });
        self.pending_repeated_invalidation_admissions
            .insert(node, 9);
    }
}
