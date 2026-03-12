use std::fmt;

use forge_core::KernelError;
#[cfg(test)]
use forge_signal::facade::NodeState;
use forge_signal::facade::{
    evaluate_in_txn_with_mode, Aspect, AspectVersion, CheckpointBarrier, DefaultComparatorResolver,
    DependencyEdge, EvaluationCondition, EvaluationRequestMode, NodeId, SignalError, SignalGraph,
    SignalRuntime, TransactionOutcome,
};
use forge_topo::projection::{
    compute_projected_topology_hash, validate_projected_topology_structural, ProjectedTopology,
    ProjectionBuilder,
};

use crate::configuration::facade::FingerprintDetail;
use crate::engine::contract::InvariantKind;
use crate::proof::{ValidationCheckpoint, ValidationConfig, ValidationResult};

use super::{projected_topology_error_to_kernel_ref, SpecEnvelope};

const TOPOLOGY_ASPECT: Aspect = Aspect::new(0);
const GEOMETRY_ASPECT: Aspect = Aspect::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum SpecEnvelopeSignalTier {
    Core,
    Deferred,
}

type SpecEnvelopeRuntime = SignalRuntime<(), (), (), (), SpecEnvelopeSignalTier>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpecEnvelopeSignalNode {
    Root,
    Projection,
    StructureValidation,
    ManifoldInvariant,
    PostFeatureCheckpoint,
    StandardFingerprint,
    FullFingerprint,
}

pub(super) struct SpecEnvelopeSignalState {
    runtime: SpecEnvelopeRuntime,
    root: NodeId,
    projection: NodeId,
    structure_validation: NodeId,
    manifold_invariant: NodeId,
    post_feature_checkpoint: NodeId,
    standard_fingerprint: NodeId,
    full_fingerprint: NodeId,
}

impl fmt::Debug for SpecEnvelopeSignalState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpecEnvelopeSignalState")
            .field("root", &self.root)
            .field("projection", &self.projection)
            .field("structure_validation", &self.structure_validation)
            .field("manifold_invariant", &self.manifold_invariant)
            .field("post_feature_checkpoint", &self.post_feature_checkpoint)
            .field("standard_fingerprint", &self.standard_fingerprint)
            .field("full_fingerprint", &self.full_fingerprint)
            .finish()
    }
}

impl SpecEnvelopeSignalState {
    pub(super) fn new() -> Self {
        let mut graph = SignalGraph::new();
        let root = graph.create_node();
        let projection = graph
            .node()
            .condition(EvaluationCondition::OnDemand)
            .build();
        let structure_validation = graph
            .node()
            .condition(EvaluationCondition::OnDemand)
            .build();
        let manifold_invariant = graph
            .node()
            .condition(EvaluationCondition::OnDemand)
            .build();
        let post_feature_checkpoint = graph
            .node()
            .condition(EvaluationCondition::OnDemand)
            .build();
        let standard_fingerprint = graph
            .node()
            .condition(EvaluationCondition::OnDemand)
            .build();
        let full_fingerprint = graph
            .node()
            .condition(EvaluationCondition::OnDemand)
            .build();

        graph
            .set_dependencies(projection, [DependencyEdge::new(root, TOPOLOGY_ASPECT)])
            .expect("spec envelope projection dependency should wire");
        graph
            .set_dependencies(
                structure_validation,
                [DependencyEdge::new(projection, TOPOLOGY_ASPECT)],
            )
            .expect("spec envelope structure dependency should wire");
        graph
            .set_dependencies(
                manifold_invariant,
                [DependencyEdge::new(projection, TOPOLOGY_ASPECT)],
            )
            .expect("spec envelope invariant dependency should wire");
        graph
            .set_dependencies(
                post_feature_checkpoint,
                [DependencyEdge::new(manifold_invariant, TOPOLOGY_ASPECT)],
            )
            .expect("spec envelope checkpoint dependency should wire");
        graph
            .set_dependencies(
                standard_fingerprint,
                [DependencyEdge::new(root, TOPOLOGY_ASPECT)],
            )
            .expect("spec envelope standard fingerprint dependency should wire");
        graph
            .set_dependencies(
                full_fingerprint,
                [DependencyEdge::new(projection, TOPOLOGY_ASPECT)],
            )
            .expect("spec envelope full fingerprint dependency should wire");

        let mut runtime = SignalRuntime::builder(graph)
            .with_tiers::<SpecEnvelopeSignalTier>()
            .checkpoint_barrier(CheckpointBarrier::PerOperation)
            .build();
        runtime.set_node_tier(root, SpecEnvelopeSignalTier::Core);
        runtime.set_node_tier(projection, SpecEnvelopeSignalTier::Core);
        runtime.set_node_tier(structure_validation, SpecEnvelopeSignalTier::Deferred);
        runtime.set_node_tier(manifold_invariant, SpecEnvelopeSignalTier::Deferred);
        runtime.set_node_tier(post_feature_checkpoint, SpecEnvelopeSignalTier::Deferred);
        runtime.set_node_tier(standard_fingerprint, SpecEnvelopeSignalTier::Deferred);
        runtime.set_node_tier(full_fingerprint, SpecEnvelopeSignalTier::Deferred);

        Self {
            runtime,
            root,
            projection,
            structure_validation,
            manifold_invariant,
            post_feature_checkpoint,
            standard_fingerprint,
            full_fingerprint,
        }
    }

