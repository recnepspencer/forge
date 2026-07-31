use crate::data::aspect::AspectMask;
use crate::data::dependency::CanonicalDependencies;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::host_computed::{admit_or_error, HostComputedApiFamily};
use crate::data::node::AuthorityPolicy;
use crate::data::node::NodeState;
use crate::data::output::PartitionSubscription;
use crate::data::output::{CanonicalChangedRegions, ChangedRegion, MemoizedResultOrigin};
use crate::data::proof::{
    ClassifiedSnapshotBatchCommit, DedupedNodeBatch, DirtyDelta, PartitionScopeSet,
    SnapshotBatchCommit, SortedSourceBatch, StructuralDelta, TouchedScopeSummary,
};
use crate::data::reuse::ReuseBasis;
use crate::data::temporal::LoweredTemporalEligibility;
use crate::data::trace::RuntimeArtifactFinalizeImage;
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::logic::evaluation::{
    apply_prepared_evaluation_after_dependencies_with_policy, EffectDependencyInputs,
    EvaluationVerdict, PendingDependencySnapshot,
};
use crate::logic::explain::RewiringSummary;
use crate::logic::planner::semantic::StageSemanticIdentity;
#[cfg(feature = "parallel")]
use crate::logic::planner::types::ApplyPlanSerialFallbackReason;
use crate::logic::planner::types::{
    EligibleTask, ExecutionRecordId, LoweredTask, StageExecutionRecord, StageExecutor,
};
use crate::logic::prepared::{
    PreparedEvaluation, PreparedEvaluationOrigin, PreparedEvaluationOutcome,
};

use super::super::execution::task_reporting::record_execution_failure;
use super::super::precompute::PreparedTaskPatch;
use super::super::types::PlanSummary;
use super::lowering_support::{count_dependency_updates, rewiring_summary_from_lowered_edges};

#[derive(Debug, Clone, Copy)]
struct StageTaskOrderWitness;

#[derive(Debug, Clone, Copy)]
pub(in crate::logic::planner) struct StageTaskOrderProof(StageTaskOrderWitness);

impl StageTaskOrderProof {
    fn established() -> Self {
        Self(StageTaskOrderWitness)
    }
}

#[derive(Debug, Clone, Copy)]
pub(in crate::logic::planner) struct ExactStageWidth(usize);

impl ExactStageWidth {
    pub(in crate::logic::planner) fn new(width: usize) -> Self {
        Self(width)
    }

    pub(in crate::logic::planner) fn get(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone)]
pub(in crate::logic::planner) struct SerialApplyInput {
    node: NodeId,
    record_id: ExecutionRecordId,
    prepared: PreparedEvaluation,
    dependency_updates: u32,
    dependency_inputs: EffectDependencyInputs,
}

impl SerialApplyInput {
    fn new(
        node: NodeId,
        record_id: ExecutionRecordId,
        prepared: PreparedEvaluation,
        dependency_updates: u32,
        dependency_inputs: EffectDependencyInputs,
    ) -> Self {
        Self {
            node,
            record_id,
            prepared,
            dependency_updates,
            dependency_inputs,
        }
    }
}

#[derive(Debug, Clone)]
pub(in crate::logic::planner) struct SerialFinalizeSeed {
    pub(in crate::logic::planner) task_index: usize,
    pub(in crate::logic::planner) node: NodeId,
    pub(in crate::logic::planner) identity: StageSemanticIdentity,
    pub(in crate::logic::planner) before_state: NodeState,
    pub(in crate::logic::planner) before_artifact_state: Option<RuntimeArtifactFinalizeImage>,
    pub(in crate::logic::planner) dependency_updates: u32,
    pub(in crate::logic::planner) recomputed: bool,
    pub(in crate::logic::planner) partition_aware: bool,
    pub(in crate::logic::planner) rewiring: Option<RewiringSummary>,
}

impl SerialFinalizeSeed {
    fn from_execution_parts(
        task_index: usize,
        node: NodeId,
        identity: StageSemanticIdentity,
        before_state: NodeState,
        before_artifact_state: Option<RuntimeArtifactFinalizeImage>,
        dependency_updates: u32,
        recomputed: bool,
        partition_aware: bool,
        rewiring: Option<RewiringSummary>,
    ) -> Self {
        Self {
            task_index,
            node,
            identity,
            before_state,
            before_artifact_state,
            dependency_updates,
            recomputed,
            partition_aware,
            rewiring,
        }
    }
}

