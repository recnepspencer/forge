use std::collections::{BTreeSet, VecDeque};
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe, RefUnwindSafe, UnwindSafe};
use std::sync::{Arc, Mutex};

use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
use crate::logic::runtime::RelationalRuntime;
use crate::payloads::data::RecordPayload;
use crate::symbols::data::InternedString;
use crate::transactions::data::{
    CreateIntent, EntityMutationIntent, MergedCommitPlan, MutationIntent, RelationMutationIntent,
};
use crate::validation::engine::state_view::{InvariantStateView, VisibleRelationMetadata};
use crate::validation::engine::{InvariantObservation, InvariantObservationKind};

use super::descriptor::CustomInvariantDescriptor;
use super::rule_id::{CustomInvariantRuleId, CustomInvariantSemanticIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomInvariantPreparationError {
    detail: Arc<str>,
}

impl CustomInvariantPreparationError {
    pub fn new(detail: impl Into<Arc<str>>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomInvariantExecutionError {
    detail: Arc<str>,
}

impl CustomInvariantExecutionError {
    pub fn new(detail: impl Into<Arc<str>>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomInvariantTraversalError {
    detail: Arc<str>,
}

impl CustomInvariantTraversalError {
    pub fn new(detail: impl Into<Arc<str>>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl From<CustomInvariantTraversalError> for CustomInvariantPreparationError {
    fn from(value: CustomInvariantTraversalError) -> Self {
        Self::new(value.detail)
    }
}

impl From<CustomInvariantTraversalError> for CustomInvariantExecutionError {
    fn from(value: CustomInvariantTraversalError) -> Self {
        Self::new(value.detail)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomInvariantVerdict {
    Pass,
    Violation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CustomInvariantRuntimePhase {
    Preparation,
    Execution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CustomInvariantFailureKind {
    PreparationError,
    ExecutionError,
    Panic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CustomInvariantFailure {
    pub(crate) identity: CustomInvariantSemanticIdentity,
    pub(crate) phase: CustomInvariantRuntimePhase,
    pub(crate) kind: CustomInvariantFailureKind,
    pub(crate) detail: Arc<str>,
}

impl CustomInvariantFailure {
    fn preparation_error(
        identity: &CustomInvariantSemanticIdentity,
        error: CustomInvariantPreparationError,
    ) -> Self {
        Self {
            identity: identity.clone(),
            phase: CustomInvariantRuntimePhase::Preparation,
            kind: CustomInvariantFailureKind::PreparationError,
            detail: Arc::from(error.detail()),
        }
    }

    fn execution_error(
        identity: &CustomInvariantSemanticIdentity,
        error: CustomInvariantExecutionError,
    ) -> Self {
        Self {
            identity: identity.clone(),
            phase: CustomInvariantRuntimePhase::Execution,
            kind: CustomInvariantFailureKind::ExecutionError,
            detail: Arc::from(error.detail()),
        }
    }

    fn panic(
        identity: &CustomInvariantSemanticIdentity,
        phase: CustomInvariantRuntimePhase,
        detail: Arc<str>,
    ) -> Self {
        Self {
            identity: identity.clone(),
            phase,
            kind: CustomInvariantFailureKind::Panic,
            detail,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PreparedCustomInvariantExecutionOutcome {
    Verdict(CustomInvariantVerdict),
    Failure(CustomInvariantFailure),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedEntityCreate {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub client_key: InternedString,
    pub payload: RecordPayload,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedRelationCreate {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub client_key: InternedString,
    pub source: EntityId,
    pub target: EntityId,
    pub payload: Option<RecordPayload>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TouchedStructuralSet {
    visible_entity_ids: Arc<[EntityId]>,
    visible_relation_ids: Arc<[RelationId]>,
    touched_partitions: Arc<[PartitionId]>,
    planned_entity_creates: Arc<[PlannedEntityCreate]>,
    planned_relation_creates: Arc<[PlannedRelationCreate]>,
}

impl TouchedStructuralSet {
    pub fn visible_entity_ids(&self) -> &[EntityId] {
        &self.visible_entity_ids
    }

    pub fn visible_relation_ids(&self) -> &[RelationId] {
        &self.visible_relation_ids
    }

    pub fn touched_partitions(&self) -> &[PartitionId] {
        &self.touched_partitions
    }

    pub fn planned_entity_creates(&self) -> &[PlannedEntityCreate] {
        &self.planned_entity_creates
    }

    pub fn planned_relation_creates(&self) -> &[PlannedRelationCreate] {
        &self.planned_relation_creates
    }

    pub(crate) fn provenance_summary(&self) -> CustomInvariantTouchedSummary {
        CustomInvariantTouchedSummary {
            visible_entity_ids: self.visible_entity_ids.clone(),
            visible_relation_ids: self.visible_relation_ids.clone(),
            touched_partition_ids: self.touched_partitions.clone(),
            planned_entity_create_count: self.planned_entity_creates.len(),
            planned_relation_create_count: self.planned_relation_creates.len(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CustomInvariantTouchedSummary {
    pub visible_entity_ids: Arc<[EntityId]>,
    pub visible_relation_ids: Arc<[RelationId]>,
    pub touched_partition_ids: Arc<[PartitionId]>,
    pub planned_entity_create_count: usize,
    pub planned_relation_create_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructuralRelationRecord {
    pub relation_id: RelationId,
    pub kind_id: KindId,
    pub source: EntityId,
    pub target: EntityId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StructuralCountView {
    visible_entity_count: usize,
    visible_relation_count: usize,
    planned_entity_create_count: usize,
    planned_relation_create_count: usize,
    touched_partition_count: usize,
}

impl StructuralCountView {
    pub fn visible_entity_count(&self) -> usize {
        self.visible_entity_count
    }

    pub fn visible_relation_count(&self) -> usize {
        self.visible_relation_count
    }

    pub fn planned_entity_create_count(&self) -> usize {
        self.planned_entity_create_count
    }

    pub fn planned_relation_create_count(&self) -> usize {
        self.planned_relation_create_count
    }

    pub fn touched_partition_count(&self) -> usize {
        self.touched_partition_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TraversalBudgetSession {
    remaining_frontier: usize,
    remaining_steps: usize,
    max_depth: usize,
    consumed_frontier: usize,
    consumed_steps: usize,
}

impl TraversalBudgetSession {
    fn from_touched_scope(touched: &TouchedStructuralSet) -> Self {
        let base_entities = touched.visible_entity_ids.len() + touched.planned_entity_creates.len();
        let base_relations =
            touched.visible_relation_ids.len() + touched.planned_relation_creates.len();
        let base = (base_entities + base_relations).max(32);
        Self {
            remaining_frontier: base.saturating_mul(8),
            remaining_steps: base.saturating_mul(32),
            max_depth: 32,
            consumed_frontier: 0,
            consumed_steps: 0,
        }
    }

    fn charge_frontier(&mut self, units: usize) -> Result<(), CustomInvariantTraversalError> {
        if units > self.remaining_frontier {
            return Err(CustomInvariantTraversalError::new(
                "custom invariant traversal exceeded its session frontier budget",
            ));
        }
        self.remaining_frontier -= units;
        self.consumed_frontier += units;
        Ok(())
    }

    fn charge_step(&mut self, units: usize) -> Result<(), CustomInvariantTraversalError> {
        if units > self.remaining_steps {
            return Err(CustomInvariantTraversalError::new(
                "custom invariant traversal exceeded its session step budget",
            ));
        }
        self.remaining_steps -= units;
        self.consumed_steps += units;
        Ok(())
    }

    fn checked_depth(
        &self,
        requested_depth: usize,
    ) -> Result<usize, CustomInvariantTraversalError> {
        if requested_depth > self.max_depth {
            return Err(CustomInvariantTraversalError::new(format!(
                "custom invariant traversal requested depth {} beyond session maximum {}",
                requested_depth, self.max_depth
            )));
        }
        Ok(requested_depth)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CustomInvariantTraversalSummary {
    pub consumed_frontier: usize,
    pub consumed_steps: usize,
    pub remaining_frontier: usize,
    pub remaining_steps: usize,
    pub max_depth: usize,
}

impl TraversalBudgetSession {
    fn summary(&self) -> CustomInvariantTraversalSummary {
        CustomInvariantTraversalSummary {
            consumed_frontier: self.consumed_frontier,
            consumed_steps: self.consumed_steps,
            remaining_frontier: self.remaining_frontier,
            remaining_steps: self.remaining_steps,
            max_depth: self.max_depth,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralTraversalResult {
    visited_entities: Arc<[EntityId]>,
    traversed_relations: Arc<[RelationId]>,
    frontier_exhausted: bool,
}

impl StructuralTraversalResult {
    pub fn visited_entities(&self) -> &[EntityId] {
        &self.visited_entities
    }

    pub fn traversed_relations(&self) -> &[RelationId] {
        &self.traversed_relations
    }

    pub fn frontier_exhausted(&self) -> bool {
        self.frontier_exhausted
    }
}

#[derive(Clone, Copy)]
pub struct StructuralPayloadView<'runtime> {
    state_view: InvariantStateView<'runtime>,
}

impl<'runtime> StructuralPayloadView<'runtime> {
    pub fn entity_payload(&self, entity_id: EntityId) -> Option<&'runtime RecordPayload> {
        self.state_view.entity_payload(entity_id)
    }

    pub fn relation_payload(&self, relation_id: RelationId) -> Option<&'runtime RecordPayload> {
        self.state_view.relation_payload(relation_id)
    }
}

#[derive(Clone, Copy)]
pub struct StructuralRelationView<'runtime> {
    runtime: &'runtime RelationalRuntime,
    state_view: InvariantStateView<'runtime>,
}

impl<'runtime> StructuralRelationView<'runtime> {
    pub fn entity_kind(&self, entity_id: EntityId) -> Option<KindId> {
        self.state_view
            .entity_metadata(entity_id)
            .map(|metadata| metadata.kind_id)
    }

    pub fn relation(&self, relation_id: RelationId) -> Option<StructuralRelationRecord> {
        self.state_view
            .relation_metadata(relation_id)
            .map(|metadata| StructuralRelationRecord {
                relation_id: metadata.relation_id,
                kind_id: metadata.kind_id,
                source: metadata.source,
                target: metadata.target,
            })
    }

    pub fn outgoing_relations_for_entity(&self, entity_id: EntityId) -> Vec<RelationId> {
        self.runtime
            .storage_access()
            .outgoing_relations_for_entity(entity_id, self.state_view.version_id())
    }

    pub fn incoming_relations_for_entity(&self, entity_id: EntityId) -> Vec<RelationId> {
        self.runtime
            .storage_access()
            .incoming_relations_for_entity(entity_id, self.state_view.version_id())
    }

    pub fn all_relations_for_entity(&self, entity_id: EntityId) -> Vec<RelationId> {
        self.runtime
            .storage_access()
            .all_relations_for_entity(entity_id, self.state_view.version_id())
    }
}

pub struct BoundedStructuralTraversal<'runtime> {
    relations: StructuralRelationView<'runtime>,
    performance: crate::performance::logic::PerformanceAccess<'runtime>,
    budget: Mutex<TraversalBudgetSession>,
}

impl<'runtime> BoundedStructuralTraversal<'runtime> {
    fn new(
        runtime: &'runtime RelationalRuntime,
        relations: StructuralRelationView<'runtime>,
        touched: &TouchedStructuralSet,
    ) -> Self {
        Self {
            relations,
            performance: runtime.performance_access(),
            budget: Mutex::new(TraversalBudgetSession::from_touched_scope(touched)),
        }
    }

    pub fn walk_outgoing_from(
        &self,
        seeds: &[EntityId],
        max_depth: usize,
    ) -> Result<StructuralTraversalResult, CustomInvariantTraversalError> {
        self.walk(seeds, max_depth, TraversalDirection::Outgoing)
    }

    pub fn walk_incoming_from(
        &self,
        seeds: &[EntityId],
        max_depth: usize,
    ) -> Result<StructuralTraversalResult, CustomInvariantTraversalError> {
        self.walk(seeds, max_depth, TraversalDirection::Incoming)
    }

    fn summary(&self) -> CustomInvariantTraversalSummary {
        self.budget
            .lock()
            .expect("custom invariant traversal budget mutex must not be poisoned")
            .summary()
    }

    fn walk(
        &self,
        seeds: &[EntityId],
        max_depth: usize,
        direction: TraversalDirection,
    ) -> Result<StructuralTraversalResult, CustomInvariantTraversalError> {
        let mut budget = self
            .budget
            .lock()
            .expect("custom invariant traversal budget mutex must not be poisoned");
        let allowed_depth = budget.checked_depth(max_depth)?;
        budget.charge_frontier(seeds.len())?;
        self.performance
            .count_custom_invariant_traversal(seeds.len(), 0);

        let mut visited_entities = BTreeSet::new();
        let mut traversed_relations = BTreeSet::new();
        let mut queue = VecDeque::new();
        for seed in seeds {
            if visited_entities.insert(*seed) {
                queue.push_back((*seed, 0usize));
            }
        }

        while let Some((entity_id, depth)) = queue.pop_front() {
            if depth >= allowed_depth {
                continue;
            }
            let relation_ids = match direction {
                TraversalDirection::Outgoing => {
                    self.relations.outgoing_relations_for_entity(entity_id)
                }
                TraversalDirection::Incoming => {
                    self.relations.incoming_relations_for_entity(entity_id)
                }
            };
            budget.charge_step(relation_ids.len())?;
            self.performance
                .count_custom_invariant_traversal(0, relation_ids.len());
            for relation_id in relation_ids {
                traversed_relations.insert(relation_id);
                let Some(relation) = self.relations.relation(relation_id) else {
                    continue;
                };
                let next_entity = match direction {
                    TraversalDirection::Outgoing => relation.target,
                    TraversalDirection::Incoming => relation.source,
                };
                if visited_entities.insert(next_entity) {
                    budget.charge_frontier(1)?;
                    self.performance.count_custom_invariant_traversal(1, 0);
                    queue.push_back((next_entity, depth + 1));
                }
            }
        }

        Ok(StructuralTraversalResult {
            visited_entities: visited_entities.into_iter().collect::<Vec<_>>().into(),
            traversed_relations: traversed_relations.into_iter().collect::<Vec<_>>().into(),
            frontier_exhausted: queue.is_empty(),
        })
    }
}

#[derive(Clone, Copy)]
enum TraversalDirection {
    Outgoing,
    Incoming,
}

pub struct CustomInvariantScopePlanner<'runtime> {
    observation_kind: InvariantObservationKind,
    version_id: VersionId,
    current_version_id: VersionId,
    touched: TouchedStructuralSet,
    payloads: StructuralPayloadView<'runtime>,
    relations: StructuralRelationView<'runtime>,
    counts: StructuralCountView,
    traversal: BoundedStructuralTraversal<'runtime>,
}

impl<'runtime> CustomInvariantScopePlanner<'runtime> {
    pub(crate) fn new(
        runtime: &'runtime RelationalRuntime,
        observation: &'runtime InvariantObservation<'runtime>,
        version_id: VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
    ) -> Self {
        let state_view = InvariantStateView::new(observation.partition_access(), version_id);
        let touched = collect_touched_structural_set(runtime, &state_view, merged_plan);
        let counts = StructuralCountView {
            visible_entity_count: touched.visible_entity_ids.len(),
            visible_relation_count: touched.visible_relation_ids.len(),
            planned_entity_create_count: touched.planned_entity_creates.len(),
            planned_relation_create_count: touched.planned_relation_creates.len(),
            touched_partition_count: touched.touched_partitions.len(),
        };
        let payloads = StructuralPayloadView { state_view };
        let relations = StructuralRelationView {
            runtime,
            state_view,
        };
        let traversal = BoundedStructuralTraversal::new(runtime, relations, &touched);
        Self {
            observation_kind: observation.kind(),
            version_id,
            current_version_id: runtime.current_version_id(),
            touched,
            payloads,
            relations,
            counts,
            traversal,
        }
    }

    pub fn observation_kind(&self) -> InvariantObservationKind {
        self.observation_kind
    }

    pub fn version_id(&self) -> VersionId {
        self.version_id
    }

    pub fn current_version_id(&self) -> VersionId {
        self.current_version_id
    }

    pub fn touched(&self) -> &TouchedStructuralSet {
        &self.touched
    }

    pub fn payloads(&self) -> StructuralPayloadView<'runtime> {
        self.payloads
    }

    pub fn relations(&self) -> StructuralRelationView<'runtime> {
        self.relations
    }

    pub fn counts(&self) -> StructuralCountView {
        self.counts
    }

    pub fn traversal(&self) -> &BoundedStructuralTraversal<'runtime> {
        &self.traversal
    }
}

pub struct CustomInvariantExecutionContext<'runtime> {
    runtime: &'runtime RelationalRuntime,
    observation_kind: InvariantObservationKind,
    version_id: VersionId,
    current_version_id: VersionId,
    touched: TouchedStructuralSet,
    payloads: StructuralPayloadView<'runtime>,
    relations: StructuralRelationView<'runtime>,
    counts: StructuralCountView,
    traversal: BoundedStructuralTraversal<'runtime>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CustomInvariantProvenance {
    pub observation_kind: InvariantObservationKind,
    pub version_id: VersionId,
    pub current_version_id: VersionId,
    pub touched: CustomInvariantTouchedSummary,
    pub counts: StructuralCountView,
    pub traversal: CustomInvariantTraversalSummary,
}

impl<'runtime> CustomInvariantExecutionContext<'runtime> {
    pub(crate) fn new(
        runtime: &'runtime RelationalRuntime,
        observation: &'runtime InvariantObservation<'runtime>,
        version_id: VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
    ) -> Self {
        let state_view = InvariantStateView::new(observation.partition_access(), version_id);
        let touched = collect_touched_structural_set(runtime, &state_view, merged_plan);
        let counts = StructuralCountView {
            visible_entity_count: touched.visible_entity_ids.len(),
            visible_relation_count: touched.visible_relation_ids.len(),
            planned_entity_create_count: touched.planned_entity_creates.len(),
            planned_relation_create_count: touched.planned_relation_creates.len(),
            touched_partition_count: touched.touched_partitions.len(),
        };
        let payloads = StructuralPayloadView { state_view };
        let relations = StructuralRelationView {
            runtime,
            state_view,
        };
        let traversal = BoundedStructuralTraversal::new(runtime, relations, &touched);
        Self {
            runtime,
            observation_kind: observation.kind(),
            version_id,
            current_version_id: runtime.current_version_id(),
            touched,
            payloads,
            relations,
            counts,
            traversal,
        }
    }

    pub fn observation_kind(&self) -> InvariantObservationKind {
        self.observation_kind
    }

    pub(crate) fn runtime(&self) -> &'runtime RelationalRuntime {
        self.runtime
    }

    pub fn version_id(&self) -> VersionId {
        self.version_id
    }

    pub fn current_version_id(&self) -> VersionId {
        self.current_version_id
    }

    pub fn touched(&self) -> &TouchedStructuralSet {
        &self.touched
    }

    pub fn payloads(&self) -> StructuralPayloadView<'runtime> {
        self.payloads
    }

    pub fn relations(&self) -> StructuralRelationView<'runtime> {
        self.relations
    }

    pub fn counts(&self) -> StructuralCountView {
        self.counts
    }

    pub fn traversal(&self) -> &BoundedStructuralTraversal<'runtime> {
        &self.traversal
    }

    pub fn provenance(&self) -> CustomInvariantProvenance {
        CustomInvariantProvenance {
            observation_kind: self.observation_kind,
            version_id: self.version_id,
            current_version_id: self.current_version_id,
            touched: self.touched.provenance_summary(),
            counts: self.counts,
            traversal: self.traversal.summary(),
        }
    }
}

pub trait CustomInvariantRule: Send + Sync + RefUnwindSafe + 'static {
    type Scope: Send + Sync + 'static;

    fn descriptor(&self) -> CustomInvariantDescriptor;

    fn prepare_scope(
        &self,
        planner: &mut CustomInvariantScopePlanner<'_>,
    ) -> Result<Self::Scope, CustomInvariantPreparationError>;

    fn evaluate(
        &self,
        context: &CustomInvariantExecutionContext<'_>,
        scope: &Self::Scope,
    ) -> Result<CustomInvariantVerdict, CustomInvariantExecutionError>;
}

pub(crate) trait PreparedCustomInvariantExecution: Send + Sync {
    fn evaluate(
        &self,
        context: &CustomInvariantExecutionContext<'_>,
    ) -> PreparedCustomInvariantExecutionOutcome;
}

pub(crate) trait ErasedCustomInvariantRule: Send + Sync {
    fn prepare_for_execution(
        &self,
        runtime: &RelationalRuntime,
        planner: &mut CustomInvariantScopePlanner<'_>,
    ) -> Arc<dyn PreparedCustomInvariantExecution>;
}

struct CustomInvariantAdapter<R: CustomInvariantRule> {
    rule: Arc<R>,
    identity: CustomInvariantSemanticIdentity,
}

struct PreparedCustomInvariantAdapter<R: CustomInvariantRule> {
    rule: Arc<R>,
    identity: CustomInvariantSemanticIdentity,
    scope: R::Scope,
}

struct FailedPreparedCustomInvariantExecution {
    failure: CustomInvariantFailure,
}

impl<R: CustomInvariantRule> PreparedCustomInvariantExecution
    for PreparedCustomInvariantAdapter<R>
{
    fn evaluate(
        &self,
        context: &CustomInvariantExecutionContext<'_>,
    ) -> PreparedCustomInvariantExecutionOutcome {
        context
            .runtime()
            .performance_access()
            .count_custom_invariant_execution();
        match run_custom_rule_safely(
            self.identity.clone(),
            CustomInvariantRuntimePhase::Execution,
            || self.rule.evaluate(context, &self.scope),
        ) {
            Ok(Ok(verdict)) => PreparedCustomInvariantExecutionOutcome::Verdict(verdict),
            Ok(Err(error)) => PreparedCustomInvariantExecutionOutcome::Failure(
                CustomInvariantFailure::execution_error(&self.identity, error),
            ),
            Err(failure) => PreparedCustomInvariantExecutionOutcome::Failure(failure),
        }
    }
}

impl PreparedCustomInvariantExecution for FailedPreparedCustomInvariantExecution {
    fn evaluate(
        &self,
        _context: &CustomInvariantExecutionContext<'_>,
    ) -> PreparedCustomInvariantExecutionOutcome {
        PreparedCustomInvariantExecutionOutcome::Failure(self.failure.clone())
    }
}

impl<R: CustomInvariantRule> ErasedCustomInvariantRule for CustomInvariantAdapter<R> {
    fn prepare_for_execution(
        &self,
        runtime: &RelationalRuntime,
        planner: &mut CustomInvariantScopePlanner<'_>,
    ) -> Arc<dyn PreparedCustomInvariantExecution> {
        let identity = self.identity.clone();
        runtime
            .performance_access()
            .count_custom_invariant_preparation();
        match run_custom_rule_safely(
            identity.clone(),
            CustomInvariantRuntimePhase::Preparation,
            || self.rule.prepare_scope(planner),
        ) {
            Ok(Ok(scope)) => Arc::new(PreparedCustomInvariantAdapter {
                rule: Arc::clone(&self.rule),
                identity,
                scope,
            }),
            Ok(Err(error)) => Arc::new(FailedPreparedCustomInvariantExecution {
                failure: CustomInvariantFailure::preparation_error(&identity, error),
            }),
            Err(failure) => {
                if failure.kind == CustomInvariantFailureKind::Panic {
                    runtime.performance_access().count_custom_invariant_panic();
                }
                Arc::new(FailedPreparedCustomInvariantExecution { failure })
            }
        }
    }
}

fn run_custom_rule_safely<T>(
    identity: CustomInvariantSemanticIdentity,
    phase: CustomInvariantRuntimePhase,
    run: impl FnOnce() -> T,
) -> Result<T, CustomInvariantFailure> {
    catch_unwind(AssertUnwindSafe(run)).map_err(|panic_payload| {
        CustomInvariantFailure::panic(&identity, phase, panic_payload_message(panic_payload))
    })
}

fn panic_payload_message(panic_payload: Box<dyn std::any::Any + Send>) -> Arc<str> {
    if let Some(message) = panic_payload.downcast_ref::<&'static str>() {
        return Arc::from(*message);
    }
    if let Some(message) = panic_payload.downcast_ref::<String>() {
        return Arc::from(message.as_str());
    }
    Arc::from("custom invariant panicked with a non-string payload")
}

#[derive(Clone)]
pub struct CustomInvariantRegistration {
    descriptor: CustomInvariantDescriptor,
    executable: Arc<dyn ErasedCustomInvariantRule>,
}

impl fmt::Debug for CustomInvariantRegistration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomInvariantRegistration")
            .field("descriptor", &self.descriptor)
            .finish_non_exhaustive()
    }
}

impl CustomInvariantRegistration {
    pub fn new<R>(rule: R) -> Result<Self, CustomInvariantRegistrationError>
    where
        R: CustomInvariantRule + UnwindSafe,
    {
        let descriptor = rule.descriptor();
        Self::validate_descriptor(&descriptor)?;
        let executable = Arc::new(CustomInvariantAdapter {
            rule: Arc::new(rule),
            identity: descriptor.identity.clone(),
        });
        Ok(Self {
            descriptor,
            executable,
        })
    }

    pub fn descriptor(&self) -> &CustomInvariantDescriptor {
        &self.descriptor
    }

    pub fn execution_point(&self) -> crate::validation::data::InvariantExecutionPoint {
        self.descriptor.operational.execution_point
    }

    pub fn groups(&self) -> crate::validation::data::InvariantGroupSet {
        self.descriptor.operational.groups
    }

    pub fn cost_class(&self) -> crate::validation::data::InvariantCostClass {
        self.descriptor.operational.cost_class
    }

    pub fn failure_effect(&self) -> crate::validation::data::InvariantFailureEffect {
        self.descriptor.operational.failure_effect
    }

    pub fn rule_id(&self) -> &CustomInvariantRuleId {
        &self.descriptor.identity.rule_id
    }

    pub(crate) fn executable(&self) -> &Arc<dyn ErasedCustomInvariantRule> {
        &self.executable
    }

    fn validate_descriptor(
        descriptor: &CustomInvariantDescriptor,
    ) -> Result<(), CustomInvariantRegistrationError> {
        if descriptor.identity.rule_id.as_str().trim().is_empty() {
            return Err(CustomInvariantRegistrationError::EmptyRuleId);
        }
        if descriptor.display_name.trim().is_empty() {
            return Err(CustomInvariantRegistrationError::EmptyDisplayName);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomInvariantRegistrationError {
    EmptyRuleId,
    EmptyDisplayName,
}

fn collect_touched_structural_set(
    runtime: &RelationalRuntime,
    state_view: &InvariantStateView<'_>,
    merged_plan: Option<&MergedCommitPlan>,
) -> TouchedStructuralSet {
    let mut visible_entities = BTreeSet::new();
    let mut visible_relations = BTreeSet::new();
    let mut touched_partitions = BTreeSet::new();
    let mut planned_entity_creates = Vec::new();
    let mut planned_relation_creates = Vec::new();

    if let Some(entity_ids) = state_view.touched_visible_entity_ids() {
        visible_entities.extend(entity_ids);
    }
    if let Some(relation_ids) = state_view.touched_visible_relation_ids() {
        visible_relations.extend(relation_ids);
    }

    if let Some(plan) = merged_plan {
        for intent in &plan.merged_intents {
            intent.seed_touched_partitions(&mut touched_partitions);
            match intent {
                MutationIntent::Create(CreateIntent::Entity(spec)) => {
                    planned_entity_creates.push(PlannedEntityCreate {
                        partition_id: spec.partition_id,
                        kind_id: spec.kind_id,
                        client_key: spec.client_key.clone(),
                        payload: spec.payload.clone(),
                    });
                }
                MutationIntent::Create(CreateIntent::BulkEntities(spec)) => {
                    for (client_key, payload) in spec.client_keys.iter().zip(&spec.payloads) {
                        planned_entity_creates.push(PlannedEntityCreate {
                            partition_id: spec.partition_id,
                            kind_id: spec.kind_id,
                            client_key: client_key.clone(),
                            payload: payload.clone(),
                        });
                    }
                }
                MutationIntent::Create(CreateIntent::Relation(spec)) => {
                    visible_entities.insert(spec.source);
                    visible_entities.insert(spec.target);
                    planned_relation_creates.push(PlannedRelationCreate {
                        partition_id: spec.partition_id,
                        kind_id: spec.kind_id,
                        client_key: spec.client_key.clone(),
                        source: spec.source,
                        target: spec.target,
                        payload: spec.payload.clone(),
                    });
                }
                MutationIntent::Create(CreateIntent::BulkRelations(spec)) => {
                    for ((source, target), (client_key, payload)) in spec
                        .endpoints
                        .iter()
                        .zip(spec.client_keys.iter().zip(&spec.payloads))
                    {
                        visible_entities.insert(*source);
                        visible_entities.insert(*target);
                        planned_relation_creates.push(PlannedRelationCreate {
                            partition_id: spec.partition_id,
                            kind_id: spec.kind_id,
                            client_key: client_key.clone(),
                            source: *source,
                            target: *target,
                            payload: payload.clone(),
                        });
                    }
                }
                MutationIntent::Entity(EntityMutationIntent::Update(spec)) => {
                    visible_entities.insert(spec.entity_id);
                }
                MutationIntent::Entity(EntityMutationIntent::Replace(spec)) => {
                    visible_entities.insert(spec.entity_id);
                }
                MutationIntent::Entity(EntityMutationIntent::Delete(spec)) => {
                    visible_entities.insert(spec.entity_id);
                }
                MutationIntent::Relation(RelationMutationIntent::Delete(spec)) => {
                    visible_relations.insert(spec.relation_id);
                    if let Some(metadata) = state_view.relation_metadata(spec.relation_id) {
                        include_relation_metadata(
                            &mut visible_entities,
                            &mut touched_partitions,
                            metadata,
                        );
                    }
                }
            }
        }
    }

    let seed_entities = visible_entities.iter().copied().collect::<Vec<_>>();
    for entity_id in seed_entities {
        for relation_id in runtime
            .storage_access()
            .all_relations_for_entity(entity_id, state_view.version_id())
        {
            visible_relations.insert(relation_id);
            if let Some(metadata) = state_view.relation_metadata(relation_id) {
                include_relation_metadata(&mut visible_entities, &mut touched_partitions, metadata);
            }
        }
    }

    TouchedStructuralSet {
        visible_entity_ids: visible_entities.into_iter().collect::<Vec<_>>().into(),
        visible_relation_ids: visible_relations.into_iter().collect::<Vec<_>>().into(),
        touched_partitions: touched_partitions.into_iter().collect::<Vec<_>>().into(),
        planned_entity_creates: planned_entity_creates.into(),
        planned_relation_creates: planned_relation_creates.into(),
    }
}

fn include_relation_metadata(
    visible_entities: &mut BTreeSet<EntityId>,
    touched_partitions: &mut BTreeSet<PartitionId>,
    metadata: VisibleRelationMetadata,
) {
    visible_entities.insert(metadata.source);
    visible_entities.insert(metadata.target);
    touched_partitions.insert(metadata.relation_id.partition_id);
    touched_partitions.insert(metadata.source.partition_id);
    touched_partitions.insert(metadata.target.partition_id);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::facade::runtime::RelationalRuntimeApi;
    use crate::facade::schema::RelationalSchemaRegistry;
    use crate::validation::data::{
        CustomInvariantOperationalMetadata, CustomInvariantSemanticIdentity,
        CustomInvariantSemanticVersion, InvariantCostClass, InvariantExecutionPoint,
        InvariantFailureEffect, InvariantGroup, InvariantGroupSet,
    };
    use crate::validation::engine::InvariantObservation;

    struct TestRule;

    impl CustomInvariantRule for TestRule {
        type Scope = usize;

        fn descriptor(&self) -> CustomInvariantDescriptor {
            CustomInvariantDescriptor {
                identity: CustomInvariantSemanticIdentity {
                    rule_id: CustomInvariantRuleId::new("test.rule"),
                    semantic_version: CustomInvariantSemanticVersion::new(1, 0),
                },
                display_name: Arc::from("Test Rule"),
                operational: CustomInvariantOperationalMetadata {
                    execution_point: InvariantExecutionPoint::CommitBoundary,
                    groups: InvariantGroupSet::of(InvariantGroup::SchemaCompliance),
                    cost_class: InvariantCostClass::Touched,
                    failure_effect: InvariantFailureEffect::BlockCommit,
                },
            }
        }

        fn prepare_scope(
            &self,
            planner: &mut CustomInvariantScopePlanner<'_>,
        ) -> Result<Self::Scope, CustomInvariantPreparationError> {
            let traversal = planner
                .traversal()
                .walk_outgoing_from(planner.touched().visible_entity_ids(), 1)?;
            Ok(traversal.visited_entities().len())
        }

        fn evaluate(
            &self,
            _context: &CustomInvariantExecutionContext<'_>,
            scope: &Self::Scope,
        ) -> Result<CustomInvariantVerdict, CustomInvariantExecutionError> {
            assert_eq!(*scope, 0);
            Ok(CustomInvariantVerdict::Pass)
        }
    }

    #[test]
    fn custom_registration_exposes_descriptor_and_rule_id() {
        let registration = CustomInvariantRegistration::new(TestRule).unwrap();
        assert_eq!(registration.rule_id().as_str(), "test.rule");
        assert_eq!(registration.descriptor().display_name.as_ref(), "Test Rule");
    }

    #[test]
    fn custom_registration_rejects_empty_ids() {
        struct EmptyRule;

        impl CustomInvariantRule for EmptyRule {
            type Scope = ();

            fn descriptor(&self) -> CustomInvariantDescriptor {
                CustomInvariantDescriptor {
                    identity: CustomInvariantSemanticIdentity {
                        rule_id: CustomInvariantRuleId::new(""),
                        semantic_version: CustomInvariantSemanticVersion::new(1, 0),
                    },
                    display_name: Arc::from("Empty"),
                    operational: CustomInvariantOperationalMetadata {
                        execution_point: InvariantExecutionPoint::CommitBoundary,
                        groups: InvariantGroupSet::of(InvariantGroup::SchemaCompliance),
                        cost_class: InvariantCostClass::Touched,
                        failure_effect: InvariantFailureEffect::BlockCommit,
                    },
                }
            }

            fn prepare_scope(
                &self,
                _planner: &mut CustomInvariantScopePlanner<'_>,
            ) -> Result<Self::Scope, CustomInvariantPreparationError> {
                Ok(())
            }

            fn evaluate(
                &self,
                _context: &CustomInvariantExecutionContext<'_>,
                _scope: &Self::Scope,
            ) -> Result<CustomInvariantVerdict, CustomInvariantExecutionError> {
                Ok(CustomInvariantVerdict::Pass)
            }
        }

        let error = CustomInvariantRegistration::new(EmptyRule).unwrap_err();
        assert_eq!(error, CustomInvariantRegistrationError::EmptyRuleId);
    }

    #[test]
    fn traversal_budget_is_session_wide() {
        let runtime = RelationalRuntimeApi::builder()
            .schema_registry(RelationalSchemaRegistry::new())
            .build();
        let observation = InvariantObservation::committed(runtime.storage_access().current_state());
        let context = CustomInvariantExecutionContext::new(
            &runtime,
            &observation,
            runtime.current_version_id(),
            None,
        );

        for _ in 0..8 {
            context.traversal().walk_outgoing_from(&[], 1).unwrap();
        }
        let large_seed_set = vec![EntityId::new(PartitionId::main(), 0, 1); 257];
        let error = context
            .traversal()
            .walk_outgoing_from(&large_seed_set, 1)
            .unwrap_err();
        assert!(error.detail().contains("session frontier budget"));
    }
}