    fn node_id(&self, node: SpecEnvelopeSignalNode) -> NodeId {
        match node {
            SpecEnvelopeSignalNode::Root => self.root,
            SpecEnvelopeSignalNode::Projection => self.projection,
            SpecEnvelopeSignalNode::StructureValidation => self.structure_validation,
            SpecEnvelopeSignalNode::ManifoldInvariant => self.manifold_invariant,
            SpecEnvelopeSignalNode::PostFeatureCheckpoint => self.post_feature_checkpoint,
            SpecEnvelopeSignalNode::StandardFingerprint => self.standard_fingerprint,
            SpecEnvelopeSignalNode::FullFingerprint => self.full_fingerprint,
        }
    }
}

impl Default for SpecEnvelopeSignalState {
    fn default() -> Self {
        Self::new()
    }
}

impl SpecEnvelope {
    fn kernel_to_signal(err: KernelError) -> SignalError {
        SignalError::internal(err.to_string())
    }

    fn signal_to_kernel(err: SignalError) -> KernelError {
        KernelError::InternalError {
            message: err.to_string(),
            context: None,
        }
    }

    fn root_aspect_version(&self) -> AspectVersion {
        let spec_hash = self.spec.spec_hash();
        let topology_version = ((spec_hash >> 64) as u64) ^ (spec_hash as u64);
        AspectVersion::from_updates([
            (TOPOLOGY_ASPECT, topology_version.max(1)),
            (GEOMETRY_ASPECT, 0),
        ])
    }

    fn projected_topology(&self) -> Result<&ProjectedTopology, KernelError> {
        self.projection
            .get_or_init(|| ProjectionBuilder::build(&self.spec))
            .as_ref()
            .map_err(projected_topology_error_to_kernel_ref)
    }