#[derive(Debug, Clone)]
pub(in crate::logic::planner) struct AppliedSerialTask {
    pub(in crate::logic::planner) node: NodeId,
    pub(in crate::logic::planner) verdict: EvaluationVerdict,
    pub(in crate::logic::planner) after_state: NodeState,
    pub(in crate::logic::planner) temporal_eligibility: Option<LoweredTemporalEligibility>,
    pub(in crate::logic::planner) memoized_origin: MemoizedResultOrigin,
    pub(in crate::logic::planner) reuse_basis: ReuseBasis,
}

impl AppliedSerialTask {
    fn from_apply_result(
        graph: &SignalGraph,
        node: NodeId,
        verdict: EvaluationVerdict,
        temporal_eligibility: Option<LoweredTemporalEligibility>,
    ) -> Result<Self, SignalError> {
        let after_state = graph.get_state(node)?;
        let after_trace = graph.node_runtime_artifact_operational_summary(node)?;
        Ok(Self {
            node,
            verdict,
            after_state,
            temporal_eligibility,
            memoized_origin: after_trace
                .as_ref()
                .map(|trace| trace.memoized_origin)
                .unwrap_or(crate::data::output::MemoizedResultOrigin::DirectCompute),
            reuse_basis: after_trace
                .map(|trace| trace.reuse_basis)
                .unwrap_or_else(crate::data::reuse::ReuseBasis::fresh_compute),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub(in crate::logic::planner) struct DeferredSnapshotBatch {
    pending_snapshots: Vec<PendingDependencySnapshot>,
}

impl DeferredSnapshotBatch {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            pending_snapshots: Vec::with_capacity(capacity),
        }
    }

    pub(in crate::logic::planner) fn push(&mut self, snapshot: PendingDependencySnapshot) {
        self.pending_snapshots.push(snapshot);
    }

    pub(in crate::logic::planner) fn len(&self) -> usize {
        self.pending_snapshots.len()
    }

    pub(in crate::logic::planner) fn classify(self) -> ClassifiedSnapshotBatchCommit {
        SnapshotBatchCommit::from_unique_pending_snapshots_in_stage_order(self.pending_snapshots)
            .classify()
    }
}

#[derive(Debug, Clone)]
struct LoweredSerialTask {
    node: NodeId,
    record_id: ExecutionRecordId,
    desired_dependencies: CanonicalDependencies,
    prepared: PreparedEvaluation,
    dependency_updates: u32,
}

#[derive(Debug, Clone)]
struct SerialStageLoweringMaterial {
    task: LoweredSerialTask,
    finalize_seed: SerialFinalizeSeed,
    produced_aspects: AspectMask,
    changed_regions: Vec<ChangedRegion>,
    touched_sources: Vec<NodeId>,
    touched_scopes: Vec<PartitionSubscription>,
    authority_policy: AuthorityPolicy,
}

#[derive(Debug, Clone)]
pub(in crate::logic::planner) struct LoweredSerialStage {
    stage_index: u32,
    stage_tasks: Vec<EligibleTask>,
    authority_policy: AuthorityPolicy,
    dirty_delta: StructuralDelta,
    maintenance_strategy: crate::data::performance::ResolvedMaintenanceStrategy,
    #[cfg(feature = "parallel")]
    serial_rejection_reason: Option<ApplyPlanSerialFallbackReason>,
    lowered_tasks: Vec<LoweredSerialTask>,
    finalize_seeds: Vec<SerialFinalizeSeed>,
    stage_order: StageTaskOrderProof,
    exact_width: ExactStageWidth,
}

impl LoweredSerialStage {
    pub(in crate::logic::planner) fn from_lowered_tasks(
        stage_index: u32,
        stage_tasks: &[EligibleTask],
        authority_policy: AuthorityPolicy,
        dirty_delta: StructuralDelta,
        maintenance_strategy: crate::data::performance::ResolvedMaintenanceStrategy,
        #[cfg(feature = "parallel")] serial_rejection_reason: Option<ApplyPlanSerialFallbackReason>,
        tasks: Vec<LoweredTask>,
        stage_identities: &[StageSemanticIdentity],
    ) -> Self {
        let mut lowered_tasks = Vec::with_capacity(tasks.len());
        let mut finalize_seeds = Vec::with_capacity(tasks.len());

        for task in tasks {
            let identity = stage_identities[task.task_index()];
            let (
                task_index,
                node,
                _produced_aspects,
                dependency_inputs,
                _path_class,
                _authority_policy,
                _footprint,
                execution,
            ) = task.into_parts();
            let (
                prepared,
                before_state,
                before_artifact_state,
                dependency_updates,
                recomputed,
                partition_aware,
                rewiring,
            ) = execution.into_parts();
            let finalize_seed = SerialFinalizeSeed::from_execution_parts(
                task_index,
                node,
                identity,
                before_state,
                before_artifact_state,
                dependency_updates,
                recomputed,
                partition_aware,
                rewiring,
            );
            lowered_tasks.push(LoweredSerialTask {
                node,
                record_id: identity.record_id,
                desired_dependencies: dependency_inputs,
                prepared,
                dependency_updates,
            });
            finalize_seeds.push(finalize_seed);
        }

        Self {
            stage_index,
            stage_tasks: stage_tasks.to_vec(),
            authority_policy,
            dirty_delta,
            maintenance_strategy,
            #[cfg(feature = "parallel")]
            serial_rejection_reason,
            exact_width: ExactStageWidth::new(lowered_tasks.len()),
            lowered_tasks,
            finalize_seeds,
            stage_order: StageTaskOrderProof::established(),
        }
    }

    pub(in crate::logic::planner) fn from_prepared_patches(
        graph: &mut SignalGraph,
        stage_index: u32,
        stage_tasks: &[EligibleTask],
        patches: Vec<PreparedTaskPatch>,
        maintenance_strategy: crate::data::performance::ResolvedMaintenanceStrategy,
        default_authority_policy: AuthorityPolicy,
        stage_identities: &[StageSemanticIdentity],
    ) -> Result<Self, SignalError> {
        let mut lowered_tasks = Vec::with_capacity(patches.len());
        let mut finalize_seeds = Vec::with_capacity(patches.len());
        let mut changed_aspects = AspectMask::EMPTY;
        let mut changed_regions = Vec::new();
        let mut touched_nodes = Vec::with_capacity(patches.len());
        let mut touched_sources = Vec::new();
        let mut touched_scopes = Vec::new();
        let mut authority_policy = default_authority_policy;

        for patch in patches {
            let material = lower_serial_task_patch(graph, patch, stage_identities)?;
            changed_aspects = changed_aspects | material.produced_aspects;
            changed_regions.extend(material.changed_regions);
            touched_nodes.push(material.task.node);
            touched_sources.extend(material.touched_sources);
            touched_scopes.extend(material.touched_scopes);
            if matches!(
                material.authority_policy,
                AuthorityPolicy::AuthoritativeOnly
            ) {
                authority_policy = AuthorityPolicy::AuthoritativeOnly;
            }
            lowered_tasks.push(material.task);
            finalize_seeds.push(material.finalize_seed);
        }

        let dirty_delta = DirtyDelta::new(
            changed_aspects,
            CanonicalChangedRegions::new(changed_regions),
            DedupedNodeBatch::new(touched_nodes.clone()),
        );
        let touched_scope = TouchedScopeSummary::new(
            PartitionScopeSet::new(touched_scopes),
            touched_nodes,
            SortedSourceBatch::new(touched_sources),
        );

        Ok(Self {
            stage_index,
            stage_tasks: stage_tasks.to_vec(),
            authority_policy,
            dirty_delta: StructuralDelta::new(Some(dirty_delta), Some(touched_scope)),
            maintenance_strategy,
            #[cfg(feature = "parallel")]
            serial_rejection_reason: None,
            exact_width: ExactStageWidth::new(lowered_tasks.len()),
            lowered_tasks,
            finalize_seeds,
            stage_order: StageTaskOrderProof::established(),
        })
    }

    pub(in crate::logic::planner) fn authority_policy(&self) -> AuthorityPolicy {
        self.authority_policy
    }

    pub(in crate::logic::planner) fn dirty_delta(&self) -> &StructuralDelta {
        &self.dirty_delta
    }

    pub(in crate::logic::planner) fn stage_width(&self) -> usize {
        self.exact_width.get()
    }

    pub(in crate::logic::planner) fn maintenance_strategy(
        &self,
    ) -> crate::data::performance::ResolvedMaintenanceStrategy {
        self.maintenance_strategy
    }

    #[cfg(feature = "parallel")]
    pub(in crate::logic::planner) fn serial_rejection_reason(
        &self,
    ) -> Option<ApplyPlanSerialFallbackReason> {
        self.serial_rejection_reason
    }
}

fn lower_serial_task_patch(
    graph: &mut SignalGraph,
    patch: PreparedTaskPatch,
    stage_identities: &[StageSemanticIdentity],
) -> Result<SerialStageLoweringMaterial, SignalError> {
    let task_index = patch.task_index;
    let node = patch.node;
    let prepared = patch.prepared;
    graph.refresh_runtime_dependencies_of(node)?;
    let current_dependencies =
        CanonicalDependencies::from_slice(graph.current_runtime_dependencies_of(node)?);
    let admitted = admit_or_error(
        HostComputedApiFamily::CorePreparedEvaluation,
        node,
        current_dependencies.as_slice(),
        prepared,
        graph.telemetry_mut(),
    )?;
    let (prepared, _admitted_reads, dependency_patch) = admitted.into_parts();
    let next_dependencies = CanonicalDependencies::from_slice(dependency_patch.next_dependencies());
    let before_state = graph.get_state(node)?;
    let before_artifact_state = graph.node_runtime_artifact_finalize_image(node)?;
    let contract = graph.get_contract(node)?;
    let recomputed = matches!(prepared.outcome, PreparedEvaluationOutcome::Evaluate)
        && !matches!(prepared.origin, PreparedEvaluationOrigin::MemoizedReuse);
    let partition_aware = !prepared.result.changed_regions.is_empty();
    let rewiring = rewiring_summary_from_lowered_edges(
        current_dependencies.as_slice(),
        next_dependencies.as_slice(),
    );
    let dependency_updates = count_dependency_updates(
        current_dependencies.as_slice(),
        next_dependencies.as_slice(),
    );

    let touched_sources = current_dependencies
        .as_slice()
        .iter()
        .chain(next_dependencies.as_slice().iter())
        .map(|edge| edge.source())
        .collect::<Vec<_>>();
    let touched_scopes = next_dependencies
        .as_slice()
        .iter()
        .filter_map(|edge| edge.scope_ref().cloned())
        .collect::<Vec<_>>();
    let changed_regions = prepared.result.changed_regions.clone();
    let identity = stage_identities[task_index];

    let finalize_seed = SerialFinalizeSeed::from_execution_parts(
        task_index,
        node,
        identity,
        before_state,
        before_artifact_state,
        dependency_updates,
        recomputed,
        partition_aware,
        rewiring,
    );

    Ok(SerialStageLoweringMaterial {
        task: LoweredSerialTask {
            node,
            record_id: identity.record_id,
            desired_dependencies: next_dependencies,
            prepared,
            dependency_updates,
        },
        finalize_seed,
        produced_aspects: contract.semantics.produces,
        changed_regions,
        touched_sources,
        touched_scopes,
        authority_policy: contract.authority.policy,
    })
}

#[derive(Debug, Clone)]
pub(in crate::logic::planner) struct PreparedSerialStageBatch {
    stage_index: u32,
    exact_width: ExactStageWidth,
    stage_tasks: Vec<EligibleTask>,
    finalize_seeds: Vec<SerialFinalizeSeed>,
    apply_inputs: Vec<SerialApplyInput>,
    pending_snapshots: DeferredSnapshotBatch,
    stage_order: StageTaskOrderProof,
}

impl PreparedSerialStageBatch {
    pub(in crate::logic::planner) fn prepare(
        graph: &mut SignalGraph,
        lowered: LoweredSerialStage,
        stage_record: &mut StageExecutionRecord,
    ) -> Result<Self, SignalError> {
        #[cfg(not(feature = "parallel"))]
        let _ = stage_record;
        #[cfg(feature = "parallel")]
        {
            stage_record.apply_mode = Some(crate::logic::planner::ParallelApplyMode::SerialApply);
            stage_record.apply_group_count = 1;
            stage_record.serial_apply_rejection_reason = lowered.serial_rejection_reason();
            stage_record.serial_fallback_group_count =
                u32::from(lowered.serial_rejection_reason().is_some());
            stage_record.serial_apply_task_count = lowered.stage_width() as u32;
        }

        let mut reconcile_batch = Vec::with_capacity(lowered.exact_width.get());
        for task in &lowered.lowered_tasks {
            reconcile_batch.push((task.node, task.desired_dependencies.as_slice()));
        }
        let reconcile_start = crate::clock::RuntimeInstant::now();
        graph.reconcile_dependencies_batch_borrowed(&reconcile_batch)?;
        graph.telemetry_mut().execution.dependency_reconcile_nanos +=
            reconcile_start.elapsed().as_nanos();

        let dependency_input_start = crate::clock::RuntimeInstant::now();
        let dependency_inputs = crate::logic::evaluation::collect_effect_dependency_inputs_iter(
            graph,
            lowered.lowered_tasks.iter().map(|task| task.node),
        )?;
        graph.telemetry_mut().execution.dependency_input_build_nanos +=
            dependency_input_start.elapsed().as_nanos();

        let apply_inputs = lowered
            .lowered_tasks
            .into_iter()
            .zip(dependency_inputs)
            .map(|(task, dependency_inputs)| {
                SerialApplyInput::new(
                    task.node,
                    task.record_id,
                    task.prepared,
                    task.dependency_updates,
                    dependency_inputs,
                )
            })
            .collect();

        Ok(Self {
            stage_index: lowered.stage_index,
            exact_width: lowered.exact_width,
            stage_tasks: lowered.stage_tasks,
            finalize_seeds: lowered.finalize_seeds,
            apply_inputs,
            pending_snapshots: DeferredSnapshotBatch::with_capacity(lowered.exact_width.get()),
            stage_order: lowered.stage_order,
        })
    }

    pub(in crate::logic::planner) fn apply(
        mut self,
        graph: &mut SignalGraph,
        summary: &PlanSummary,
        comparator_resolver: &mut impl crate::data::comparator::ComparatorPolicyResolver,
        executor: StageExecutor,
    ) -> Result<AppliedSerialStageBatch, SignalError> {
        let mut applied_tasks = Vec::with_capacity(self.exact_width.get());

        for input in self.apply_inputs {
            let apply_result = apply_prepared_evaluation_after_dependencies_with_policy(
                graph,
                input.node,
                input.prepared,
                comparator_resolver,
                None,
                input.dependency_updates,
                Some(input.dependency_inputs),
                true,
            )
            .inspect_err(|err| {
                record_execution_failure(
                    graph,
                    ExecutionFailureContext::new(
                        ExecutionFailurePhase::Apply,
                        Some(self.stage_index),
                        Some(input.node),
                        Some(executor),
                        Some(input.record_id),
                        Some(*summary),
                        err.to_string(),
                    ),
                );
            })?;

            if let Some(snapshot) = apply_result.pending_snapshot {
                self.pending_snapshots.push(snapshot);
            }
            applied_tasks.push(AppliedSerialTask::from_apply_result(
                graph,
                input.node,
                apply_result.report.verdict,
                apply_result.temporal_eligibility,
            )?);
        }

        let applied_tasks = StageOrderedAppliedTasks::new(self.exact_width, applied_tasks)?;
        let task_count = applied_tasks.len();
        graph.telemetry_mut().execution.group_local_packet_breadth += task_count as u64;
        graph.telemetry_mut().execution.reduction_packet_breadth += 1;
        graph.telemetry_mut().execution.reduction_group_count += 1;
        graph
            .telemetry_mut()
            .execution
            .shared_surface_publication_breadth +=
            (task_count + self.pending_snapshots.len()) as u64;

        Ok(AppliedSerialStageBatch {
            stage_index: self.stage_index,
            exact_width: self.exact_width,
            stage_tasks: self.stage_tasks,
            finalize_seeds: self.finalize_seeds,
            applied_tasks,
            pending_snapshots: self.pending_snapshots,
            stage_order: self.stage_order,
        })
    }
}

#[derive(Debug, Clone)]
struct StageOrderedAppliedTasks {
    exact_width: ExactStageWidth,
    tasks: Vec<AppliedSerialTask>,
}

impl StageOrderedAppliedTasks {
    fn new(
        exact_width: ExactStageWidth,
        tasks: Vec<AppliedSerialTask>,
    ) -> Result<Self, SignalError> {
        if tasks.len() != exact_width.get() {
            return Err(SignalError::internal(
                "serial batch apply must produce exactly one ordered applied task per prepared input",
            ));
        }
        Ok(Self { exact_width, tasks })
    }

    fn len(&self) -> usize {
        self.tasks.len()
    }

    fn exact_width(&self) -> ExactStageWidth {
        self.exact_width
    }
}

#[derive(Debug, Clone)]
pub(in crate::logic::planner) struct AppliedSerialStageBatch {
    stage_index: u32,
    exact_width: ExactStageWidth,
    stage_tasks: Vec<EligibleTask>,
    finalize_seeds: Vec<SerialFinalizeSeed>,
    applied_tasks: StageOrderedAppliedTasks,
    pending_snapshots: DeferredSnapshotBatch,
    stage_order: StageTaskOrderProof,
}

impl AppliedSerialStageBatch {
    pub(in crate::logic::planner) fn split_pending_snapshots(
        self,
    ) -> (Self, ClassifiedSnapshotBatchCommit) {
        let AppliedSerialStageBatch {
            stage_index,
            exact_width,
            stage_tasks,
            finalize_seeds,
            applied_tasks,
            pending_snapshots,
            stage_order,
        } = self;
        (
            Self {
                stage_index,
                exact_width,
                stage_tasks,
                finalize_seeds,
                applied_tasks,
                pending_snapshots: DeferredSnapshotBatch::default(),
                stage_order,
            },
            pending_snapshots.classify(),
        )
    }

    pub(in crate::logic::planner) fn into_ready_for_finalize(
        self,
    ) -> Result<ReadySerialFinalizeBatch, SignalError> {
        let AppliedSerialStageBatch {
            stage_index: _stage_index,
            exact_width,
            stage_tasks,
            finalize_seeds,
            applied_tasks,
            pending_snapshots,
            stage_order,
        } = self;
        if !pending_snapshots.pending_snapshots.is_empty() {
            return Err(SignalError::internal(
                "serial finalize input must not retain uncommitted stage-owned snapshots",
            ));
        }
        if finalize_seeds.len() != exact_width.get() {
            return Err(SignalError::internal(
                "serial finalize seeds must match the prepared stage width",
            ));
        }
        if stage_tasks.len() != exact_width.get() {
            return Err(SignalError::internal(
                "serial stage task witness must match the prepared stage width",
            ));
        }
        if applied_tasks.exact_width().get() != exact_width.get() {
            return Err(SignalError::internal(
                "serial applied task batch width must remain aligned with the prepared stage width",
            ));
        }

        Ok(ReadySerialFinalizeBatch::new(
            stage_tasks,
            finalize_seeds,
            applied_tasks,
            stage_order,
        ))
    }
}

#[derive(Debug, Clone)]
pub(in crate::logic::planner) struct ReadySerialFinalizeBatch {
    stage_tasks: Vec<EligibleTask>,
    finalize_seeds: Vec<SerialFinalizeSeed>,
    applied_tasks: StageOrderedAppliedTasks,
    stage_order: StageTaskOrderProof,
}

impl ReadySerialFinalizeBatch {
    fn new(
        stage_tasks: Vec<EligibleTask>,
        finalize_seeds: Vec<SerialFinalizeSeed>,
        applied_tasks: StageOrderedAppliedTasks,
        stage_order: StageTaskOrderProof,
    ) -> Self {
        Self {
            stage_tasks,
            finalize_seeds,
            applied_tasks,
            stage_order,
        }
    }

    pub(in crate::logic::planner) fn into_parts(
        self,
    ) -> (
        Vec<EligibleTask>,
        Vec<SerialFinalizeSeed>,
        Vec<AppliedSerialTask>,
        StageTaskOrderProof,
    ) {
        (
            self.stage_tasks,
            self.finalize_seeds,
            self.applied_tasks.tasks,
            self.stage_order,
        )
    }
}

#[derive(Debug, Clone)]
pub(in crate::logic::planner) struct FinalizedSerialStageBatch {
    semantic_task_range: crate::logic::planner::types::SemanticTaskRange,
    task_records: Vec<crate::logic::planner::types::TaskExecutionRecord>,
    semantic_segment_count: u32,
}

impl FinalizedSerialStageBatch {
    pub(in crate::logic::planner) fn new(
        semantic_task_range: crate::logic::planner::types::SemanticTaskRange,
        task_records: Vec<crate::logic::planner::types::TaskExecutionRecord>,
        semantic_segment_count: u32,
    ) -> Self {
        Self {
            semantic_task_range,
            task_records,
            semantic_segment_count,
        }
    }

    pub(in crate::logic::planner) fn record_into(
        self,
        report: &mut crate::logic::planner::types::ExecutionReport,
        stage_record: &mut StageExecutionRecord,
    ) {
        stage_record.semantic_task_range = Some(self.semantic_task_range);
        stage_record.semantic_segment_count = self.semantic_segment_count;
        report.semantic_segment_count += self.semantic_segment_count;
        stage_record.task_records = self.task_records;
    }
}