    fn ensure_signal_node(&self, node: SpecEnvelopeSignalNode) -> Result<(), KernelError> {
        let mut signal = self.signal.borrow_mut();
        let node_id = signal.node_id(node);
        let root_id = signal.root;
        let projection_id = signal.projection;
        let structure_validation_id = signal.structure_validation;
        let manifold_invariant_id = signal.manifold_invariant;
        let post_feature_checkpoint_id = signal.post_feature_checkpoint;
        let standard_fingerprint_id = signal.standard_fingerprint;
        let full_fingerprint_id = signal.full_fingerprint;
        let mut compute =
            |id: NodeId, _graph: &SignalGraph| -> Result<AspectVersion, SignalError> {
                if id == root_id {
                    return Ok(self.root_aspect_version());
                }

                if id == projection_id {
                    let projection = self
                        .projection
                        .get_or_init(|| ProjectionBuilder::build(&self.spec));
                    let projection = projection.as_ref().map_err(|err| {
                        Self::kernel_to_signal(projected_topology_error_to_kernel_ref(err))
                    })?;
                    let projection_hash = compute_projected_topology_hash(projection);
                    let projection_version =
                        ((projection_hash >> 64) as u64) ^ (projection_hash as u64);
                    return Ok(AspectVersion::from_updates([(
                        TOPOLOGY_ASPECT,
                        projection_version.max(1),
                    )]));
                }

                if id == structure_validation_id || id == manifold_invariant_id {
                    self.validate_projected_structure_now()
                        .map_err(Self::kernel_to_signal)?;
                    let projection = self.projected_topology().map_err(Self::kernel_to_signal)?;
                    let projection_hash = compute_projected_topology_hash(projection);
                    let projection_version =
                        ((projection_hash >> 64) as u64) ^ (projection_hash as u64);
                    return Ok(AspectVersion::from_updates([(
                        TOPOLOGY_ASPECT,
                        projection_version.max(1),
                    )]));
                }

                if id == post_feature_checkpoint_id {
                    self.validate_projected_structure_now()
                        .map_err(Self::kernel_to_signal)?;
                    let total_entities =
                        self.entity_count_now().map_err(Self::kernel_to_signal)? as u64;
                    return Ok(AspectVersion::from_updates([(
                        TOPOLOGY_ASPECT,
                        total_entities.max(1),
                    )]));
                }

                if id == standard_fingerprint_id {
                    let hash = self
                        .standard_fingerprint
                        .get_or_init(|| Ok(self.spec.spec_hash()))
                        .as_ref()
                        .map(|hash| *hash)
                        .map_err(Clone::clone)
                        .map_err(Self::kernel_to_signal)?;
                    let version = ((hash >> 64) as u64) ^ (hash as u64);
                    return Ok(AspectVersion::from_updates([(
                        TOPOLOGY_ASPECT,
                        version.max(1),
                    )]));
                }

                if id == full_fingerprint_id {
                    let hash = self
                        .full_fingerprint
                        .get_or_init(|| {
                            let projection = self.projected_topology()?;
                            Ok(compute_projected_topology_hash(projection))
                        })
                        .as_ref()
                        .map(|hash| *hash)
                        .map_err(Clone::clone)
                        .map_err(Self::kernel_to_signal)?;
                    let version = ((hash >> 64) as u64) ^ (hash as u64);
                    return Ok(AspectVersion::from_updates([(
                        TOPOLOGY_ASPECT,
                        version.max(1),
                    )]));
                }

                Err(SignalError::internal("unknown spec envelope signal node"))
            };

        let mut txn = signal.runtime.begin();
        let result = evaluate_in_txn_with_mode(
            &mut txn,
            node_id,
            &mut compute,
            DefaultComparatorResolver,
            EvaluationRequestMode::ForceOnDemand,
        );
        if let Err(err) = result {
            let mut runtime_ctx = ();
            let _ = txn.rollback(&mut runtime_ctx);
            return Err(Self::signal_to_kernel(err));
        }

        let mut runtime_ctx = ();
        match txn
            .commit(&mut runtime_ctx)
            .map_err(Self::signal_to_kernel)?
        {
            TransactionOutcome::Committed => Ok(()),
            TransactionOutcome::RolledBack | TransactionOutcome::Poisoned => {
                Err(KernelError::InternalError {
                    message: "spec envelope signal evaluation rolled back".to_string(),
                    context: None,
                })
            }
        }
    }

    pub(super) fn ensure_projection_ready(&self) -> Result<(), KernelError> {
        self.ensure_signal_node(SpecEnvelopeSignalNode::Projection)
    }

    pub(super) fn ensure_structure_validated(&self) -> Result<(), KernelError> {
        self.ensure_signal_node(SpecEnvelopeSignalNode::StructureValidation)
    }

    pub(super) fn ensure_invariant_validated(
        &self,
        kind: &InvariantKind,
    ) -> Result<(), KernelError> {
        match kind {
            InvariantKind::ManifoldEdges => {
                self.ensure_signal_node(SpecEnvelopeSignalNode::ManifoldInvariant)
            }
            InvariantKind::G1Continuity => Ok(()),
            InvariantKind::NoSelfIntersection => Ok(()),
            InvariantKind::NoSliverFaces => Ok(()),
        }
    }

    pub(super) fn ensure_checkpoint_validated(
        &self,
        checkpoint: ValidationCheckpoint,
    ) -> Result<(), KernelError> {
        match checkpoint {
            ValidationCheckpoint::PerOp
            | ValidationCheckpoint::PostCommit
            | ValidationCheckpoint::PostBoolean
            | ValidationCheckpoint::PostImport
            | ValidationCheckpoint::OnDemand => self.ensure_structure_validated(),
            ValidationCheckpoint::PostFeature => {
                self.ensure_signal_node(SpecEnvelopeSignalNode::PostFeatureCheckpoint)
            }
        }
    }

    pub(super) fn fingerprint_now(&self, detail: FingerprintDetail) -> Result<u128, KernelError> {
        match detail {
            FingerprintDetail::Standard => {
                self.ensure_signal_node(SpecEnvelopeSignalNode::StandardFingerprint)?;
                self.standard_fingerprint
                    .get_or_init(|| Ok(self.spec.spec_hash()))
                    .as_ref()
                    .map(|hash| *hash)
                    .map_err(Clone::clone)
            }
            FingerprintDetail::Full => {
                self.ensure_signal_node(SpecEnvelopeSignalNode::FullFingerprint)?;
                self.full_fingerprint
                    .get_or_init(|| {
                        let projection = self.projected_topology()?;
                        Ok(compute_projected_topology_hash(projection))
                    })
                    .as_ref()
                    .map(|hash| *hash)
                    .map_err(Clone::clone)
            }
        }
    }

    pub(super) fn validate_projected_structure_now(&self) -> Result<(), KernelError> {
        let projected = self.projected_topology()?;
        validate_projected_topology_structural(projected)
    }

    pub(super) fn entity_count_now(&self) -> Result<usize, KernelError> {
        let projection = self.projected_topology()?;
        Ok(projection.face_count()
            + projection.half_edge_count()
            + projection.vertex_count()
            + projection.loop_count())
    }

    pub(super) fn checkpoint_result_now(
        &self,
        config: &ValidationConfig,
        checkpoint: ValidationCheckpoint,
    ) -> Result<ValidationResult, KernelError> {
        let total_entities = self.entity_count_now()?;

        if !config.is_active(checkpoint) {
            return Ok(ValidationResult::skipped(checkpoint, total_entities));
        }

        if config.should_skip_for_entity_count(total_entities) {
            return Ok(ValidationResult::skipped(checkpoint, total_entities));
        }

        let start = std::time::Instant::now();
        self.ensure_checkpoint_validated(checkpoint)?;
        let duration_micros = start.elapsed().as_micros() as u64;

        Ok(ValidationResult::passed(
            checkpoint,
            total_entities,
            false,
            duration_micros,
        ))
    }

    #[cfg(test)]
    pub(crate) fn debug_signal_telemetry(&self) -> forge_signal::facade::RuntimeTelemetry {
        self.signal.borrow().runtime.telemetry().to_owned()
    }

    #[cfg(test)]
    pub(crate) fn debug_signal_node_state(&self, node: &str) -> Option<NodeState> {
        let signal = self.signal.borrow();
        let id = match node {
            "root" => signal.root,
            "projection" => signal.projection,
            "structure" => signal.structure_validation,
            "invariant" => signal.manifold_invariant,
            "checkpoint" => signal.post_feature_checkpoint,
            "standard_fingerprint" => signal.standard_fingerprint,
            "full_fingerprint" => signal.full_fingerprint,
            _ => return None,
        };
        signal
            .runtime
            .graph()
            .get_entry(id)
            .ok()
            .map(|entry| *entry.get_state())
    }
}
