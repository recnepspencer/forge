use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::Instant;

use serde_json::json;

use super::domains::fintech::{
    perf_capture_baseline_observability, perf_capture_intraday_risk_probe,
    perf_capture_post_mutation_observability, perf_capture_trade_correction_probe,
    perf_correct_trade_correction, perf_emit_trade_correction_audit, perf_open_analysis_branch,
    perf_stress_intraday_risk, setup_intraday_risk_perf_world, setup_trade_correction_perf_world,
};
use super::performance_support::*;
use crate::capabilities::{DurabilityRead, DurabilityWrite};
use crate::facade::config::{AdjacencyBackend, RelationalRuntimeProfile};
use crate::facade::history::BranchId;
use crate::facade::indexes::{
    DerivedIndexBuildRequest, DerivedIndexDefinition, DerivedIndexId, DerivedIndexKind,
};
use crate::facade::inspection::{
    ConnectivityInspectionBudget, ConnectivityInspectionRequest, InspectionRecordClass,
    InspectionScope, KindInspectionRequest, RecentCommitInspectionRequest,
    StructuralIdentityQueryRequest,
};
use crate::facade::lineage::{
    HistoricalResolutionBoundednessBasis, HistoricalResolutionRequest, LineageDivergenceRequest,
    LineageDivergenceTraversalBasis,
};
use crate::facade::merge::{MergeExecutionRequest, MergeIntent};
use crate::facade::query::{
    DeterministicQueryPlanKey, FallbackParityMode, PlannedQueryPacket, QueryExecutionShape,
    QueryFallbackContract, QueryLocalityClass, QueryOrderingContract, QueryScope,
    ReductionDiscipline,
};
use crate::facade::replay::{RelationalReplayRequest, ReplayExecutionMode, ReplayVerificationMode};
use crate::facade::runtime::{CompiledArtifactCompatibility, EntityRecordProjection};
use crate::facade::symbols::Symbol;
use crate::tests::support::*;
use crate::validation::data::{
    CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
    CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
    CustomInvariantRegistration, CustomInvariantRule, CustomInvariantRuleId,
    CustomInvariantScopePlanner, CustomInvariantSemanticIdentity, CustomInvariantSemanticVersion,
    CustomInvariantVerdict, InvariantCostClass, InvariantExecutionPoint, InvariantFailureEffect,
    InvariantGroup, InvariantGroupSet,
};

#[derive(Debug, Clone, Copy, Default)]
struct MockBridgeEvaluationMetrics {
    nodes_evaluated: u64,
    nodes_recomputed: u64,
    suppressed_downstream_propagations: u64,
}

#[derive(Debug, Clone, Copy, Default)]
struct MockBridgePlannerMetrics {
    tasks_scheduled: u64,
    tasks_pruned_before_execution: u64,
}

#[derive(Debug, Clone, Copy, Default)]
struct MockBridgeObservation {
    evaluation: MockBridgeEvaluationMetrics,
    planner: MockBridgePlannerMetrics,
}

#[derive(Debug)]
struct MockBridgeRuntime {
    development_profile: bool,
    source_versions: Vec<u64>,
    bridge_versions: Vec<u64>,
    target_versions: Vec<u64>,
    observation: MockBridgeObservation,
    history_entries: usize,
    has_latest_flow: bool,
}

#[derive(Debug)]
struct GameEngineFrameSeedOutcome {
    entities: Vec<crate::facade::identity::EntityId>,
    frame_targets: Vec<crate::facade::identity::EntityId>,
    explicit_targets: Vec<crate::facade::identity::EntityId>,
    propagation_seeds: Vec<crate::facade::identity::EntityId>,
    relation_count: usize,
    region_count: usize,
}

impl MockBridgeRuntime {
    fn new(development_profile: bool, source_count: usize) -> Self {
        let bounded = source_count.max(4);
        Self {
            development_profile,
            source_versions: vec![1; bounded],
            bridge_versions: (0..bounded).map(|index| 100 + index as u64 * 10).collect(),
            target_versions: (0..bounded)
                .map(|index| 1_000 + index as u64 * 100)
                .collect(),
            observation: MockBridgeObservation::default(),
            history_entries: 0,
            has_latest_flow: false,
        }
    }

    fn warmup(&mut self) {
        self.apply_changes(self.source_versions.len());
        self.history_entries = 0;
        self.has_latest_flow = false;
        self.observation = MockBridgeObservation::default();
    }

    fn observe(&self) -> MockBridgeObservation {
        self.observation
    }

    fn recent_history_len(&self) -> usize {
        self.history_entries
    }

    fn latest_flow_diagnostics(&self) -> Option<()> {
        self.has_latest_flow.then_some(())
    }

    fn apply_changes(&mut self, affected_sources: usize) {
        let bounded = affected_sources.min(self.source_versions.len());
        let mut affected_targets = BTreeSet::new();
        for index in 0..bounded {
            self.source_versions[index] += 1;
            self.bridge_versions[index] += 10;
            self.target_versions[index] += 100;
            affected_targets.insert(index);
            affected_targets
                .insert((index + self.target_versions.len() - 1) % self.target_versions.len());
        }

        let affected_target_count = affected_targets.len() as u64;
        let bounded = bounded as u64;
        let tasks_scheduled = bounded + affected_target_count;
        let nodes_recomputed = bounded * 2 + affected_target_count;
        let nodes_evaluated = if self.development_profile {
            nodes_recomputed + affected_target_count
        } else {
            nodes_recomputed
        };
        let tasks_pruned_before_execution = if self.development_profile {
            0
        } else {
            bounded / 2
        };
        let suppressed_downstream_propagations = if self.development_profile {
            0
        } else {
            bounded.saturating_sub(1)
        };

        self.observation.evaluation.nodes_evaluated += nodes_evaluated;
        self.observation.evaluation.nodes_recomputed += nodes_recomputed;
        self.observation
            .evaluation
            .suppressed_downstream_propagations += suppressed_downstream_propagations;
        self.observation.planner.tasks_scheduled += tasks_scheduled;
        self.observation.planner.tasks_pruned_before_execution += tasks_pruned_before_execution;
        self.history_entries += if self.development_profile { 3 } else { 1 };
        self.has_latest_flow = true;
    }
}

fn diagnostic_artifact_kind_count(
    artifacts: &[crate::facade::diagnostics::RelationalDiagnosticArtifact],
    kind: DiagnosticsArtifactKind,
) -> usize {
    artifacts
        .iter()
        .filter(|artifact| artifact.kind == kind)
        .count()
}

fn diagnostic_artifact_scope_count(
    artifacts: &[crate::facade::diagnostics::RelationalDiagnosticArtifact],
    scope: DiagnosticsScope,
) -> usize {
    artifacts
        .iter()
        .filter(|artifact| artifact.scope == scope)
        .count()
}

fn diagnostic_entry_code_count(
    artifacts: &[crate::facade::diagnostics::RelationalDiagnosticArtifact],
    code: DiagnosticCode,
) -> usize {
    artifacts
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .filter(|entry| entry.code == code)
        .count()
}

fn diagnostic_entry_count(
    artifacts: &[crate::facade::diagnostics::RelationalDiagnosticArtifact],
) -> usize {
    artifacts
        .iter()
        .map(|artifact| artifact.entries.len())
        .sum()
}

fn runtime_execution_lane_code(profile: RelationalRuntimeProfile) -> u64 {
    match profile.boundary_policy().execution_lane {
        crate::facade::config::RuntimeExecutionLane::OperationalThin => 1,
        crate::facade::config::RuntimeExecutionLane::RichInteractive => 2,
        crate::facade::config::RuntimeExecutionLane::AuditReplayHeavy => 3,
    }
}

fn diagnostics_boundary_code(profile: RelationalRuntimeProfile) -> u64 {
    match profile.boundary_policy().diagnostics_boundary {
        crate::facade::config::DiagnosticsBoundary::MinimalHotTruth => 1,
        crate::facade::config::DiagnosticsBoundary::RichCertification => 2,
        crate::facade::config::DiagnosticsBoundary::DurableWorkflow => 3,
    }
}

fn profile_boundary_metrics(
    runtime: &crate::logic::runtime::RelationalRuntime,
    profile: RelationalRuntimeProfile,
) -> serde_json::Value {
    let boundary = runtime.config.boundary_policy();
    json!({
        "execution_lane_code": runtime_execution_lane_code(profile),
        "diagnostics_boundary_code": diagnostics_boundary_code(profile),
        "prefers_checkpoint_compaction": u64::from(boundary.prefers_checkpoint_compaction),
        "allows_compiled_lane": u64::from(boundary.allows_compiled_lane),
        "keeps_replay_hot_path_thin": u64::from(boundary.keeps_replay_hot_path_thin),
        "matches_defaults": u64::from(runtime.config.profile_boundary_matches_defaults()),
    })
}

fn build_mock_bridge_runtime(development_profile: bool, source_count: usize) -> MockBridgeRuntime {
    let mut runtime = MockBridgeRuntime::new(development_profile, source_count);
    runtime.warmup();
    runtime
}

fn seed_bridge_region_world(
    runtime: &mut RelationalRuntime,
    label: &str,
    node_count: usize,
    cross_link_stride: usize,
) -> Vec<crate::facade::identity::EntityId> {
    let mut entities = Vec::with_capacity(node_count);
    for index in 0..node_count {
        let created = create_entity_outcome(runtime, &format!("{label}-node-{index}"));
        entities.push(changed_entities(&created)[0]);
    }

    for window in entities.windows(2).enumerate() {
        let (index, pair) = window;
        create_relation_outcome(
            runtime,
            pair[0],
            pair[1],
            &format!("{label}-link-chain-{index}"),
        );
    }

    if cross_link_stride > 1 {
        for index in 0..entities.len() {
            let target = index + cross_link_stride;
            if target < entities.len() {
                create_relation_outcome(
                    runtime,
                    entities[index],
                    entities[target],
                    &format!("{label}-link-cross-{index}-{target}"),
                );
            }
        }
    }

    entities
}

fn seed_game_engine_frame_world(
    runtime: &mut RelationalRuntime,
    label: &str,
    region_count: usize,
    nodes_per_region: usize,
) -> GameEngineFrameSeedOutcome {
    let mut entities = Vec::with_capacity(region_count * nodes_per_region);
    let mut frame_targets = Vec::with_capacity(region_count);
    let mut explicit_targets = Vec::with_capacity(region_count * 2);
    let mut propagation_seeds = Vec::with_capacity(region_count.min(6));
    let mut relation_count = 0usize;

    for region in 0..region_count {
        let partition_id = PartitionId(700 + region as u32);
        let region_start = entities.len();
        for node in 0..nodes_per_region {
            let entity = create_entity_in_partition(
                runtime,
                &format!("{label}-region-{region}-node-{node}"),
                partition_id,
            );
            entities.push(entity);
        }

        let region_end = entities.len();
        let region_entities = &entities[region_start..region_end];
        for index in 0..(region_entities.len().saturating_sub(1)) {
            create_relation_in_partition(
                runtime,
                region_entities[index],
                region_entities[index + 1],
                &format!("{label}-region-{region}-chain-{index}"),
                PartitionId(760 + region as u32),
            );
            relation_count += 1;
        }

        for index in (0..region_entities.len().saturating_sub(3)).step_by(3) {
            create_relation_in_partition(
                runtime,
                region_entities[index],
                region_entities[index + 3],
                &format!("{label}-region-{region}-skip-{index}"),
                PartitionId(820 + region as u32),
            );
            relation_count += 1;
        }

        let actor = region_entities[region_entities.len() / 2];
        frame_targets.push(actor);
        explicit_targets.push(region_entities[1]);
        explicit_targets.push(region_entities[region_entities.len() - 2]);
        if region < 6 {
            propagation_seeds.push(actor);
        }
    }

    for region in 0..region_count {
        let current = frame_targets[region];
        let next = frame_targets[(region + 1) % region_count];
        create_relation_in_partition(
            runtime,
            current,
            next,
            &format!("{label}-region-bridge-{region}"),
            PartitionId(900 + region as u32),
        );
        relation_count += 1;
    }

    GameEngineFrameSeedOutcome {
        entities,
        frame_targets,
        explicit_targets,
        propagation_seeds,
        relation_count,
        region_count,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EntityIdentityProjection {
    entity_id: crate::facade::identity::EntityId,
}

impl EntityRecordProjection for EntityIdentityProjection {
    const KIND: KindId = KindId(1);

    fn from_record(record: crate::facade::runtime::EntityProjectionRecord<'_>) -> Option<Self> {
        Some(Self {
            entity_id: record.entity_id(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct MaterializationWaveRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MaterializationWaveScope {
    visible_entities: usize,
    visible_relations: usize,
    traversed_entities: usize,
    traversed_relations: usize,
    touched_partitions: usize,
}

impl CustomInvariantRule for MaterializationWaveRule {
    type Scope = MaterializationWaveScope;

    fn descriptor(&self) -> CustomInvariantDescriptor {
        CustomInvariantDescriptor {
            identity: CustomInvariantSemanticIdentity {
                rule_id: CustomInvariantRuleId::new("perf.materialization.wave"),
                semantic_version: CustomInvariantSemanticVersion::new(1, 0),
            },
            display_name: Arc::from("Perf Materialization Wave"),
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
        let touched = planner.touched();
        let traversal = planner
            .traversal()
            .walk_outgoing_from(touched.visible_entity_ids(), 2)?;
        Ok(MaterializationWaveScope {
            visible_entities: touched.visible_entity_ids().len(),
            visible_relations: touched.visible_relation_ids().len(),
            traversed_entities: traversal.visited_entities().len(),
            traversed_relations: traversal.traversed_relations().len(),
            touched_partitions: touched.touched_partitions().len(),
        })
    }

    fn evaluate(
        &self,
        context: &CustomInvariantExecutionContext<'_>,
        scope: &Self::Scope,
    ) -> Result<CustomInvariantVerdict, CustomInvariantExecutionError> {
        let counts = context.counts();
        let traversal = context
            .traversal()
            .walk_outgoing_from(context.touched().visible_entity_ids(), 2)?;
        if counts.visible_entity_count() == scope.visible_entities
            && counts.visible_relation_count() == scope.visible_relations
            && counts.touched_partition_count() == scope.touched_partitions
            && traversal.visited_entities().len() == scope.traversed_entities
            && traversal.traversed_relations().len() == scope.traversed_relations
        {
            Ok(CustomInvariantVerdict::Pass)
        } else {
            Ok(CustomInvariantVerdict::Violation)
        }
    }
}

fn runtime_with_test_schema_profile_and_custom_invariant(
    profile: RelationalRuntimeProfile,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(profile)
        .schema_registry(test_schema_registry())
        .custom_invariant(CustomInvariantRegistration::new(MaterializationWaveRule).unwrap())
        .build()
}

fn fresh_diagnostics_metrics(
    runtime: &RelationalRuntime,
    diagnostics_start: usize,
) -> (usize, usize) {
    let publication = runtime.publication();
    let diagnostics = publication.diagnostic_artifacts();
    let fresh_artifacts = &diagnostics[diagnostics_start..];
    let detailed_trace_entries = fresh_artifacts
        .iter()
        .filter(|artifact| {
            artifact.kind == crate::facade::diagnostics::DiagnosticsArtifactKind::DetailedTrace
        })
        .map(|artifact| artifact.entries.len())
        .sum::<usize>();
    (fresh_artifacts.len(), detailed_trace_entries)
}

fn dense_patch_record_count(runtime: &RelationalRuntime) -> usize {
    runtime
        .publication()
        .latest_patch()
        .map(|patch| {
            patch
                .records
                .iter()
                .filter(|record| {
                    matches!(
                        record.detail,
                        crate::publication::patch::data::PatchDetail::DenseBitset(_)
                    )
                })
                .count()
        })
        .unwrap_or(0)
}

fn entity_name_index_packet(
    runtime: &RelationalRuntime,
    snapshot: &crate::facade::snapshots::SnapshotHandle,
    label: &str,
    value: &str,
) -> PlannedQueryPacket {
    let context = runtime
        .read_truth()
        .query_plan_context(snapshot)
        .expect("query plan context");
    PlannedQueryPacket {
        label: label.to_string(),
        context_id: context,
        scope: QueryScope::EntityFieldEquals {
            field: field_key("name"),
            value: string_aspect_value(value),
            partition_scope: None,
        },
        locality: QueryLocalityClass::CrossPartitionTraversal,
        ordering: QueryOrderingContract::CanonicalEntityIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(1901),
        target_count_hint: 0,
    }
}

const DEFAULT_ROCKETSHIP_NODE_COUNT: usize = 100_000;
const DEFAULT_ROCKETSHIP_QUERY_TARGET_COUNT: usize = 256;
const ROCKETSHIP_PARTITION_WIDTH: usize = 32;
const ROCKETSHIP_CHUNK_SIZE: usize = 4_096;
const ROCKETSHIP_RELATION_SEED_BATCH_SIZE: usize = 16_000;
const ROCKETSHIP_SUBSYSTEM_ENTITY_PARTITION_FANOUT: usize = 8;

#[derive(Debug)]
struct RocketshipSeedOutcome {
    entities: Vec<crate::facade::identity::EntityId>,
    relation_count: usize,
    entity_commit_micros: u128,
    relation_commit_micros: u128,
    relation_commit_phase_timing: crate::transactions::data::CommitPhaseTiming,
}

#[derive(Debug)]
struct RocketshipPseudoRealisticSeedOutcome {
    entities: Vec<crate::facade::identity::EntityId>,
    mixed_query_targets: Vec<RecordRef>,
    traversal_seeds: Vec<crate::facade::identity::EntityId>,
    hot_update_target: crate::facade::identity::EntityId,
    relation_count: usize,
    subsystem_count: usize,
    entity_commit_micros: u128,
    relation_commit_micros: u128,
    relation_commit_phase_timing: crate::transactions::data::CommitPhaseTiming,
}

#[derive(Clone, Copy)]
struct RocketshipSubsystemLayout {
    section: &'static str,
    subsystem: &'static str,
    weight: usize,
    partition_base: u32,
}

const ROCKETSHIP_SUBSYSTEM_LAYOUTS: [RocketshipSubsystemLayout; 12] = [
    RocketshipSubsystemLayout {
        section: "nose",
        subsystem: "guidance",
        weight: 4,
        partition_base: 11,
    },
    RocketshipSubsystemLayout {
        section: "nose",
        subsystem: "avionics",
        weight: 5,
        partition_base: 21,
    },
    RocketshipSubsystemLayout {
        section: "upper_stage",
        subsystem: "payload_fairing",
        weight: 6,
        partition_base: 31,
    },
    RocketshipSubsystemLayout {
        section: "upper_stage",
        subsystem: "lox_tank",
        weight: 9,
        partition_base: 41,
    },
    RocketshipSubsystemLayout {
        section: "upper_stage",
        subsystem: "methane_tank",
        weight: 9,
        partition_base: 51,
    },
    RocketshipSubsystemLayout {
        section: "interstage",
        subsystem: "separation_ring",
        weight: 5,
        partition_base: 61,
    },
    RocketshipSubsystemLayout {
        section: "booster",
        subsystem: "forward_tank",
        weight: 12,
        partition_base: 71,
    },
    RocketshipSubsystemLayout {
        section: "booster",
        subsystem: "aft_tank",
        weight: 12,
        partition_base: 81,
    },
    RocketshipSubsystemLayout {
        section: "booster",
        subsystem: "thrust_frame",
        weight: 8,
        partition_base: 91,
    },
    RocketshipSubsystemLayout {
        section: "booster",
        subsystem: "engine_cluster",
        weight: 14,
        partition_base: 101,
    },
    RocketshipSubsystemLayout {
        section: "booster",
        subsystem: "plumbing_and_feed",
        weight: 8,
        partition_base: 111,
    },
    RocketshipSubsystemLayout {
        section: "aero",
        subsystem: "fins_and_actuation",
        weight: 8,
        partition_base: 121,
    },
];

fn rocketship_node_count() -> usize {
    std::env::var("FORGE_RELATIONAL_ROCKETSHIP_NODE_COUNT")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value >= 1_024)
        .unwrap_or(DEFAULT_ROCKETSHIP_NODE_COUNT)
}

fn rocketship_query_target_count(node_count: usize) -> usize {
    std::env::var("FORGE_RELATIONAL_ROCKETSHIP_QUERY_TARGET_COUNT")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_ROCKETSHIP_QUERY_TARGET_COUNT)
        .min(node_count)
}

fn seed_rocketship_world(
    runtime: &mut RelationalRuntime,
    node_count: usize,
) -> RocketshipSeedOutcome {
    let entity_commit_started_at = Instant::now();
    let entity_outcome = {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        let mut batch = WorkerIntentBatch::new("rocketship-entities-bulk");
        let mut entity_specs = Vec::with_capacity(node_count);
        for index in 0..node_count {
            let partition_id = PartitionId(1 + (index % ROCKETSHIP_PARTITION_WIDTH) as u32);
            entity_specs.push(crate::transactions::data::EntitySpec {
                partition_id,
                kind_id: KindId(1),
                client_key: crate::symbols::data::ClientKey::raw(format!("rocket-node-{index}")),
                fields: crate::tests::support::aspect_field_patch_from_values([
                    (
                        "name",
                        crate::tests::support::string_aspect_value(&format!("rocket-node-{index}")),
                    ),
                    (
                        "zone",
                        crate::tests::support::usize_aspect_value(
                            index % ROCKETSHIP_PARTITION_WIDTH,
                        ),
                    ),
                ]),
            });
        }
        for intent in bulk_entity_create_intents(&entity_specs) {
            batch = batch.push(intent);
        }
        txn.push_batch(batch);
        txn.commit().expect("rocketship entity seed commit")
    };
    let entity_commit_micros = entity_commit_started_at.elapsed().as_micros();
    let entities = changed_entities(&entity_outcome);
    assert_eq!(
        entities.len(),
        node_count,
        "rocketship entity seed should create the requested entity count"
    );

    let mut relation_specs = Vec::with_capacity(entities.len() + (entities.len() / 64));
    for index in 0..(entities.len() - 1) {
        relation_specs.push(crate::transactions::data::RelationSpec {
            partition_id: PartitionId(101 + (index % ROCKETSHIP_PARTITION_WIDTH) as u32),
            kind_id: KindId(2),
            client_key: crate::symbols::data::ClientKey::raw(format!("rocket-edge-{index}")),
            source: crate::transactions::data::EntityReference::Existing(entities[index]),
            target: crate::transactions::data::EntityReference::Existing(entities[index + 1]),
            fields: crate::transactions::data::AspectFieldPatch::default(),
        });
        if index + ROCKETSHIP_PARTITION_WIDTH < entities.len() && index % 64 == 0 {
            relation_specs.push(crate::transactions::data::RelationSpec {
                partition_id: PartitionId(201 + ((index / 64) % ROCKETSHIP_PARTITION_WIDTH) as u32),
                kind_id: KindId(2),
                client_key: crate::symbols::data::ClientKey::raw(format!("rocket-rib-{index}")),
                source: crate::transactions::data::EntityReference::Existing(entities[index]),
                target: crate::transactions::data::EntityReference::Existing(
                    entities[index + ROCKETSHIP_PARTITION_WIDTH],
                ),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            });
        }
    }
    let relation_count = relation_specs.len();
    let mut relation_commit_micros = 0u128;
    let mut relation_commit_phase_timing = crate::transactions::data::CommitPhaseTiming::default();
    for (chunk_index, relation_chunk) in relation_specs
        .chunks(ROCKETSHIP_RELATION_SEED_BATCH_SIZE)
        .enumerate()
    {
        let relation_commit_started_at = Instant::now();
        let outcome = {
            let mut txn = runtime.begin_transaction(TransactionOptions::default());
            let mut batch =
                WorkerIntentBatch::new(format!("rocketship-relations-bulk-{chunk_index}"));
            for intent in bulk_relation_create_intents(relation_chunk) {
                batch = batch.push(intent);
            }
            txn.push_batch(batch);
            txn.commit().expect("rocketship relation seed commit chunk")
        };
        relation_commit_micros += relation_commit_started_at.elapsed().as_micros();
        relation_commit_phase_timing.draft_preparation_micros +=
            outcome.execution.phase_timing.draft_preparation_micros;
        relation_commit_phase_timing.draft_bulk_admission_micros +=
            outcome.execution.phase_timing.draft_bulk_admission_micros;
        relation_commit_phase_timing.draft_merge_plan_micros +=
            outcome.execution.phase_timing.draft_merge_plan_micros;
        relation_commit_phase_timing.draft_structural_summary_micros += outcome
            .execution
            .phase_timing
            .draft_structural_summary_micros;
        relation_commit_phase_timing.draft_working_state_clone_micros += outcome
            .execution
            .phase_timing
            .draft_working_state_clone_micros;
        relation_commit_phase_timing.working_state_preparation_micros += outcome
            .execution
            .phase_timing
            .working_state_preparation_micros;
        relation_commit_phase_timing.invariant_pre_check_micros +=
            outcome.execution.phase_timing.invariant_pre_check_micros;
        relation_commit_phase_timing.authoritative_mutation_micros +=
            outcome.execution.phase_timing.authoritative_mutation_micros;
        relation_commit_phase_timing.history_resolution_micros +=
            outcome.execution.phase_timing.history_resolution_micros;
        relation_commit_phase_timing.invariant_post_check_micros +=
            outcome.execution.phase_timing.invariant_post_check_micros;
        relation_commit_phase_timing.artifact_assembly_micros +=
            outcome.execution.phase_timing.artifact_assembly_micros;
        relation_commit_phase_timing.durable_append_micros +=
            outcome.execution.phase_timing.durable_append_micros;
        relation_commit_phase_timing.publication_micros +=
            outcome.execution.phase_timing.publication_micros;
        relation_commit_phase_timing.publication_storage_commit_micros += outcome
            .execution
            .phase_timing
            .publication_storage_commit_micros;
        assert_eq!(
            changed_relations(&outcome).len(),
            relation_chunk.len(),
            "rocketship relation seed chunk should create the expected relation count"
        );
    }

    RocketshipSeedOutcome {
        entities,
        relation_count,
        entity_commit_micros,
        relation_commit_micros,
        relation_commit_phase_timing,
    }
}

fn seed_pseudorealistic_rocketship_world(
    runtime: &mut RelationalRuntime,
    node_count: usize,
    query_target_count: usize,
) -> RocketshipPseudoRealisticSeedOutcome {
    let total_weight: usize = ROCKETSHIP_SUBSYSTEM_LAYOUTS
        .iter()
        .map(|layout| layout.weight)
        .sum();
    let mut assigned = 0usize;
    let mut subsystem_ranges = Vec::with_capacity(ROCKETSHIP_SUBSYSTEM_LAYOUTS.len());

    let entity_commit_started_at = Instant::now();
    let entity_outcome = {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        let mut batch = WorkerIntentBatch::new("rocketship-pseudorealistic-entities");
        let mut entity_specs = Vec::with_capacity(node_count);
        for (layout_index, layout) in ROCKETSHIP_SUBSYSTEM_LAYOUTS.iter().enumerate() {
            let remaining_layouts = ROCKETSHIP_SUBSYSTEM_LAYOUTS.len() - layout_index;
            let remaining_nodes = node_count.saturating_sub(assigned);
            let subsystem_count = if remaining_layouts == 1 {
                remaining_nodes
            } else {
                ((node_count * layout.weight) / total_weight).max(512)
            }
            .min(remaining_nodes.saturating_sub(remaining_layouts - 1));
            let start = assigned;
            let end = start + subsystem_count;
            assigned = end;
            subsystem_ranges.push((start, end, *layout));

            for local_index in 0..subsystem_count {
                let aspect = match local_index % 4 {
                    0 => "structure",
                    1 => "thermal",
                    2 => "fluid",
                    _ => "control",
                };
                let partition_id = PartitionId(
                    layout.partition_base
                        + (local_index % ROCKETSHIP_SUBSYSTEM_ENTITY_PARTITION_FANOUT) as u32,
                );
                entity_specs.push(crate::transactions::data::EntitySpec {
                    partition_id,
                    kind_id: KindId(1),
                    client_key: crate::symbols::data::ClientKey::raw(format!(
                        "rocket.{}.{}.{}",
                        layout.section, layout.subsystem, local_index
                    )),
                    fields: crate::tests::support::aspect_field_patch_from_values([
                        (
                            "section",
                            crate::tests::support::string_aspect_value(layout.section),
                        ),
                        (
                            "subsystem",
                            crate::tests::support::string_aspect_value(layout.subsystem),
                        ),
                        ("aspect", crate::tests::support::string_aspect_value(aspect)),
                        (
                            "ordinal",
                            crate::tests::support::usize_aspect_value(local_index),
                        ),
                    ]),
                });
            }
        }
        for intent in bulk_entity_create_intents(&entity_specs) {
            batch = batch.push(intent);
        }
        txn.push_batch(batch);
        txn.commit()
            .expect("pseudorealistic rocketship entity seed commit")
    };
    let entity_commit_micros = entity_commit_started_at.elapsed().as_micros();
    assert_eq!(
        changed_entities(&entity_outcome).len(),
        node_count,
        "pseudorealistic rocketship should seed all entities"
    );
    let entities = rebuild_pseudorealistic_entity_order(runtime, &subsystem_ranges, node_count);

    let mut relation_specs = Vec::new();
    let mut mixed_query_targets = Vec::new();
    let mut traversal_seeds = Vec::new();

    for (range_index, (start, end, layout)) in subsystem_ranges.iter().enumerate() {
        let subsystem_entities = &entities[*start..*end];
        for local_index in 0..subsystem_entities.len().saturating_sub(1) {
            relation_specs.push(crate::transactions::data::RelationSpec {
                partition_id: PartitionId(201 + ((range_index + local_index) % 32) as u32),
                kind_id: KindId(2),
                client_key: crate::symbols::data::ClientKey::raw(format!(
                    "rocket.local.{}.{}.{}",
                    layout.section, layout.subsystem, local_index
                ))
                .into(),
                source: crate::transactions::data::EntityReference::Existing(
                    subsystem_entities[local_index],
                ),
                target: crate::transactions::data::EntityReference::Existing(
                    subsystem_entities[local_index + 1],
                ),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            });
            if local_index + 8 < subsystem_entities.len() && local_index % 16 == 0 {
                relation_specs.push(crate::transactions::data::RelationSpec {
                    partition_id: PartitionId(301 + ((range_index + local_index) % 32) as u32),
                    kind_id: KindId(2),
                    client_key: crate::symbols::data::ClientKey::raw(format!(
                        "rocket.aspect.{}.{}.{}",
                        layout.section, layout.subsystem, local_index
                    )),
                    source: crate::transactions::data::EntityReference::Existing(
                        subsystem_entities[local_index],
                    ),
                    target: crate::transactions::data::EntityReference::Existing(
                        subsystem_entities[local_index + 8],
                    ),
                    fields: crate::transactions::data::AspectFieldPatch::default(),
                });
            }
        }

        let midpoint = subsystem_entities.len() / 2;
        mixed_query_targets.push(RecordRef::Entity(subsystem_entities[midpoint]));
        traversal_seeds.push(subsystem_entities[midpoint]);
        if subsystem_entities.len() > 64 {
            mixed_query_targets.push(RecordRef::Entity(
                subsystem_entities[subsystem_entities.len() / 4],
            ));
            mixed_query_targets.push(RecordRef::Entity(
                subsystem_entities[(subsystem_entities.len() * 3) / 4],
            ));
        }
    }

    for pair in subsystem_ranges.windows(2) {
        let (left_start, left_end, left_layout) = pair[0];
        let (right_start, right_end, right_layout) = pair[1];
        let left_entities = &entities[left_start..left_end];
        let right_entities = &entities[right_start..right_end];
        let interface_stride = (left_entities.len().min(right_entities.len()) / 96).max(1);
        for interface_index in
            (0..left_entities.len().min(right_entities.len())).step_by(interface_stride)
        {
            relation_specs.push(crate::transactions::data::RelationSpec {
                partition_id: PartitionId(401 + (interface_index % 32) as u32),
                kind_id: KindId(2),
                client_key: crate::symbols::data::ClientKey::raw(format!(
                    "rocket.interface.{}.{}.{}.{}",
                    left_layout.section,
                    left_layout.subsystem,
                    right_layout.section,
                    interface_index
                )),
                source: crate::transactions::data::EntityReference::Existing(
                    left_entities[interface_index],
                ),
                target: crate::transactions::data::EntityReference::Existing(
                    right_entities[interface_index],
                ),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            });
        }
    }

    let guidance_anchor = traversal_seeds[0];
    let avionics_anchor = traversal_seeds[1];
    let engine_anchor = traversal_seeds[9];
    let plumbing_anchor = traversal_seeds[10];
    let fin_anchor = traversal_seeds[11];
    relation_specs.push(crate::transactions::data::RelationSpec {
        partition_id: PartitionId(501),
        kind_id: KindId(2),
        client_key: crate::symbols::data::ClientKey::raw("rocket.control.guidance-avionics"),
        source: crate::transactions::data::EntityReference::Existing(guidance_anchor),
        target: crate::transactions::data::EntityReference::Existing(avionics_anchor),
        fields: crate::transactions::data::AspectFieldPatch::default(),
    });
    relation_specs.push(crate::transactions::data::RelationSpec {
        partition_id: PartitionId(502),
        kind_id: KindId(2),
        client_key: crate::symbols::data::ClientKey::raw("rocket.control.avionics-engine"),
        source: crate::transactions::data::EntityReference::Existing(avionics_anchor),
        target: crate::transactions::data::EntityReference::Existing(engine_anchor),
        fields: crate::transactions::data::AspectFieldPatch::default(),
    });
    relation_specs.push(crate::transactions::data::RelationSpec {
        partition_id: PartitionId(503),
        kind_id: KindId(2),
        client_key: crate::symbols::data::ClientKey::raw("rocket.feed.plumbing-engine"),
        source: crate::transactions::data::EntityReference::Existing(plumbing_anchor),
        target: crate::transactions::data::EntityReference::Existing(engine_anchor),
        fields: crate::transactions::data::AspectFieldPatch::default(),
    });
    relation_specs.push(crate::transactions::data::RelationSpec {
        partition_id: PartitionId(504),
        kind_id: KindId(2),
        client_key: crate::symbols::data::ClientKey::raw("rocket.control.avionics-fin"),
        source: crate::transactions::data::EntityReference::Existing(avionics_anchor),
        target: crate::transactions::data::EntityReference::Existing(fin_anchor),
        fields: crate::transactions::data::AspectFieldPatch::default(),
    });

    let relation_count = relation_specs.len();
    let mut relation_commit_micros = 0u128;
    let mut relation_commit_phase_timing = crate::transactions::data::CommitPhaseTiming::default();
    for (chunk_index, relation_chunk) in relation_specs
        .chunks(ROCKETSHIP_RELATION_SEED_BATCH_SIZE)
        .enumerate()
    {
        let relation_commit_started_at = Instant::now();
        let outcome = {
            let mut txn = runtime.begin_transaction(TransactionOptions::default());
            let mut batch = WorkerIntentBatch::new(format!(
                "rocketship-pseudorealistic-relations-bulk-{chunk_index}"
            ));
            for intent in bulk_relation_create_intents(relation_chunk) {
                batch = batch.push(intent);
            }
            txn.push_batch(batch);
            txn.commit()
                .expect("pseudorealistic rocketship relation seed commit chunk")
        };
        relation_commit_micros += relation_commit_started_at.elapsed().as_micros();
        relation_commit_phase_timing.draft_preparation_micros +=
            outcome.execution.phase_timing.draft_preparation_micros;
        relation_commit_phase_timing.draft_bulk_admission_micros +=
            outcome.execution.phase_timing.draft_bulk_admission_micros;
        relation_commit_phase_timing.draft_merge_plan_micros +=
            outcome.execution.phase_timing.draft_merge_plan_micros;
        relation_commit_phase_timing.draft_structural_summary_micros += outcome
            .execution
            .phase_timing
            .draft_structural_summary_micros;
        relation_commit_phase_timing.draft_working_state_clone_micros += outcome
            .execution
            .phase_timing
            .draft_working_state_clone_micros;
        relation_commit_phase_timing.working_state_preparation_micros += outcome
            .execution
            .phase_timing
            .working_state_preparation_micros;
        relation_commit_phase_timing.invariant_pre_check_micros +=
            outcome.execution.phase_timing.invariant_pre_check_micros;
        relation_commit_phase_timing.authoritative_mutation_micros +=
            outcome.execution.phase_timing.authoritative_mutation_micros;
        relation_commit_phase_timing.history_resolution_micros +=
            outcome.execution.phase_timing.history_resolution_micros;
        relation_commit_phase_timing.invariant_post_check_micros +=
            outcome.execution.phase_timing.invariant_post_check_micros;
        relation_commit_phase_timing.artifact_assembly_micros +=
            outcome.execution.phase_timing.artifact_assembly_micros;
        relation_commit_phase_timing.durable_append_micros +=
            outcome.execution.phase_timing.durable_append_micros;
        relation_commit_phase_timing.publication_micros +=
            outcome.execution.phase_timing.publication_micros;
        relation_commit_phase_timing.publication_storage_commit_micros += outcome
            .execution
            .phase_timing
            .publication_storage_commit_micros;
        assert_eq!(changed_relations(&outcome).len(), relation_chunk.len());
    }

    mixed_query_targets.truncate(query_target_count.max(ROCKETSHIP_SUBSYSTEM_LAYOUTS.len()));
    let hot_update_target =
        entities[subsystem_ranges[9].0 + ((subsystem_ranges[9].1 - subsystem_ranges[9].0) / 2)];

    RocketshipPseudoRealisticSeedOutcome {
        entities,
        mixed_query_targets,
        traversal_seeds,
        hot_update_target,
        relation_count,
        subsystem_count: ROCKETSHIP_SUBSYSTEM_LAYOUTS.len(),
        entity_commit_micros,
        relation_commit_micros,
        relation_commit_phase_timing,
    }
}

fn rebuild_pseudorealistic_entity_order(
    runtime: &mut RelationalRuntime,
    subsystem_ranges: &[(usize, usize, RocketshipSubsystemLayout)],
    node_count: usize,
) -> Vec<crate::facade::identity::EntityId> {
    let expected_ranges = subsystem_ranges
        .iter()
        .map(|(start, end, layout)| ((layout.section, layout.subsystem), (*start, *end)))
        .collect::<BTreeMap<_, _>>();
    let snapshot = runtime.visibility_authority().snapshot();
    let read = runtime
        .read_truth()
        .read_snapshot(&snapshot)
        .expect("pseudorealistic entity snapshot");
    let mut ordered = vec![None; node_count];

    for record in read.entities() {
        if record.kind.kind_id != KindId(1) {
            continue;
        }
        let Some(section) = read_entity_field(record, "section") else {
            continue;
        };
        let Some(subsystem) = read_entity_field(record, "subsystem") else {
            continue;
        };
        let Some(ordinal) =
            read_entity_field(record, "ordinal").and_then(|value| value.parse::<usize>().ok())
        else {
            continue;
        };
        let Some((start, end)) = expected_ranges
            .get(&(section.as_str(), subsystem.as_str()))
            .copied()
        else {
            continue;
        };
        assert!(
            ordinal < end - start,
            "pseudorealistic entity ordinal must fit subsystem range"
        );
        ordered[start + ordinal] = Some(record.entity_id);
    }

    let released = runtime.visibility_authority().release_snapshot(&snapshot);
    assert!(
        released,
        "pseudorealistic entity reorder snapshot should release"
    );

    ordered
        .into_iter()
        .enumerate()
        .map(|(index, entity)| {
            entity.unwrap_or_else(|| panic!("missing pseudorealistic entity ordering slot {index}"))
        })
        .collect()
}

fn bulk_relation_create_intents(
    relation_specs: &[crate::transactions::data::RelationSpec],
) -> Vec<MutationIntent> {
    let mut by_partition: BTreeMap<
        (PartitionId, KindId),
        (
            Vec<crate::symbols::data::ClientKey>,
            Vec<(
                crate::transactions::data::EntityReference,
                crate::transactions::data::EntityReference,
            )>,
            Vec<crate::transactions::data::AspectFieldPatch>,
        ),
    > = BTreeMap::new();

    for relation in relation_specs {
        let entry = by_partition
            .entry((relation.partition_id, relation.kind_id))
            .or_insert_with(|| (Vec::new(), Vec::new(), Vec::new()));
        entry.0.push(relation.client_key.clone());
        entry
            .1
            .push((relation.source.clone(), relation.target.clone()));
        entry.2.push(relation.fields.clone());
    }

    by_partition
        .into_iter()
        .map(
            |((partition_id, kind_id), (client_keys, endpoints, field_patches))| {
                MutationIntent::Create(CreateIntent::BulkRelations(
                    crate::transactions::data::BulkRelationCreateIntent {
                        partition_id,
                        kind_id,
                        client_keys,
                        endpoints,
                        field_patches,
                    },
                ))
            },
        )
        .collect()
}

fn bulk_entity_create_intents(
    entity_specs: &[crate::transactions::data::EntitySpec],
) -> Vec<MutationIntent> {
    let mut by_partition: BTreeMap<
        (PartitionId, KindId),
        (
            Vec<crate::symbols::data::ClientKey>,
            Vec<crate::transactions::data::AspectFieldPatch>,
        ),
    > = BTreeMap::new();

    for entity in entity_specs {
        let entry = by_partition
            .entry((entity.partition_id, entity.kind_id))
            .or_insert_with(|| (Vec::new(), Vec::new()));
        entry.0.push(entity.client_key.clone());
        entry.1.push(entity.fields.clone());
    }

    by_partition
        .into_iter()
        .map(|((partition_id, kind_id), (client_keys, field_patches))| {
            MutationIntent::Create(CreateIntent::BulkEntities(
                crate::transactions::data::BulkEntityCreateIntent {
                    partition_id,
                    kind_id,
                    client_keys,
                    field_patches,
                },
            ))
        })
        .collect()
}

fn commit_measurement(
    runtime: &mut RelationalRuntime,
    run: impl FnOnce(&mut RelationalRuntime) -> CommitResult,
) -> PerfMeasurement {
    runtime.performance_access().reset_counters();
    let started_at = Instant::now();
    let outcome = run(runtime);
    let counters = runtime.performance_access().counters();
    let phase_timing = outcome.execution.phase_timing.clone();

    measurement_from(started_at, || {
        json!({
            "changed_records": outcome.changed_records.len(),
            "commit_topology": format!("{:?}", outcome.structural_summary().commit_topology),
            "touched_partitions": outcome.structural_summary().touched_partitions.len(),
            "packet_count": outcome.complexity_delta().preparation_packet_count,
            "query_packet_count": outcome.complexity_delta().query_packet_count,
            "snapshot_pin_full_rebuilds": outcome.complexity_delta().snapshot_pin_full_rebuilds,
            "phase_timing": {
                "working_state_preparation_micros": phase_timing.working_state_preparation_micros,
                "invariant_pre_check_micros": phase_timing.invariant_pre_check_micros,
                "authoritative_mutation_micros": phase_timing.authoritative_mutation_micros,
                "history_resolution_micros": phase_timing.history_resolution_micros,
                "invariant_post_check_micros": phase_timing.invariant_post_check_micros,
                "artifact_assembly_micros": phase_timing.artifact_assembly_micros,
                "durable_append_micros": phase_timing.durable_append_micros,
                "publication_micros": phase_timing.publication_micros
            },
            "counters": counters,
        })
    })
}

#[test]
#[ignore = "performance harness audit; run with -- --ignored --nocapture --test-threads=1"]
fn perf_harness_measurement_matrix() {
    let suite = "harness_measurement_matrix";

    let samples = capture_perf_samples(
        suite,
        "post_measurement_metrics_do_not_pollute_elapsed",
        || {
            let started_at = Instant::now();
            measurement_from(started_at, || {
                let metrics_started_at = Instant::now();
                let payload = (0..20_000u64)
                    .map(|index| {
                        json!({
                            "id": index,
                            "label": format!("measurement-audit-{index}"),
                            "value": index % 97,
                        })
                    })
                    .collect::<Vec<_>>();
                let payload_build_micros = metrics_started_at.elapsed().as_micros();
                json!({
                    "payload_build_micros": payload_build_micros,
                    "payload_item_count": payload.len(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "post_measurement_metrics_do_not_pollute_elapsed",
        &samples,
        &[
            ("payload_build_micros", &["payload_build_micros"]),
            ("payload_item_count", &["payload_item_count"]),
        ],
    );
    assert!(
        samples.iter().all(|sample| {
            metric_u64(&sample.metrics, "payload_build_micros") as u128
                > sample.elapsed_micros.saturating_mul(5)
        }),
        "measurement payload construction should remain outside reported elapsed time"
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_commit_delta_matrix() {
    let suite = "commit_delta_matrix";

    let narrow_samples = capture_perf_samples(suite, "single_partition_create_burst", || {
        let mut runtime = runtime_with_test_schema();
        commit_measurement(&mut runtime, |runtime| {
            let mut txn = runtime.begin_transaction(TransactionOptions::default());
            for index in 0..64 {
                txn.push_batch(batch_create(&format!("perf-entity-{index}")));
            }
            txn.commit().expect("single-partition create burst commit")
        })
    });
    assert!(narrow_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &narrow_samples,
        "single-partition commits should remain sparse and clone-free",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "snapshot_pin_full_rebuilds") == 0
                && counter_u64(metrics, "partitions_touched_by_commit") == 1
                && metric_u64(metrics, "packet_count") <= 4
        },
    );

    let cross_partition_samples =
        capture_perf_samples(suite, "cross_partition_relation_burst", || {
            let mut runtime = runtime_with_test_schema();
            let sources = (0..24)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("src-{index}"),
                        PartitionId(1),
                    )
                })
                .collect::<Vec<_>>();
            let targets = (0..24)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("dst-{index}"),
                        PartitionId(7),
                    )
                })
                .collect::<Vec<_>>();

            commit_measurement(&mut runtime, |runtime| {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                let mut batch = WorkerIntentBatch::new("cross-partition-relations");
                for (index, (source, target)) in sources.iter().zip(targets.iter()).enumerate() {
                    batch = batch.push(MutationIntent::Create(CreateIntent::Relation(
                        crate::transactions::data::RelationSpec {
                            partition_id: PartitionId(9),
                            kind_id: KindId(2),
                            client_key: crate::symbols::data::ClientKey::raw(format!(
                                "cross-{index}"
                            )),
                            source: crate::transactions::data::EntityReference::Existing(*source),
                            target: crate::transactions::data::EntityReference::Existing(*target),
                            fields: crate::transactions::data::AspectFieldPatch::default(),
                        },
                    )));
                }
                txn.push_batch(batch);
                txn.commit().expect("cross-partition relation burst commit")
            })
        });
    assert!(cross_partition_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &cross_partition_samples,
        "cross-partition relation bursts should avoid global cloning and stay packet-bounded",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "snapshot_pin_full_rebuilds") == 0
                && metric_u64(metrics, "touched_partitions") <= 3
                && counter_u64(metrics, "bulk_mutation_cross_partition_relation_count") == 24
                && metric_u64(metrics, "packet_count") <= 8
        },
    );

    let persisted_single_create_samples =
        capture_perf_samples(suite, "persisted_single_entity_create", || {
            let mut runtime = persisted_runtime_with_test_schema();
            commit_measurement(&mut runtime, |runtime| {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                txn.push_batch(batch_create("persisted-single"));
                txn.commit().expect("persisted single entity create")
            })
        });
    emit_metric_summaries(
        suite,
        "persisted_single_entity_create",
        &persisted_single_create_samples,
        &[
            (
                "working_state_preparation_micros",
                &["phase_timing", "working_state_preparation_micros"],
            ),
            (
                "authoritative_mutation_micros",
                &["phase_timing", "authoritative_mutation_micros"],
            ),
            (
                "artifact_assembly_micros",
                &["phase_timing", "artifact_assembly_micros"],
            ),
            (
                "durable_append_micros",
                &["phase_timing", "durable_append_micros"],
            ),
            (
                "publication_micros",
                &["phase_timing", "publication_micros"],
            ),
        ],
    );
    assert!(persisted_single_create_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &persisted_single_create_samples,
        "persisted single creates should remain clone-free and single-partition",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "snapshot_pin_full_rebuilds") == 0
                && counter_u64(metrics, "partitions_touched_by_commit") == 1
                && metric_u64(metrics, "packet_count") <= 4
                && metrics["phase_timing"]["durable_append_micros"]
                    .as_u64()
                    .unwrap_or(0)
                    > 0
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_durability_append_matrix() {
    let suite = "durability_append_matrix";

    let fresh_append_samples =
        capture_perf_samples(suite, "append_canonical_envelope_fresh_store", || {
            let mut source = runtime_with_test_schema();
            let envelope = create_entity_outcome(&mut source, "fresh-source")
                .publication
                .envelope
                .as_ref()
                .clone();
            let mut runtime = persisted_runtime_with_test_schema();

            let started_at = Instant::now();
            runtime
                .append_durable_envelope(&envelope)
                .expect("append canonical envelope to fresh store");
            let elapsed_micros = started_at.elapsed().as_micros();

            let store = runtime.durable_store().expect("durable store after append");
            let latest_segment = store
                .segments
                .last()
                .expect("segment manifest after append");
            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "segment_count": store.segments.len(),
                    "latest_segment_commit_count": latest_segment.commit_count,
                    "durable_log_len": runtime.durable_log().len(),
                }),
            }
        });
    assert!(fresh_append_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &fresh_append_samples,
        "fresh durable append should create one segment with one canonical envelope",
        |metrics| {
            metrics["segment_count"].as_u64() == Some(1)
                && metrics["latest_segment_commit_count"].as_u64() == Some(1)
                && metrics["durable_log_len"].as_u64() == Some(1)
        },
    );

    let warm_append_samples =
        capture_perf_samples(suite, "append_canonical_envelope_existing_segment", || {
            let mut source = runtime_with_test_schema();
            let envelope_a = create_entity_outcome(&mut source, "warm-source-a")
                .publication
                .envelope
                .as_ref()
                .clone();
            let envelope_b = create_entity_outcome(&mut source, "warm-source-b")
                .publication
                .envelope
                .as_ref()
                .clone();
            let mut runtime = persisted_runtime_with_test_schema();
            runtime
                .append_durable_envelope(&envelope_a)
                .expect("seed durable append");

            let started_at = Instant::now();
            runtime
                .append_durable_envelope(&envelope_b)
                .expect("append canonical envelope to existing segment");
            let elapsed_micros = started_at.elapsed().as_micros();

            let store = runtime.durable_store().expect("durable store after append");
            let latest_segment = store
                .segments
                .last()
                .expect("segment manifest after append");
            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "segment_count": store.segments.len(),
                    "latest_segment_commit_count": latest_segment.commit_count,
                    "durable_log_len": runtime.durable_log().len(),
                }),
            }
        });
    assert!(warm_append_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &warm_append_samples,
        "warm durable append should stay on the same segment up to capacity and extend the log by one",
        |metrics| {
            metrics["segment_count"].as_u64() == Some(1)
                && metrics["latest_segment_commit_count"].as_u64() == Some(2)
                && metrics["durable_log_len"].as_u64() == Some(2)
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_query_packet_matrix() {
    let suite = "query_packet_matrix";

    let explicit_target_samples =
        capture_perf_samples(suite, "explicit_targets_cross_partition", || {
            let mut runtime = runtime_with_test_schema_execution_model(
                crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation,
            );
            let targets = (0..64)
                .map(|index| {
                    let partition_id = match index % 4 {
                        0 => PartitionId(1),
                        1 => PartitionId(3),
                        2 => PartitionId(5),
                        _ => PartitionId(7),
                    };
                    RecordRef::Entity(create_entity_in_partition(
                        &mut runtime,
                        &format!("target-{index}"),
                        partition_id,
                    ))
                })
                .rev()
                .collect::<Vec<_>>();
            let snapshot = runtime.visibility_authority().snapshot();
            let packet = explicit_query_packet(&runtime, &snapshot, "explicit-targets", targets);

            runtime.performance_access().reset_counters();
            let planning_started_at = Instant::now();
            let planned = runtime
                .read_truth()
                .plan_query_packet(&snapshot, packet)
                .expect("planned explicit query");
            let planning_micros = planning_started_at.elapsed().as_micros();
            let execution_started_at = Instant::now();
            let outcome = runtime
                .read_truth()
                .execute_query_plan(planned)
                .expect("explicit target query outcome");
            let execution_micros = execution_started_at.elapsed().as_micros();
            let elapsed_micros = planning_micros + execution_micros;
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "result_entities": outcome.result.entities.len(),
                    "result_relations": outcome.result.relations.len(),
                    "phase_timing": {
                        "planning_micros": planning_micros,
                        "execution_micros": execution_micros,
                    },
                    "shape_metrics": {
                        "packet_count": outcome.complexity.packet_count,
                        "scope_unit_count": counters.query_scope_unit_count,
                    },
                    "complexity": outcome.complexity,
                    "counters": counters,
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "explicit_targets_cross_partition",
        &explicit_target_samples,
        &[
            ("planning_micros", &["phase_timing", "planning_micros"]),
            ("execution_micros", &["phase_timing", "execution_micros"]),
            ("packet_count", &["shape_metrics", "packet_count"]),
            ("scope_unit_count", &["shape_metrics", "scope_unit_count"]),
        ],
    );
    assert!(explicit_target_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &explicit_target_samples,
        "explicit target queries should stay packetized and avoid fallback scans",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "query_packet_count") <= 4
                && counter_u64(metrics, "query_index_attempt_count") == 0
                && metrics["result_entities"].as_u64() == Some(64)
        },
    );

    let kind_scan_samples =
        capture_perf_samples(suite, "entity_kind_scan_partition_matrix", || {
            let mut runtime = runtime_with_test_schema_execution_model(
                crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation,
            );
            for index in 0..128 {
                let partition_id = match index % 4 {
                    0 => PartitionId(1),
                    1 => PartitionId(3),
                    2 => PartitionId(5),
                    _ => PartitionId(7),
                };
                let _ = create_entity_in_partition(
                    &mut runtime,
                    &format!("scan-{index}"),
                    partition_id,
                );
            }
            let snapshot = runtime.visibility_authority().snapshot();
            let context = runtime
                .read_truth()
                .query_plan_context(&snapshot)
                .expect("query plan context");
            let packet = PlannedQueryPacket {
                label: "entity-kind-scan".to_string(),
                context_id: context,
                scope: QueryScope::EntityKindScan {
                    kind_id: KindId(1),
                    partition_scope: Some(Arc::from([
                        PartitionId(1),
                        PartitionId(3),
                        PartitionId(5),
                        PartitionId(7),
                    ])),
                },
                locality: QueryLocalityClass::PartitionBounded {
                    partitions: Arc::from([
                        PartitionId(1),
                        PartitionId(3),
                        PartitionId(5),
                        PartitionId(7),
                    ]),
                },
                ordering: QueryOrderingContract::CanonicalEntityIdOrder,
                fallback: QueryFallbackContract::StorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(20_001),
                target_count_hint: 0,
            };

            runtime.performance_access().reset_counters();
            let started_at = Instant::now();
            let outcome = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, packet)
                        .expect("planned query packet"),
                )
                .expect("entity kind scan outcome");
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "result_entities": outcome.result.entities.len(),
                    "phase_timing": {
                        "planning_micros": 0,
                        "execution_micros": elapsed_micros,
                    },
                    "shape_metrics": {
                        "packet_count": outcome.complexity.packet_count,
                        "scope_unit_count": counters.query_scope_unit_count,
                    },
                    "complexity": outcome.complexity,
                    "counters": counters,
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "entity_kind_scan_partition_matrix",
        &kind_scan_samples,
        &[
            ("execution_micros", &["phase_timing", "execution_micros"]),
            ("packet_count", &["shape_metrics", "packet_count"]),
            ("scope_unit_count", &["shape_metrics", "scope_unit_count"]),
        ],
    );
    assert!(kind_scan_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &kind_scan_samples,
        "partition-bounded kind scans should remain bounded to the requested entity surface",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "query_packet_count") <= 4
                && counter_u64(metrics, "query_entity_records_emitted") == 128
                && metrics["result_entities"].as_u64() == Some(128)
        },
    );

    let traversal_samples =
        capture_perf_samples(suite, "connectivity_traversal_cross_partition", || {
            let mut runtime = runtime_with_test_schema_execution_model(
                crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation,
            );
            let seeds = (0..12)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("seed-{index}"),
                        PartitionId(10 + index as u32),
                    )
                })
                .collect::<Vec<_>>();
            let neighbors = (0..12)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("neighbor-{index}"),
                        PartitionId(40 + index as u32),
                    )
                })
                .collect::<Vec<_>>();
            for (index, (seed, neighbor)) in seeds.iter().zip(neighbors.iter()).enumerate() {
                let _ = create_relation_in_partition(
                    &mut runtime,
                    *seed,
                    *neighbor,
                    &format!("edge-{index}"),
                    PartitionId(70 + index as u32),
                );
            }
            let snapshot = runtime.visibility_authority().snapshot();
            let context = runtime
                .read_truth()
                .query_plan_context(&snapshot)
                .expect("query plan context");
            let packet = PlannedQueryPacket {
                label: "connectivity-traversal".to_string(),
                context_id: context,
                scope: QueryScope::ConnectivityTraversal {
                    seeds: Arc::from(seeds.clone()),
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                    max_depth: Some(1),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                fallback: QueryFallbackContract::StorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(20_002),
                target_count_hint: seeds.len(),
            };

            runtime.performance_access().reset_counters();
            let planning_started_at = Instant::now();
            let planned = runtime
                .read_truth()
                .plan_query_packet(&snapshot, packet)
                .expect("planned traversal packet");
            let planning_micros = planning_started_at.elapsed().as_micros();
            let execution_started_at = Instant::now();
            let outcome = runtime
                .read_truth()
                .execute_query_plan(planned)
                .expect("connectivity traversal outcome");
            let execution_micros = execution_started_at.elapsed().as_micros();
            let elapsed_micros = planning_micros + execution_micros;
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "result_entities": outcome.result.entities.len(),
                    "result_relations": outcome.result.relations.len(),
                    "phase_timing": {
                        "planning_micros": planning_micros,
                        "execution_micros": execution_micros,
                    },
                    "shape_metrics": {
                        "packet_count": outcome.complexity.packet_count,
                        "scope_unit_count": counters.query_scope_unit_count,
                    },
                    "complexity": outcome.complexity,
                    "counters": counters,
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "connectivity_traversal_cross_partition",
        &traversal_samples,
        &[
            ("planning_micros", &["phase_timing", "planning_micros"]),
            ("execution_micros", &["phase_timing", "execution_micros"]),
            ("packet_count", &["shape_metrics", "packet_count"]),
            ("scope_unit_count", &["shape_metrics", "scope_unit_count"]),
        ],
    );
    assert!(traversal_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &traversal_samples,
        "connectivity traversal should stay narrow, clone-free, and relation-bounded",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "query_packet_count") <= 3
                && counter_u64(metrics, "query_scope_unit_count") <= 12
                && counter_u64(metrics, "query_relation_records_emitted") == 12
                && counter_u64(metrics, "query_packet_peak_width_total") <= 4
                && metrics["result_entities"].as_u64() == Some(24)
                && metrics["result_relations"].as_u64() == Some(12)
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_snapshot_materialization_matrix() {
    let suite = "snapshot_materialization_matrix";

    let snapshot_read_samples = capture_perf_samples(suite, "snapshot_read_view_current", || {
        let mut runtime = runtime_with_test_schema();
        for index in 0..128 {
            let _ = create_entity_in_partition(
                &mut runtime,
                &format!("entity-{index}"),
                PartitionId(1 + (index % 4) as u32),
            );
        }
        let snapshot = runtime.visibility_authority().snapshot();

        runtime.performance_access().reset_counters();
        let started_at = Instant::now();
        let read = runtime
            .read_truth()
            .read_snapshot(&snapshot)
            .expect("snapshot read");
        let elapsed_micros = started_at.elapsed().as_micros();
        let counters = runtime.performance_access().counters();

        PerfMeasurement {
            elapsed_micros,
            metrics: json!({
                "entities": read.entities().len(),
                "relations": read.relations().len(),
                "counters": counters,
            }),
        }
    });
    assert!(snapshot_read_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &snapshot_read_samples,
        "snapshot reads should remain cache-local and materialize only the live entity surface",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "snapshot_pin_full_rebuilds") == 0
                && counter_u64(metrics, "visible_entity_records_materialized") == 128
                && metrics["entities"].as_u64() == Some(128)
        },
    );

    let historical_read_samples =
        capture_perf_samples(suite, "version_read_view_historical", || {
            let mut runtime = runtime_with_test_schema();
            for index in 0..96 {
                let _ = create_entity_in_partition(
                    &mut runtime,
                    &format!("before-{index}"),
                    PartitionId(1 + (index % 3) as u32),
                );
            }
            let pinned_snapshot = runtime.visibility_authority().snapshot();
            for index in 0..24 {
                let entity_id = create_entity_in_partition(
                    &mut runtime,
                    &format!("after-{index}"),
                    PartitionId(9 + index as u32),
                );
                let _ = update_entity(&mut runtime, entity_id, &format!("after-updated-{index}"));
            }

            runtime.performance_access().reset_counters();
            let started_at = Instant::now();
            let read = runtime
                .read_truth()
                .read_version(pinned_snapshot.version_id);
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "entities": read.entities().len(),
                    "relations": read.relations().len(),
                    "counters": counters,
                }),
            }
        });
    assert!(historical_read_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &historical_read_samples,
        "historical reads should reconstruct only the pinned version surface",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "snapshot_pin_full_rebuilds") == 0
                && counter_u64(metrics, "visible_entity_records_materialized") == 96
                && metrics["entities"].as_u64() == Some(96)
        },
    );

    let projection_samples =
        capture_perf_samples(suite, "projection_entity_identity_surface", || {
            let mut runtime = runtime_with_test_schema();
            for index in 0..128 {
                let _ = create_entity_in_partition(
                    &mut runtime,
                    &format!("projection-{index}"),
                    PartitionId(1 + (index % 4) as u32),
                );
            }
            let snapshot = runtime.visibility_authority().snapshot();

            runtime.performance_access().reset_counters();
            let started_at = Instant::now();
            let projected = runtime
                .read_truth()
                .project_snapshot(&snapshot)
                .expect("projection snapshot")
                .entities::<EntityIdentityProjection>();
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "projected_entities": projected.len(),
                    "counters": counters,
                }),
            }
        });
    assert!(projection_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &projection_samples,
        "identity projection should remain narrow and allocation-light",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "snapshot_pin_full_rebuilds") == 0
                && counter_u64(metrics, "visible_entity_records_materialized") == 0
                && metrics["projected_entities"].as_u64() == Some(128)
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_retention_reclaim_matrix() {
    let suite = "retention_reclaim_matrix";

    let snapshot_pin_samples =
        capture_perf_samples(suite, "snapshot_release_to_reclaimable_entity", || {
            let mut runtime = runtime_with_test_schema();
            let created = create_entity_outcome(&mut runtime, "retained");
            let created_snapshot = runtime.visibility_authority().snapshot();
            let entity = changed_entities(&created)[0];
            let _deleted = delete_entity(&mut runtime, entity);
            let deleted_snapshot = runtime.visibility_authority().snapshot();

            assert!(runtime
                .visibility_authority()
                .release_snapshot(&created_snapshot));
            assert!(runtime
                .visibility_authority()
                .release_snapshot(&deleted_snapshot));

            runtime.performance_access().reset_counters();
            let inspect_started_at = Instant::now();
            let plan = runtime.retention().inspect_plan();
            let inspect_plan_micros = inspect_started_at.elapsed().as_micros();
            let pass_started_at = Instant::now();
            let pass = runtime.retention().run_pass();
            let run_pass_micros = pass_started_at.elapsed().as_micros();
            let elapsed_micros = inspect_plan_micros + run_pass_micros;
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "active_snapshot_count": plan.active_snapshot_count,
                    "reclaimable_entities": plan.reclaimable_entities,
                    "entity_reclaimable": pass.entity_reclaimable,
                    "entity_reclaimed": pass.entity_reclaimed,
                    "phase_timing": {
                        "inspect_plan_micros": inspect_plan_micros,
                        "run_pass_micros": run_pass_micros,
                    },
                    "counters": counters,
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "snapshot_release_to_reclaimable_entity",
        &snapshot_pin_samples,
        &[
            (
                "inspect_plan_micros",
                &["phase_timing", "inspect_plan_micros"],
            ),
            ("run_pass_micros", &["phase_timing", "run_pass_micros"]),
        ],
    );
    assert!(snapshot_pin_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &snapshot_pin_samples,
        "retention reclaim should become reclaimable after snapshot release without full rebuilds",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && metrics["active_snapshot_count"].as_u64() == Some(0)
                && metrics["reclaimable_entities"].as_u64().unwrap_or(0) >= 1
                && metrics["entity_reclaimable"].as_u64().unwrap_or(0) >= 1
        },
    );

    let replay_pin_samples =
        capture_perf_samples(suite, "replay_pin_release_deleted_relation", || {
            let mut runtime = runtime_with_test_schema();
            let source = create_entity(&mut runtime, "replay-left");
            let target = create_entity(&mut runtime, "replay-right");
            let created = create_relation_outcome(&mut runtime, source, target, "replay-r1");
            let relation = changed_relations(&created)[0];
            let deleted = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                txn.push_batch(WorkerIntentBatch::new("delete-relation").push(
                    MutationIntent::Relation(RelationMutationIntent::Delete(
                        DeleteRelationIntent {
                            relation_id: relation,
                        },
                    )),
                ));
                txn.commit().expect("delete relation")
            };

            assert!(runtime
                .visibility_authority()
                .release_snapshot(&created.snapshot));
            assert!(runtime
                .visibility_authority()
                .release_snapshot(&deleted.snapshot));
            assert!(runtime
                .history_authority()
                .retain_version_for_replay(created.version_id));

            runtime.performance_access().reset_counters();
            let inspect_started_at = Instant::now();
            let pinned = runtime.retention().inspect_plan();
            let inspect_pinned_micros = inspect_started_at.elapsed().as_micros();
            let release_started_at = Instant::now();
            assert!(runtime
                .history_authority()
                .release_version_replay_retention(created.version_id));
            let release_replay_pin_micros = release_started_at.elapsed().as_micros();
            let inspect_released_started_at = Instant::now();
            let released = runtime.retention().inspect_plan();
            let inspect_released_micros = inspect_released_started_at.elapsed().as_micros();
            let elapsed_micros =
                inspect_pinned_micros + release_replay_pin_micros + inspect_released_micros;
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "pinned_replay_relations": pinned.replay_pinned_relations,
                    "pinned_reclaimable_relations": pinned.reclaimable_relations,
                    "released_branch_pinned_relations": released.branch_pinned_relations,
                    "released_replay_relations": released.replay_pinned_relations,
                    "released_reclaimable_relations": released.reclaimable_relations,
                    "phase_timing": {
                        "inspect_pinned_micros": inspect_pinned_micros,
                        "release_replay_pin_micros": release_replay_pin_micros,
                        "inspect_released_micros": inspect_released_micros,
                    },
                    "counters": counters,
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "replay_pin_release_deleted_relation",
        &replay_pin_samples,
        &[
            (
                "inspect_pinned_micros",
                &["phase_timing", "inspect_pinned_micros"],
            ),
            (
                "release_replay_pin_micros",
                &["phase_timing", "release_replay_pin_micros"],
            ),
            (
                "inspect_released_micros",
                &["phase_timing", "inspect_released_micros"],
            ),
        ],
    );
    assert!(replay_pin_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &replay_pin_samples,
        "replay retention should pin relations until release and then expose reclaimability",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && metrics["pinned_replay_relations"].as_u64().unwrap_or(0) >= 1
                && metrics["pinned_reclaimable_relations"].as_u64() == Some(0)
                && metrics["released_replay_relations"].as_u64() == Some(0)
                && metrics["released_branch_pinned_relations"]
                    .as_u64()
                    .unwrap_or(0)
                    >= 1
                && metrics["released_reclaimable_relations"].as_u64() == Some(0)
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_replay_recovery_matrix() {
    let suite = "replay_recovery_matrix";

    let durable_replay_samples = capture_perf_samples(
        suite,
        "durable_replay_lineage_basis",
        || {
            let mut runtime = persisted_runtime_with_test_schema();
            let first = create_entity_outcome(&mut runtime, "source");
            let second = create_entity_outcome(&mut runtime, "target");
            let first_lineage = runtime
                .lineage_access()
                .for_record(changed_entities(&first)[0])
                .expect("first lineage")
                .lineage_id;
            let second_lineage = runtime
                .lineage_access()
                .for_record(changed_entities(&second)[0])
                .expect("second lineage")
                .lineage_id;
            let candidate = runtime.lineage_authority().record_correspondence_candidate(
                BranchId("main".to_string()),
                vec![first_lineage],
                vec![second_lineage],
                "perf-lineage-replay",
            );
            let promotion = runtime
                .lineage_authority()
                .promote_correspondence(candidate.candidate_id, second.commit.clone())
                .expect("promote correspondence");
            let promoted_commit_id = promotion
                .promoted_commit_id()
                .expect("metadata-only promotion commit");

            runtime.performance_access().reset_counters();
            let replay_started_at = Instant::now();
            let outcome = runtime
                .replay_authority()
                .replay_commit(RelationalReplayRequest {
                    commit_id: promoted_commit_id,
                    branch_id: BranchId("main".to_string()),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                    verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros: replay_commit_micros,
                metrics: json!({
                    "failure": outcome.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "mismatch_count": outcome.mismatches.len(),
                    "compared_surface_count": outcome.compared_surfaces.len(),
                    "reconstructed_commit_closure": outcome.reconstructed_commit_closure.len(),
                    "lineage_authority_basis": outcome
                        .lineage_authority_basis
                        .as_ref()
                        .map(|basis: &crate::replay::data::ReplayLineageAuthorityBasis| format!("{:?}", basis.kind())),
                    "phase_timing": {
                        "replay_commit_micros": replay_commit_micros,
                    },
                    "counters": counters,
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "durable_replay_lineage_basis",
        &durable_replay_samples,
        &[(
            "replay_commit_micros",
            &["phase_timing", "replay_commit_micros"],
        )],
    );
    assert!(durable_replay_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &durable_replay_samples,
        "durable replay should resolve against canonical lineage artifacts without mismatches",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && metrics["failure"].is_null()
                && metrics["mismatch_count"].as_u64() == Some(0)
                && metrics["lineage_authority_basis"].as_str() == Some("DurableLogCanonical".into())
                && counter_u64(metrics, "replay_lineage_authority_lookup_requests") == 1
        },
    );

    let checkpoint_recovery_samples =
        capture_perf_samples(suite, "checkpoint_recover_suffix_replay", || {
            let mut runtime = persisted_runtime_with_test_schema();
            let first = create_entity_outcome(&mut runtime, "source");
            let second = create_entity_outcome(&mut runtime, "target");
            let first_lineage = runtime
                .lineage_access()
                .for_record(changed_entities(&first)[0])
                .expect("first lineage")
                .lineage_id;
            let second_lineage = runtime
                .lineage_access()
                .for_record(changed_entities(&second)[0])
                .expect("second lineage")
                .lineage_id;
            runtime
                .durability_authority()
                .checkpoint()
                .expect("checkpoint");
            let candidate = runtime.lineage_authority().record_correspondence_candidate(
                BranchId("main".to_string()),
                vec![first_lineage],
                vec![second_lineage],
                "perf-recovery-promotion",
            );
            let promoted = runtime
                .lineage_authority()
                .promote_correspondence(candidate.candidate_id, second.commit.clone())
                .expect("promote correspondence");
            let promoted_commit_id = promoted.promoted_commit_id().expect("promotion commit id");

            let recovery_plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema();

            recovered.performance_access().reset_counters();
            let recovery_started_at = Instant::now();
            let outcome = recovered
                .durability_authority()
                .recover(recovery_plan)
                .expect("recover plan");
            let recovery_micros = recovery_started_at.elapsed().as_micros();
            let replay_started_at = Instant::now();
            let replay = recovered
                .replay_authority()
                .replay_commit(RelationalReplayRequest {
                    commit_id: promoted_commit_id,
                    branch_id: BranchId("main".to_string()),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                    verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();
            let elapsed_micros = recovery_micros + replay_commit_micros;
            let counters = recovered.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "recovered_commits": outcome.recovered_commits,
                    "checkpoint_commits": outcome.coverage.checkpoint_commits,
                    "replayed_tail_commits": outcome.coverage.replayed_tail_commits,
                    "selected_checkpoint": outcome.cursor.checkpoint_id.is_some(),
                    "replay_failure": replay.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "replay_mismatch_count": replay.mismatches.len(),
                    "phase_timing": {
                        "recovery_micros": recovery_micros,
                        "replay_commit_micros": replay_commit_micros,
                    },
                    "counters": counters,
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "checkpoint_recover_suffix_replay",
        &checkpoint_recovery_samples,
        &[
            ("recovery_micros", &["phase_timing", "recovery_micros"]),
            (
                "replay_commit_micros",
                &["phase_timing", "replay_commit_micros"],
            ),
        ],
    );
    assert!(checkpoint_recovery_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &checkpoint_recovery_samples,
        "checkpoint recovery plus suffix replay should recover commits cleanly without replay drift",
        |metrics| {
            metrics["recovered_commits"].as_u64().unwrap_or(0) >= 1
                && metrics["checkpoint_commits"].as_u64().unwrap_or(0) >= 1
                && metrics["replayed_tail_commits"].as_u64().unwrap_or(0) >= 1
                && metrics["selected_checkpoint"].as_bool() == Some(true)
                && metrics["replay_failure"].is_null()
                && metrics["replay_mismatch_count"].as_u64() == Some(0)
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_invariant_materialization_matrix() {
    let suite = "invariant_materialization_matrix";

    let custom_surface_samples =
        capture_perf_samples(suite, "custom_structural_surface_commit_wave", || {
            let mut runtime = runtime_with_test_schema_profile_and_custom_invariant(
                RelationalRuntimeProfile::CertificationCore,
            );
            let entities = (0..12)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("invariant-node-{index}"),
                        PartitionId((index % 4) as u32 + 1),
                    )
                })
                .collect::<Vec<_>>();
            for index in 0..(entities.len() - 1) {
                create_relation_in_partition(
                    &mut runtime,
                    entities[index],
                    entities[index + 1],
                    &format!("invariant-link-{index}"),
                    PartitionId(20 + (index % 4) as u32),
                );
            }

            runtime.performance_access().reset_counters();
            let started_at = Instant::now();
            let outcome = create_relation_outcome(
                &mut runtime,
                entities[2],
                entities[9],
                "invariant-wave-bridge",
            );
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();
            let phase_timing = outcome.execution.phase_timing.clone();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "changed_records": outcome.changed_records.len(),
                    "phase_timing": {
                        "invariant_pre_check_micros": phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": phase_timing.authoritative_mutation_micros,
                        "invariant_post_check_micros": phase_timing.invariant_post_check_micros,
                    },
                    "counters": counters,
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "custom_structural_surface_commit_wave",
        &custom_surface_samples,
        &[
            (
                "invariant_pre_check_micros",
                &["phase_timing", "invariant_pre_check_micros"],
            ),
            (
                "authoritative_mutation_micros",
                &["phase_timing", "authoritative_mutation_micros"],
            ),
            (
                "invariant_post_check_micros",
                &["phase_timing", "invariant_post_check_micros"],
            ),
        ],
    );
    assert!(custom_surface_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &custom_surface_samples,
        "custom invariant surface waves should execute touched-only invariant work without clone-heavy materialization",
        |metrics| {
            metrics["changed_records"].as_u64() == Some(1)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "custom_invariant_preparation_count") == 1
                && counter_u64(metrics, "custom_invariant_execution_count") == 1
                && counter_u64(metrics, "custom_invariant_panic_count") == 0
                && counter_u64(metrics, "custom_invariant_traversal_frontier_count") >= 1
                && counter_u64(metrics, "custom_invariant_traversal_step_count") >= 1
                && (counter_u64(metrics, "invariant_entity_slot_scans") >= 1
                    || counter_u64(metrics, "invariant_relation_slot_scans") >= 1)
                && counter_u64(metrics, "invariant_entity_slot_scans")
                    + counter_u64(metrics, "invariant_relation_slot_scans")
                    >= 1
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_geometry_kernel_matrix() {
    let suite = "geometry_kernel_matrix";

    let topology_identity_samples = capture_perf_samples(
        suite,
        "topology_identity_survival_recovery_round_trip",
        || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::GeometryKernel,
            );
            let created = create_entity_outcome(&mut runtime, "topology-source");
            let entity = changed_entities(&created)[0];
            let start_lineage = runtime
                .lineage_access()
                .for_record(entity)
                .expect("initial lineage")
                .lineage_id;

            let update_started_at = Instant::now();
            let replacement = update_entity(&mut runtime, entity, "topology-source-updated");
            let update_commit_micros = update_started_at.elapsed().as_micros();
            let replaced_entity = changed_entities(&replacement)[0];
            let replacement_lineage = runtime
                .lineage_access()
                .for_record(replaced_entity)
                .expect("replacement lineage")
                .lineage_id;

            runtime.performance_access().reset_counters();
            let resolution_started_at = Instant::now();
            let resolution =
                runtime
                    .lineage_access()
                    .resolve_historical_lineage(HistoricalResolutionRequest {
                        branch_id: BranchId("main".to_string()),
                        lineage_id: start_lineage,
                        boundedness_basis:
                            HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed,
                    });
            let lineage_resolution_micros = resolution_started_at.elapsed().as_micros();
            let resolution_counters = runtime.performance_access().counters();

            let checkpoint_started_at = Instant::now();
            runtime
                .durability_authority()
                .checkpoint()
                .expect("geometry topology checkpoint");
            let checkpoint_micros = checkpoint_started_at.elapsed().as_micros();

            let plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::GeometryKernel,
            );
            let recover_started_at = Instant::now();
            recovered
                .durability_authority()
                .recover(plan)
                .expect("geometry topology recovery");
            let recover_micros = recover_started_at.elapsed().as_micros();

            recovered.performance_access().reset_counters();
            let recovered_resolution_started_at = Instant::now();
            let recovered_resolution = recovered.lineage_access().resolve_historical_lineage(
                HistoricalResolutionRequest {
                    branch_id: BranchId("main".to_string()),
                    lineage_id: start_lineage,
                    boundedness_basis:
                        HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed,
                },
            );
            let recovered_lineage_resolution_micros =
                recovered_resolution_started_at.elapsed().as_micros();
            let recovered_counters = recovered.performance_access().counters();

            PerfMeasurement {
                elapsed_micros: update_commit_micros
                    + lineage_resolution_micros
                    + checkpoint_micros
                    + recover_micros
                    + recovered_lineage_resolution_micros,
                metrics: json!({
                    "update_commit_micros": update_commit_micros,
                    "lineage_resolution_micros": lineage_resolution_micros,
                    "checkpoint_micros": checkpoint_micros,
                    "recover_micros": recover_micros,
                    "recovered_lineage_resolution_micros": recovered_lineage_resolution_micros,
                    "resolved_lineage_count": resolution.metrics.resolved_lineage_count,
                    "traversed_event_count": resolution.traversed_event_ids.len(),
                    "replacement_lineage_matches": resolution.resolved == vec![replacement_lineage],
                    "recovered_resolution_matches": recovered_resolution.resolved == resolution.resolved
                        && recovered_resolution.traversed_event_ids == resolution.traversed_event_ids
                        && recovered_resolution.digest_basis() == resolution.digest_basis(),
                    "counters": resolution_counters,
                    "recovered_counters": recovered_counters
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "topology_identity_survival_recovery_round_trip",
        &topology_identity_samples,
        &[
            ("update_commit_micros", &["update_commit_micros"]),
            ("lineage_resolution_micros", &["lineage_resolution_micros"]),
            ("checkpoint_micros", &["checkpoint_micros"]),
            ("recover_micros", &["recover_micros"]),
            (
                "recovered_lineage_resolution_micros",
                &["recovered_lineage_resolution_micros"],
            ),
            ("resolved_lineage_count", &["resolved_lineage_count"]),
            ("traversed_event_count", &["traversed_event_count"]),
        ],
    );
    assert!(topology_identity_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &topology_identity_samples,
        "geometry identity survival should preserve exact lineage truth across recovery",
        |metrics| {
            metrics["replacement_lineage_matches"].as_bool() == Some(true)
                && metrics["recovered_resolution_matches"].as_bool() == Some(true)
                && metrics["resolved_lineage_count"].as_u64() == Some(1)
                && metrics["checkpoint_micros"].as_u64().unwrap_or(0) > 0
                && metrics["recover_micros"].as_u64().unwrap_or(0) > 0
                && metrics["counters"]["lineage_historical_resolution_requests"].as_u64() == Some(1)
                && metrics["recovered_counters"]["lineage_historical_resolution_requests"].as_u64()
                    == Some(1)
        },
    );

    let topology_bridge_samples =
        capture_perf_samples(suite, "topology_bridge_connectivity_wave", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
            let mut cluster_a = Vec::new();
            let mut cluster_b = Vec::new();
            for index in 0..6 {
                cluster_a.push(create_entity_in_partition(
                    &mut runtime,
                    &format!("cluster-a-{index}"),
                    PartitionId((index % 3) as u32 + 1),
                ));
                cluster_b.push(create_entity_in_partition(
                    &mut runtime,
                    &format!("cluster-b-{index}"),
                    PartitionId((index % 3) as u32 + 5),
                ));
            }
            for index in 0..(cluster_a.len() - 1) {
                create_relation_in_partition(
                    &mut runtime,
                    cluster_a[index],
                    cluster_a[index + 1],
                    &format!("a-link-{index}"),
                    PartitionId(11),
                );
                create_relation_in_partition(
                    &mut runtime,
                    cluster_b[index],
                    cluster_b[index + 1],
                    &format!("b-link-{index}"),
                    PartitionId(12),
                );
            }

            runtime.performance_access().reset_counters();
            let bridge_started_at = Instant::now();
            let bridge_outcome = create_relation_outcome(
                &mut runtime,
                cluster_a[2],
                cluster_b[2],
                "bridge-topology-wave",
            );
            let bridge_commit_micros = bridge_started_at.elapsed().as_micros();

            let connectivity_started_at = Instant::now();
            let summary = runtime.inspect_what_happened().connectivity_summary(
                &ConnectivityInspectionRequest {
                    scope: InspectionScope::Current,
                    partition_scope: None,
                    relation_kind_scope: Some(vec![KindId(2)]),
                    include_members: false,
                    budget: ConnectivityInspectionBudget {
                        max_entities: 64,
                        max_relations: 64,
                        max_frontier: 64,
                        max_components: 8,
                        max_work_units: 256,
                    },
                },
            );
            let connectivity_summary_micros = connectivity_started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros: bridge_commit_micros + connectivity_summary_micros,
                metrics: json!({
                    "bridge_commit_micros": bridge_commit_micros,
                    "connectivity_summary_micros": connectivity_summary_micros,
                    "bridge_changed_records": bridge_outcome.changed_records.len(),
                    "component_count": summary.component_count,
                    "largest_component_size": summary.largest_component_size,
                    "enumerated_entity_count": summary.enumerated_entity_count,
                    "availability": format!("{:?}", summary.availability),
                    "degradation_count": summary.degradations.len(),
                    "counters": counters,
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "topology_bridge_connectivity_wave",
        &topology_bridge_samples,
        &[
            ("bridge_commit_micros", &["bridge_commit_micros"]),
            (
                "connectivity_summary_micros",
                &["connectivity_summary_micros"],
            ),
            ("component_count", &["component_count"]),
            ("largest_component_size", &["largest_component_size"]),
            ("enumerated_entity_count", &["enumerated_entity_count"]),
        ],
    );
    assert!(topology_bridge_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &topology_bridge_samples,
        "geometry topology bridge should collapse two local components into one bounded connectivity surface",
        |metrics| {
            metrics["bridge_changed_records"].as_u64() == Some(1)
                && metrics["component_count"].as_u64() == Some(1)
                && metrics["largest_component_size"].as_u64() == Some(12)
                && metrics["enumerated_entity_count"].as_u64() == Some(12)
                && metrics["availability"].as_str() == Some("Direct".into())
                && metrics["degradation_count"].as_u64() == Some(1)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "relation_slots_touched_by_commit") == 1
                && counter_u64(metrics, "inspection_connectivity_summary_requests") == 1
                && counter_u64(metrics, "inspection_connectivity_components_evaluated") == 1
                && counter_u64(metrics, "inspection_connectivity_frontier_expansions") >= 1
                && counter_u64(metrics, "inspection_connectivity_entity_scans") == 12
                && counter_u64(metrics, "inspection_connectivity_relation_scans") == 11
        },
    );

    let topology_bridge_rich_geometry_samples = capture_perf_samples(
        suite,
        "topology_bridge_connectivity_wave_rich_geometry_profile",
        || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let mut cluster_a = Vec::new();
            let mut cluster_b = Vec::new();
            for index in 0..6 {
                cluster_a.push(create_entity_in_partition(
                    &mut runtime,
                    &format!("rich-cluster-a-{index}"),
                    PartitionId((index % 3) as u32 + 1),
                ));
                cluster_b.push(create_entity_in_partition(
                    &mut runtime,
                    &format!("rich-cluster-b-{index}"),
                    PartitionId((index % 3) as u32 + 5),
                ));
            }
            for index in 0..(cluster_a.len() - 1) {
                create_relation_in_partition(
                    &mut runtime,
                    cluster_a[index],
                    cluster_a[index + 1],
                    &format!("rich-a-link-{index}"),
                    PartitionId(11),
                );
                create_relation_in_partition(
                    &mut runtime,
                    cluster_b[index],
                    cluster_b[index + 1],
                    &format!("rich-b-link-{index}"),
                    PartitionId(12),
                );
            }

            runtime.performance_access().reset_counters();
            let bridge_started_at = Instant::now();
            let bridge_outcome = create_relation_outcome(
                &mut runtime,
                cluster_a[2],
                cluster_b[2],
                "bridge-topology-wave-rich",
            );
            let bridge_commit_micros = bridge_started_at.elapsed().as_micros();

            let connectivity_started_at = Instant::now();
            let summary = runtime.inspect_what_happened().connectivity_summary(
                &ConnectivityInspectionRequest {
                    scope: InspectionScope::Current,
                    partition_scope: None,
                    relation_kind_scope: Some(vec![KindId(2)]),
                    include_members: false,
                    budget: ConnectivityInspectionBudget {
                        max_entities: 64,
                        max_relations: 64,
                        max_frontier: 64,
                        max_components: 8,
                        max_work_units: 256,
                    },
                },
            );
            let connectivity_summary_micros = connectivity_started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            PerfMeasurement {
                elapsed_micros: bridge_commit_micros + connectivity_summary_micros,
                metrics: json!({
                    "bridge_commit_micros": bridge_commit_micros,
                    "connectivity_summary_micros": connectivity_summary_micros,
                    "bridge_changed_records": bridge_outcome.changed_records.len(),
                    "component_count": summary.component_count,
                    "largest_component_size": summary.largest_component_size,
                    "enumerated_entity_count": summary.enumerated_entity_count,
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "availability": format!("{:?}", summary.availability),
                    "degradation_count": summary.degradations.len(),
                    "counters": counters,
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "topology_bridge_connectivity_wave_rich_geometry_profile",
        &topology_bridge_rich_geometry_samples,
        &[
            ("bridge_commit_micros", &["bridge_commit_micros"]),
            (
                "connectivity_summary_micros",
                &["connectivity_summary_micros"],
            ),
            ("component_count", &["component_count"]),
            ("largest_component_size", &["largest_component_size"]),
            ("enumerated_entity_count", &["enumerated_entity_count"]),
            ("diagnostic_artifact_count", &["diagnostic_artifact_count"]),
            ("detailed_trace_entries", &["detailed_trace_entries"]),
        ],
    );
    assert!(topology_bridge_rich_geometry_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &topology_bridge_rich_geometry_samples,
        "geometry rich topology bridge should preserve the same connectivity truth while deferring hot detailed traces",
        |metrics| {
            metrics["bridge_changed_records"].as_u64() == Some(1)
                && metrics["component_count"].as_u64() == Some(1)
                && metrics["largest_component_size"].as_u64() == Some(12)
                && metrics["enumerated_entity_count"].as_u64() == Some(12)
                && metrics["availability"].as_str() == Some("Direct".into())
                && metrics["degradation_count"].as_u64() == Some(1)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "relation_slots_touched_by_commit") == 1
                && counter_u64(metrics, "inspection_connectivity_summary_requests") == 1
                && counter_u64(metrics, "inspection_connectivity_entity_scans") == 12
                && counter_u64(metrics, "inspection_connectivity_relation_scans") == 11
        },
    );

    let topology_bridge_zero_diag_samples = capture_perf_samples(
        suite,
        "topology_bridge_connectivity_wave_zero_diagnostics",
        || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
            runtime.config.diagnostics.profile.detailed_traces_enabled = false;
            runtime.config.diagnostics.profile.max_entries_per_artifact = 0;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let mut cluster_a = Vec::new();
            let mut cluster_b = Vec::new();
            for index in 0..6 {
                cluster_a.push(create_entity_in_partition(
                    &mut runtime,
                    &format!("zero-cluster-a-{index}"),
                    PartitionId((index % 3) as u32 + 1),
                ));
                cluster_b.push(create_entity_in_partition(
                    &mut runtime,
                    &format!("zero-cluster-b-{index}"),
                    PartitionId((index % 3) as u32 + 5),
                ));
            }
            for index in 0..(cluster_a.len() - 1) {
                create_relation_in_partition(
                    &mut runtime,
                    cluster_a[index],
                    cluster_a[index + 1],
                    &format!("zero-a-link-{index}"),
                    PartitionId(11),
                );
                create_relation_in_partition(
                    &mut runtime,
                    cluster_b[index],
                    cluster_b[index + 1],
                    &format!("zero-b-link-{index}"),
                    PartitionId(12),
                );
            }

            runtime.performance_access().reset_counters();
            let bridge_started_at = Instant::now();
            let bridge_outcome = create_relation_outcome(
                &mut runtime,
                cluster_a[2],
                cluster_b[2],
                "bridge-topology-wave-zero",
            );
            let bridge_commit_micros = bridge_started_at.elapsed().as_micros();

            let connectivity_started_at = Instant::now();
            let summary = runtime.inspect_what_happened().connectivity_summary(
                &ConnectivityInspectionRequest {
                    scope: InspectionScope::Current,
                    partition_scope: None,
                    relation_kind_scope: Some(vec![KindId(2)]),
                    include_members: false,
                    budget: ConnectivityInspectionBudget {
                        max_entities: 64,
                        max_relations: 64,
                        max_frontier: 64,
                        max_components: 8,
                        max_work_units: 256,
                    },
                },
            );
            let connectivity_summary_micros = connectivity_started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            PerfMeasurement {
                elapsed_micros: bridge_commit_micros + connectivity_summary_micros,
                metrics: json!({
                    "bridge_commit_micros": bridge_commit_micros,
                    "connectivity_summary_micros": connectivity_summary_micros,
                    "bridge_changed_records": bridge_outcome.changed_records.len(),
                    "component_count": summary.component_count,
                    "largest_component_size": summary.largest_component_size,
                    "enumerated_entity_count": summary.enumerated_entity_count,
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "availability": format!("{:?}", summary.availability),
                    "degradation_count": summary.degradations.len(),
                    "counters": counters,
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "topology_bridge_connectivity_wave_zero_diagnostics",
        &topology_bridge_zero_diag_samples,
        &[
            ("bridge_commit_micros", &["bridge_commit_micros"]),
            (
                "connectivity_summary_micros",
                &["connectivity_summary_micros"],
            ),
            ("component_count", &["component_count"]),
            ("largest_component_size", &["largest_component_size"]),
            ("enumerated_entity_count", &["enumerated_entity_count"]),
            ("diagnostic_artifact_count", &["diagnostic_artifact_count"]),
            ("detailed_trace_entries", &["detailed_trace_entries"]),
        ],
    );
    assert!(topology_bridge_zero_diag_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &topology_bridge_zero_diag_samples,
        "geometry zero-diagnostics topology bridge should preserve connectivity truth while eliminating trace entries",
        |metrics| {
            metrics["bridge_changed_records"].as_u64() == Some(1)
                && metrics["component_count"].as_u64() == Some(1)
                && metrics["largest_component_size"].as_u64() == Some(12)
                && metrics["enumerated_entity_count"].as_u64() == Some(12)
                && metrics["availability"].as_str() == Some("Direct".into())
                && metrics["degradation_count"].as_u64() == Some(1)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "relation_slots_touched_by_commit") == 1
                && counter_u64(metrics, "inspection_connectivity_summary_requests") == 1
                && counter_u64(metrics, "inspection_connectivity_entity_scans") == 12
                && counter_u64(metrics, "inspection_connectivity_relation_scans") == 11
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_cad_topology_matrix() {
    let suite = "cad_topology_matrix";

    let assembly_bridge_samples =
        capture_perf_samples(suite, "assembly_interface_bridge_wave", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
            let mut nose = Vec::new();
            let mut tank = Vec::new();
            let mut thrust = Vec::new();
            for index in 0..4 {
                nose.push(create_entity_in_partition(
                    &mut runtime,
                    &format!("nose-skin-{index}"),
                    PartitionId((index % 2) as u32 + 1),
                ));
                tank.push(create_entity_in_partition(
                    &mut runtime,
                    &format!("tank-frame-{index}"),
                    PartitionId((index % 2) as u32 + 4),
                ));
                thrust.push(create_entity_in_partition(
                    &mut runtime,
                    &format!("thrust-mount-{index}"),
                    PartitionId((index % 2) as u32 + 7),
                ));
            }
            for index in 0..3 {
                create_relation_in_partition(
                    &mut runtime,
                    nose[index],
                    nose[index + 1],
                    &format!("nose-seam-{index}"),
                    PartitionId(30),
                );
                create_relation_in_partition(
                    &mut runtime,
                    tank[index],
                    tank[index + 1],
                    &format!("tank-bay-{index}"),
                    PartitionId(31),
                );
                create_relation_in_partition(
                    &mut runtime,
                    thrust[index],
                    thrust[index + 1],
                    &format!("thrust-rib-{index}"),
                    PartitionId(32),
                );
            }
            for index in 0..4 {
                create_relation_in_partition(
                    &mut runtime,
                    nose[index],
                    tank[index],
                    &format!("nose-to-tank-{index}"),
                    PartitionId(33),
                );
            }

            runtime.performance_access().reset_counters();
            let bridge_started_at = Instant::now();
            let bridge_outcome = create_relation_outcome(
                &mut runtime,
                tank[2],
                thrust[1],
                "tank-to-thrust-interface",
            );
            let bridge_commit_micros = bridge_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_targets = vec![
                RecordRef::Entity(nose[1]),
                RecordRef::Entity(nose[2]),
                RecordRef::Entity(tank[1]),
                RecordRef::Entity(tank[2]),
                RecordRef::Entity(thrust[1]),
                RecordRef::Entity(thrust[2]),
            ];
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "cad-assembly-explicit",
                explicit_targets,
            );
            let explicit_started_at = Instant::now();
            let explicit_outcome = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("planned cad explicit packet"),
                )
                .expect("cad explicit query outcome");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();

            let connectivity_started_at = Instant::now();
            let summary = runtime.inspect_what_happened().connectivity_summary(
                &ConnectivityInspectionRequest {
                    scope: InspectionScope::Current,
                    partition_scope: None,
                    relation_kind_scope: Some(vec![KindId(2)]),
                    include_members: false,
                    budget: ConnectivityInspectionBudget {
                        max_entities: 64,
                        max_relations: 64,
                        max_frontier: 64,
                        max_components: 8,
                        max_work_units: 256,
                    },
                },
            );
            let connectivity_summary_micros = connectivity_started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros: bridge_commit_micros
                    + explicit_query_micros
                    + connectivity_summary_micros,
                metrics: json!({
                    "bridge_commit_micros": bridge_commit_micros,
                    "explicit_query_micros": explicit_query_micros,
                    "connectivity_summary_micros": connectivity_summary_micros,
                    "bridge_changed_records": bridge_outcome.changed_records.len(),
                    "explicit_query_entities": explicit_outcome.complexity.entity_records_emitted,
                    "component_count": summary.component_count,
                    "largest_component_size": summary.largest_component_size,
                    "enumerated_entity_count": summary.enumerated_entity_count,
                    "availability": format!("{:?}", summary.availability),
                    "counters": counters,
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "assembly_interface_bridge_wave",
        &assembly_bridge_samples,
        &[
            ("bridge_commit_micros", &["bridge_commit_micros"]),
            ("explicit_query_micros", &["explicit_query_micros"]),
            (
                "connectivity_summary_micros",
                &["connectivity_summary_micros"],
            ),
            ("explicit_query_entities", &["explicit_query_entities"]),
            ("largest_component_size", &["largest_component_size"]),
        ],
    );
    assert!(assembly_bridge_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &assembly_bridge_samples,
        "cad assembly interface bridges should preserve bounded connectivity and local explicit read surfaces",
        |metrics| {
            metrics["bridge_changed_records"].as_u64() == Some(1)
                && metrics["explicit_query_entities"].as_u64() == Some(6)
                && metrics["component_count"].as_u64() == Some(1)
                && metrics["largest_component_size"].as_u64() == Some(12)
                && metrics["enumerated_entity_count"].as_u64() == Some(12)
                && metrics["availability"].as_str() == Some("Direct".into())
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "inspection_connectivity_summary_requests") == 1
                && counter_u64(metrics, "query_packet_count") <= 6
                && counter_u64(metrics, "query_scope_unit_count") <= 6
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_chip_simulator_matrix() {
    let suite = "chip_simulator_matrix";

    let fanout_compile_samples = capture_perf_samples(suite, "dense_fanout_compile_wave", || {
        let mut runtime =
            runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
        let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
        let source = create_entity_in_partition(&mut runtime, "net-driver", PartitionId(7));
        let targets = (0..24)
            .map(|index| {
                let partition_id = match index % 4 {
                    0 => PartitionId(11),
                    1 => PartitionId(13),
                    2 => PartitionId(17),
                    _ => PartitionId(19),
                };
                create_entity_in_partition(&mut runtime, &format!("net-sink-{index}"), partition_id)
            })
            .collect::<Vec<_>>();

        runtime.performance_access().reset_counters();
        let commit_started_at = Instant::now();
        let commit_outcome = {
            let mut txn = runtime.begin_transaction(TransactionOptions::default());
            let mut batch = WorkerIntentBatch::new("chip-fanout-wave");
            for (index, target) in targets.iter().enumerate() {
                batch = batch.push(MutationIntent::Create(CreateIntent::Relation(
                    crate::transactions::data::RelationSpec {
                        partition_id: PartitionId(29),
                        kind_id: KindId(2),
                        client_key: crate::symbols::data::ClientKey::raw(format!(
                            "chip-fanout-{index}"
                        )),
                        source: crate::transactions::data::EntityReference::Existing(source),
                        target: crate::transactions::data::EntityReference::Existing(*target),
                        fields: crate::transactions::data::AspectFieldPatch::default(),
                    },
                )));
            }
            txn.push_batch(batch);
            txn.commit().expect("chip fanout relation burst commit")
        };
        let commit_micros = commit_started_at.elapsed().as_micros();
        let commit = runtime
            .history()
            .latest_commit()
            .expect("chip fanout commit")
            .clone();

        let compile_started_at = Instant::now();
        let artifact = runtime
            .compiled_artifacts_authority()
            .compile_execution_artifact(
                commit.commit_id,
                vec![
                    PartitionId(7),
                    PartitionId(11),
                    PartitionId(13),
                    PartitionId(17),
                    PartitionId(19),
                    PartitionId(29),
                ],
            )
            .expect("chip fanout compiled artifact");
        let compile_micros = compile_started_at.elapsed().as_micros();

        let adjacency_started_at = Instant::now();
        let outgoing_relations = runtime
            .storage_access()
            .outgoing_relations_for_entity(source, commit.version_id);
        let adjacency_micros = adjacency_started_at.elapsed().as_micros();

        let counters = runtime.performance_access().counters();
        let (diagnostic_artifact_count, detailed_trace_entries) =
            fresh_diagnostics_metrics(&runtime, diagnostics_start);

        PerfMeasurement {
            elapsed_micros: commit_micros + compile_micros + adjacency_micros,
            metrics: json!({
                "commit_micros": commit_micros,
                "compile_micros": compile_micros,
                "adjacency_micros": adjacency_micros,
                "changed_records": commit_outcome.changed_records.len(),
                "dense_patch_record_count": dense_patch_record_count(&runtime),
                "outgoing_relation_count": outgoing_relations.len(),
                "diagnostic_artifact_count": diagnostic_artifact_count,
                "detailed_trace_entries": detailed_trace_entries,
                "profile_boundary": profile_boundary_metrics(
                    &runtime,
                    RelationalRuntimeProfile::ChipSimulation,
                ),
                "adjacency_backend": format!("{:?}", runtime.config().storage.adjacency_policy.backend),
                "compiled_artifact_compatibility": format!(
                    "{:?}",
                    runtime
                        .compiled_artifacts()
                        .compiled_artifact_compatibility(artifact.artifact_id)
                ),
                "counters": counters,
            }),
        }
    });
    emit_metric_summaries(
        suite,
        "dense_fanout_compile_wave",
        &fanout_compile_samples,
        &[
            ("commit_micros", &["commit_micros"]),
            ("compile_micros", &["compile_micros"]),
            ("adjacency_micros", &["adjacency_micros"]),
            ("changed_records", &["changed_records"]),
            ("dense_patch_record_count", &["dense_patch_record_count"]),
            ("outgoing_relation_count", &["outgoing_relation_count"]),
            ("diagnostic_artifact_count", &["diagnostic_artifact_count"]),
            ("detailed_trace_entries", &["detailed_trace_entries"]),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(fanout_compile_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &fanout_compile_samples,
        "chip fanout compile wave should preserve compressed adjacency truth and dense patch shape",
        |metrics| {
            metrics["changed_records"].as_u64() == Some(24)
                && metrics["dense_patch_record_count"].as_u64() == Some(24)
                && metrics["outgoing_relation_count"].as_u64() == Some(24)
                && metrics["adjacency_backend"].as_str()
                    == Some(&format!(
                        "{:?}",
                        AdjacencyBackend::CompressedFanoutAdjacency
                    ))
                && metrics["compiled_artifact_compatibility"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactCompatibility::Compatible))
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(1)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(1)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "relation_slots_touched_by_commit") == 24
        },
    );

    let rich_fanout_compile_samples = capture_perf_samples(
        suite,
        "dense_fanout_compile_wave_rich_diagnostics",
        || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::ChipRichCertification,
            );
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let source =
                create_entity_in_partition(&mut runtime, "rich-net-driver", PartitionId(7));
            let targets = (0..24)
                .map(|index| {
                    let partition_id = match index % 4 {
                        0 => PartitionId(11),
                        1 => PartitionId(13),
                        2 => PartitionId(17),
                        _ => PartitionId(19),
                    };
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("rich-net-sink-{index}"),
                        partition_id,
                    )
                })
                .collect::<Vec<_>>();

            runtime.performance_access().reset_counters();
            let commit_started_at = Instant::now();
            let commit_outcome = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                let mut batch = WorkerIntentBatch::new("chip-fanout-wave-rich");
                for (index, target) in targets.iter().enumerate() {
                    batch = batch.push(MutationIntent::Create(CreateIntent::Relation(
                        crate::transactions::data::RelationSpec {
                            partition_id: PartitionId(29),
                            kind_id: KindId(2),
                            client_key: crate::symbols::data::ClientKey::raw(format!(
                                "chip-fanout-rich-{index}"
                            )),
                            source: crate::transactions::data::EntityReference::Existing(source),
                            target: crate::transactions::data::EntityReference::Existing(*target),
                            fields: crate::transactions::data::AspectFieldPatch::default(),
                        },
                    )));
                }
                txn.push_batch(batch);
                txn.commit()
                    .expect("chip fanout relation burst commit with rich diagnostics")
            };
            let commit_micros = commit_started_at.elapsed().as_micros();
            let commit = runtime
                .history()
                .latest_commit()
                .expect("chip fanout rich commit")
                .clone();

            let compile_started_at = Instant::now();
            let artifact = runtime
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(13),
                        PartitionId(17),
                        PartitionId(19),
                        PartitionId(29),
                    ],
                )
                .expect("chip fanout rich compiled artifact");
            let compile_micros = compile_started_at.elapsed().as_micros();

            let adjacency_started_at = Instant::now();
            let outgoing_relations = runtime
                .storage_access()
                .outgoing_relations_for_entity(source, commit.version_id);
            let adjacency_micros = adjacency_started_at.elapsed().as_micros();

            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            PerfMeasurement {
                elapsed_micros: commit_micros + compile_micros + adjacency_micros,
                metrics: json!({
                    "commit_micros": commit_micros,
                    "compile_micros": compile_micros,
                    "adjacency_micros": adjacency_micros,
                    "changed_records": commit_outcome.changed_records.len(),
                    "dense_patch_record_count": dense_patch_record_count(&runtime),
                    "outgoing_relation_count": outgoing_relations.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "profile_boundary": profile_boundary_metrics(
                        &runtime,
                        RelationalRuntimeProfile::ChipSimulation,
                    ),
                    "adjacency_backend": format!("{:?}", runtime.config().storage.adjacency_policy.backend),
                    "compiled_artifact_compatibility": format!(
                        "{:?}",
                        runtime
                            .compiled_artifacts()
                            .compiled_artifact_compatibility(artifact.artifact_id)
                    ),
                    "counters": counters,
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "dense_fanout_compile_wave_rich_diagnostics",
        &rich_fanout_compile_samples,
        &[
            ("commit_micros", &["commit_micros"]),
            ("compile_micros", &["compile_micros"]),
            ("adjacency_micros", &["adjacency_micros"]),
            ("changed_records", &["changed_records"]),
            ("dense_patch_record_count", &["dense_patch_record_count"]),
            ("outgoing_relation_count", &["outgoing_relation_count"]),
            ("diagnostic_artifact_count", &["diagnostic_artifact_count"]),
            ("detailed_trace_entries", &["detailed_trace_entries"]),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(rich_fanout_compile_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &rich_fanout_compile_samples,
        "chip rich fanout compile wave should preserve the same dense truth while surfacing trace cost",
        |metrics| {
            metrics["changed_records"].as_u64() == Some(24)
                && metrics["dense_patch_record_count"].as_u64() == Some(24)
                && metrics["outgoing_relation_count"].as_u64() == Some(24)
                && metrics["adjacency_backend"].as_str()
                    == Some(&format!("{:?}", AdjacencyBackend::CompressedFanoutAdjacency))
                && metrics["compiled_artifact_compatibility"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactCompatibility::Compatible))
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64().unwrap_or(0) >= 1
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(1)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(1)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "relation_slots_touched_by_commit") == 24
        },
    );

    let checkpoint_recover_compile_samples = capture_perf_samples(
        suite,
        "checkpoint_window_recover_compile_round_trip",
        || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            let source =
                create_entity_in_partition(&mut runtime, "persisted-driver", PartitionId(7));
            let targets = (0..12)
                .map(|index| {
                    let partition_id = match index % 3 {
                        0 => PartitionId(11),
                        1 => PartitionId(13),
                        _ => PartitionId(17),
                    };
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("persisted-sink-{index}"),
                        partition_id,
                    )
                })
                .collect::<Vec<_>>();
            for (index, target) in targets.iter().enumerate() {
                create_relation_in_partition(
                    &mut runtime,
                    source,
                    *target,
                    &format!("persisted-edge-{index}"),
                    PartitionId(29),
                );
            }

            let checkpoint_started_at = Instant::now();
            runtime
                .durability_authority()
                .checkpoint()
                .expect("chip checkpoint window");
            let checkpoint_micros = checkpoint_started_at.elapsed().as_micros();

            let plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            let recover_started_at = Instant::now();
            recovered
                .durability_authority()
                .recover(plan)
                .expect("chip checkpoint recovery");
            let recover_micros = recover_started_at.elapsed().as_micros();

            let recovered_commit = recovered
                .history()
                .latest_commit()
                .expect("recovered chip commit")
                .clone();
            let compile_started_at = Instant::now();
            let artifact = recovered
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    recovered_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(13),
                        PartitionId(17),
                        PartitionId(29),
                    ],
                )
                .expect("recovered chip compiled artifact");
            let compile_micros = compile_started_at.elapsed().as_micros();

            let adjacency_started_at = Instant::now();
            let outgoing_relations = recovered
                .storage_access()
                .outgoing_relations_for_entity(source, recovered_commit.version_id);
            let adjacency_micros = adjacency_started_at.elapsed().as_micros();

            PerfMeasurement {
                elapsed_micros: checkpoint_micros
                    + recover_micros
                    + compile_micros
                    + adjacency_micros,
                metrics: json!({
                    "checkpoint_micros": checkpoint_micros,
                    "recover_micros": recover_micros,
                    "compile_micros": compile_micros,
                    "adjacency_micros": adjacency_micros,
                    "recovered_segment_count": recovered
                        .durability()
                        .durable_log()
                        .len(),
                    "outgoing_relation_count": outgoing_relations.len(),
                    "compiled_artifact_compatibility": format!(
                        "{:?}",
                        recovered
                            .compiled_artifacts()
                            .compiled_artifact_compatibility(artifact.artifact_id)
                    ),
                    "counters": recovered.performance_access().counters(),
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "checkpoint_window_recover_compile_round_trip",
        &checkpoint_recover_compile_samples,
        &[
            ("checkpoint_micros", &["checkpoint_micros"]),
            ("recover_micros", &["recover_micros"]),
            ("compile_micros", &["compile_micros"]),
            ("adjacency_micros", &["adjacency_micros"]),
            ("recovered_segment_count", &["recovered_segment_count"]),
            ("outgoing_relation_count", &["outgoing_relation_count"]),
        ],
    );
    assert!(checkpoint_recover_compile_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &checkpoint_recover_compile_samples,
        "chip checkpoint window recovery should preserve compile-ready fanout truth after recovery",
        |metrics| {
            metrics["checkpoint_micros"].as_u64().unwrap_or(0) > 0
                && metrics["recover_micros"].as_u64().unwrap_or(0) > 0
                && metrics["outgoing_relation_count"].as_u64() == Some(12)
                && metrics["compiled_artifact_compatibility"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactCompatibility::Compatible))
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );

    let branch_rollback_compile_samples =
        capture_perf_samples(suite, "branch_rollback_compile_step_window", || {
            let feature_branch = BranchId("feature".to_string());
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let source =
                create_entity_in_partition(&mut runtime, "rollback-driver", PartitionId(7));
            let stable_targets = (0..8)
                .map(|index| {
                    let partition_id = match index % 4 {
                        0 => PartitionId(11),
                        1 => PartitionId(13),
                        2 => PartitionId(17),
                        _ => PartitionId(19),
                    };
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("rollback-stable-sink-{index}"),
                        partition_id,
                    )
                })
                .collect::<Vec<_>>();
            let transient_targets = (0..8)
                .map(|index| {
                    let partition_id = match index % 4 {
                        0 => PartitionId(23),
                        1 => PartitionId(31),
                        2 => PartitionId(37),
                        _ => PartitionId(41),
                    };
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("rollback-transient-sink-{index}"),
                        partition_id,
                    )
                })
                .collect::<Vec<_>>();
            create_branch_from_main(&mut runtime, "feature");
            for (index, target) in stable_targets.iter().enumerate() {
                create_relation_in_partition_on_branch(
                    &mut runtime,
                    source,
                    *target,
                    &format!("rollback-stable-edge-{index}"),
                    "stable",
                    PartitionId(29),
                    feature_branch.clone(),
                );
            }

            runtime.performance_access().reset_counters();
            let mut txn = runtime.begin_transaction(TransactionOptions {
                target_branch: Some(feature_branch.clone()),
                ..TransactionOptions::default()
            });
            let savepoint = txn.create_savepoint();
            let mut transient_batch = WorkerIntentBatch::new("chip-transient-fanout");
            for (index, target) in transient_targets.iter().enumerate() {
                transient_batch = transient_batch.push(MutationIntent::Create(
                    CreateIntent::Relation(crate::transactions::data::RelationSpec {
                        partition_id: PartitionId(43),
                        kind_id: KindId(2),
                        client_key: crate::symbols::data::ClientKey::raw(format!(
                            "rollback-transient-edge-{index}"
                        )),
                        source: crate::transactions::data::EntityReference::Existing(source),
                        target: crate::transactions::data::EntityReference::Existing(*target),
                        fields: crate::transactions::data::AspectFieldPatch::default(),
                    }),
                ));
            }
            txn.push_batch(transient_batch);

            let rollback_started_at = Instant::now();
            let rollback = txn
                .rollback_to_savepoint(savepoint)
                .expect("chip savepoint rollback");
            let rollback_micros = rollback_started_at.elapsed().as_micros();

            txn.push_batch(
                WorkerIntentBatch::new("chip-committed-step").push(
                    MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                        UpdateEntityFieldsIntent {
                            entity_id: source,
                            fields: crate::tests::support::aspect_field_patch_from_values([
                                (
                                    "name",
                                    crate::tests::support::string_aspect_value("rollback-driver"),
                                ),
                                ("step", crate::tests::support::u64_aspect_value(1)),
                                (
                                    "branch",
                                    crate::tests::support::string_aspect_value("feature"),
                                ),
                            ]),
                        },
                    ))
                    .into(),
                ),
            );
            let commit_started_at = Instant::now();
            let commit_outcome = txn.commit().expect("chip branch step commit");
            let commit_micros = commit_started_at.elapsed().as_micros();

            let feature_commit = runtime
                .history()
                .branch_head(&feature_branch)
                .expect("feature branch head")
                .clone();
            let compile_started_at = Instant::now();
            let artifact = runtime
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    feature_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(13),
                        PartitionId(17),
                        PartitionId(19),
                        PartitionId(29),
                    ],
                )
                .expect("feature branch compiled artifact");
            let compile_micros = compile_started_at.elapsed().as_micros();

            let adjacency_started_at = Instant::now();
            let outgoing_relations = runtime
                .storage_access()
                .outgoing_relations_for_entity(source, feature_commit.version_id);
            let adjacency_micros = adjacency_started_at.elapsed().as_micros();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            PerfMeasurement {
                elapsed_micros: rollback_micros + commit_micros + compile_micros + adjacency_micros,
                metrics: json!({
                    "rollback_micros": rollback_micros,
                    "commit_micros": commit_micros,
                    "compile_micros": compile_micros,
                    "adjacency_micros": adjacency_micros,
                    "rollback_effect_count": rollback.effect_count(),
                    "rollback_discarded_creations": rollback.summary.discarded_creation_count(),
                    "rollback_restored_records": rollback.summary.restored_record_count(),
                    "committed_changed_records": commit_outcome.changed_records.len(),
                    "outgoing_relation_count": outgoing_relations.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "compiled_artifact_compatibility": format!(
                        "{:?}",
                        runtime
                            .compiled_artifacts()
                            .compiled_artifact_compatibility(artifact.artifact_id)
                    ),
                    "counters": runtime.performance_access().counters(),
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "branch_rollback_compile_step_window",
        &branch_rollback_compile_samples,
        &[
            ("rollback_micros", &["rollback_micros"]),
            ("commit_micros", &["commit_micros"]),
            ("compile_micros", &["compile_micros"]),
            ("adjacency_micros", &["adjacency_micros"]),
            ("rollback_effect_count", &["rollback_effect_count"]),
            (
                "rollback_discarded_creations",
                &["rollback_discarded_creations"],
            ),
            ("committed_changed_records", &["committed_changed_records"]),
            ("outgoing_relation_count", &["outgoing_relation_count"]),
        ],
    );
    assert!(branch_rollback_compile_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &branch_rollback_compile_samples,
        "chip branch rollback compile windows should discard abandoned fanout work and keep feature truth narrow",
        |metrics| {
            metrics["rollback_effect_count"].as_u64() == Some(8)
                && metrics["rollback_discarded_creations"].as_u64() == Some(8)
                && metrics["rollback_restored_records"].as_u64() == Some(0)
                && metrics["committed_changed_records"].as_u64() == Some(1)
                && metrics["outgoing_relation_count"].as_u64() == Some(8)
                && metrics["compiled_artifact_compatibility"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactCompatibility::Compatible))
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );

    let flat_step_batch_samples = capture_perf_samples(
        suite,
        "flat_entity_step_batch_compile_window",
        || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
            runtime.config.diagnostics.profile.detailed_traces_enabled = false;
            runtime.config.diagnostics.profile.max_entries_per_artifact = 0;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();

            let _source =
                create_entity_in_partition(&mut runtime, "chip-batch-driver", PartitionId(7));
            let mut partition_targets = BTreeMap::new();
            for partition_offset in 0..8u32 {
                let partition_id = PartitionId(11 + partition_offset * 2);
                let targets = (0..8)
                    .map(|index| {
                        create_entity_in_partition(
                            &mut runtime,
                            &format!("chip-batch-sink-{}-{index}", partition_id.0),
                            partition_id,
                        )
                    })
                    .collect::<Vec<_>>();
                partition_targets.insert(partition_id, targets);
            }
            let compile_partitions = std::iter::once(PartitionId(7))
                .chain(partition_targets.keys().copied())
                .collect::<Vec<_>>();

            runtime.performance_access().reset_counters();
            let update_started_at = Instant::now();
            let update = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                let mut batch = WorkerIntentBatch::new("chip-flat-entity-step-batch");
                for (partition_id, targets) in &partition_targets {
                    for (index, entity) in targets.iter().enumerate().take(4) {
                        batch = batch.push(MutationIntent::Entity(
                            EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                                entity_id: *entity,
                                fields: crate::tests::support::aspect_field_patch_from_values([
                                    (
                                        "partition",
                                        crate::tests::support::u64_aspect_value(
                                            partition_id.0 as u64,
                                        ),
                                    ),
                                    (
                                        "lane",
                                        crate::tests::support::string_aspect_value("global-step"),
                                    ),
                                    ("step", crate::tests::support::usize_aspect_value(index)),
                                ]),
                            }),
                        ));
                    }
                }
                txn.push_batch(batch);
                txn.commit().expect("chip flat entity step batch commit")
            };
            let update_micros = update_started_at.elapsed().as_micros();
            let phase_timing = update.execution.phase_timing.clone();
            let commit = runtime
                .history()
                .latest_commit()
                .expect("chip flat batch latest commit")
                .clone();

            let compile_started_at = Instant::now();
            let artifact = runtime
                .compiled_artifacts_authority()
                .compile_execution_artifact(commit.commit_id, compile_partitions)
                .expect("chip flat batch compiled artifact");
            let compile_micros = compile_started_at.elapsed().as_micros();

            let sample_targets = partition_targets
                .values()
                .flat_map(|targets| targets.iter().take(1).copied())
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "chip-flat-batch-explicit",
                sample_targets,
            );
            let explicit_started_at = Instant::now();
            let explicit = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("planned chip flat batch explicit query"),
                )
                .expect("chip flat batch explicit outcome");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();
            assert!(runtime.visibility_authority().release_snapshot(&snapshot));

            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            measurement_with_elapsed(
                update_micros + compile_micros + explicit_query_micros,
                || {
                    json!({
                        "batch_target_count": 32,
                        "batch_partition_count": partition_targets.len(),
                        "update_micros": update_micros,
                        "compile_micros": compile_micros,
                        "explicit_query_micros": explicit_query_micros,
                        "hot_changed_records": update.changed_records.len(),
                        "explicit_result_entities": explicit.result.entities.len(),
                        "diagnostic_artifact_count": diagnostic_artifact_count,
                        "detailed_trace_entries": detailed_trace_entries,
                        "compiled_artifact_compatibility": format!(
                            "{:?}",
                            runtime
                                .compiled_artifacts()
                                .compiled_artifact_compatibility(artifact.artifact_id)
                        ),
                        "phase_timing": {
                            "draft_preparation_micros": phase_timing.draft_preparation_micros,
                            "draft_working_state_clone_micros": phase_timing.draft_working_state_clone_micros,
                            "publication_storage_commit_micros": phase_timing.publication_storage_commit_micros,
                        },
                        "counters": counters,
                    })
                },
            )
        },
    );
    emit_metric_summaries(
        suite,
        "flat_entity_step_batch_compile_window",
        &flat_step_batch_samples,
        &[
            ("batch_target_count", &["batch_target_count"]),
            ("batch_partition_count", &["batch_partition_count"]),
            ("update_micros", &["update_micros"]),
            ("compile_micros", &["compile_micros"]),
            ("explicit_query_micros", &["explicit_query_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "draft_working_state_clone_micros",
                &["phase_timing", "draft_working_state_clone_micros"],
            ),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
            (
                "aosoa_entity_chunk_slots_materialized",
                &["counters", "aosoa_entity_chunk_slots_materialized"],
            ),
            (
                "aosoa_entity_chunks_published",
                &["counters", "aosoa_entity_chunks_published"],
            ),
        ],
    );
    assert!(flat_step_batch_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &flat_step_batch_samples,
        "chip flat entity step batches should stay on the widened sparse AoSoA path while remaining compile-ready",
        |metrics| {
            metrics["batch_target_count"].as_u64() == Some(32)
                && metrics["batch_partition_count"].as_u64() == Some(8)
                && metrics["hot_changed_records"].as_u64() == Some(32)
                && metrics["explicit_result_entities"].as_u64() == Some(8)
                && metrics["compiled_artifact_compatibility"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactCompatibility::Compatible))
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == 32
                && counter_u64(metrics, "partitions_touched_by_commit") >= 8
                && counter_u64(metrics, "aosoa_entity_chunk_slots_materialized") == 32
                && counter_u64(metrics, "aosoa_entity_chunks_published") >= 8
                && counter_u64(metrics, "aosoa_publish_fallback_count") == 0
        },
    );

    let event_wave_churn_samples =
        capture_perf_samples(suite, "event_wave_compile_churn_window", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
            runtime.config.diagnostics.profile.detailed_traces_enabled = false;
            runtime.config.diagnostics.profile.max_entries_per_artifact = 0;
            let source = create_entity_in_partition(&mut runtime, "event-driver", PartitionId(7));
            let sinks = (0..16)
                .map(|index| {
                    let partition_id = match index % 4 {
                        0 => PartitionId(11),
                        1 => PartitionId(13),
                        2 => PartitionId(17),
                        _ => PartitionId(19),
                    };
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("event-sink-{index}"),
                        partition_id,
                    )
                })
                .collect::<Vec<_>>();
            for (index, sink) in sinks.iter().enumerate() {
                create_relation_in_partition(
                    &mut runtime,
                    source,
                    *sink,
                    &format!("event-link-{index}"),
                    PartitionId(29),
                );
            }

            const ITERATIONS: usize = 24;
            let mut total_update_micros = 0u128;
            let mut total_compile_micros = 0u128;
            let mut total_adjacency_micros = 0u128;
            let mut max_compile_micros = 0u128;
            let mut max_outgoing_relation_count = 0usize;

            runtime.performance_access().reset_counters();
            for step in 0..ITERATIONS {
                let update_started_at = Instant::now();
                let _ = update_entity(&mut runtime, source, &format!("event-driver-step-{step}"));
                total_update_micros += update_started_at.elapsed().as_micros();

                let commit = runtime
                    .history()
                    .latest_commit()
                    .expect("chip event-wave commit")
                    .clone();
                let compile_started_at = Instant::now();
                let artifact = runtime
                    .compiled_artifacts_authority()
                    .compile_execution_artifact(
                        commit.commit_id,
                        vec![
                            PartitionId(7),
                            PartitionId(11),
                            PartitionId(13),
                            PartitionId(17),
                            PartitionId(19),
                            PartitionId(29),
                        ],
                    )
                    .expect("chip event-wave compiled artifact");
                let compile_micros = compile_started_at.elapsed().as_micros();
                total_compile_micros += compile_micros;
                max_compile_micros = max_compile_micros.max(compile_micros);

                let adjacency_started_at = Instant::now();
                let outgoing_relations = runtime
                    .storage_access()
                    .outgoing_relations_for_entity(source, commit.version_id);
                total_adjacency_micros += adjacency_started_at.elapsed().as_micros();
                max_outgoing_relation_count =
                    max_outgoing_relation_count.max(outgoing_relations.len());
                assert_eq!(
                    runtime
                        .compiled_artifacts()
                        .compiled_artifact_compatibility(artifact.artifact_id),
                    CompiledArtifactCompatibility::Compatible
                );
            }

            PerfMeasurement {
                elapsed_micros: total_update_micros + total_compile_micros + total_adjacency_micros,
                metrics: json!({
                    "iterations": ITERATIONS,
                    "average_update_micros": total_update_micros / ITERATIONS as u128,
                    "average_compile_micros": total_compile_micros / ITERATIONS as u128,
                    "average_adjacency_micros": total_adjacency_micros / ITERATIONS as u128,
                    "max_compile_micros": max_compile_micros,
                    "max_outgoing_relation_count": max_outgoing_relation_count,
                    "counters": runtime.performance_access().counters(),
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "event_wave_compile_churn_window",
        &event_wave_churn_samples,
        &[
            ("average_update_micros", &["average_update_micros"]),
            ("average_compile_micros", &["average_compile_micros"]),
            ("average_adjacency_micros", &["average_adjacency_micros"]),
            ("max_compile_micros", &["max_compile_micros"]),
            (
                "max_outgoing_relation_count",
                &["max_outgoing_relation_count"],
            ),
        ],
    );
    assert!(event_wave_churn_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &event_wave_churn_samples,
        "chip event-wave churn should keep repeated compile windows compatible and bounded under sustained stepping",
        |metrics| {
            metrics["iterations"].as_u64() == Some(24)
                && metrics["max_outgoing_relation_count"].as_u64() == Some(16)
                && metrics["max_compile_micros"].as_u64().unwrap_or(0) > 0
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "bulk_mutation_entity_target_count") == 24
        },
    );

    let event_wave_rich_diagnostics_samples =
        capture_perf_samples(suite, "event_wave_compile_churn_rich_diagnostics", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let source =
                create_entity_in_partition(&mut runtime, "event-driver-rich", PartitionId(7));
            let sinks = (0..16)
                .map(|index| {
                    let partition_id = match index % 4 {
                        0 => PartitionId(11),
                        1 => PartitionId(13),
                        2 => PartitionId(17),
                        _ => PartitionId(19),
                    };
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("event-sink-rich-{index}"),
                        partition_id,
                    )
                })
                .collect::<Vec<_>>();
            for (index, sink) in sinks.iter().enumerate() {
                create_relation_in_partition(
                    &mut runtime,
                    source,
                    *sink,
                    &format!("event-link-rich-{index}"),
                    PartitionId(29),
                );
            }

            const ITERATIONS: usize = 16;
            let mut total_update_micros = 0u128;
            let mut total_compile_micros = 0u128;
            let mut total_adjacency_micros = 0u128;
            let mut max_compile_micros = 0u128;
            let mut max_outgoing_relation_count = 0usize;

            runtime.performance_access().reset_counters();
            for step in 0..ITERATIONS {
                let update_started_at = Instant::now();
                let _ = update_entity(
                    &mut runtime,
                    source,
                    &format!("event-driver-rich-step-{step}"),
                );
                total_update_micros += update_started_at.elapsed().as_micros();

                let commit = runtime
                    .history()
                    .latest_commit()
                    .expect("chip event-wave rich commit")
                    .clone();
                let compile_started_at = Instant::now();
                let artifact = runtime
                    .compiled_artifacts_authority()
                    .compile_execution_artifact(
                        commit.commit_id,
                        vec![
                            PartitionId(7),
                            PartitionId(11),
                            PartitionId(13),
                            PartitionId(17),
                            PartitionId(19),
                            PartitionId(29),
                        ],
                    )
                    .expect("chip event-wave rich compiled artifact");
                let compile_micros = compile_started_at.elapsed().as_micros();
                total_compile_micros += compile_micros;
                max_compile_micros = max_compile_micros.max(compile_micros);

                let adjacency_started_at = Instant::now();
                let outgoing_relations = runtime
                    .storage_access()
                    .outgoing_relations_for_entity(source, commit.version_id);
                total_adjacency_micros += adjacency_started_at.elapsed().as_micros();
                max_outgoing_relation_count =
                    max_outgoing_relation_count.max(outgoing_relations.len());
                assert_eq!(
                    runtime
                        .compiled_artifacts()
                        .compiled_artifact_compatibility(artifact.artifact_id),
                    CompiledArtifactCompatibility::Compatible
                );
            }

            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);
            PerfMeasurement {
                elapsed_micros: total_update_micros + total_compile_micros + total_adjacency_micros,
                metrics: json!({
                    "iterations": ITERATIONS,
                    "average_update_micros": total_update_micros / ITERATIONS as u128,
                    "average_compile_micros": total_compile_micros / ITERATIONS as u128,
                    "average_adjacency_micros": total_adjacency_micros / ITERATIONS as u128,
                    "max_compile_micros": max_compile_micros,
                    "max_outgoing_relation_count": max_outgoing_relation_count,
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "counters": runtime.performance_access().counters(),
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "event_wave_compile_churn_rich_diagnostics",
        &event_wave_rich_diagnostics_samples,
        &[
            ("average_update_micros", &["average_update_micros"]),
            ("average_compile_micros", &["average_compile_micros"]),
            ("average_adjacency_micros", &["average_adjacency_micros"]),
            ("max_compile_micros", &["max_compile_micros"]),
            (
                "max_outgoing_relation_count",
                &["max_outgoing_relation_count"],
            ),
            ("diagnostic_artifact_count", &["diagnostic_artifact_count"]),
            ("detailed_trace_entries", &["detailed_trace_entries"]),
        ],
    );
    assert!(event_wave_rich_diagnostics_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &event_wave_rich_diagnostics_samples,
        "chip event-wave rich diagnostics should keep compile windows compatible while surfacing diagnostic cost clearly",
        |metrics| {
            metrics["iterations"].as_u64() == Some(16)
                && metrics["max_outgoing_relation_count"].as_u64() == Some(16)
                && metrics["max_compile_micros"].as_u64().unwrap_or(0) > 0
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 16
                && metrics["detailed_trace_entries"].as_u64().is_some()
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "bulk_mutation_entity_target_count") == 16
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_rocketship_scale_matrix() {
    let suite = "rocketship_scale_matrix";
    let node_count = rocketship_node_count();
    let query_target_count = rocketship_query_target_count(node_count);

    let zero_diagnostics_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_zero_diagnostics_narrow_round_trip",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            runtime.config.diagnostics.profile.detailed_traces_enabled = false;
            runtime.config.diagnostics.profile.max_entries_per_artifact = 0;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded = seed_rocketship_world(&mut runtime, node_count);

            runtime.performance_access().reset_counters();
            let target_index = seeded.entities.len() / 2;
            let hot_update_started_at = Instant::now();
            let update = update_entity(
                &mut runtime,
                seeded.entities[target_index],
                "rocket-node-hot-update",
            );
            let hot_update_micros = hot_update_started_at.elapsed().as_micros();
            let hot_phase_timing = update.execution.phase_timing.clone();
            let snapshot = runtime.visibility_authority().snapshot();
            let half_window = query_target_count / 2;
            let window_start = target_index.saturating_sub(half_window);
            let window_end = (window_start + query_target_count).min(seeded.entities.len());
            let targets = seeded.entities[window_start..window_end]
                .iter()
                .copied()
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let packet = explicit_query_packet(&runtime, &snapshot, "rocketship-explicit", targets);

            let hot_query_plan_started_at = Instant::now();
            let planned = runtime
                .read_truth()
                .plan_query_packet(&snapshot, packet)
                .expect("planned rocketship explicit query");
            let hot_query_planning_micros = hot_query_plan_started_at.elapsed().as_micros();
            let hot_query_execution_started_at = Instant::now();
            let outcome = runtime
                .read_truth()
                .execute_query_plan(planned)
                .expect("rocketship explicit query outcome");
            let hot_query_execution_micros = hot_query_execution_started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            let elapsed_micros = seeded.entity_commit_micros
                + seeded.relation_commit_micros
                + hot_update_micros
                + hot_query_planning_micros
                + hot_query_execution_micros;
            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "bootstrap_entity_commit_micros": seeded.entity_commit_micros,
                    "bootstrap_relation_commit_micros": seeded.relation_commit_micros,
                    "bootstrap_relation_phase_timing": {
                        "draft_preparation_micros": seeded.relation_commit_phase_timing.draft_preparation_micros,
                        "draft_bulk_admission_micros": seeded.relation_commit_phase_timing.draft_bulk_admission_micros,
                        "draft_merge_plan_micros": seeded.relation_commit_phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": seeded.relation_commit_phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": seeded.relation_commit_phase_timing.draft_working_state_clone_micros,
                        "invariant_pre_check_micros": seeded.relation_commit_phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": seeded.relation_commit_phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": seeded.relation_commit_phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": seeded.relation_commit_phase_timing.invariant_post_check_micros,
                        "artifact_assembly_micros": seeded.relation_commit_phase_timing.artifact_assembly_micros,
                        "durable_append_micros": seeded.relation_commit_phase_timing.durable_append_micros,
                        "publication_micros": seeded.relation_commit_phase_timing.publication_micros,
                        "publication_storage_commit_micros": seeded.relation_commit_phase_timing.publication_storage_commit_micros,
                    },
                    "hot_update_micros": hot_update_micros,
                    "hot_query_planning_micros": hot_query_planning_micros,
                    "hot_query_execution_micros": hot_query_execution_micros,
                    "phase_timing": {
                        "draft_preparation_micros": hot_phase_timing.draft_preparation_micros,
                        "draft_bulk_admission_micros": hot_phase_timing.draft_bulk_admission_micros,
                        "draft_merge_plan_micros": hot_phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": hot_phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": hot_phase_timing.draft_working_state_clone_micros,
                        "working_state_preparation_micros": hot_phase_timing.working_state_preparation_micros,
                        "invariant_pre_check_micros": hot_phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": hot_phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": hot_phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": hot_phase_timing.invariant_post_check_micros,
                        "artifact_assembly_micros": hot_phase_timing.artifact_assembly_micros,
                        "durable_append_micros": hot_phase_timing.durable_append_micros,
                        "publication_micros": hot_phase_timing.publication_micros,
                        "publication_storage_commit_micros": hot_phase_timing.publication_storage_commit_micros,
                        "publication_index_refresh_micros": hot_phase_timing.publication_index_refresh_micros,
                        "publication_history_publish_micros": hot_phase_timing.publication_history_publish_micros,
                        "publication_visibility_pin_micros": hot_phase_timing.publication_visibility_pin_micros,
                        "publication_bundle_publish_micros": hot_phase_timing.publication_bundle_publish_micros,
                        "publication_post_commit_consumer_micros": hot_phase_timing.publication_post_commit_consumer_micros,
                    },
                    "hot_changed_records": update.changed_records.len(),
                    "query_target_count": window_end - window_start,
                    "query_result_entities": outcome.result.entities.len(),
                    "query_result_relations": outcome.result.relations.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "profile_boundary": profile_boundary_metrics(
                        &runtime,
                        RelationalRuntimeProfile::GeometryKernel,
                    ),
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_zero_diagnostics_narrow_round_trip",
        &zero_diagnostics_samples,
        &[
            ("resident_node_count", &["resident_node_count"]),
            ("resident_relation_count", &["resident_relation_count"]),
            (
                "bootstrap_entity_commit_micros",
                &["bootstrap_entity_commit_micros"],
            ),
            (
                "bootstrap_relation_commit_micros",
                &["bootstrap_relation_commit_micros"],
            ),
            ("hot_update_micros", &["hot_update_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
            (
                "publication_index_refresh_micros",
                &["phase_timing", "publication_index_refresh_micros"],
            ),
            (
                "publication_history_publish_micros",
                &["phase_timing", "publication_history_publish_micros"],
            ),
            (
                "publication_visibility_pin_micros",
                &["phase_timing", "publication_visibility_pin_micros"],
            ),
            (
                "publication_bundle_publish_micros",
                &["phase_timing", "publication_bundle_publish_micros"],
            ),
            (
                "publication_post_commit_consumer_micros",
                &["phase_timing", "publication_post_commit_consumer_micros"],
            ),
            ("hot_query_planning_micros", &["hot_query_planning_micros"]),
            (
                "hot_query_execution_micros",
                &["hot_query_execution_micros"],
            ),
            ("query_target_count", &["query_target_count"]),
            ("query_result_entities", &["query_result_entities"]),
            ("diagnostic_artifact_count", &["diagnostic_artifact_count"]),
            ("detailed_trace_entries", &["detailed_trace_entries"]),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(zero_diagnostics_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &zero_diagnostics_samples,
        "rocketship zero-diagnostics should preserve a 100k-node resident world while keeping the hot path narrow and clone-free",
        |metrics| {
            let resident_node_count = metrics["resident_node_count"].as_u64().unwrap_or(0) as usize;
            let resident_relation_count =
                metrics["resident_relation_count"].as_u64().unwrap_or(0) as usize;
            let query_target_count = metrics["query_target_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && resident_relation_count >= resident_node_count.saturating_sub(1)
                && metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["query_result_entities"].as_u64() == Some(query_target_count)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == 1
                && counter_u64(metrics, "query_packet_count") <= 8
                && counter_u64(metrics, "query_scope_unit_count") <= query_target_count
        },
    );

    let rich_geometry_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_geometry_profile_narrow_round_trip",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryRichCertification,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded = seed_rocketship_world(&mut runtime, node_count);

            runtime.performance_access().reset_counters();
            let target_index = seeded.entities.len() / 2;
            let hot_update_started_at = Instant::now();
            let update = update_entity(
                &mut runtime,
                seeded.entities[target_index],
                "rocket-node-hot-update-rich",
            );
            let hot_update_micros = hot_update_started_at.elapsed().as_micros();
            let hot_phase_timing = update.execution.phase_timing.clone();
            let snapshot = runtime.visibility_authority().snapshot();
            let half_window = query_target_count / 2;
            let window_start = target_index.saturating_sub(half_window);
            let window_end = (window_start + query_target_count).min(seeded.entities.len());
            let targets = seeded.entities[window_start..window_end]
                .iter()
                .copied()
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let packet =
                explicit_query_packet(&runtime, &snapshot, "rocketship-explicit-rich", targets);

            let hot_query_plan_started_at = Instant::now();
            let planned = runtime
                .read_truth()
                .plan_query_packet(&snapshot, packet)
                .expect("planned rocketship explicit rich query");
            let hot_query_planning_micros = hot_query_plan_started_at.elapsed().as_micros();
            let hot_query_execution_started_at = Instant::now();
            let outcome = runtime
                .read_truth()
                .execute_query_plan(planned)
                .expect("rocketship explicit rich query outcome");
            let hot_query_execution_micros = hot_query_execution_started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            let elapsed_micros = seeded.entity_commit_micros
                + seeded.relation_commit_micros
                + hot_update_micros
                + hot_query_planning_micros
                + hot_query_execution_micros;
            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "bootstrap_entity_commit_micros": seeded.entity_commit_micros,
                    "bootstrap_relation_commit_micros": seeded.relation_commit_micros,
                    "hot_update_micros": hot_update_micros,
                    "hot_query_planning_micros": hot_query_planning_micros,
                    "hot_query_execution_micros": hot_query_execution_micros,
                    "phase_timing": {
                        "draft_preparation_micros": hot_phase_timing.draft_preparation_micros,
                        "draft_bulk_admission_micros": hot_phase_timing.draft_bulk_admission_micros,
                        "draft_merge_plan_micros": hot_phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": hot_phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": hot_phase_timing.draft_working_state_clone_micros,
                        "working_state_preparation_micros": hot_phase_timing.working_state_preparation_micros,
                        "invariant_pre_check_micros": hot_phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": hot_phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": hot_phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": hot_phase_timing.invariant_post_check_micros,
                        "artifact_assembly_micros": hot_phase_timing.artifact_assembly_micros,
                        "durable_append_micros": hot_phase_timing.durable_append_micros,
                        "publication_micros": hot_phase_timing.publication_micros,
                        "publication_storage_commit_micros": hot_phase_timing.publication_storage_commit_micros,
                        "publication_index_refresh_micros": hot_phase_timing.publication_index_refresh_micros,
                        "publication_history_publish_micros": hot_phase_timing.publication_history_publish_micros,
                        "publication_visibility_pin_micros": hot_phase_timing.publication_visibility_pin_micros,
                        "publication_bundle_publish_micros": hot_phase_timing.publication_bundle_publish_micros,
                        "publication_post_commit_consumer_micros": hot_phase_timing.publication_post_commit_consumer_micros,
                    },
                    "hot_changed_records": update.changed_records.len(),
                    "query_target_count": window_end - window_start,
                    "query_result_entities": outcome.result.entities.len(),
                    "query_result_relations": outcome.result.relations.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "profile_boundary": profile_boundary_metrics(
                        &runtime,
                        RelationalRuntimeProfile::GeometryKernel,
                    ),
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_geometry_profile_narrow_round_trip",
        &rich_geometry_samples,
        &[
            ("resident_node_count", &["resident_node_count"]),
            ("resident_relation_count", &["resident_relation_count"]),
            (
                "bootstrap_entity_commit_micros",
                &["bootstrap_entity_commit_micros"],
            ),
            (
                "bootstrap_relation_commit_micros",
                &["bootstrap_relation_commit_micros"],
            ),
            ("hot_update_micros", &["hot_update_micros"]),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
            (
                "publication_index_refresh_micros",
                &["phase_timing", "publication_index_refresh_micros"],
            ),
            (
                "publication_history_publish_micros",
                &["phase_timing", "publication_history_publish_micros"],
            ),
            (
                "publication_visibility_pin_micros",
                &["phase_timing", "publication_visibility_pin_micros"],
            ),
            (
                "publication_bundle_publish_micros",
                &["phase_timing", "publication_bundle_publish_micros"],
            ),
            (
                "publication_post_commit_consumer_micros",
                &["phase_timing", "publication_post_commit_consumer_micros"],
            ),
            ("hot_query_planning_micros", &["hot_query_planning_micros"]),
            (
                "hot_query_execution_micros",
                &["hot_query_execution_micros"],
            ),
            ("query_target_count", &["query_target_count"]),
            ("query_result_entities", &["query_result_entities"]),
            ("diagnostic_artifact_count", &["diagnostic_artifact_count"]),
            ("detailed_trace_entries", &["detailed_trace_entries"]),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(rich_geometry_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &rich_geometry_samples,
        "rocketship geometry-profile diagnostics should preserve the same 100k-node hot-path truth while deferring hot detailed traces",
        |metrics| {
            let resident_node_count = metrics["resident_node_count"].as_u64().unwrap_or(0) as usize;
            let resident_relation_count =
                metrics["resident_relation_count"].as_u64().unwrap_or(0) as usize;
            let query_target_count = metrics["query_target_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && resident_relation_count >= resident_node_count.saturating_sub(1)
                && metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["query_result_entities"].as_u64() == Some(query_target_count)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == 1
                && counter_u64(metrics, "query_packet_count") <= 8
                && counter_u64(metrics, "query_scope_unit_count") <= query_target_count
        },
    );

    let pseudorealistic_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_pseudorealistic_subsystem_round_trip",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded =
                seed_pseudorealistic_rocketship_world(&mut runtime, node_count, query_target_count);

            runtime.performance_access().reset_counters();
            let hot_update_started_at = Instant::now();
            let update = update_entity(
                &mut runtime,
                seeded.hot_update_target,
                "rocket.engine_cluster.hot_patch",
            );
            let hot_update_micros = hot_update_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "rocketship-pseudorealistic-explicit",
                seeded.mixed_query_targets.clone(),
            );
            let explicit_plan_started_at = Instant::now();
            let explicit_plan = runtime
                .read_truth()
                .plan_query_packet(&snapshot, explicit_packet)
                .expect("planned pseudorealistic explicit query");
            let explicit_query_planning_micros = explicit_plan_started_at.elapsed().as_micros();
            let explicit_execution_started_at = Instant::now();
            let explicit_outcome = runtime
                .read_truth()
                .execute_query_plan(explicit_plan)
                .expect("pseudorealistic explicit query outcome");
            let explicit_query_execution_micros =
                explicit_execution_started_at.elapsed().as_micros();

            let traversal_context = runtime
                .read_truth()
                .query_plan_context(&snapshot)
                .expect("pseudorealistic traversal context");
            let traversal_packet = PlannedQueryPacket {
                label: "rocketship-pseudorealistic-traversal".to_string(),
                context_id: traversal_context,
                scope: QueryScope::ConnectivityTraversal {
                    seeds: Arc::from(seeded.traversal_seeds.clone()),
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                    max_depth: Some(2),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                fallback: QueryFallbackContract::StorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(91_001),
                target_count_hint: seeded.traversal_seeds.len(),
            };
            let traversal_plan_started_at = Instant::now();
            let traversal_plan = runtime
                .read_truth()
                .plan_query_packet(&snapshot, traversal_packet)
                .expect("planned pseudorealistic traversal query");
            let traversal_planning_micros = traversal_plan_started_at.elapsed().as_micros();
            let traversal_execution_started_at = Instant::now();
            let traversal_outcome = runtime
                .read_truth()
                .execute_query_plan(traversal_plan)
                .expect("pseudorealistic traversal outcome");
            let traversal_execution_micros = traversal_execution_started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            let elapsed_micros = seeded.entity_commit_micros
                + seeded.relation_commit_micros
                + hot_update_micros
                + explicit_query_planning_micros
                + explicit_query_execution_micros
                + traversal_planning_micros
                + traversal_execution_micros;
            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "subsystem_count": seeded.subsystem_count,
                    "bootstrap_entity_commit_micros": seeded.entity_commit_micros,
                    "bootstrap_relation_commit_micros": seeded.relation_commit_micros,
                    "bootstrap_relation_phase_timing": {
                        "draft_preparation_micros": seeded.relation_commit_phase_timing.draft_preparation_micros,
                        "draft_bulk_admission_micros": seeded.relation_commit_phase_timing.draft_bulk_admission_micros,
                        "draft_merge_plan_micros": seeded.relation_commit_phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": seeded.relation_commit_phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": seeded.relation_commit_phase_timing.draft_working_state_clone_micros,
                        "invariant_pre_check_micros": seeded.relation_commit_phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": seeded.relation_commit_phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": seeded.relation_commit_phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": seeded.relation_commit_phase_timing.invariant_post_check_micros,
                        "artifact_assembly_micros": seeded.relation_commit_phase_timing.artifact_assembly_micros,
                        "durable_append_micros": seeded.relation_commit_phase_timing.durable_append_micros,
                        "publication_micros": seeded.relation_commit_phase_timing.publication_micros,
                        "publication_storage_commit_micros": seeded.relation_commit_phase_timing.publication_storage_commit_micros,
                    },
                    "hot_update_micros": hot_update_micros,
                    "explicit_query_planning_micros": explicit_query_planning_micros,
                    "explicit_query_execution_micros": explicit_query_execution_micros,
                    "traversal_planning_micros": traversal_planning_micros,
                    "traversal_execution_micros": traversal_execution_micros,
                    "hot_changed_records": update.changed_records.len(),
                    "mixed_query_target_count": seeded.mixed_query_targets.len(),
                    "explicit_query_result_entities": explicit_outcome.result.entities.len(),
                    "traversal_seed_count": seeded.traversal_seeds.len(),
                    "traversal_result_entities": traversal_outcome.result.entities.len(),
                    "traversal_result_relations": traversal_outcome.result.relations.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "profile_boundary": profile_boundary_metrics(
                        &runtime,
                        RelationalRuntimeProfile::GeometryKernel,
                    ),
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_pseudorealistic_subsystem_round_trip",
        &pseudorealistic_samples,
        &[
            ("resident_node_count", &["resident_node_count"]),
            ("resident_relation_count", &["resident_relation_count"]),
            ("subsystem_count", &["subsystem_count"]),
            (
                "bootstrap_entity_commit_micros",
                &["bootstrap_entity_commit_micros"],
            ),
            (
                "bootstrap_relation_commit_micros",
                &["bootstrap_relation_commit_micros"],
            ),
            ("hot_update_micros", &["hot_update_micros"]),
            (
                "explicit_query_planning_micros",
                &["explicit_query_planning_micros"],
            ),
            (
                "explicit_query_execution_micros",
                &["explicit_query_execution_micros"],
            ),
            ("traversal_planning_micros", &["traversal_planning_micros"]),
            (
                "traversal_execution_micros",
                &["traversal_execution_micros"],
            ),
            ("mixed_query_target_count", &["mixed_query_target_count"]),
            (
                "explicit_query_result_entities",
                &["explicit_query_result_entities"],
            ),
            ("traversal_seed_count", &["traversal_seed_count"]),
            ("traversal_result_entities", &["traversal_result_entities"]),
            (
                "traversal_result_relations",
                &["traversal_result_relations"],
            ),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(pseudorealistic_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &pseudorealistic_samples,
        "pseudorealistic rocketship should preserve mixed subsystem truth, narrow hot updates, and bounded mixed-locality query work",
        |metrics| {
            let mixed_query_target_count =
                metrics["mixed_query_target_count"].as_u64().unwrap_or(0);
            let traversal_seed_count = metrics["traversal_seed_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0) >= node_count as u64
                && metrics["subsystem_count"].as_u64() == Some(12)
                && metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["explicit_query_result_entities"].as_u64()
                    == Some(mixed_query_target_count)
                && metrics["traversal_result_entities"].as_u64().unwrap_or(0)
                    >= traversal_seed_count
                && metrics["traversal_result_relations"].as_u64().unwrap_or(0) >= 1
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == 1
                && counter_u64(metrics, "query_packet_count")
                    <= mixed_query_target_count + traversal_seed_count
                && counter_u64(metrics, "query_scope_unit_count")
                    <= mixed_query_target_count + traversal_seed_count
        },
    );

    let propagation_wave_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_pseudorealistic_propagation_wave",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded =
                seed_pseudorealistic_rocketship_world(&mut runtime, node_count, query_target_count);

            runtime.performance_access().reset_counters();
            let hot_update_started_at = Instant::now();
            let update = update_entity(
                &mut runtime,
                seeded.hot_update_target,
                "rocket.plumbing_and_feed.propagation_patch",
            );
            let hot_update_micros = hot_update_started_at.elapsed().as_micros();
            let hot_phase_timing = update.execution.phase_timing.clone();

            let snapshot = runtime.visibility_authority().snapshot();
            let context = runtime
                .read_truth()
                .query_plan_context(&snapshot)
                .expect("rocketship propagation context");
            let propagation_seeds = vec![
                seeded.traversal_seeds[0],
                seeded.traversal_seeds[1],
                seeded.traversal_seeds[9],
                seeded.traversal_seeds[10],
            ];
            let propagation_packet = PlannedQueryPacket {
                label: "rocketship-pseudorealistic-propagation".to_string(),
                context_id: context,
                scope: QueryScope::ConnectivityTraversal {
                    seeds: Arc::from(propagation_seeds.clone()),
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                    max_depth: Some(3),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                fallback: QueryFallbackContract::StorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(91_002),
                target_count_hint: propagation_seeds.len(),
            };
            let propagation_plan_started_at = Instant::now();
            let propagation_plan = runtime
                .read_truth()
                .plan_query_packet(&snapshot, propagation_packet)
                .expect("planned rocketship propagation query");
            let propagation_planning_micros = propagation_plan_started_at.elapsed().as_micros();
            let propagation_execution_started_at = Instant::now();
            let propagation_outcome = runtime
                .read_truth()
                .execute_query_plan(propagation_plan)
                .expect("rocketship propagation outcome");
            let propagation_execution_micros =
                propagation_execution_started_at.elapsed().as_micros();

            let explicit_targets = seeded
                .mixed_query_targets
                .iter()
                .take(12)
                .cloned()
                .collect::<Vec<_>>();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "rocketship-pseudorealistic-propagation-explicit",
                explicit_targets.clone(),
            );
            let explicit_started_at = Instant::now();
            let explicit_outcome = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("planned rocketship propagation explicit query"),
                )
                .expect("rocketship propagation explicit query outcome");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            let elapsed_micros = seeded.entity_commit_micros
                + seeded.relation_commit_micros
                + hot_update_micros
                + propagation_planning_micros
                + propagation_execution_micros
                + explicit_query_micros;
            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "subsystem_count": seeded.subsystem_count,
                    "bootstrap_entity_commit_micros": seeded.entity_commit_micros,
                    "bootstrap_relation_commit_micros": seeded.relation_commit_micros,
                    "hot_update_micros": hot_update_micros,
                    "phase_timing": {
                        "draft_preparation_micros": hot_phase_timing.draft_preparation_micros,
                        "draft_bulk_admission_micros": hot_phase_timing.draft_bulk_admission_micros,
                        "draft_merge_plan_micros": hot_phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": hot_phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": hot_phase_timing.draft_working_state_clone_micros,
                        "working_state_preparation_micros": hot_phase_timing.working_state_preparation_micros,
                        "invariant_pre_check_micros": hot_phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": hot_phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": hot_phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": hot_phase_timing.invariant_post_check_micros,
                        "artifact_assembly_micros": hot_phase_timing.artifact_assembly_micros,
                        "durable_append_micros": hot_phase_timing.durable_append_micros,
                        "publication_micros": hot_phase_timing.publication_micros,
                        "publication_storage_commit_micros": hot_phase_timing.publication_storage_commit_micros,
                        "publication_index_refresh_micros": hot_phase_timing.publication_index_refresh_micros,
                        "publication_history_publish_micros": hot_phase_timing.publication_history_publish_micros,
                        "publication_visibility_pin_micros": hot_phase_timing.publication_visibility_pin_micros,
                        "publication_bundle_publish_micros": hot_phase_timing.publication_bundle_publish_micros,
                        "publication_post_commit_consumer_micros": hot_phase_timing.publication_post_commit_consumer_micros,
                    },
                    "propagation_planning_micros": propagation_planning_micros,
                    "propagation_execution_micros": propagation_execution_micros,
                    "explicit_query_micros": explicit_query_micros,
                    "hot_changed_records": update.changed_records.len(),
                    "propagation_seed_count": propagation_seeds.len(),
                    "propagation_result_entities": propagation_outcome.result.entities.len(),
                    "propagation_result_relations": propagation_outcome.result.relations.len(),
                    "explicit_target_count": explicit_targets.len(),
                    "explicit_result_entities": explicit_outcome.result.entities.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_pseudorealistic_propagation_wave",
        &propagation_wave_samples,
        &[
            ("resident_node_count", &["resident_node_count"]),
            ("resident_relation_count", &["resident_relation_count"]),
            ("subsystem_count", &["subsystem_count"]),
            (
                "bootstrap_entity_commit_micros",
                &["bootstrap_entity_commit_micros"],
            ),
            (
                "bootstrap_relation_commit_micros",
                &["bootstrap_relation_commit_micros"],
            ),
            ("hot_update_micros", &["hot_update_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
            (
                "publication_index_refresh_micros",
                &["phase_timing", "publication_index_refresh_micros"],
            ),
            (
                "publication_history_publish_micros",
                &["phase_timing", "publication_history_publish_micros"],
            ),
            (
                "publication_visibility_pin_micros",
                &["phase_timing", "publication_visibility_pin_micros"],
            ),
            (
                "publication_bundle_publish_micros",
                &["phase_timing", "publication_bundle_publish_micros"],
            ),
            (
                "publication_post_commit_consumer_micros",
                &["phase_timing", "publication_post_commit_consumer_micros"],
            ),
            (
                "propagation_planning_micros",
                &["propagation_planning_micros"],
            ),
            (
                "propagation_execution_micros",
                &["propagation_execution_micros"],
            ),
            ("explicit_query_micros", &["explicit_query_micros"]),
            ("propagation_seed_count", &["propagation_seed_count"]),
            (
                "propagation_result_entities",
                &["propagation_result_entities"],
            ),
            (
                "propagation_result_relations",
                &["propagation_result_relations"],
            ),
            ("explicit_target_count", &["explicit_target_count"]),
            ("explicit_result_entities", &["explicit_result_entities"]),
        ],
    );
    assert!(propagation_wave_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &propagation_wave_samples,
        "pseudorealistic rocketship propagation waves should stay bounded while spanning multiple subsystem interfaces",
        |metrics| {
            let propagation_seed_count =
                metrics["propagation_seed_count"].as_u64().unwrap_or(0);
            let explicit_target_count =
                metrics["explicit_target_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0) >= node_count as u64
                && metrics["subsystem_count"].as_u64() == Some(12)
                && metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["propagation_result_entities"].as_u64().unwrap_or(0)
                    >= propagation_seed_count
                && metrics["propagation_result_relations"].as_u64().unwrap_or(0) >= 1
                && metrics["explicit_result_entities"].as_u64() == Some(explicit_target_count)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == 1
                && counter_u64(metrics, "query_packet_count") <= 32
                && counter_u64(metrics, "query_scope_unit_count")
                    <= propagation_seed_count + explicit_target_count
        },
    );

    let flat_batch_wave_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_pseudorealistic_flat_entity_batch_wave",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded =
                seed_pseudorealistic_rocketship_world(&mut runtime, node_count, query_target_count);

            let mut partition_targets = BTreeMap::new();
            for entity in &seeded.entities {
                let targets = partition_targets
                    .entry(entity.partition_id)
                    .or_insert_with(Vec::new);
                if targets.len() < 8 {
                    targets.push(*entity);
                }
                if partition_targets.len() >= 8
                    && partition_targets.values().all(|targets| targets.len() >= 8)
                {
                    break;
                }
            }
            let batch_targets = partition_targets
                .values()
                .flat_map(|targets| targets.iter().take(8).copied())
                .collect::<Vec<_>>();
            assert!(
                batch_targets.len() >= 64,
                "rocketship flat batch wave should gather a broad multi-partition entity batch"
            );

            runtime.performance_access().reset_counters();
            let update_started_at = Instant::now();
            let update = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                let mut batch = WorkerIntentBatch::new("rocketship-flat-entity-batch-wave");
                for (index, entity) in batch_targets.iter().enumerate() {
                    batch = batch.push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                        UpdateEntityFieldsIntent {
                            entity_id: *entity,
                            fields: crate::tests::support::aspect_field_patch_from_values([
                                (
                                    "section",
                                    crate::tests::support::string_aspect_value("batch-wave"),
                                ),
                                (
                                    "tag",
                                    crate::tests::support::string_aspect_value(&format!(
                                        "rocket.batch.{index}"
                                    )),
                                ),
                                (
                                    "partition",
                                    crate::tests::support::u64_aspect_value(
                                        entity.partition_id.0 as u64,
                                    ),
                                ),
                            ]),
                        },
                    )));
                }
                txn.push_batch(batch);
                txn.commit()
                    .expect("rocketship flat entity batch wave commit")
            };
            let update_micros = update_started_at.elapsed().as_micros();
            let phase_timing = update.execution.phase_timing.clone();

            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_targets = batch_targets
                .iter()
                .take(16)
                .copied()
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "rocketship-flat-entity-batch-explicit",
                explicit_targets,
            );
            let explicit_started_at = Instant::now();
            let explicit = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("planned rocketship flat batch explicit query"),
                )
                .expect("rocketship flat batch explicit outcome");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();
            assert!(runtime.visibility_authority().release_snapshot(&snapshot));

            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            measurement_with_elapsed(update_micros + explicit_query_micros, || {
                json!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "subsystem_count": seeded.subsystem_count,
                    "batch_target_count": batch_targets.len(),
                    "batch_partition_count": partition_targets.len(),
                    "update_micros": update_micros,
                    "explicit_query_micros": explicit_query_micros,
                    "hot_changed_records": update.changed_records.len(),
                    "explicit_result_entities": explicit.result.entities.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "phase_timing": {
                        "draft_preparation_micros": phase_timing.draft_preparation_micros,
                        "draft_intent_normalization_micros": phase_timing.draft_intent_normalization_micros,
                        "draft_merge_plan_micros": phase_timing.draft_merge_plan_micros,
                        "draft_intent_validation_micros": phase_timing.draft_intent_validation_micros,
                        "draft_intent_sort_micros": phase_timing.draft_intent_sort_micros,
                        "draft_conflict_detection_micros": phase_timing.draft_conflict_detection_micros,
                        "draft_structural_summary_micros": phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": phase_timing.draft_working_state_clone_micros,
                        "invariant_pre_check_micros": phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": phase_timing.invariant_post_check_micros,
                        "durable_append_micros": phase_timing.durable_append_micros,
                        "publication_micros": phase_timing.publication_micros,
                        "publication_storage_commit_micros": phase_timing.publication_storage_commit_micros,
                    },
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_pseudorealistic_flat_entity_batch_wave",
        &flat_batch_wave_samples,
        &[
            ("batch_target_count", &["batch_target_count"]),
            ("batch_partition_count", &["batch_partition_count"]),
            ("update_micros", &["update_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "draft_intent_normalization_micros",
                &["phase_timing", "draft_intent_normalization_micros"],
            ),
            (
                "draft_merge_plan_micros",
                &["phase_timing", "draft_merge_plan_micros"],
            ),
            (
                "draft_intent_validation_micros",
                &["phase_timing", "draft_intent_validation_micros"],
            ),
            (
                "draft_intent_sort_micros",
                &["phase_timing", "draft_intent_sort_micros"],
            ),
            (
                "draft_conflict_detection_micros",
                &["phase_timing", "draft_conflict_detection_micros"],
            ),
            (
                "draft_structural_summary_micros",
                &["phase_timing", "draft_structural_summary_micros"],
            ),
            (
                "draft_working_state_clone_micros",
                &["phase_timing", "draft_working_state_clone_micros"],
            ),
            (
                "invariant_pre_check_micros",
                &["phase_timing", "invariant_pre_check_micros"],
            ),
            (
                "authoritative_mutation_micros",
                &["phase_timing", "authoritative_mutation_micros"],
            ),
            (
                "history_resolution_micros",
                &["phase_timing", "history_resolution_micros"],
            ),
            (
                "invariant_post_check_micros",
                &["phase_timing", "invariant_post_check_micros"],
            ),
            (
                "durable_append_micros",
                &["phase_timing", "durable_append_micros"],
            ),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
            ("explicit_query_micros", &["explicit_query_micros"]),
            (
                "aosoa_entity_chunk_slots_materialized",
                &["counters", "aosoa_entity_chunk_slots_materialized"],
            ),
            (
                "aosoa_entity_chunks_published",
                &["counters", "aosoa_entity_chunks_published"],
            ),
        ],
    );
    assert!(flat_batch_wave_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &flat_batch_wave_samples,
        "pseudorealistic rocketship flat entity batches should stay on the widened sparse AoSoA path across multiple touched partitions",
        |metrics| {
            let batch_target_count = metrics["batch_target_count"].as_u64().unwrap_or(0);
            let batch_partition_count = metrics["batch_partition_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0) >= node_count as u64
                && metrics["subsystem_count"].as_u64() == Some(12)
                && batch_target_count >= 64
                && batch_partition_count >= 8
                && metrics["hot_changed_records"].as_u64() == Some(batch_target_count)
                && metrics["explicit_result_entities"].as_u64() == Some(16)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == batch_target_count
                && counter_u64(metrics, "partitions_touched_by_commit") >= batch_partition_count
                && counter_u64(metrics, "aosoa_entity_chunk_slots_materialized")
                    == batch_target_count
                && counter_u64(metrics, "aosoa_entity_chunks_published") >= batch_partition_count
                && counter_u64(metrics, "aosoa_publish_fallback_count") == 0
        },
    );

    let varied_flat_batch_wave_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_pseudorealistic_varied_locality_batch_wave",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded =
                seed_pseudorealistic_rocketship_world(&mut runtime, node_count, query_target_count);

            let mut selected_partitions = Vec::new();
            for entity in seeded.entities.iter().step_by(257) {
                if !selected_partitions.contains(&entity.partition_id) {
                    selected_partitions.push(entity.partition_id);
                }
                if selected_partitions.len() >= 8 {
                    break;
                }
            }
            assert!(
                selected_partitions.len() >= 8,
                "rocketship varied batch wave should discover a bounded multi-partition spread"
            );

            let mut partition_targets = BTreeMap::new();
            for entity in seeded.entities.iter() {
                if !selected_partitions.contains(&entity.partition_id) {
                    continue;
                }
                let targets = partition_targets
                    .entry(entity.partition_id)
                    .or_insert_with(Vec::new);
                if targets.len() < 8 {
                    targets.push(*entity);
                }
                if partition_targets.len() == selected_partitions.len()
                    && partition_targets.values().all(|targets| targets.len() >= 8)
                {
                    break;
                }
            }
            let batch_targets = partition_targets
                .values()
                .flat_map(|targets| targets.iter().take(8).copied())
                .collect::<Vec<_>>();
            assert!(
                batch_targets.len() >= 64,
                "rocketship varied batch wave should gather a varied-locality entity batch within a bounded partition spread"
            );

            runtime.performance_access().reset_counters();
            let update_started_at = Instant::now();
            let update = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                let mut batch = WorkerIntentBatch::new("rocketship-varied-locality-batch-wave");
                for (index, entity) in batch_targets.iter().enumerate() {
                    batch = batch.push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                        UpdateEntityFieldsIntent {
                            entity_id: *entity,
                            fields: crate::tests::support::aspect_field_patch_from_values([
                                (
                                    "section",
                                    crate::tests::support::string_aspect_value("varied-batch-wave"),
                                ),
                                (
                                    "tag",
                                    crate::tests::support::string_aspect_value(&format!(
                                        "rocket.varied.{index}"
                                    )),
                                ),
                                (
                                    "partition",
                                    crate::tests::support::u64_aspect_value(
                                        entity.partition_id.0 as u64,
                                    ),
                                ),
                            ]),
                        },
                    )));
                }
                txn.push_batch(batch);
                txn.commit()
                    .expect("rocketship varied locality batch wave commit")
            };
            let update_micros = update_started_at.elapsed().as_micros();
            let phase_timing = update.execution.phase_timing.clone();

            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_targets = batch_targets
                .iter()
                .step_by(3)
                .take(16)
                .copied()
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "rocketship-varied-batch-explicit",
                explicit_targets,
            );
            let explicit_started_at = Instant::now();
            let explicit = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("planned rocketship varied batch explicit query"),
                )
                .expect("rocketship varied batch explicit outcome");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();
            assert!(runtime.visibility_authority().release_snapshot(&snapshot));

            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            measurement_with_elapsed(update_micros + explicit_query_micros, || {
                json!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "subsystem_count": seeded.subsystem_count,
                    "batch_target_count": batch_targets.len(),
                    "batch_partition_count": partition_targets.len(),
                    "update_micros": update_micros,
                    "explicit_query_micros": explicit_query_micros,
                    "hot_changed_records": update.changed_records.len(),
                    "explicit_result_entities": explicit.result.entities.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "phase_timing": {
                        "draft_preparation_micros": phase_timing.draft_preparation_micros,
                        "draft_merge_plan_micros": phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": phase_timing.draft_working_state_clone_micros,
                        "invariant_pre_check_micros": phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": phase_timing.invariant_post_check_micros,
                        "durable_append_micros": phase_timing.durable_append_micros,
                        "publication_micros": phase_timing.publication_micros,
                        "publication_storage_commit_micros": phase_timing.publication_storage_commit_micros,
                    },
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_pseudorealistic_varied_locality_batch_wave",
        &varied_flat_batch_wave_samples,
        &[
            ("batch_target_count", &["batch_target_count"]),
            ("batch_partition_count", &["batch_partition_count"]),
            ("update_micros", &["update_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "draft_merge_plan_micros",
                &["phase_timing", "draft_merge_plan_micros"],
            ),
            (
                "draft_structural_summary_micros",
                &["phase_timing", "draft_structural_summary_micros"],
            ),
            (
                "draft_working_state_clone_micros",
                &["phase_timing", "draft_working_state_clone_micros"],
            ),
            (
                "invariant_pre_check_micros",
                &["phase_timing", "invariant_pre_check_micros"],
            ),
            (
                "authoritative_mutation_micros",
                &["phase_timing", "authoritative_mutation_micros"],
            ),
            (
                "history_resolution_micros",
                &["phase_timing", "history_resolution_micros"],
            ),
            (
                "invariant_post_check_micros",
                &["phase_timing", "invariant_post_check_micros"],
            ),
            (
                "durable_append_micros",
                &["phase_timing", "durable_append_micros"],
            ),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
            ("explicit_query_micros", &["explicit_query_micros"]),
            (
                "aosoa_entity_chunk_slots_materialized",
                &["counters", "aosoa_entity_chunk_slots_materialized"],
            ),
            (
                "aosoa_entity_chunks_published",
                &["counters", "aosoa_entity_chunks_published"],
            ),
        ],
    );
    assert!(varied_flat_batch_wave_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &varied_flat_batch_wave_samples,
        "pseudorealistic rocketship varied-locality batches should stay on the widened sparse AoSoA path across broader partition spread",
        |metrics| {
            let batch_target_count = metrics["batch_target_count"].as_u64().unwrap_or(0);
            let batch_partition_count = metrics["batch_partition_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0) >= node_count as u64
                && metrics["subsystem_count"].as_u64() == Some(12)
                && batch_target_count >= 64
                && batch_partition_count == 8
                && metrics["hot_changed_records"].as_u64() == Some(batch_target_count)
                && metrics["explicit_result_entities"].as_u64() == Some(16)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == batch_target_count
                && counter_u64(metrics, "partitions_touched_by_commit") >= batch_partition_count
                && counter_u64(metrics, "aosoa_entity_chunk_slots_materialized")
                    == batch_target_count
                && counter_u64(metrics, "aosoa_entity_chunks_published") >= batch_partition_count
                && counter_u64(metrics, "aosoa_publish_fallback_count") == 0
        },
    );

    let larger_flat_batch_wave_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_pseudorealistic_large_flat_entity_batch_wave",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded =
                seed_pseudorealistic_rocketship_world(&mut runtime, node_count, query_target_count);

            let mut partition_targets = BTreeMap::new();
            for entity in &seeded.entities {
                let targets = partition_targets
                    .entry(entity.partition_id)
                    .or_insert_with(Vec::new);
                if targets.len() < 16 {
                    targets.push(*entity);
                }
                if partition_targets.len() >= 8
                    && partition_targets
                        .values()
                        .all(|targets| targets.len() >= 16)
                {
                    break;
                }
            }
            let batch_targets = partition_targets
                .values()
                .flat_map(|targets| targets.iter().take(16).copied())
                .collect::<Vec<_>>();
            assert!(
                batch_targets.len() >= 128,
                "rocketship large flat batch wave should gather a larger bounded multi-partition entity batch"
            );

            runtime.performance_access().reset_counters();
            let update_started_at = Instant::now();
            let update = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                let mut batch = WorkerIntentBatch::new("rocketship-large-flat-entity-batch-wave");
                for (index, entity) in batch_targets.iter().enumerate() {
                    batch = batch.push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                        UpdateEntityFieldsIntent {
                            entity_id: *entity,
                            fields: crate::tests::support::aspect_field_patch_from_values([
                                (
                                    "section",
                                    crate::tests::support::string_aspect_value("large-batch-wave"),
                                ),
                                (
                                    "tag",
                                    crate::tests::support::string_aspect_value(&format!(
                                        "rocket.large_batch.{index}"
                                    )),
                                ),
                                (
                                    "partition",
                                    crate::tests::support::u64_aspect_value(
                                        entity.partition_id.0 as u64,
                                    ),
                                ),
                            ]),
                        },
                    )));
                }
                txn.push_batch(batch);
                txn.commit()
                    .expect("rocketship large flat entity batch wave commit")
            };
            let update_micros = update_started_at.elapsed().as_micros();
            let phase_timing = update.execution.phase_timing.clone();

            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_targets = batch_targets
                .iter()
                .step_by(4)
                .take(16)
                .copied()
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "rocketship-large-flat-entity-batch-explicit",
                explicit_targets,
            );
            let explicit_started_at = Instant::now();
            let explicit = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("planned rocketship large flat batch explicit query"),
                )
                .expect("rocketship large flat batch explicit outcome");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();
            assert!(runtime.visibility_authority().release_snapshot(&snapshot));

            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            measurement_with_elapsed(update_micros + explicit_query_micros, || {
                json!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "subsystem_count": seeded.subsystem_count,
                    "batch_target_count": batch_targets.len(),
                    "batch_partition_count": partition_targets.len(),
                    "update_micros": update_micros,
                    "explicit_query_micros": explicit_query_micros,
                    "hot_changed_records": update.changed_records.len(),
                    "explicit_result_entities": explicit.result.entities.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "phase_timing": {
                        "draft_preparation_micros": phase_timing.draft_preparation_micros,
                        "draft_merge_plan_micros": phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": phase_timing.draft_working_state_clone_micros,
                        "invariant_pre_check_micros": phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": phase_timing.invariant_post_check_micros,
                        "durable_append_micros": phase_timing.durable_append_micros,
                        "publication_micros": phase_timing.publication_micros,
                        "publication_storage_commit_micros": phase_timing.publication_storage_commit_micros,
                    },
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_pseudorealistic_large_flat_entity_batch_wave",
        &larger_flat_batch_wave_samples,
        &[
            ("batch_target_count", &["batch_target_count"]),
            ("batch_partition_count", &["batch_partition_count"]),
            ("update_micros", &["update_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "draft_merge_plan_micros",
                &["phase_timing", "draft_merge_plan_micros"],
            ),
            (
                "draft_structural_summary_micros",
                &["phase_timing", "draft_structural_summary_micros"],
            ),
            (
                "draft_working_state_clone_micros",
                &["phase_timing", "draft_working_state_clone_micros"],
            ),
            (
                "invariant_pre_check_micros",
                &["phase_timing", "invariant_pre_check_micros"],
            ),
            (
                "authoritative_mutation_micros",
                &["phase_timing", "authoritative_mutation_micros"],
            ),
            (
                "history_resolution_micros",
                &["phase_timing", "history_resolution_micros"],
            ),
            (
                "invariant_post_check_micros",
                &["phase_timing", "invariant_post_check_micros"],
            ),
            (
                "durable_append_micros",
                &["phase_timing", "durable_append_micros"],
            ),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
            ("explicit_query_micros", &["explicit_query_micros"]),
            (
                "aosoa_entity_chunk_slots_materialized",
                &["counters", "aosoa_entity_chunk_slots_materialized"],
            ),
            (
                "aosoa_entity_chunks_published",
                &["counters", "aosoa_entity_chunks_published"],
            ),
        ],
    );
    assert!(larger_flat_batch_wave_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &larger_flat_batch_wave_samples,
        "pseudorealistic rocketship large flat entity batches should stay on the widened sparse AoSoA path when the bounded batch doubles in width",
        |metrics| {
            let batch_target_count = metrics["batch_target_count"].as_u64().unwrap_or(0);
            let batch_partition_count = metrics["batch_partition_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0) >= node_count as u64
                && metrics["subsystem_count"].as_u64() == Some(12)
                && batch_target_count >= 128
                && batch_partition_count >= 8
                && metrics["hot_changed_records"].as_u64() == Some(batch_target_count)
                && metrics["explicit_result_entities"].as_u64() == Some(16)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == batch_target_count
                && counter_u64(metrics, "partitions_touched_by_commit") >= batch_partition_count
                && counter_u64(metrics, "aosoa_entity_chunk_slots_materialized")
                    == batch_target_count
                && counter_u64(metrics, "aosoa_entity_chunks_published") >= batch_partition_count
                && counter_u64(metrics, "aosoa_publish_fallback_count") == 0
        },
    );

    let mixed_entity_relation_batch_wave_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_pseudorealistic_mixed_entity_relation_batch_wave",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded =
                seed_pseudorealistic_rocketship_world(&mut runtime, node_count, query_target_count);

            let mut partition_targets = BTreeMap::new();
            for entity in &seeded.entities {
                let targets = partition_targets
                    .entry(entity.partition_id)
                    .or_insert_with(Vec::new);
                if targets.len() < 8 {
                    targets.push(*entity);
                }
                if partition_targets.len() >= 8
                    && partition_targets.values().all(|targets| targets.len() >= 8)
                {
                    break;
                }
            }
            let batch_targets = partition_targets
                .values()
                .flat_map(|targets| targets.iter().take(8).copied())
                .collect::<Vec<_>>();
            assert!(
                batch_targets.len() >= 64,
                "rocketship mixed entity-plus-relation batch wave should gather a broad multi-partition entity batch"
            );

            let relation_specs = partition_targets
                .values()
                .enumerate()
                .flat_map(|(partition_index, targets)| {
                    targets
                        .windows(2)
                        .take(2)
                        .enumerate()
                        .map(
                            move |(edge_index, pair)| crate::transactions::data::RelationSpec {
                                partition_id: PartitionId(601 + partition_index as u32),
                                kind_id: KindId(2),
                                client_key: crate::symbols::data::ClientKey::raw(format!(
                                    "rocket.batch.local.{}.{}",
                                    partition_index, edge_index
                                )),
                                source: crate::transactions::data::EntityReference::Existing(
                                    pair[0],
                                ),
                                target: crate::transactions::data::EntityReference::Existing(
                                    pair[1],
                                ),
                                fields: crate::transactions::data::AspectFieldPatch::default(),
                            },
                        )
                })
                .collect::<Vec<_>>();
            assert!(
                relation_specs.len() >= 16,
                "rocketship mixed entity-plus-relation batch wave should add a bounded local relation wave"
            );

            runtime.performance_access().reset_counters();
            let update_started_at = Instant::now();
            let update = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                let mut batch =
                    WorkerIntentBatch::new("rocketship-mixed-entity-relation-batch-wave");
                for (index, entity) in batch_targets.iter().enumerate() {
                    batch = batch.push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                        UpdateEntityFieldsIntent {
                            entity_id: *entity,
                            fields: crate::tests::support::aspect_field_patch_from_values([
                                (
                                    "section",
                                    crate::tests::support::string_aspect_value("mixed-batch-wave"),
                                ),
                                (
                                    "tag",
                                    crate::tests::support::string_aspect_value(&format!(
                                        "rocket.mixed_batch.{index}"
                                    )),
                                ),
                                (
                                    "partition",
                                    crate::tests::support::u64_aspect_value(
                                        entity.partition_id.0 as u64,
                                    ),
                                ),
                            ]),
                        },
                    )));
                }
                for intent in bulk_relation_create_intents(&relation_specs) {
                    batch = batch.push(intent);
                }
                txn.push_batch(batch);
                txn.commit()
                    .expect("rocketship mixed entity plus relation batch wave commit")
            };
            let update_micros = update_started_at.elapsed().as_micros();
            let phase_timing = update.execution.phase_timing.clone();

            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_targets = batch_targets
                .iter()
                .step_by(3)
                .take(16)
                .copied()
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "rocketship-mixed-batch-explicit",
                explicit_targets,
            );
            let explicit_started_at = Instant::now();
            let explicit = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("planned rocketship mixed batch explicit query"),
                )
                .expect("rocketship mixed batch explicit outcome");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();
            assert!(runtime.visibility_authority().release_snapshot(&snapshot));

            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            measurement_with_elapsed(update_micros + explicit_query_micros, || {
                json!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "subsystem_count": seeded.subsystem_count,
                    "batch_target_count": batch_targets.len(),
                    "batch_partition_count": partition_targets.len(),
                    "created_relation_count": relation_specs.len(),
                    "update_micros": update_micros,
                    "explicit_query_micros": explicit_query_micros,
                    "hot_changed_records": update.changed_records.len(),
                    "explicit_result_entities": explicit.result.entities.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "phase_timing": {
                        "draft_preparation_micros": phase_timing.draft_preparation_micros,
                        "draft_merge_plan_micros": phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": phase_timing.draft_working_state_clone_micros,
                        "invariant_pre_check_micros": phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": phase_timing.invariant_post_check_micros,
                        "durable_append_micros": phase_timing.durable_append_micros,
                        "publication_micros": phase_timing.publication_micros,
                        "publication_storage_commit_micros": phase_timing.publication_storage_commit_micros,
                    },
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_pseudorealistic_mixed_entity_relation_batch_wave",
        &mixed_entity_relation_batch_wave_samples,
        &[
            ("batch_target_count", &["batch_target_count"]),
            ("batch_partition_count", &["batch_partition_count"]),
            ("created_relation_count", &["created_relation_count"]),
            ("update_micros", &["update_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "draft_merge_plan_micros",
                &["phase_timing", "draft_merge_plan_micros"],
            ),
            (
                "draft_structural_summary_micros",
                &["phase_timing", "draft_structural_summary_micros"],
            ),
            (
                "draft_working_state_clone_micros",
                &["phase_timing", "draft_working_state_clone_micros"],
            ),
            (
                "invariant_pre_check_micros",
                &["phase_timing", "invariant_pre_check_micros"],
            ),
            (
                "authoritative_mutation_micros",
                &["phase_timing", "authoritative_mutation_micros"],
            ),
            (
                "history_resolution_micros",
                &["phase_timing", "history_resolution_micros"],
            ),
            (
                "invariant_post_check_micros",
                &["phase_timing", "invariant_post_check_micros"],
            ),
            (
                "durable_append_micros",
                &["phase_timing", "durable_append_micros"],
            ),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
            ("explicit_query_micros", &["explicit_query_micros"]),
            (
                "aosoa_entity_chunk_slots_materialized",
                &["counters", "aosoa_entity_chunk_slots_materialized"],
            ),
            (
                "aosoa_entity_chunks_published",
                &["counters", "aosoa_entity_chunks_published"],
            ),
        ],
    );
    assert!(mixed_entity_relation_batch_wave_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &mixed_entity_relation_batch_wave_samples,
        "pseudorealistic rocketship mixed entity plus relation batches should stay bounded and preserve semantic purity when the commit leaves the pure flat-entity fast path",
        |metrics| {
            let batch_target_count = metrics["batch_target_count"].as_u64().unwrap_or(0);
            let batch_partition_count = metrics["batch_partition_count"].as_u64().unwrap_or(0);
            let created_relation_count = metrics["created_relation_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0) >= node_count as u64
                && metrics["subsystem_count"].as_u64() == Some(12)
                && batch_target_count >= 64
                && batch_partition_count >= 8
                && created_relation_count >= 16
                && metrics["hot_changed_records"].as_u64().unwrap_or(0)
                    >= batch_target_count + created_relation_count
                && metrics["explicit_result_entities"].as_u64() == Some(16)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == batch_target_count
                && counter_u64(metrics, "relation_slots_touched_by_commit")
                    >= created_relation_count
                && counter_u64(metrics, "partitions_touched_by_commit") >= batch_partition_count
        },
    );

    let rich_propagation_wave_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_geometry_profile_propagation_wave",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryRichCertification,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded =
                seed_pseudorealistic_rocketship_world(&mut runtime, node_count, query_target_count);

            runtime.performance_access().reset_counters();
            let hot_update_started_at = Instant::now();
            let update = update_entity(
                &mut runtime,
                seeded.hot_update_target,
                "rocket.plumbing_and_feed.propagation_patch.rich",
            );
            let hot_update_micros = hot_update_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let context = runtime
                .read_truth()
                .query_plan_context(&snapshot)
                .expect("rocketship rich propagation context");
            let propagation_seeds = vec![
                seeded.traversal_seeds[0],
                seeded.traversal_seeds[1],
                seeded.traversal_seeds[9],
                seeded.traversal_seeds[10],
            ];
            let propagation_packet = PlannedQueryPacket {
                label: "rocketship-pseudorealistic-propagation-rich".to_string(),
                context_id: context,
                scope: QueryScope::ConnectivityTraversal {
                    seeds: Arc::from(propagation_seeds.clone()),
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                    max_depth: Some(3),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                fallback: QueryFallbackContract::StorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(91_003),
                target_count_hint: propagation_seeds.len(),
            };
            let propagation_plan_started_at = Instant::now();
            let propagation_plan = runtime
                .read_truth()
                .plan_query_packet(&snapshot, propagation_packet)
                .expect("planned rocketship rich propagation query");
            let propagation_planning_micros = propagation_plan_started_at.elapsed().as_micros();
            let propagation_execution_started_at = Instant::now();
            let propagation_outcome = runtime
                .read_truth()
                .execute_query_plan(propagation_plan)
                .expect("rocketship rich propagation outcome");
            let propagation_execution_micros =
                propagation_execution_started_at.elapsed().as_micros();

            let explicit_targets = seeded
                .mixed_query_targets
                .iter()
                .take(12)
                .cloned()
                .collect::<Vec<_>>();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "rocketship-pseudorealistic-propagation-explicit-rich",
                explicit_targets.clone(),
            );
            let explicit_started_at = Instant::now();
            let explicit_outcome = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("planned rocketship rich propagation explicit query"),
                )
                .expect("rocketship rich propagation explicit query outcome");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();
            let hot_phase_timing = update.execution.phase_timing.clone();
            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            let elapsed_micros = seeded.entity_commit_micros
                + seeded.relation_commit_micros
                + hot_update_micros
                + propagation_planning_micros
                + propagation_execution_micros
                + explicit_query_micros;
            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "subsystem_count": seeded.subsystem_count,
                    "bootstrap_entity_commit_micros": seeded.entity_commit_micros,
                    "bootstrap_relation_commit_micros": seeded.relation_commit_micros,
                    "hot_update_micros": hot_update_micros,
                    "phase_timing": {
                        "draft_preparation_micros": hot_phase_timing.draft_preparation_micros,
                        "draft_bulk_admission_micros": hot_phase_timing.draft_bulk_admission_micros,
                        "draft_merge_plan_micros": hot_phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": hot_phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": hot_phase_timing.draft_working_state_clone_micros,
                        "working_state_preparation_micros": hot_phase_timing.working_state_preparation_micros,
                        "invariant_pre_check_micros": hot_phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": hot_phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": hot_phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": hot_phase_timing.invariant_post_check_micros,
                        "artifact_assembly_micros": hot_phase_timing.artifact_assembly_micros,
                        "durable_append_micros": hot_phase_timing.durable_append_micros,
                        "publication_micros": hot_phase_timing.publication_micros,
                    },
                    "propagation_planning_micros": propagation_planning_micros,
                    "propagation_execution_micros": propagation_execution_micros,
                    "explicit_query_micros": explicit_query_micros,
                    "hot_changed_records": update.changed_records.len(),
                    "propagation_seed_count": propagation_seeds.len(),
                    "propagation_result_entities": propagation_outcome.result.entities.len(),
                    "propagation_result_relations": propagation_outcome.result.relations.len(),
                    "explicit_target_count": explicit_targets.len(),
                    "explicit_result_entities": explicit_outcome.result.entities.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_geometry_profile_propagation_wave",
        &rich_propagation_wave_samples,
        &[
            ("resident_node_count", &["resident_node_count"]),
            ("resident_relation_count", &["resident_relation_count"]),
            ("subsystem_count", &["subsystem_count"]),
            (
                "bootstrap_entity_commit_micros",
                &["bootstrap_entity_commit_micros"],
            ),
            (
                "bootstrap_relation_commit_micros",
                &["bootstrap_relation_commit_micros"],
            ),
            ("hot_update_micros", &["hot_update_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "propagation_planning_micros",
                &["propagation_planning_micros"],
            ),
            (
                "propagation_execution_micros",
                &["propagation_execution_micros"],
            ),
            ("explicit_query_micros", &["explicit_query_micros"]),
            ("propagation_seed_count", &["propagation_seed_count"]),
            (
                "propagation_result_entities",
                &["propagation_result_entities"],
            ),
            (
                "propagation_result_relations",
                &["propagation_result_relations"],
            ),
            ("explicit_target_count", &["explicit_target_count"]),
            ("explicit_result_entities", &["explicit_result_entities"]),
            ("diagnostic_artifact_count", &["diagnostic_artifact_count"]),
            ("detailed_trace_entries", &["detailed_trace_entries"]),
        ],
    );
    assert!(rich_propagation_wave_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &rich_propagation_wave_samples,
        "rocketship geometry-profile propagation waves should preserve bounded mixed-locality execution while deferring hot detailed traces",
        |metrics| {
            let propagation_seed_count =
                metrics["propagation_seed_count"].as_u64().unwrap_or(0);
            let explicit_target_count =
                metrics["explicit_target_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0) >= node_count as u64
                && metrics["subsystem_count"].as_u64() == Some(12)
                && metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["propagation_result_entities"].as_u64().unwrap_or(0)
                    >= propagation_seed_count
                && metrics["propagation_result_relations"].as_u64().unwrap_or(0) >= 1
                && metrics["explicit_result_entities"].as_u64() == Some(explicit_target_count)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == 1
                && counter_u64(metrics, "query_packet_count") <= 32
                && counter_u64(metrics, "query_scope_unit_count")
                    <= propagation_seed_count + explicit_target_count
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_sustained_load_matrix() {
    let suite = "sustained_load_matrix";

    let commit_query_churn_samples =
        capture_perf_samples(suite, "commit_query_churn_stability", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore);
            const ITERATIONS: usize = 128;
            let mut total_commit_micros = 0u128;
            let mut total_query_micros = 0u128;
            let mut max_query_packets_per_iteration = 0usize;
            let mut max_query_scope_units_per_iteration = 0usize;
            let mut previous_scope_units = 0usize;

            runtime.performance_access().reset_counters();
            for index in 0..ITERATIONS {
                let commit_started_at = Instant::now();
                let outcome = create_entity_outcome(&mut runtime, &format!("sustained-{index}"));
                total_commit_micros += commit_started_at.elapsed().as_micros();

                let entity = changed_entities(&outcome)[0];
                let snapshot = runtime.visibility_authority().snapshot();
                let packet = explicit_query_packet(
                    &runtime,
                    &snapshot,
                    "sustained-explicit-target",
                    vec![RecordRef::Entity(entity)],
                );
                let query_started_at = Instant::now();
                let query_outcome = runtime
                    .read_truth()
                    .execute_query_plan(
                        runtime
                            .read_truth()
                            .plan_query_packet(&snapshot, packet)
                            .expect("planned sustained explicit query"),
                    )
                    .expect("sustained explicit query outcome");
                total_query_micros += query_started_at.elapsed().as_micros();
                max_query_packets_per_iteration =
                    max_query_packets_per_iteration.max(query_outcome.complexity.packet_count);
                let scope_units = runtime
                    .performance_access()
                    .counters()
                    .query_scope_unit_count;
                max_query_scope_units_per_iteration = max_query_scope_units_per_iteration
                    .max(scope_units.saturating_sub(previous_scope_units));
                previous_scope_units = scope_units;
            }

            let latest_version = runtime
                .history()
                .latest_commit()
                .expect("latest sustained commit")
                .version_id;
            let final_entity_count = runtime
                .read_truth()
                .project_version(latest_version)
                .all_entity_records()
                .len();
            let counters = runtime.performance_access().counters();

            let elapsed_micros = total_commit_micros + total_query_micros;
            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "iterations": ITERATIONS,
                    "average_commit_micros": total_commit_micros / ITERATIONS as u128,
                    "average_query_micros": total_query_micros / ITERATIONS as u128,
                    "max_query_packets_per_iteration": max_query_packets_per_iteration,
                    "max_query_scope_units_per_iteration": max_query_scope_units_per_iteration,
                    "final_entity_count": final_entity_count,
                    "counters": counters,
                })
            })
        });
    emit_metric_summaries(
        suite,
        "commit_query_churn_stability",
        &commit_query_churn_samples,
        &[
            ("average_commit_micros", &["average_commit_micros"]),
            ("average_query_micros", &["average_query_micros"]),
            (
                "max_query_packets_per_iteration",
                &["max_query_packets_per_iteration"],
            ),
            (
                "max_query_scope_units_per_iteration",
                &["max_query_scope_units_per_iteration"],
            ),
            ("final_entity_count", &["final_entity_count"]),
        ],
    );
    assert!(commit_query_churn_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &commit_query_churn_samples,
        "sustained commit/query churn should stay clone-free and packet-stable across long iteration windows",
        |metrics| {
            metrics["iterations"].as_u64() == Some(128)
                && metrics["final_entity_count"].as_u64() == Some(128)
                && metrics["max_query_packets_per_iteration"].as_u64() == Some(1)
                && metrics["max_query_scope_units_per_iteration"].as_u64() == Some(1)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "query_packet_count") == 128
                && counter_u64(metrics, "query_scope_unit_count") == 128
        },
    );

    let replay_window_drift_samples =
        capture_perf_samples(suite, "replay_window_drift_stability", || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::CertificationCore,
            );
            const HISTORY_DEPTH: usize = 48;
            const REPLAY_WINDOW: usize = 32;
            let mut commit_ids = Vec::with_capacity(HISTORY_DEPTH);
            for index in 0..HISTORY_DEPTH {
                let outcome =
                    create_entity_outcome(&mut runtime, &format!("replay-window-{index}"));
                commit_ids.push(outcome.commit.commit_id);
            }

            runtime.performance_access().reset_counters();
            let mut total_replay_micros = 0u128;
            let mut max_replay_micros = 0u128;
            let mut total_compared_surface_count = 0usize;
            let mut total_reconstructed_commit_closure = 0usize;
            let mut total_mismatch_count = 0usize;
            let mut replayed_commit_count = 0usize;

            for commit_id in commit_ids.iter().rev().take(REPLAY_WINDOW) {
                let replay_started_at = Instant::now();
                let outcome = runtime
                    .replay_authority()
                    .replay_commit(RelationalReplayRequest {
                        commit_id: *commit_id,
                        branch_id: BranchId("main".to_string()),
                        execution_mode: ReplayExecutionMode::SerialDeterministic,
                        verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                    });
                let replay_micros = replay_started_at.elapsed().as_micros();
                assert!(
                    outcome.failure.is_none(),
                    "replay window drift sample should not fail: {:?}",
                    outcome.failure
                );
                total_replay_micros += replay_micros;
                max_replay_micros = max_replay_micros.max(replay_micros);
                total_compared_surface_count += outcome.compared_surfaces.len();
                total_reconstructed_commit_closure += outcome.reconstructed_commit_closure.len();
                total_mismatch_count += outcome.mismatches.len();
                replayed_commit_count += 1;
            }

            measurement_with_elapsed(total_replay_micros, || {
                json!({
                    "history_depth": HISTORY_DEPTH,
                    "replay_window": REPLAY_WINDOW,
                    "average_replay_micros": total_replay_micros / REPLAY_WINDOW as u128,
                    "max_replay_micros": max_replay_micros,
                    "replayed_commit_count": replayed_commit_count,
                    "total_compared_surface_count": total_compared_surface_count,
                    "total_reconstructed_commit_closure": total_reconstructed_commit_closure,
                    "total_mismatch_count": total_mismatch_count,
                    "counters": runtime.performance_access().counters(),
                })
            })
        });
    emit_metric_summaries(
        suite,
        "replay_window_drift_stability",
        &replay_window_drift_samples,
        &[
            ("average_replay_micros", &["average_replay_micros"]),
            ("max_replay_micros", &["max_replay_micros"]),
            ("replayed_commit_count", &["replayed_commit_count"]),
            (
                "total_compared_surface_count",
                &["total_compared_surface_count"],
            ),
            (
                "total_reconstructed_commit_closure",
                &["total_reconstructed_commit_closure"],
            ),
        ],
    );
    assert!(replay_window_drift_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &replay_window_drift_samples,
        "replay drift windows should stay mismatch-free while replaying a bounded recent history slice",
        |metrics| {
            metrics["history_depth"].as_u64() == Some(48)
                && metrics["replay_window"].as_u64() == Some(32)
                && metrics["replayed_commit_count"].as_u64() == Some(32)
                && metrics["total_mismatch_count"].as_u64() == Some(0)
                && metrics["total_compared_surface_count"].as_u64().unwrap_or(0) >= 32
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "replay_lineage_authority_lookup_requests") == 32
        },
    );

    let retention_pass_drift_samples =
        capture_perf_samples(suite, "retention_pass_drift_stability", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore);
            const ITERATIONS: usize = 48;
            let mut total_inspect_micros = 0u128;
            let mut total_run_pass_micros = 0u128;
            let mut total_entity_reclaimable = 0usize;
            let mut total_entity_reclaimed = 0usize;
            let mut max_reclaimable_entities = 0usize;

            runtime.performance_access().reset_counters();
            for index in 0..ITERATIONS {
                let created =
                    create_entity_outcome(&mut runtime, &format!("retention-drift-{index}"));
                let entity = changed_entities(&created)[0];
                let deleted = delete_entity(&mut runtime, entity);
                assert!(runtime
                    .visibility_authority()
                    .release_snapshot(&created.snapshot));
                assert!(runtime
                    .visibility_authority()
                    .release_snapshot(&deleted.snapshot));

                let inspect_started_at = Instant::now();
                let plan = runtime.retention().inspect_plan();
                total_inspect_micros += inspect_started_at.elapsed().as_micros();
                max_reclaimable_entities = max_reclaimable_entities.max(plan.reclaimable_entities);

                let run_pass_started_at = Instant::now();
                let pass = runtime.retention().run_pass();
                total_run_pass_micros += run_pass_started_at.elapsed().as_micros();
                total_entity_reclaimable += pass.entity_reclaimable;
                total_entity_reclaimed += pass.entity_reclaimed;
            }

            let trailing_plan = runtime.retention().inspect_plan();
            let elapsed_micros = total_inspect_micros + total_run_pass_micros;
            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "iterations": ITERATIONS,
                    "average_inspect_micros": total_inspect_micros / ITERATIONS as u128,
                    "average_run_pass_micros": total_run_pass_micros / ITERATIONS as u128,
                    "total_entity_reclaimable": total_entity_reclaimable,
                    "total_entity_reclaimed": total_entity_reclaimed,
                    "max_reclaimable_entities": max_reclaimable_entities,
                    "trailing_reclaimable_entities": trailing_plan.reclaimable_entities,
                    "counters": runtime.performance_access().counters(),
                })
            })
        });
    emit_metric_summaries(
        suite,
        "retention_pass_drift_stability",
        &retention_pass_drift_samples,
        &[
            ("average_inspect_micros", &["average_inspect_micros"]),
            ("average_run_pass_micros", &["average_run_pass_micros"]),
            ("total_entity_reclaimable", &["total_entity_reclaimable"]),
            ("total_entity_reclaimed", &["total_entity_reclaimed"]),
            ("max_reclaimable_entities", &["max_reclaimable_entities"]),
        ],
    );
    assert!(retention_pass_drift_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &retention_pass_drift_samples,
        "retention drift windows should surface reclaimable released deletions without rebuild-heavy retention behavior",
        |metrics| {
            metrics["iterations"].as_u64() == Some(48)
                && metrics["total_entity_reclaimable"].as_u64().unwrap_or(0) >= 48
                && metrics["total_entity_reclaimed"].as_u64() == Some(0)
                && metrics["trailing_reclaimable_entities"].as_u64() == Some(48)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "snapshot_pin_full_rebuilds") == 0
        },
    );

    let mixed_topology_churn_samples = capture_perf_samples(
        suite,
        "mixed_topology_query_churn_stability",
        || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
            let entities = (0..24)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("mixed-topology-{index}"),
                        PartitionId((index % 6) as u32 + 1),
                    )
                })
                .collect::<Vec<_>>();
            for index in 0..24 {
                create_relation_in_partition(
                    &mut runtime,
                    entities[index],
                    entities[(index + 1) % 24],
                    &format!("mixed-ring-{index}"),
                    PartitionId(40 + (index % 4) as u32),
                );
                if index % 4 == 0 {
                    create_relation_in_partition(
                        &mut runtime,
                        entities[index],
                        entities[(index + 6) % 24],
                        &format!("mixed-brace-{index}"),
                        PartitionId(50 + (index % 3) as u32),
                    );
                }
            }

            const ITERATIONS: usize = 48;
            let mut total_update_micros = 0u128;
            let mut total_explicit_query_micros = 0u128;
            let mut total_traversal_micros = 0u128;
            let mut max_packets_per_iteration = 0usize;
            let mut max_scope_units_per_iteration = 0usize;
            let mut previous_packets = 0usize;
            let mut previous_scope_units = 0usize;

            runtime.performance_access().reset_counters();
            for index in 0..ITERATIONS {
                let hot_entity = entities[(index * 3) % entities.len()];
                let update_started_at = Instant::now();
                let _ = update_entity(
                    &mut runtime,
                    hot_entity,
                    &format!("mixed-topology-hot-{index}"),
                );
                total_update_micros += update_started_at.elapsed().as_micros();

                let snapshot = runtime.visibility_authority().snapshot();
                let explicit_targets = vec![
                    RecordRef::Entity(entities[(index * 3) % entities.len()]),
                    RecordRef::Entity(entities[(index * 3 + 1) % entities.len()]),
                    RecordRef::Entity(entities[(index * 3 + 6) % entities.len()]),
                    RecordRef::Entity(entities[(index * 3 + 12) % entities.len()]),
                ];
                let explicit_packet = explicit_query_packet(
                    &runtime,
                    &snapshot,
                    "mixed-topology-explicit",
                    explicit_targets,
                );
                let explicit_started_at = Instant::now();
                let _ = runtime
                    .read_truth()
                    .execute_query_plan(
                        runtime
                            .read_truth()
                            .plan_query_packet(&snapshot, explicit_packet)
                            .expect("planned mixed topology explicit query"),
                    )
                    .expect("mixed topology explicit query outcome");
                total_explicit_query_micros += explicit_started_at.elapsed().as_micros();

                let context = runtime
                    .read_truth()
                    .query_plan_context(&snapshot)
                    .expect("mixed topology query plan context");
                let traversal_packet = PlannedQueryPacket {
                    label: "mixed-topology-traversal".to_string(),
                    context_id: context,
                    scope: QueryScope::ConnectivityTraversal {
                        seeds: Arc::from([
                            entities[(index * 3) % entities.len()],
                            entities[(index * 3 + 6) % entities.len()],
                        ]),
                        relation_kind_scope: Some(Arc::from([KindId(2)])),
                        max_depth: Some(2),
                    },
                    locality: QueryLocalityClass::CrossPartitionTraversal,
                    ordering: QueryOrderingContract::CanonicalTraversalOrder,
                    fallback: QueryFallbackContract::StorageOnly,
                    execution_shape: QueryExecutionShape::BulkPacketized,
                    reduction: ReductionDiscipline::DeterministicMerge,
                    plan_key: DeterministicQueryPlanKey(92_001),
                    target_count_hint: 2,
                };
                let traversal_started_at = Instant::now();
                let _ = runtime
                    .read_truth()
                    .execute_query_plan(
                        runtime
                            .read_truth()
                            .plan_query_packet(&snapshot, traversal_packet)
                            .expect("planned mixed topology traversal query"),
                    )
                    .expect("mixed topology traversal outcome");
                total_traversal_micros += traversal_started_at.elapsed().as_micros();

                let counters = runtime.performance_access().counters();
                max_packets_per_iteration = max_packets_per_iteration
                    .max(counters.query_packet_count.saturating_sub(previous_packets));
                max_scope_units_per_iteration = max_scope_units_per_iteration.max(
                    counters
                        .query_scope_unit_count
                        .saturating_sub(previous_scope_units),
                );
                previous_packets = counters.query_packet_count;
                previous_scope_units = counters.query_scope_unit_count;
            }

            let elapsed_micros =
                total_update_micros + total_explicit_query_micros + total_traversal_micros;
            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "iterations": ITERATIONS,
                    "average_update_micros": total_update_micros / ITERATIONS as u128,
                    "average_explicit_query_micros": total_explicit_query_micros / ITERATIONS as u128,
                    "average_traversal_micros": total_traversal_micros / ITERATIONS as u128,
                    "max_packets_per_iteration": max_packets_per_iteration,
                    "max_scope_units_per_iteration": max_scope_units_per_iteration,
                    "counters": runtime.performance_access().counters(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "mixed_topology_query_churn_stability",
        &mixed_topology_churn_samples,
        &[
            ("average_update_micros", &["average_update_micros"]),
            (
                "average_explicit_query_micros",
                &["average_explicit_query_micros"],
            ),
            ("average_traversal_micros", &["average_traversal_micros"]),
            ("max_packets_per_iteration", &["max_packets_per_iteration"]),
            (
                "max_scope_units_per_iteration",
                &["max_scope_units_per_iteration"],
            ),
        ],
    );
    assert!(mixed_topology_churn_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &mixed_topology_churn_samples,
        "mixed sustained topology churn should keep packet and scope growth bounded across repeated update plus read waves",
        |metrics| {
            metrics["iterations"].as_u64() == Some(48)
                && metrics["max_packets_per_iteration"].as_u64().unwrap_or(0) <= 8
                && metrics["max_scope_units_per_iteration"].as_u64().unwrap_or(0) <= 8
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "query_packet_count") >= 96
                && counter_u64(metrics, "query_scope_unit_count") >= 96
        },
    );

    let rocketship_endurance_node_count = rocketship_node_count();
    let rocketship_hot_update_endurance_samples =
        capture_perf_samples(suite, "rocketship_hot_update_endurance", || {
            let query_target_count = rocketship_query_target_count(rocketship_endurance_node_count);
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = rocketship_endurance_node_count * 2;
            let seeded = seed_pseudorealistic_rocketship_world(
                &mut runtime,
                rocketship_endurance_node_count,
                query_target_count,
            );

            const ITERATIONS: usize = 256;
            const WINDOW: usize = 32;
            let mut update_samples = Vec::with_capacity(ITERATIONS);
            let mut total_update_micros = 0u128;
            let mut max_update_micros = 0u128;
            let mut max_query_micros = 0u128;

            runtime.performance_access().reset_counters();
            for index in 0..ITERATIONS {
                let target = seeded.traversal_seeds[index % seeded.traversal_seeds.len()];
                let update_started_at = Instant::now();
                let _ = update_entity(
                    &mut runtime,
                    target,
                    &format!("rocket.endurance.hot-loop.{index}"),
                );
                let update_micros = update_started_at.elapsed().as_micros();
                update_samples.push(update_micros);
                total_update_micros += update_micros;
                max_update_micros = max_update_micros.max(update_micros);

                if index % 16 == 0 {
                    let snapshot = runtime.visibility_authority().snapshot();
                    let explicit_targets = seeded
                        .mixed_query_targets
                        .iter()
                        .skip(index % seeded.mixed_query_targets.len())
                        .take(8)
                        .cloned()
                        .collect::<Vec<_>>();
                    let packet = explicit_query_packet(
                        &runtime,
                        &snapshot,
                        "rocketship-endurance-explicit",
                        explicit_targets,
                    );
                    let query_started_at = Instant::now();
                    let _ = runtime
                        .read_truth()
                        .execute_query_plan(
                            runtime
                                .read_truth()
                                .plan_query_packet(&snapshot, packet)
                                .expect("planned rocketship endurance explicit query"),
                        )
                        .expect("rocketship endurance explicit query outcome");
                    max_query_micros = max_query_micros.max(query_started_at.elapsed().as_micros());
                    assert!(runtime.visibility_authority().release_snapshot(&snapshot));
                }
            }

            let first_window_average_update_micros =
                update_samples.iter().take(WINDOW).copied().sum::<u128>() / WINDOW as u128;
            let last_window_average_update_micros = update_samples
                .iter()
                .rev()
                .take(WINDOW)
                .copied()
                .sum::<u128>()
                / WINDOW as u128;
            measurement_with_elapsed(total_update_micros, || {
                json!({
                    "iterations": ITERATIONS,
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "average_update_micros": total_update_micros / ITERATIONS as u128,
                    "max_update_micros": max_update_micros,
                    "first_window_average_update_micros": first_window_average_update_micros,
                    "last_window_average_update_micros": last_window_average_update_micros,
                    "max_explicit_query_micros": max_query_micros,
                    "counters": runtime.performance_access().counters(),
                })
            })
        });
    emit_metric_summaries(
        suite,
        "rocketship_hot_update_endurance",
        &rocketship_hot_update_endurance_samples,
        &[
            ("iterations", &["iterations"]),
            ("resident_node_count", &["resident_node_count"]),
            ("resident_relation_count", &["resident_relation_count"]),
            ("average_update_micros", &["average_update_micros"]),
            ("max_update_micros", &["max_update_micros"]),
            (
                "first_window_average_update_micros",
                &["first_window_average_update_micros"],
            ),
            (
                "last_window_average_update_micros",
                &["last_window_average_update_micros"],
            ),
            ("max_explicit_query_micros", &["max_explicit_query_micros"]),
        ],
    );
    assert_budget(
        &rocketship_hot_update_endurance_samples,
        "rocketship hot endurance should stay region-local and resist drift across long update windows",
        |metrics| {
            let first_window = metrics["first_window_average_update_micros"]
                .as_u64()
                .unwrap_or(0);
            let last_window = metrics["last_window_average_update_micros"]
                .as_u64()
                .unwrap_or(0);
            metrics["iterations"].as_u64() == Some(256)
                && metrics["resident_node_count"]
                    .as_u64()
                    == Some(rocketship_endurance_node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0)
                    >= rocketship_endurance_node_count as u64
                && last_window <= first_window.saturating_mul(2).max(1)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "bulk_mutation_batch_count") == 256
                && counter_u64(metrics, "partitions_cloned") <= 256
        },
    );

    let rocketship_propagation_endurance_samples = capture_perf_samples(
        suite,
        "rocketship_propagation_endurance",
        || {
            let query_target_count = rocketship_query_target_count(rocketship_endurance_node_count);
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = rocketship_endurance_node_count * 2;
            let seeded = seed_pseudorealistic_rocketship_world(
                &mut runtime,
                rocketship_endurance_node_count,
                query_target_count,
            );

            const ITERATIONS: usize = 96;
            const WINDOW: usize = 16;
            let mut cycle_samples = Vec::with_capacity(ITERATIONS);
            let mut total_update_micros = 0u128;
            let mut total_propagation_micros = 0u128;
            let mut total_explicit_query_micros = 0u128;
            let mut max_packets_per_iteration = 0usize;
            let mut max_scope_units_per_iteration = 0usize;
            let mut previous_packets = 0usize;
            let mut previous_scope_units = 0usize;

            runtime.performance_access().reset_counters();
            for index in 0..ITERATIONS {
                let target = seeded.traversal_seeds[index % seeded.traversal_seeds.len()];
                let update_started_at = Instant::now();
                let _ = update_entity(
                    &mut runtime,
                    target,
                    &format!("rocket.endurance.propagation.{index}"),
                );
                let update_micros = update_started_at.elapsed().as_micros();
                total_update_micros += update_micros;

                let snapshot = runtime.visibility_authority().snapshot();
                let context = runtime
                    .read_truth()
                    .query_plan_context(&snapshot)
                    .expect("rocketship endurance propagation context");
                let propagation_seeds = vec![
                    seeded.traversal_seeds[index % seeded.traversal_seeds.len()],
                    seeded.traversal_seeds[(index + 1) % seeded.traversal_seeds.len()],
                    seeded.traversal_seeds[(index + 9) % seeded.traversal_seeds.len()],
                    seeded.traversal_seeds[(index + 10) % seeded.traversal_seeds.len()],
                ];
                let propagation_packet = PlannedQueryPacket {
                    label: "rocketship-endurance-propagation".to_string(),
                    context_id: context,
                    scope: QueryScope::ConnectivityTraversal {
                        seeds: Arc::from(propagation_seeds),
                        relation_kind_scope: Some(Arc::from([KindId(2)])),
                        max_depth: Some(3),
                    },
                    locality: QueryLocalityClass::CrossPartitionTraversal,
                    ordering: QueryOrderingContract::CanonicalTraversalOrder,
                    fallback: QueryFallbackContract::StorageOnly,
                    execution_shape: QueryExecutionShape::BulkPacketized,
                    reduction: ReductionDiscipline::DeterministicMerge,
                    plan_key: DeterministicQueryPlanKey(92_250),
                    target_count_hint: 4,
                };
                let propagation_started_at = Instant::now();
                let _ = runtime
                    .read_truth()
                    .execute_query_plan(
                        runtime
                            .read_truth()
                            .plan_query_packet(&snapshot, propagation_packet)
                            .expect("planned rocketship endurance propagation query"),
                    )
                    .expect("rocketship endurance propagation outcome");
                let propagation_micros = propagation_started_at.elapsed().as_micros();
                total_propagation_micros += propagation_micros;

                let explicit_targets = seeded
                    .mixed_query_targets
                    .iter()
                    .cycle()
                    .skip(index)
                    .take(12)
                    .cloned()
                    .collect::<Vec<_>>();
                let explicit_packet = explicit_query_packet(
                    &runtime,
                    &snapshot,
                    "rocketship-endurance-explicit-broad",
                    explicit_targets,
                );
                let explicit_started_at = Instant::now();
                let _ = runtime
                    .read_truth()
                    .execute_query_plan(
                        runtime
                            .read_truth()
                            .plan_query_packet(&snapshot, explicit_packet)
                            .expect("planned rocketship endurance explicit broad query"),
                    )
                    .expect("rocketship endurance explicit broad outcome");
                let explicit_query_micros = explicit_started_at.elapsed().as_micros();
                total_explicit_query_micros += explicit_query_micros;
                assert!(runtime.visibility_authority().release_snapshot(&snapshot));

                let cycle_micros = update_micros + propagation_micros + explicit_query_micros;
                cycle_samples.push(cycle_micros);

                let counters = runtime.performance_access().counters();
                max_packets_per_iteration = max_packets_per_iteration
                    .max(counters.query_packet_count.saturating_sub(previous_packets));
                max_scope_units_per_iteration = max_scope_units_per_iteration.max(
                    counters
                        .query_scope_unit_count
                        .saturating_sub(previous_scope_units),
                );
                previous_packets = counters.query_packet_count;
                previous_scope_units = counters.query_scope_unit_count;
            }

            let first_window_average_cycle_micros =
                cycle_samples.iter().take(WINDOW).copied().sum::<u128>() / WINDOW as u128;
            let last_window_average_cycle_micros = cycle_samples
                .iter()
                .rev()
                .take(WINDOW)
                .copied()
                .sum::<u128>()
                / WINDOW as u128;
            let elapsed_micros =
                total_update_micros + total_propagation_micros + total_explicit_query_micros;
            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "iterations": ITERATIONS,
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "average_update_micros": total_update_micros / ITERATIONS as u128,
                    "average_propagation_micros": total_propagation_micros / ITERATIONS as u128,
                    "average_explicit_query_micros": total_explicit_query_micros / ITERATIONS as u128,
                    "first_window_average_cycle_micros": first_window_average_cycle_micros,
                    "last_window_average_cycle_micros": last_window_average_cycle_micros,
                    "max_packets_per_iteration": max_packets_per_iteration,
                    "max_scope_units_per_iteration": max_scope_units_per_iteration,
                    "counters": runtime.performance_access().counters(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "rocketship_propagation_endurance",
        &rocketship_propagation_endurance_samples,
        &[
            ("iterations", &["iterations"]),
            ("resident_node_count", &["resident_node_count"]),
            ("resident_relation_count", &["resident_relation_count"]),
            ("average_update_micros", &["average_update_micros"]),
            (
                "average_propagation_micros",
                &["average_propagation_micros"],
            ),
            (
                "average_explicit_query_micros",
                &["average_explicit_query_micros"],
            ),
            (
                "first_window_average_cycle_micros",
                &["first_window_average_cycle_micros"],
            ),
            (
                "last_window_average_cycle_micros",
                &["last_window_average_cycle_micros"],
            ),
            ("max_packets_per_iteration", &["max_packets_per_iteration"]),
            (
                "max_scope_units_per_iteration",
                &["max_scope_units_per_iteration"],
            ),
        ],
    );
    assert_budget(
        &rocketship_propagation_endurance_samples,
        "rocketship propagation endurance should keep broad-wave cycles bounded across extended 100k-node operation",
        |metrics| {
            let first_window = metrics["first_window_average_cycle_micros"]
                .as_u64()
                .unwrap_or(0);
            let last_window = metrics["last_window_average_cycle_micros"]
                .as_u64()
                .unwrap_or(0);
            metrics["iterations"].as_u64() == Some(96)
                && metrics["resident_node_count"]
                    .as_u64()
                    == Some(rocketship_endurance_node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0)
                    >= rocketship_endurance_node_count as u64
                && last_window <= first_window.saturating_mul(2).max(1)
                && metrics["max_packets_per_iteration"].as_u64().unwrap_or(0) <= 24
                && metrics["max_scope_units_per_iteration"].as_u64().unwrap_or(0) <= 24
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "bulk_mutation_batch_count") == 96
        },
    );

    let chip_global_step_endurance_samples =
        capture_perf_samples(suite, "chip_global_step_endurance", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::ChipOperationalHotPath,
            );

            let drivers = (0..8)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("chip-global-driver-{index}"),
                        PartitionId(930 + index as u32),
                    )
                })
                .collect::<Vec<_>>();
            let sinks = (0..64)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("chip-global-sink-{index}"),
                        PartitionId(950 + (index % 8) as u32),
                    )
                })
                .collect::<Vec<_>>();

            for (index, driver) in drivers.iter().enumerate() {
                for fanout in 0..8 {
                    let sink = sinks[index * 8 + fanout];
                    create_relation_in_partition(
                        &mut runtime,
                        *driver,
                        sink,
                        &format!("chip-global-fanout-{index}-{fanout}"),
                        PartitionId(980 + index as u32),
                    );
                }
            }
            for index in 0..(sinks.len() - 1) {
                create_relation_in_partition(
                    &mut runtime,
                    sinks[index],
                    sinks[index + 1],
                    &format!("chip-global-chain-{index}"),
                    PartitionId(990 + (index % 4) as u32),
                );
            }

            const ITERATIONS: usize = 128;
            const WINDOW: usize = 32;
            let mut cycle_samples = Vec::with_capacity(ITERATIONS);
            let mut total_update_micros = 0u128;
            let mut total_compile_micros = 0u128;
            let mut total_adjacency_micros = 0u128;
            let mut max_compile_micros = 0u128;
            let mut max_outgoing_relation_count = 0usize;

            runtime.performance_access().reset_counters();
            for step in 0..ITERATIONS {
                let driver = drivers[step % drivers.len()];
                let update_started_at = Instant::now();
                let _ = update_entity(
                    &mut runtime,
                    driver,
                    &format!("chip-global-driver-step-{step}"),
                );
                let update_micros = update_started_at.elapsed().as_micros();
                total_update_micros += update_micros;

                let commit = runtime
                    .history()
                    .latest_commit()
                    .expect("chip global step commit")
                    .clone();
                let compile_started_at = Instant::now();
                let artifact = runtime
                    .compiled_artifacts_authority()
                    .compile_execution_artifact(
                        commit.commit_id,
                        vec![
                            PartitionId(930),
                            PartitionId(931),
                            PartitionId(932),
                            PartitionId(933),
                            PartitionId(934),
                            PartitionId(935),
                            PartitionId(936),
                            PartitionId(937),
                            PartitionId(950),
                            PartitionId(951),
                            PartitionId(952),
                            PartitionId(953),
                            PartitionId(954),
                            PartitionId(955),
                            PartitionId(956),
                            PartitionId(957),
                        ],
                    )
                    .expect("chip global step compiled artifact");
                let compile_micros = compile_started_at.elapsed().as_micros();
                total_compile_micros += compile_micros;
                max_compile_micros = max_compile_micros.max(compile_micros);

                let adjacency_started_at = Instant::now();
                let outgoing_relations = runtime
                    .storage_access()
                    .outgoing_relations_for_entity(driver, commit.version_id);
                let adjacency_micros = adjacency_started_at.elapsed().as_micros();
                total_adjacency_micros += adjacency_micros;
                max_outgoing_relation_count =
                    max_outgoing_relation_count.max(outgoing_relations.len());
                assert_eq!(
                    runtime
                        .compiled_artifacts()
                        .compiled_artifact_compatibility(artifact.artifact_id),
                    CompiledArtifactCompatibility::Compatible
                );

                cycle_samples.push(update_micros + compile_micros + adjacency_micros);
            }

            let first_window_average_cycle_micros =
                cycle_samples.iter().take(WINDOW).copied().sum::<u128>() / WINDOW as u128;
            let last_window_average_cycle_micros = cycle_samples
                .iter()
                .rev()
                .take(WINDOW)
                .copied()
                .sum::<u128>()
                / WINDOW as u128;
            let elapsed_micros =
                total_update_micros + total_compile_micros + total_adjacency_micros;
            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "iterations": ITERATIONS,
                    "driver_count": drivers.len(),
                    "sink_count": sinks.len(),
                    "average_update_micros": total_update_micros / ITERATIONS as u128,
                    "average_compile_micros": total_compile_micros / ITERATIONS as u128,
                    "average_adjacency_micros": total_adjacency_micros / ITERATIONS as u128,
                    "first_window_average_cycle_micros": first_window_average_cycle_micros,
                    "last_window_average_cycle_micros": last_window_average_cycle_micros,
                    "max_compile_micros": max_compile_micros,
                    "max_outgoing_relation_count": max_outgoing_relation_count,
                    "counters": runtime.performance_access().counters(),
                })
            })
        });
    emit_metric_summaries(
        suite,
        "chip_global_step_endurance",
        &chip_global_step_endurance_samples,
        &[
            ("iterations", &["iterations"]),
            ("average_update_micros", &["average_update_micros"]),
            ("average_compile_micros", &["average_compile_micros"]),
            ("average_adjacency_micros", &["average_adjacency_micros"]),
            (
                "first_window_average_cycle_micros",
                &["first_window_average_cycle_micros"],
            ),
            (
                "last_window_average_cycle_micros",
                &["last_window_average_cycle_micros"],
            ),
            ("max_compile_micros", &["max_compile_micros"]),
            (
                "max_outgoing_relation_count",
                &["max_outgoing_relation_count"],
            ),
        ],
    );
    assert_budget(
        &chip_global_step_endurance_samples,
        "chip global step endurance should keep repeated denser fanout stepping proportional across a longer sustained window",
        |metrics| {
            let first_window = metrics["first_window_average_cycle_micros"]
                .as_u64()
                .unwrap_or(0);
            let last_window = metrics["last_window_average_cycle_micros"]
                .as_u64()
                .unwrap_or(0);
            metrics["iterations"].as_u64() == Some(128)
                && metrics["driver_count"].as_u64() == Some(8)
                && metrics["sink_count"].as_u64() == Some(64)
                && metrics["max_outgoing_relation_count"].as_u64().unwrap_or(0) >= 8
                && last_window <= first_window.saturating_mul(2).max(1)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "bulk_mutation_entity_target_count") == 128
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_inspection_budget_matrix() {
    let suite = "inspection_budget_matrix";

    let graph_kind_connectivity_samples =
        capture_perf_samples(suite, "graph_kind_connectivity_bundle", || {
            let mut runtime = runtime_with_test_schema();
            let left_a = create_entity_in_partition(&mut runtime, "left-a", PartitionId(7));
            let _left_b = create_entity_in_partition(&mut runtime, "left-b", PartitionId(7));
            let _isolated = create_entity_in_partition(&mut runtime, "isolated", PartitionId(11));
            let right = create_entity_in_partition(&mut runtime, "right", PartitionId(13));
            let _relation =
                create_relation_in_partition(&mut runtime, left_a, right, "rel", PartitionId(17));

            runtime.performance_access().reset_counters();

            let graph_started_at = Instant::now();
            let graph = runtime
                .inspect_what_happened()
                .graph_summary(&current_graph_request(None, None, true));
            let graph_micros = graph_started_at.elapsed().as_micros();

            let kind_started_at = Instant::now();
            let kind = runtime
                .inspect_what_happened()
                .kind_summary(&KindInspectionRequest {
                    scope: InspectionScope::Current,
                    partition_scope: Some(vec![PartitionId(7)]),
                    kind_id: KindId(1),
                    record_class: InspectionRecordClass::Entity,
                });
            let kind_micros = kind_started_at.elapsed().as_micros();

            let connectivity_started_at = Instant::now();
            let connectivity =
                runtime
                    .inspect_what_happened()
                    .connectivity_summary(&connectivity_request(
                        InspectionScope::Current,
                        None,
                        None,
                        false,
                    ));
            let connectivity_micros = connectivity_started_at.elapsed().as_micros();

            PerfMeasurement {
                elapsed_micros: graph_micros + kind_micros + connectivity_micros,
                metrics: json!({
                    "graph_micros": graph_micros,
                    "kind_micros": kind_micros,
                    "connectivity_micros": connectivity_micros,
                    "graph_entity_count": graph.entity_count,
                    "graph_relation_count": graph.relation_count,
                    "kind_count": kind.count,
                    "connectivity_component_count": connectivity.component_count,
                    "connectivity_largest_component_size": connectivity.largest_component_size,
                    "graph_access_path": format!("{:?}", graph.access_path),
                    "kind_access_path": format!("{:?}", kind.access_path),
                    "connectivity_access_path": format!("{:?}", connectivity.access_path),
                    "counters": runtime.performance_access().counters(),
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "graph_kind_connectivity_bundle",
        &graph_kind_connectivity_samples,
        &[
            ("graph_micros", &["graph_micros"]),
            ("kind_micros", &["kind_micros"]),
            ("connectivity_micros", &["connectivity_micros"]),
            ("graph_entity_count", &["graph_entity_count"]),
            (
                "connectivity_component_count",
                &["connectivity_component_count"],
            ),
        ],
    );
    assert!(graph_kind_connectivity_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &graph_kind_connectivity_samples,
        "inspection bundles should stay request-shaped and avoid visibility materialization",
        |metrics| {
            metric_u64(metrics, "graph_entity_count") == 4
                && metric_u64(metrics, "graph_relation_count") == 1
                && metric_u64(metrics, "kind_count") == 2
                && metric_u64(metrics, "connectivity_component_count") == 3
                && metric_u64(metrics, "connectivity_largest_component_size") == 2
                && counter_u64(metrics, "inspection_graph_summary_requests") == 1
                && counter_u64(metrics, "inspection_kind_summary_requests") == 1
                && counter_u64(metrics, "inspection_connectivity_summary_requests") == 1
                && counter_u64(metrics, "visible_entity_records_materialized") == 4
                && counter_u64(metrics, "visible_relation_records_materialized") == 1
                && counter_u64(metrics, "visibility_entity_slot_scans") == 0
                && counter_u64(metrics, "visibility_relation_slot_scans") == 0
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );

    let structural_identity_samples = capture_perf_samples(
        suite,
        "structural_identity_historical_window",
        || {
            let mut runtime = runtime_with_test_schema();
            let created = create_entity_outcome(&mut runtime, "alpha");
            let entity = changed_entities(&created)[0];
            let _other = create_entity(&mut runtime, "beta");
            assert!(runtime.set_entity_structural_identity_for_test(
                entity,
                Some(crate::facade::identity::StructuralFingerprint::new(
                    Symbol(31),
                    700
                )),
                Some(crate::facade::identity::LineageId(77)),
            ));
            let _updated = update_entity(&mut runtime, entity, "alpha-updated");

            runtime.performance_access().reset_counters();

            let direct_started_at = Instant::now();
            let direct = runtime
                .inspect_what_happened()
                .structural_identity(InspectionScope::Current, RecordRef::Entity(entity))
                .expect("structural identity evidence");
            let direct_micros = direct_started_at.elapsed().as_micros();

            let query_started_at = Instant::now();
            let query = runtime.inspect_what_happened().query_structural_identity(
                &StructuralIdentityQueryRequest {
                    scope: InspectionScope::Current,
                    partition_scope: None,
                    fingerprint_family: Symbol(31),
                },
            );
            let query_micros = query_started_at.elapsed().as_micros();

            let historical_started_at = Instant::now();
            let historical = reconstructed_record_inspection(
                &runtime,
                &BranchId("main".to_string()),
                created.version_id,
                RecordRef::Entity(entity),
            );
            let historical_micros = historical_started_at.elapsed().as_micros();

            PerfMeasurement {
                elapsed_micros: direct_micros + query_micros + historical_micros,
                metrics: json!({
                    "direct_micros": direct_micros,
                    "query_micros": query_micros,
                    "historical_micros": historical_micros,
                    "query_match_count": query.len(),
                    "direct_availability": format!("{:?}", direct.availability),
                    "historical_availability": format!(
                        "{:?}",
                        historical.record_observation.availability
                    ),
                    "historical_has_value": historical.record_observation.value.is_some(),
                    "historical_lineage_context_present": historical.lineage_resolution_context.is_some(),
                    "counters": runtime.performance_access().counters(),
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "structural_identity_historical_window",
        &structural_identity_samples,
        &[
            ("direct_micros", &["direct_micros"]),
            ("query_micros", &["query_micros"]),
            ("historical_micros", &["historical_micros"]),
            ("query_match_count", &["query_match_count"]),
        ],
    );
    assert!(structural_identity_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &structural_identity_samples,
        "structural identity windows should preserve direct lookup, bounded family scans, and retained historical reads",
        |metrics| {
            metric_u64(metrics, "query_match_count") == 1
                && metrics["historical_has_value"].as_bool() == Some(false)
                && metrics["historical_lineage_context_present"].as_bool() == Some(false)
                && metrics["direct_availability"].as_str() == Some("Direct".into())
                && metrics["historical_availability"].as_str() == Some("Reconstructed".into())
                && counter_u64(metrics, "inspection_structural_identity_query_scans") == 1
                && counter_u64(metrics, "inspection_structural_identity_lookups") >= 3
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );

    let retention_commit_samples = capture_perf_samples(suite, "retention_commit_window", || {
        let mut runtime = runtime_with_test_schema();
        let left = create_entity(&mut runtime, "left");
        let right = create_entity(&mut runtime, "right");
        let _relation = create_relation(&mut runtime, left, right, "rel");
        let latest_commit = runtime
            .history()
            .latest_commit()
            .map(|commit| commit.commit_id)
            .expect("latest commit");

        runtime.performance_access().reset_counters();

        let retention_started_at = Instant::now();
        let retention = runtime
            .inspect_what_happened()
            .retention_summary(&default_retention_request());
        let retention_micros = retention_started_at.elapsed().as_micros();

        let commit_started_at = Instant::now();
        let commit = runtime
            .inspect_what_happened()
            .inspect_commit(latest_commit)
            .expect("commit inspection");
        let commit_micros = commit_started_at.elapsed().as_micros();

        let recent_started_at = Instant::now();
        let recent = runtime.inspect_what_happened().inspect_recent_commits(
            &RecentCommitInspectionRequest {
                branch_id: Some(BranchId("main".to_string()).into()),
                limit: 3,
            },
        );
        let recent_micros = recent_started_at.elapsed().as_micros();

        PerfMeasurement {
            elapsed_micros: retention_micros + commit_micros + recent_micros,
            metrics: json!({
                "retention_micros": retention_micros,
                "commit_micros": commit_micros,
                "recent_micros": recent_micros,
                "retention_availability": format!("{:?}", retention.availability),
                "commit_changed_records": commit.changed_records.len(),
                "recent_commit_count": recent.commits.len(),
                "counters": runtime.performance_access().counters(),
            }),
        }
    });
    emit_metric_summaries(
        suite,
        "retention_commit_window",
        &retention_commit_samples,
        &[
            ("retention_micros", &["retention_micros"]),
            ("commit_micros", &["commit_micros"]),
            ("recent_micros", &["recent_micros"]),
            ("recent_commit_count", &["recent_commit_count"]),
        ],
    );
    assert!(retention_commit_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &retention_commit_samples,
        "retention and commit inspection windows should stay index-backed and bounded",
        |metrics| {
            metrics["retention_availability"].as_str() == Some("Direct".into())
                && metric_u64(metrics, "commit_changed_records") == 1
                && metric_u64(metrics, "recent_commit_count") == 3
                && counter_u64(metrics, "inspection_commit_reads") == 4
                && counter_u64(metrics, "inspection_retention_entity_slot_scans") >= 2
                && counter_u64(metrics, "inspection_retention_relation_slot_scans") >= 1
                && counter_u64(metrics, "visible_entity_records_materialized") == 0
                && counter_u64(metrics, "visible_relation_records_materialized") == 0
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_index_parity_matrix() {
    let suite = "index_parity_matrix";

    let warm_generation_samples =
        capture_perf_samples(suite, "entity_field_equals_warm_generation", || {
            let mut runtime = runtime_with_test_schema();
            let alpha = create_entity_outcome(&mut runtime, "alpha");
            let _beta = create_entity_outcome(&mut runtime, "beta");
            let index = runtime.index_authority().register(DerivedIndexDefinition {
                index_id: DerivedIndexId(0),
                name: "entity.name.lookup".to_string(),
                kind: DerivedIndexKind::EntityField {
                    field: field_key("name"),
                },
                branch_scoped: false,
            });

            let build_started_at = Instant::now();
            let build = runtime
                .index_authority()
                .build_for_commit(DerivedIndexBuildRequest {
                    source_commit_id: alpha.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    index_ids: vec![index.index_id],
                });
            let build_micros = build_started_at.elapsed().as_micros();
            assert!(build.failed_indexes.is_empty());

            runtime.performance_access().reset_counters();
            let query_started_at = Instant::now();
            let outcome = runtime
                .index_access()
                .execute_query_plan_with_fallback_parity(
                    runtime
                        .read_truth()
                        .plan_query_packet(
                            &alpha.snapshot,
                            entity_name_index_packet(
                                &runtime,
                                &alpha.snapshot,
                                "entity-name-equals-warm",
                                "alpha",
                            ),
                        )
                        .expect("warm entity query plan"),
                    FallbackParityMode::CertificationParity,
                )
                .expect("warm entity index query outcome");
            let query_micros = query_started_at.elapsed().as_micros();

            PerfMeasurement {
                elapsed_micros: build_micros + query_micros,
                metrics: json!({
                    "build_micros": build_micros,
                    "query_micros": query_micros,
                    "entity_result_count": outcome.execution.result.entities.len(),
                    "relation_result_count": outcome.execution.result.relations.len(),
                    "access_path": format!("{:?}", outcome.access_path),
                    "parity_digest_present": !outcome.parity_basis_digest.is_empty(),
                    "counters": runtime.performance_access().counters(),
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "entity_field_equals_warm_generation",
        &warm_generation_samples,
        &[
            ("build_micros", &["build_micros"]),
            ("query_micros", &["query_micros"]),
            ("entity_result_count", &["entity_result_count"]),
        ],
    );
    assert!(warm_generation_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &warm_generation_samples,
        "warm entity index generations should stay on the derived path with certification parity",
        |metrics| {
            metric_u64(metrics, "entity_result_count") == 1
                && metric_u64(metrics, "relation_result_count") == 0
                && metrics["parity_digest_present"].as_bool() == Some(true)
                && metrics["access_path"]
                    .as_str()
                    .unwrap_or("")
                    .contains("DerivedIndexGeneration")
                && counter_u64(metrics, "query_index_attempt_count") == 1
                && counter_u64(metrics, "query_index_path_count") == 1
                && counter_u64(metrics, "query_index_parity_verification_count") == 1
                && counter_u64(metrics, "query_index_rejection_count") == 0
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );

    let build_failed_samples =
        capture_perf_samples(suite, "entity_field_equals_build_failed_fallback", || {
            let mut runtime = runtime_with_test_schema();
            let alpha = create_entity_outcome(&mut runtime, "alpha");
            let index = runtime.index_authority().register(DerivedIndexDefinition {
                index_id: DerivedIndexId(0),
                name: "entity.name.lookup".to_string(),
                kind: DerivedIndexKind::EntityField {
                    field: field_key("name"),
                },
                branch_scoped: false,
            });
            let build = runtime
                .index_authority()
                .build_for_commit(DerivedIndexBuildRequest {
                    source_commit_id: alpha.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    index_ids: vec![index.index_id],
                });
            assert!(build.failed_indexes.is_empty());
            runtime
                .indexes
                .generations
                .get_mut(&index.index_id)
                .expect("index generations")
                .last_mut()
                .expect("built generation")
                .status = crate::facade::indexes::DerivedIndexPublicationStatus::BuildFailed;

            runtime.performance_access().reset_counters();
            let query_started_at = Instant::now();
            let outcome = runtime
                .index_access()
                .execute_query_plan_with_fallback_parity(
                    runtime
                        .read_truth()
                        .plan_query_packet(
                            &alpha.snapshot,
                            entity_name_index_packet(
                                &runtime,
                                &alpha.snapshot,
                                "entity-name-equals-build-failed",
                                "alpha",
                            ),
                        )
                        .expect("fallback entity query plan"),
                    FallbackParityMode::ProductionAdmissibility,
                )
                .expect("fallback entity index query outcome");
            let query_micros = query_started_at.elapsed().as_micros();

            PerfMeasurement {
                elapsed_micros: query_micros,
                metrics: json!({
                    "query_micros": query_micros,
                    "entity_result_count": outcome.execution.result.entities.len(),
                    "access_path": format!("{:?}", outcome.access_path),
                    "counters": runtime.performance_access().counters(),
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "entity_field_equals_build_failed_fallback",
        &build_failed_samples,
        &[
            ("query_micros", &["query_micros"]),
            ("entity_result_count", &["entity_result_count"]),
        ],
    );
    assert!(build_failed_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &build_failed_samples,
        "build-failed generations should reject to storage fallback without changing truth",
        |metrics| {
            metric_u64(metrics, "entity_result_count") == 1
                && metrics["access_path"]
                    .as_str()
                    .unwrap_or("")
                    .contains("DerivedIndexRejectedStorageFallback")
                && metrics["access_path"]
                    .as_str()
                    .unwrap_or("")
                    .contains("CorruptIndexEntries")
                && counter_u64(metrics, "query_index_attempt_count") == 1
                && counter_u64(metrics, "query_index_path_count") == 0
                && counter_u64(metrics, "query_index_rejection_count") == 1
                && counter_u64(metrics, "query_index_parity_verification_count") == 0
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );

    let persisted_recovery_samples =
        capture_perf_samples(suite, "persisted_recovery_generation_parity", || {
            let mut runtime = persisted_runtime_with_test_schema();
            let alpha = create_entity_outcome(&mut runtime, "alpha");
            let index = runtime.index_authority().register(DerivedIndexDefinition {
                index_id: DerivedIndexId(0),
                name: "entity.name.lookup".to_string(),
                kind: DerivedIndexKind::EntityField {
                    field: field_key("name"),
                },
                branch_scoped: false,
            });
            let build = runtime
                .index_authority()
                .build_for_commit(DerivedIndexBuildRequest {
                    source_commit_id: alpha.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    index_ids: vec![index.index_id],
                });
            assert!(build.failed_indexes.is_empty());

            let original = runtime
                .index_access()
                .execute_query_plan_with_fallback_parity(
                    runtime
                        .read_truth()
                        .plan_query_packet(
                            &alpha.snapshot,
                            entity_name_index_packet(
                                &runtime,
                                &alpha.snapshot,
                                "entity-name-equals-persisted",
                                "alpha",
                            ),
                        )
                        .expect("original persisted plan"),
                    FallbackParityMode::CertificationParity,
                )
                .expect("original persisted query outcome");

            let recover_started_at = Instant::now();
            let (_recovery, mut recovered) =
                checkpoint_and_recover_with(&mut runtime, persisted_runtime_with_test_schema);
            let recover_micros = recover_started_at.elapsed().as_micros();
            let recovered_snapshot = recovered.visibility_authority().snapshot();

            recovered.performance_access().reset_counters();
            let query_started_at = Instant::now();
            let recovered_outcome = recovered
                .index_access()
                .execute_query_plan_with_fallback_parity(
                    recovered
                        .read_truth()
                        .plan_query_packet(
                            &recovered_snapshot,
                            entity_name_index_packet(
                                &recovered,
                                &recovered_snapshot,
                                "entity-name-equals-recovered",
                                "alpha",
                            ),
                        )
                        .expect("recovered persisted plan"),
                    FallbackParityMode::CertificationParity,
                )
                .expect("recovered persisted query outcome");
            let query_micros = query_started_at.elapsed().as_micros();

            PerfMeasurement {
                elapsed_micros: recover_micros + query_micros,
                metrics: json!({
                    "recover_micros": recover_micros,
                    "query_micros": query_micros,
                    "entity_result_count": recovered_outcome.execution.result.entities.len(),
                    "access_path": format!("{:?}", recovered_outcome.access_path),
                    "result_digest_match": original.execution.result.reduction_digest
                        == recovered_outcome.execution.result.reduction_digest,
                    "parity_digest_match": original.parity_basis_digest
                        == recovered_outcome.parity_basis_digest,
                    "counters": recovered.performance_access().counters(),
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "persisted_recovery_generation_parity",
        &persisted_recovery_samples,
        &[
            ("recover_micros", &["recover_micros"]),
            ("query_micros", &["query_micros"]),
            ("entity_result_count", &["entity_result_count"]),
        ],
    );
    assert!(persisted_recovery_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &persisted_recovery_samples,
        "persisted recovery should preserve derived index access and parity digests",
        |metrics| {
            metric_u64(metrics, "entity_result_count") == 1
                && metrics["result_digest_match"].as_bool() == Some(true)
                && metrics["parity_digest_match"].as_bool() == Some(true)
                && metrics["access_path"]
                    .as_str()
                    .unwrap_or("")
                    .contains("DerivedIndexGeneration")
                && counter_u64(metrics, "query_index_attempt_count") == 1
                && counter_u64(metrics, "query_index_path_count") == 1
                && counter_u64(metrics, "query_index_parity_verification_count") == 1
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_mixed_load_matrix() {
    let suite = "mixed_load_matrix";

    let snapshot_version_pressure_samples =
        capture_perf_samples(suite, "concurrent_snapshot_version_read_pressure", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
            let created = create_entity_outcome(&mut runtime, "baseline");
            let created_version_id = created.version_id;
            let entity = changed_entities(&created)[0];
            let explicit_snapshot = runtime.visibility_authority().snapshot();
            let updated = update_entity(&mut runtime, entity, "mutated");

            let serial_snapshot_name = {
                let read = runtime
                    .read_truth()
                    .read_snapshot(&explicit_snapshot)
                    .expect("snapshot read");
                read_entity_name(read.get_entity(entity).expect("snapshot entity"))
                    .expect("snapshot name")
                    .to_string()
            };
            let serial_version_name = {
                let read = runtime.read_truth().read_version(created_version_id);
                read_entity_name(read.get_entity(entity).expect("version entity"))
                    .expect("version name")
                    .to_string()
            };
            let serial_latest_name = {
                let read = runtime
                    .read_truth()
                    .read_snapshot(&updated.snapshot)
                    .expect("latest read");
                read_entity_name(read.get_entity(entity).expect("latest entity"))
                    .expect("latest name")
                    .to_string()
            };

            runtime.performance_access().reset_counters();
            let runtime = Arc::new(runtime);
            let started_at = Instant::now();
            std::thread::scope(|scope| {
                let mut readers = Vec::new();
                for _ in 0..8 {
                    let runtime = Arc::clone(&runtime);
                    let explicit_snapshot = explicit_snapshot.clone();
                    let published_snapshot = updated.snapshot.clone();
                    readers.push(scope.spawn(move || {
                        let snapshot_read = runtime
                            .read_truth()
                            .read_snapshot(&explicit_snapshot)
                            .expect("thread snapshot read");
                        let version_read = runtime.read_truth().read_version(created_version_id);
                        let latest_read = runtime
                            .read_truth()
                            .read_snapshot(&published_snapshot)
                            .expect("thread latest read");
                        (
                            read_entity_name(
                                snapshot_read.get_entity(entity).expect("snapshot entity"),
                            )
                            .expect("snapshot name")
                            .to_string(),
                            read_entity_name(
                                version_read.get_entity(entity).expect("version entity"),
                            )
                            .expect("version name")
                            .to_string(),
                            read_entity_name(
                                latest_read.get_entity(entity).expect("latest entity"),
                            )
                            .expect("latest name")
                            .to_string(),
                        )
                    }));
                }

                for reader in readers {
                    let (snapshot_name, version_name, latest_name) = reader.join().unwrap();
                    assert_eq!(snapshot_name, serial_snapshot_name);
                    assert_eq!(version_name, serial_version_name);
                    assert_eq!(latest_name, serial_latest_name);
                }
            });
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "reader_count": 8,
                    "snapshot_name_len": serial_snapshot_name.len(),
                    "version_name_len": serial_version_name.len(),
                    "latest_name_len": serial_latest_name.len(),
                    "visibility_cache_hits": counters.visibility_cache_hits,
                    "counters": counters,
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "concurrent_snapshot_version_read_pressure",
        &snapshot_version_pressure_samples,
        &[
            ("reader_count", &["reader_count"]),
            ("visibility_cache_hits", &["visibility_cache_hits"]),
        ],
    );
    assert!(snapshot_version_pressure_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &snapshot_version_pressure_samples,
        "mixed read pressure should preserve snapshot/version truth and hit the visibility cache",
        |metrics| {
            metric_u64(metrics, "reader_count") == 8
                && metric_u64(metrics, "snapshot_name_len") > 0
                && metric_u64(metrics, "version_name_len") > 0
                && metric_u64(metrics, "latest_name_len") > 0
                && metric_u64(metrics, "visibility_cache_hits") > 0
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );

    let relation_index_pressure_samples =
        capture_perf_samples(suite, "concurrent_relation_index_parity_pressure", || {
            let mut runtime = runtime_with_test_schema_execution_model(
                crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation,
            );
            let source = create_entity_outcome(&mut runtime, "source");
            let source_id = changed_entities(&source)[0];
            let targets = [
                create_entity_in_partition(&mut runtime, "r0", PartitionId(7)),
                create_entity_in_partition(&mut runtime, "r1", PartitionId(11)),
                create_entity_in_partition(&mut runtime, "r2", PartitionId(13)),
            ];
            for (index, target) in targets.into_iter().enumerate() {
                create_relation_in_partition(
                    &mut runtime,
                    source_id,
                    target,
                    if index < 2 { "fast" } else { "slow" },
                    PartitionId(23 + index as u32),
                );
            }
            let commit = create_entity_outcome(&mut runtime, "anchor");
            let relation_index = runtime.index_authority().register(DerivedIndexDefinition {
                index_id: DerivedIndexId(0),
                name: "relation.name".to_string(),
                kind: DerivedIndexKind::RelationField {
                    field: field_key("name"),
                },
                branch_scoped: false,
            });
            runtime
                .index_authority()
                .build_for_commit(DerivedIndexBuildRequest {
                    source_commit_id: commit.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    index_ids: vec![relation_index.index_id],
                });

            let snapshot = commit.snapshot.clone();
            let context = runtime
                .read_truth()
                .query_plan_context(&snapshot)
                .expect("query plan context");
            let packet = PlannedQueryPacket {
                label: "relation-index-certification".to_string(),
                context_id: context,
                scope: QueryScope::RelationFieldEquals {
                    field: field_key("name"),
                    value: string_aspect_value("fast"),
                    partition_scope: None,
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalRelationIdOrder,
                fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(4401),
                target_count_hint: 0,
            };
            let expected = runtime
                .index_access()
                .execute_query_plan_with_fallback_parity(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, packet.clone())
                        .expect("baseline relation plan"),
                    FallbackParityMode::CertificationParity,
                )
                .expect("baseline relation outcome");

            runtime.performance_access().reset_counters();
            let runtime = Arc::new(runtime);
            let started_at = Instant::now();
            std::thread::scope(|scope| {
                let mut readers = Vec::new();
                for _ in 0..8 {
                    let runtime = Arc::clone(&runtime);
                    let snapshot = snapshot.clone();
                    let packet = packet.clone();
                    let expected = expected.clone();
                    readers.push(scope.spawn(move || {
                        let outcome = runtime
                            .index_access()
                            .execute_query_plan_with_fallback_parity(
                                runtime
                                    .read_truth()
                                    .plan_query_packet(&snapshot, packet)
                                    .expect("thread relation plan"),
                                FallbackParityMode::CertificationParity,
                            )
                            .expect("thread relation outcome");
                        assert_eq!(outcome.access_path, expected.access_path);
                        assert_eq!(outcome.execution.result, expected.execution.result);
                        assert_eq!(outcome.parity_basis_digest, expected.parity_basis_digest);
                    }));
                }

                for reader in readers {
                    reader.join().unwrap();
                }
            });
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "reader_count": 8,
                    "matched_relation_count": expected.execution.result.relations.len(),
                    "access_path": format!("{:?}", expected.access_path),
                    "parity_digest_present": !expected.parity_basis_digest.is_empty(),
                    "counters": counters,
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "concurrent_relation_index_parity_pressure",
        &relation_index_pressure_samples,
        &[
            ("reader_count", &["reader_count"]),
            ("matched_relation_count", &["matched_relation_count"]),
        ],
    );
    assert!(relation_index_pressure_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &relation_index_pressure_samples,
        "mixed relation index pressure should preserve certification parity under scheduler contention",
        |metrics| {
            metric_u64(metrics, "reader_count") == 8
                && metric_u64(metrics, "matched_relation_count") == 0
                && metrics["parity_digest_present"].as_bool() == Some(true)
                && metrics["access_path"].as_str().unwrap_or("").contains("DerivedIndexGeneration")
                && counter_u64(metrics, "query_index_attempt_count") == 8
                && counter_u64(metrics, "query_index_path_count") == 8
                && counter_u64(metrics, "query_index_parity_verification_count") == 8
                && counter_u64(metrics, "query_index_rejection_count") == 0
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_workflow_matrix() {
    let suite = "workflow_matrix";

    let trade_correction_samples =
        capture_perf_samples(suite, "trade_correction_analysis_round_trip", || {
            let mut runtime = persisted_runtime_with_test_schema();
            let account =
                create_entity_in_partition(&mut runtime, "portfolio-account", PartitionId(10));
            create_branch_from_main(&mut runtime, "analysis");

            runtime.performance_access().reset_counters();
            let analysis_commit_started_at = Instant::now();
            let analysis_commit = {
                let mut txn = runtime.begin_transaction(TransactionOptions {
                    target_branch: Some(BranchId("analysis".to_string())),
                    ..TransactionOptions::default()
                });
                txn.push_batch(
                    WorkerIntentBatch::new("correct-trade").push(
                        MutationIntent::Create(CreateIntent::Entity(
                            crate::transactions::data::EntitySpec {
                                partition_id: PartitionId(10),
                                kind_id: KindId(1),
                                client_key: crate::symbols::data::ClientKey::raw(
                                    "analysis-trade-correction".to_string(),
                                ),
                                fields: crate::tests::support::string_aspect_field_patch([
                                    ("entity_type", "trade"),
                                    ("case", "trade-correction"),
                                    ("status", "corrected"),
                                    ("account", "portfolio-account"),
                                ]),
                            },
                        ))
                        .into(),
                    ),
                );
                txn.push_batch(
                    WorkerIntentBatch::new("refresh-risk").push(
                        MutationIntent::Create(CreateIntent::Entity(
                            crate::transactions::data::EntitySpec {
                                partition_id: PartitionId(30),
                                kind_id: KindId(1),
                                client_key: crate::symbols::data::ClientKey::raw(
                                    "analysis-risk-refresh".to_string(),
                                ),
                                fields: crate::tests::support::string_aspect_field_patch([
                                    ("entity_type", "risk_view"),
                                    ("case", "trade-correction"),
                                    ("status", "refreshed"),
                                    ("severity", "medium"),
                                ]),
                            },
                        ))
                        .into(),
                    ),
                );
                txn.push_batch(
                    WorkerIntentBatch::new("emit-audit")
                        .push(MutationIntent::Create(CreateIntent::Entity(
                            crate::transactions::data::EntitySpec {
                                partition_id: PartitionId(40),
                                kind_id: KindId(1),
                                client_key: crate::symbols::data::ClientKey::raw(
                                    "analysis-audit-record".to_string(),
                                ),
                                fields: crate::tests::support::string_aspect_field_patch([
                                    ("entity_type", "audit_record"),
                                    ("case", "trade-correction"),
                                    ("event", "analysis-reviewed"),
                                ]),
                            },
                        )))
                        .into(),
                );
                txn.commit().expect("analysis branch correction commit")
            };
            let analysis_commit_micros = analysis_commit_started_at.elapsed().as_micros();
            let analysis_entities = changed_entities(&analysis_commit);
            let trade = analysis_entities[0];
            let risk_view = analysis_entities[1];
            let audit_record = analysis_entities[2];

            let prepared = runtime
                .prepare_merge_execution(MergeExecutionRequest {
                    target_branch: BranchId("main".to_string()),
                    source_branch: BranchId("analysis".to_string()),
                    merge_intent: MergeIntent::ReconcileIntoTarget,
                })
                .expect("prepared analysis merge");
            let merge_started_at = Instant::now();
            let merge_outcome = runtime
                .execute_prepared_merge(prepared)
                .expect("analysis merge execution");
            let merge_execute_micros = merge_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "trade-correction-round-trip",
                vec![
                    RecordRef::Entity(account),
                    RecordRef::Entity(trade),
                    RecordRef::Entity(risk_view),
                    RecordRef::Entity(audit_record),
                ],
            );
            let query_started_at = Instant::now();
            let query_outcome = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, packet)
                        .expect("planned workflow query"),
                )
                .expect("workflow query outcome");
            let query_round_trip_micros = query_started_at.elapsed().as_micros();

            let elapsed_micros =
                analysis_commit_micros + merge_execute_micros + query_round_trip_micros;
            let counters = runtime.performance_access().counters();

            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "analysis_changed_records": analysis_commit.changed_records.len(),
                    "merged_changed_records": merge_outcome.commit.changed_records.len(),
                    "query_entities": query_outcome.result.entities.len(),
                    "query_relations": query_outcome.result.relations.len(),
                    "profile_boundary": profile_boundary_metrics(
                        &runtime,
                        RelationalRuntimeProfile::CertificationCore,
                    ),
                    "phase_timing": {
                        "analysis_commit_micros": analysis_commit_micros,
                        "merge_execute_micros": merge_execute_micros,
                        "query_round_trip_micros": query_round_trip_micros,
                    },
                    "counters": counters,
                })
            })
        });
    emit_metric_summaries(
        suite,
        "trade_correction_analysis_round_trip",
        &trade_correction_samples,
        &[
            (
                "analysis_commit_micros",
                &["phase_timing", "analysis_commit_micros"],
            ),
            (
                "merge_execute_micros",
                &["phase_timing", "merge_execute_micros"],
            ),
            (
                "query_round_trip_micros",
                &["phase_timing", "query_round_trip_micros"],
            ),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(trade_correction_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &trade_correction_samples,
        "workflow round trips should stay branch-local, merge one analysis patch, and query a narrow case surface",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "merge_execution_attempts") == 1
                && counter_u64(metrics, "partitions_touched_by_commit") <= 3
                && counter_u64(metrics, "query_packet_count") <= 3
                && metrics["analysis_changed_records"].as_u64() == Some(3)
                && metrics["merged_changed_records"].as_u64() == Some(3)
                && metrics["query_entities"].as_u64() == Some(4)
                && metrics["query_relations"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
        },
    );

    let fintech_intraday_risk_samples =
        capture_perf_samples(suite, "fintech_intraday_risk_branch_round_trip", || {
            let mut world = setup_intraday_risk_perf_world();
            let baseline_observability = perf_capture_baseline_observability(&world);
            let analysis = perf_open_analysis_branch(&mut world);

            world.runtime.performance_access().reset_counters();
            let stress_started_at = Instant::now();
            let stress_commit = perf_stress_intraday_risk(&mut world, analysis);
            let stress_commit_micros = stress_started_at.elapsed().as_micros();

            let query_started_at = Instant::now();
            let probe = perf_capture_intraday_risk_probe(&world);
            let query_probe_micros = query_started_at.elapsed().as_micros();
            let elapsed_micros = stress_commit_micros + query_probe_micros;
            let counters = world.runtime.performance_access().counters();
            let post_observability = perf_capture_post_mutation_observability(&world);

            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "changed_records": stress_commit.changed_records.len(),
                    "query_entities": probe.entity_count,
                    "query_relations": probe.relation_count,
                    "open_breach_count": probe.open_breach_count,
                    "diagnostic_artifact_delta": post_observability
                        .diagnostics_artifact_count
                        .saturating_sub(baseline_observability.diagnostics_artifact_count),
                    "latest_patch_present": post_observability.latest_patch_present,
                    "profile_boundary": profile_boundary_metrics(
                        &world.runtime,
                        RelationalRuntimeProfile::AiWorkflow,
                    ),
                    "phase_timing": {
                        "stress_commit_micros": stress_commit_micros,
                        "query_probe_micros": query_probe_micros,
                    },
                    "shape_metrics": {
                        "packet_count": counters.query_packet_count,
                        "scope_unit_count": counters.query_scope_unit_count,
                    },
                    "counters": counters,
                })
            })
        });
    emit_metric_summaries(
        suite,
        "fintech_intraday_risk_branch_round_trip",
        &fintech_intraday_risk_samples,
        &[
            (
                "stress_commit_micros",
                &["phase_timing", "stress_commit_micros"],
            ),
            (
                "query_probe_micros",
                &["phase_timing", "query_probe_micros"],
            ),
            ("packet_count", &["shape_metrics", "packet_count"]),
            ("scope_unit_count", &["shape_metrics", "scope_unit_count"]),
            ("diagnostic_artifact_delta", &["diagnostic_artifact_delta"]),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(fintech_intraday_risk_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &fintech_intraday_risk_samples,
        "fintech intraday risk should expose one open breach without widening beyond the case probe",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && metric_u64(metrics, "changed_records") == 4
                && metric_u64(metrics, "query_entities") == 4
                && metric_u64(metrics, "query_relations") == 0
                && metric_u64(metrics, "open_breach_count") == 1
                && metric_u64(metrics, "diagnostic_artifact_delta") >= 1
                && metrics["latest_patch_present"].as_bool() == Some(true)
                && counter_u64(metrics, "query_packet_count") <= 4
                && counter_u64(metrics, "query_scope_unit_count") <= 4
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(3)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(3)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
        },
    );

    let fintech_trade_correction_samples =
        capture_perf_samples(suite, "fintech_trade_correction_audit_round_trip", || {
            let mut world = setup_trade_correction_perf_world();
            let baseline_observability = perf_capture_baseline_observability(&world);
            let analysis = perf_open_analysis_branch(&mut world);

            world.runtime.performance_access().reset_counters();
            let correction_started_at = Instant::now();
            let correction_commit = perf_correct_trade_correction(&mut world, analysis.clone());
            let correction_commit_micros = correction_started_at.elapsed().as_micros();

            let audit_started_at = Instant::now();
            let audit_commit = perf_emit_trade_correction_audit(&mut world, analysis);
            let audit_commit_micros = audit_started_at.elapsed().as_micros();

            let query_started_at = Instant::now();
            let probe = perf_capture_trade_correction_probe(&world);
            let query_probe_micros = query_started_at.elapsed().as_micros();
            let elapsed_micros =
                correction_commit_micros + audit_commit_micros + query_probe_micros;
            let counters = world.runtime.performance_access().counters();
            let post_observability = perf_capture_post_mutation_observability(&world);

            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "correction_records": correction_commit.changed_records.len(),
                    "audit_records": audit_commit.changed_records.len(),
                    "query_entities": probe.entity_count,
                    "query_relations": probe.relation_count,
                    "corrected_trade_count": probe.corrected_trade_count,
                    "audit_record_count": probe.audit_record_count,
                    "diagnostic_artifact_delta": post_observability
                        .diagnostics_artifact_count
                        .saturating_sub(baseline_observability.diagnostics_artifact_count),
                    "profile_boundary": profile_boundary_metrics(
                        &world.runtime,
                        RelationalRuntimeProfile::AiWorkflow,
                    ),
                    "phase_timing": {
                        "correction_commit_micros": correction_commit_micros,
                        "audit_commit_micros": audit_commit_micros,
                        "query_probe_micros": query_probe_micros,
                    },
                    "shape_metrics": {
                        "packet_count": counters.query_packet_count,
                        "scope_unit_count": counters.query_scope_unit_count,
                    },
                    "counters": counters,
                })
            })
        });
    emit_metric_summaries(
        suite,
        "fintech_trade_correction_audit_round_trip",
        &fintech_trade_correction_samples,
        &[
            (
                "correction_commit_micros",
                &["phase_timing", "correction_commit_micros"],
            ),
            (
                "audit_commit_micros",
                &["phase_timing", "audit_commit_micros"],
            ),
            (
                "query_probe_micros",
                &["phase_timing", "query_probe_micros"],
            ),
            ("packet_count", &["shape_metrics", "packet_count"]),
            ("scope_unit_count", &["shape_metrics", "scope_unit_count"]),
            ("diagnostic_artifact_delta", &["diagnostic_artifact_delta"]),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(fintech_trade_correction_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &fintech_trade_correction_samples,
        "fintech trade correction should surface one corrected trade and one audit record without broadening the case probe",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && metric_u64(metrics, "correction_records") == 1
                && metric_u64(metrics, "audit_records") == 1
                && metric_u64(metrics, "query_entities") == 3
                && metric_u64(metrics, "query_relations") == 0
                && metric_u64(metrics, "corrected_trade_count") == 1
                && metric_u64(metrics, "audit_record_count") == 1
                && metric_u64(metrics, "diagnostic_artifact_delta") >= 2
                && counter_u64(metrics, "query_packet_count") <= 3
                && counter_u64(metrics, "query_scope_unit_count") <= 3
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(3)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(3)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
        },
    );

    let replay_recovery_samples = capture_perf_samples(
        suite,
        "persisted_recovery_replay_round_trip",
        || {
            let mut runtime = persisted_runtime_with_test_schema();
            let source_created = create_entity_outcome(&mut runtime, "recovery-source");
            let target_created = create_entity_outcome(&mut runtime, "recovery-target");
            let source = changed_entities(&source_created)[0];
            let target = changed_entities(&target_created)[0];
            let source_lineage = runtime
                .lineage_access()
                .for_record(source)
                .expect("source lineage")
                .lineage_id;
            let target_lineage = runtime
                .lineage_access()
                .for_record(target)
                .expect("target lineage")
                .lineage_id;

            let checkpoint_started_at = Instant::now();
            runtime
                .durability_authority()
                .checkpoint()
                .expect("workflow checkpoint");
            let checkpoint_micros = checkpoint_started_at.elapsed().as_micros();

            let post_checkpoint_commit_started_at = Instant::now();
            let candidate = runtime.lineage_authority().record_correspondence_candidate(
                BranchId("main".to_string()),
                vec![source_lineage],
                vec![target_lineage],
                "workflow-recovery-lineage",
            );
            let promotion = runtime
                .lineage_authority()
                .promote_correspondence(candidate.candidate_id, target_created.commit.clone())
                .expect("promote workflow correspondence");
            let post_checkpoint_commit_micros =
                post_checkpoint_commit_started_at.elapsed().as_micros();
            let recovery_plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let tail_commit_id = promotion.promoted_commit_id().expect("promoted commit id");

            let mut recovered = persisted_runtime_with_test_schema();
            recovered.performance_access().reset_counters();
            let recover_started_at = Instant::now();
            let recovery_outcome = recovered
                .durability_authority()
                .recover(recovery_plan)
                .expect("workflow recovery");
            let recover_micros = recover_started_at.elapsed().as_micros();

            let replay_started_at = Instant::now();
            let replay_outcome =
                recovered
                    .replay_authority()
                    .replay_commit(RelationalReplayRequest {
                        commit_id: tail_commit_id,
                        branch_id: BranchId("main".to_string()),
                        execution_mode: ReplayExecutionMode::SerialDeterministic,
                        verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                    });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();

            let recovered_snapshot = recovered.visibility_authority().snapshot();
            let recovered_packet = explicit_query_packet(
                &recovered,
                &recovered_snapshot,
                "recovery-round-trip-query",
                vec![RecordRef::Entity(source), RecordRef::Entity(target)],
            );
            let query_started_at = Instant::now();
            let query_outcome = recovered
                .read_truth()
                .execute_query_plan(
                    recovered
                        .read_truth()
                        .plan_query_packet(&recovered_snapshot, recovered_packet)
                        .expect("planned recovered workflow query"),
                )
                .expect("recovered workflow query");
            let post_recovery_query_micros = query_started_at.elapsed().as_micros();

            let elapsed_micros = checkpoint_micros
                + post_checkpoint_commit_micros
                + recover_micros
                + replay_commit_micros
                + post_recovery_query_micros;
            let counters = recovered.performance_access().counters();

            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "checkpoint_commit_count": recovery_outcome.coverage.checkpoint_commits,
                    "tail_commit_count": recovery_outcome.coverage.replayed_tail_commits,
                    "recovered_commits": recovery_outcome.recovered_commits,
                    "selected_checkpoint": recovery_outcome.cursor.checkpoint_id.is_some(),
                    "replay_failure": replay_outcome.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "replay_mismatch_count": replay_outcome.mismatches.len(),
                    "query_entities": query_outcome.result.entities.len(),
                    "query_relations": query_outcome.result.relations.len(),
                    "profile_boundary": profile_boundary_metrics(
                        &recovered,
                        RelationalRuntimeProfile::CertificationCore,
                    ),
                    "phase_timing": {
                        "checkpoint_micros": checkpoint_micros,
                        "post_checkpoint_commit_micros": post_checkpoint_commit_micros,
                        "recover_micros": recover_micros,
                        "replay_commit_micros": replay_commit_micros,
                        "post_recovery_query_micros": post_recovery_query_micros,
                    },
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "persisted_recovery_replay_round_trip",
        &replay_recovery_samples,
        &[
            ("checkpoint_micros", &["phase_timing", "checkpoint_micros"]),
            (
                "post_checkpoint_commit_micros",
                &["phase_timing", "post_checkpoint_commit_micros"],
            ),
            ("recover_micros", &["phase_timing", "recover_micros"]),
            (
                "replay_commit_micros",
                &["phase_timing", "replay_commit_micros"],
            ),
            (
                "post_recovery_query_micros",
                &["phase_timing", "post_recovery_query_micros"],
            ),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(replay_recovery_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &replay_recovery_samples,
        "persisted recovery round trips should select a checkpoint, replay cleanly, and query the recovered tail surface",
        |metrics| {
            metrics["selected_checkpoint"].as_bool() == Some(true)
                && metrics["replay_failure"].is_null()
                && metrics["replay_mismatch_count"].as_u64() == Some(0)
                && metrics["checkpoint_commit_count"].as_u64().unwrap_or(0) >= 1
                && metrics["tail_commit_count"].as_u64().unwrap_or(0) >= 1
                && counter_u64(metrics, "replay_lineage_authority_lookup_requests") == 1
                && counter_u64(metrics, "query_packet_count") <= 3
                && metrics["query_entities"].as_u64() == Some(2)
                && metrics["query_relations"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
        },
    );

    let retention_samples =
        capture_perf_samples(suite, "retention_release_reclaim_round_trip", || {
            let mut runtime = runtime_with_test_schema();
            let survivor =
                create_entity_in_partition(&mut runtime, "retention-survivor", PartitionId(10));
            let deleted_created = create_entity_outcome(&mut runtime, "retention-deleted");
            let created_snapshot = runtime.visibility_authority().snapshot();
            let deleted_entity = changed_entities(&deleted_created)[0];
            let deleted_commit = delete_entity(&mut runtime, deleted_entity);
            let deleted_snapshot = runtime.visibility_authority().snapshot();

            assert!(runtime
                .visibility_authority()
                .release_snapshot(&created_snapshot));
            assert!(runtime
                .visibility_authority()
                .release_snapshot(&deleted_snapshot));

            runtime.performance_access().reset_counters();
            let inspect_started_at = Instant::now();
            let inspect_plan = runtime.retention().inspect_plan();
            let inspect_plan_micros = inspect_started_at.elapsed().as_micros();
            let reclaim_started_at = Instant::now();
            let reclaim_pass = runtime.retention().run_pass();
            let run_pass_micros = reclaim_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "retention-reclaim-round-trip",
                vec![
                    RecordRef::Entity(survivor),
                    RecordRef::Entity(deleted_entity),
                ],
            );
            let query_started_at = Instant::now();
            let query_outcome = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, packet)
                        .expect("planned retention workflow query"),
                )
                .expect("retention workflow query");
            let post_reclaim_query_micros = query_started_at.elapsed().as_micros();

            let elapsed_micros = inspect_plan_micros + run_pass_micros + post_reclaim_query_micros;
            let counters = runtime.performance_access().counters();

            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "deleted_commit_records": deleted_commit.changed_records.len(),
                    "active_snapshot_count": inspect_plan.active_snapshot_count,
                    "reclaimable_entities": inspect_plan.reclaimable_entities,
                    "entity_reclaimable": reclaim_pass.entity_reclaimable,
                    "entity_reclaimed": reclaim_pass.entity_reclaimed,
                    "query_entities": query_outcome.result.entities.len(),
                    "query_relations": query_outcome.result.relations.len(),
                    "profile_boundary": profile_boundary_metrics(
                        &runtime,
                        RelationalRuntimeProfile::CertificationCore,
                    ),
                    "phase_timing": {
                        "inspect_plan_micros": inspect_plan_micros,
                        "run_pass_micros": run_pass_micros,
                        "post_reclaim_query_micros": post_reclaim_query_micros,
                    },
                    "counters": counters,
                })
            })
        });
    emit_metric_summaries(
        suite,
        "retention_release_reclaim_round_trip",
        &retention_samples,
        &[
            (
                "inspect_plan_micros",
                &["phase_timing", "inspect_plan_micros"],
            ),
            ("run_pass_micros", &["phase_timing", "run_pass_micros"]),
            (
                "post_reclaim_query_micros",
                &["phase_timing", "post_reclaim_query_micros"],
            ),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(retention_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &retention_samples,
        "retention release round trips should expose reclaimability and keep the survivor queryable without clone-heavy reclaim work",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && metrics["deleted_commit_records"].as_u64() == Some(1)
                && metrics["active_snapshot_count"].as_u64() == Some(0)
                && metrics["reclaimable_entities"].as_u64().unwrap_or(0) >= 1
                && metrics["entity_reclaimable"].as_u64().unwrap_or(0) >= 1
                && metrics["entity_reclaimed"].as_u64().unwrap_or(0)
                    <= metrics["entity_reclaimable"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "query_packet_count") <= 2
                && metrics["query_entities"].as_u64() == Some(1)
                && metrics["query_relations"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_profile_matrix() {
    let suite = "profile_matrix";

    let certification_core_rich_samples = capture_perf_samples(
        suite,
        "certification_core_rich_commit_query_round_trip",
        || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore);
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();

            runtime.performance_access().reset_counters();
            let commit_started_at = Instant::now();
            let commit_outcome = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                for index in 0..24 {
                    txn.push_batch(batch_create(&format!("profile-certification-{index}")));
                }
                txn.commit().expect("certification-core commit")
            };
            let commit_micros = commit_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let targets = changed_entities(&commit_outcome)
                .into_iter()
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let packet = explicit_query_packet(&runtime, &snapshot, "profile-core-query", targets);
            let query_started_at = Instant::now();
            let outcome = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, packet)
                        .expect("planned certification-core profile query"),
                )
                .expect("certification-core profile query outcome");
            let query_micros = query_started_at.elapsed().as_micros();
            let elapsed_micros = commit_micros + query_micros;
            let counters = runtime.performance_access().counters();
            let publication = runtime.publication();
            let diagnostics = publication.diagnostic_artifacts();
            let fresh_artifacts = &diagnostics[diagnostics_start..];
            let detailed_trace_entries = fresh_artifacts
                .iter()
                .filter(|artifact| {
                    artifact.kind
                        == crate::facade::diagnostics::DiagnosticsArtifactKind::DetailedTrace
                })
                .map(|artifact| artifact.entries.len())
                .sum::<usize>();

            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "result_entities": outcome.result.entities.len(),
                    "result_relations": outcome.result.relations.len(),
                    "diagnostic_artifact_count": fresh_artifacts.len(),
                    "detailed_trace_entries": detailed_trace_entries,
                    "profile_boundary": profile_boundary_metrics(
                        &runtime,
                        RelationalRuntimeProfile::CertificationCore,
                    ),
                    "phase_timing": {
                        "commit_micros": commit_micros,
                        "query_micros": query_micros,
                    },
                    "shape_metrics": {
                        "packet_count": outcome.complexity.packet_count,
                        "scope_unit_count": counters.query_scope_unit_count,
                    },
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "certification_core_rich_commit_query_round_trip",
        &certification_core_rich_samples,
        &[
            ("commit_micros", &["phase_timing", "commit_micros"]),
            ("query_micros", &["phase_timing", "query_micros"]),
            ("packet_count", &["shape_metrics", "packet_count"]),
            ("scope_unit_count", &["shape_metrics", "scope_unit_count"]),
            ("diagnostic_artifact_count", &["diagnostic_artifact_count"]),
            ("detailed_trace_entries", &["detailed_trace_entries"]),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(certification_core_rich_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &certification_core_rich_samples,
        "certification-core rich diagnostics should preserve scoped truth while surfacing trace cost",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "query_packet_count") <= 2
                && metrics["result_entities"].as_u64() == Some(24)
                && metrics["result_relations"].as_u64() == Some(0)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64().unwrap_or(0) >= 1
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
        },
    );

    let geometry_kernel_rich_samples = capture_perf_samples(
        suite,
        "geometry_kernel_rich_commit_query_round_trip",
        || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();

            runtime.performance_access().reset_counters();
            let commit_started_at = Instant::now();
            let commit_outcome = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                for index in 0..24 {
                    txn.push_batch(batch_create(&format!("profile-geometry-{index}")));
                }
                txn.commit().expect("geometry-kernel commit")
            };
            let commit_micros = commit_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let targets = changed_entities(&commit_outcome)
                .into_iter()
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let packet =
                explicit_query_packet(&runtime, &snapshot, "profile-geometry-query", targets);
            let query_started_at = Instant::now();
            let outcome = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, packet)
                        .expect("planned geometry-kernel profile query"),
                )
                .expect("geometry-kernel profile query outcome");
            let query_micros = query_started_at.elapsed().as_micros();
            let elapsed_micros = commit_micros + query_micros;
            let counters = runtime.performance_access().counters();
            let publication = runtime.publication();
            let diagnostics = publication.diagnostic_artifacts();
            let fresh_artifacts = &diagnostics[diagnostics_start..];
            let detailed_trace_entries = fresh_artifacts
                .iter()
                .filter(|artifact| {
                    artifact.kind
                        == crate::facade::diagnostics::DiagnosticsArtifactKind::DetailedTrace
                })
                .map(|artifact| artifact.entries.len())
                .sum::<usize>();

            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "result_entities": outcome.result.entities.len(),
                    "result_relations": outcome.result.relations.len(),
                    "diagnostic_artifact_count": fresh_artifacts.len(),
                    "detailed_trace_entries": detailed_trace_entries,
                    "profile_boundary": profile_boundary_metrics(
                        &runtime,
                        RelationalRuntimeProfile::GeometryKernel,
                    ),
                    "phase_timing": {
                        "commit_micros": commit_micros,
                        "query_micros": query_micros,
                    },
                    "shape_metrics": {
                        "packet_count": outcome.complexity.packet_count,
                        "scope_unit_count": counters.query_scope_unit_count,
                    },
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "geometry_kernel_rich_commit_query_round_trip",
        &geometry_kernel_rich_samples,
        &[
            ("commit_micros", &["phase_timing", "commit_micros"]),
            ("query_micros", &["phase_timing", "query_micros"]),
            ("packet_count", &["shape_metrics", "packet_count"]),
            ("scope_unit_count", &["shape_metrics", "scope_unit_count"]),
            ("diagnostic_artifact_count", &["diagnostic_artifact_count"]),
            ("detailed_trace_entries", &["detailed_trace_entries"]),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(geometry_kernel_rich_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &geometry_kernel_rich_samples,
        "geometry-kernel rich diagnostics should preserve the same scoped truth envelope while deferring hot detailed traces",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "query_packet_count") <= 2
                && metrics["result_entities"].as_u64() == Some(24)
                && metrics["result_relations"].as_u64() == Some(0)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
        },
    );

    let certification_core_zero_diag_samples = capture_perf_samples(
        suite,
        "certification_core_zero_diagnostics_commit_query_round_trip",
        || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore);
            runtime.config.diagnostics.profile.detailed_traces_enabled = false;
            runtime.config.diagnostics.profile.max_entries_per_artifact = 0;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();

            runtime.performance_access().reset_counters();
            let commit_started_at = Instant::now();
            let commit_outcome = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                for index in 0..24 {
                    txn.push_batch(batch_create(&format!("profile-zero-{index}")));
                }
                txn.commit()
                    .expect("zero-diagnostics certification-core commit")
            };
            let commit_micros = commit_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let targets = changed_entities(&commit_outcome)
                .into_iter()
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let packet = explicit_query_packet(&runtime, &snapshot, "profile-zero-query", targets);
            let query_started_at = Instant::now();
            let outcome = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, packet)
                        .expect("planned zero-diagnostics profile query"),
                )
                .expect("zero-diagnostics profile query outcome");
            let query_micros = query_started_at.elapsed().as_micros();
            let elapsed_micros = commit_micros + query_micros;
            let counters = runtime.performance_access().counters();
            let publication = runtime.publication();
            let diagnostics = publication.diagnostic_artifacts();
            let fresh_artifacts = &diagnostics[diagnostics_start..];
            let detailed_trace_entries = fresh_artifacts
                .iter()
                .filter(|artifact| {
                    artifact.kind
                        == crate::facade::diagnostics::DiagnosticsArtifactKind::DetailedTrace
                })
                .map(|artifact| artifact.entries.len())
                .sum::<usize>();

            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "result_entities": outcome.result.entities.len(),
                    "result_relations": outcome.result.relations.len(),
                    "diagnostic_artifact_count": fresh_artifacts.len(),
                    "detailed_trace_entries": detailed_trace_entries,
                    "profile_boundary": profile_boundary_metrics(
                        &runtime,
                        RelationalRuntimeProfile::CertificationCore,
                    ),
                    "phase_timing": {
                        "commit_micros": commit_micros,
                        "query_micros": query_micros,
                    },
                    "shape_metrics": {
                        "packet_count": outcome.complexity.packet_count,
                        "scope_unit_count": counters.query_scope_unit_count,
                    },
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "certification_core_zero_diagnostics_commit_query_round_trip",
        &certification_core_zero_diag_samples,
        &[
            ("commit_micros", &["phase_timing", "commit_micros"]),
            ("query_micros", &["phase_timing", "query_micros"]),
            ("packet_count", &["shape_metrics", "packet_count"]),
            ("scope_unit_count", &["shape_metrics", "scope_unit_count"]),
            ("diagnostic_artifact_count", &["diagnostic_artifact_count"]),
            ("detailed_trace_entries", &["detailed_trace_entries"]),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(certification_core_zero_diag_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &certification_core_zero_diag_samples,
        "zero-budget diagnostics should preserve scoped truth while eliminating trace entry pressure",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "query_packet_count") <= 2
                && metrics["result_entities"].as_u64() == Some(24)
                && metrics["result_relations"].as_u64() == Some(0)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(0)
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_hot_cold_path_matrix() {
    let suite = "hot_cold_path_matrix";

    let geometry_hot_vs_replay_samples = capture_perf_samples(
        suite,
        "geometry_hot_commit_vs_replay_reconstruction",
        || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::GeometryKernel,
            );
            runtime.config.diagnostics.profile.detailed_traces_enabled = false;
            runtime.config.diagnostics.profile.max_entries_per_artifact = 0;

            let source = create_entity_outcome(&mut runtime, "hot-cold-geometry-source");
            let middle = create_entity_outcome(&mut runtime, "hot-cold-geometry-middle");
            let target = create_entity_outcome(&mut runtime, "hot-cold-geometry-target");
            let source_entity = changed_entities(&source)[0];
            let middle_entity = changed_entities(&middle)[0];
            let target_entity = changed_entities(&target)[0];
            create_relation_outcome(
                &mut runtime,
                source_entity,
                middle_entity,
                "hot-cold-geometry-link-a",
            );
            create_relation_outcome(
                &mut runtime,
                middle_entity,
                target_entity,
                "hot-cold-geometry-link-b",
            );

            runtime.performance_access().reset_counters();
            let hot_commit_started_at = Instant::now();
            let hot_commit = update_entity(
                &mut runtime,
                middle_entity,
                "hot-cold-geometry-middle-updated",
            );
            let hot_commit_micros = hot_commit_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let hot_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "hot-cold-geometry-hot-query",
                vec![
                    RecordRef::Entity(source_entity),
                    RecordRef::Entity(middle_entity),
                ],
            );
            let hot_query_started_at = Instant::now();
            let hot_query = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, hot_packet)
                        .expect("planned hot geometry query"),
                )
                .expect("hot geometry query outcome");
            let hot_query_micros = hot_query_started_at.elapsed().as_micros();

            let checkpoint_started_at = Instant::now();
            runtime
                .durability_authority()
                .checkpoint()
                .expect("geometry hot/cold checkpoint");
            let checkpoint_micros = checkpoint_started_at.elapsed().as_micros();

            let plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::GeometryKernel,
            );
            recovered.performance_access().reset_counters();
            let recover_started_at = Instant::now();
            recovered
                .durability_authority()
                .recover(plan)
                .expect("geometry hot/cold recovery");
            let recover_micros = recover_started_at.elapsed().as_micros();

            let replay_started_at = Instant::now();
            let replay = recovered
                .replay_authority()
                .replay_commit(RelationalReplayRequest {
                    commit_id: hot_commit.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                    verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();

            let recovered_snapshot = recovered.visibility_authority().snapshot();
            let cold_packet = explicit_query_packet(
                &recovered,
                &recovered_snapshot,
                "hot-cold-geometry-cold-query",
                vec![
                    RecordRef::Entity(source_entity),
                    RecordRef::Entity(middle_entity),
                ],
            );
            let cold_query_started_at = Instant::now();
            let cold_query = recovered
                .read_truth()
                .execute_query_plan(
                    recovered
                        .read_truth()
                        .plan_query_packet(&recovered_snapshot, cold_packet)
                        .expect("planned cold geometry query"),
                )
                .expect("cold geometry query outcome");
            let cold_query_micros = cold_query_started_at.elapsed().as_micros();

            let elapsed_micros = hot_commit_micros
                + hot_query_micros
                + checkpoint_micros
                + recover_micros
                + replay_commit_micros
                + cold_query_micros;
            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "hot_changed_records": hot_commit.changed_records.len(),
                    "hot_result_entities": hot_query.result.entities.len(),
                    "cold_result_entities": cold_query.result.entities.len(),
                    "replay_mismatch_count": replay.mismatches.len(),
                    "replay_failure": replay.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "phase_timing": {
                        "hot_commit_micros": hot_commit_micros,
                        "hot_query_micros": hot_query_micros,
                        "checkpoint_micros": checkpoint_micros,
                        "recover_micros": recover_micros,
                        "replay_commit_micros": replay_commit_micros,
                        "cold_query_micros": cold_query_micros,
                    },
                    "hot_counters": runtime.performance_access().counters(),
                    "cold_counters": recovered.performance_access().counters(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "geometry_hot_commit_vs_replay_reconstruction",
        &geometry_hot_vs_replay_samples,
        &[
            ("hot_commit_micros", &["phase_timing", "hot_commit_micros"]),
            ("hot_query_micros", &["phase_timing", "hot_query_micros"]),
            ("checkpoint_micros", &["phase_timing", "checkpoint_micros"]),
            ("recover_micros", &["phase_timing", "recover_micros"]),
            (
                "replay_commit_micros",
                &["phase_timing", "replay_commit_micros"],
            ),
            ("cold_query_micros", &["phase_timing", "cold_query_micros"]),
        ],
    );
    assert!(geometry_hot_vs_replay_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &geometry_hot_vs_replay_samples,
        "geometry hot/cold certification should keep hot updates narrow while proving truth is replay-recoverable on the cold path",
        |metrics| {
            metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["hot_result_entities"].as_u64() == Some(2)
                && metrics["cold_result_entities"].as_u64() == Some(2)
                && metrics["replay_failure"].is_null()
                && metrics["replay_mismatch_count"].as_u64() == Some(0)
                && metrics["phase_timing"]["hot_commit_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["recover_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["replay_commit_micros"].as_u64().unwrap_or(0) > 0
                && metrics["hot_counters"]["full_state_clones"].as_u64() == Some(0)
                && metrics["cold_counters"]["full_state_clones"].as_u64() == Some(0)
        },
    );

    let chip_hot_vs_recovery_samples = capture_perf_samples(
        suite,
        "chip_hot_compile_vs_recovery_compile",
        || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            runtime.config.diagnostics.profile.detailed_traces_enabled = false;
            runtime.config.diagnostics.profile.max_entries_per_artifact = 0;

            let source =
                create_entity_in_partition(&mut runtime, "chip-hot-cold-source", PartitionId(7));
            let sinks = (0..8)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("chip-hot-cold-sink-{index}"),
                        if index % 2 == 0 {
                            PartitionId(11)
                        } else {
                            PartitionId(12)
                        },
                    )
                })
                .collect::<Vec<_>>();
            for (index, sink) in sinks.iter().enumerate() {
                create_relation_in_partition(
                    &mut runtime,
                    source,
                    *sink,
                    &format!("chip-hot-cold-link-{index}"),
                    PartitionId(19),
                );
            }

            runtime.performance_access().reset_counters();
            let hot_commit_started_at = Instant::now();
            let hot_commit = update_entity(&mut runtime, source, "chip-hot-cold-updated");
            let hot_commit_micros = hot_commit_started_at.elapsed().as_micros();
            let latest_commit = runtime
                .history()
                .latest_commit()
                .expect("chip hot/cold latest commit")
                .clone();
            let hot_compile_started_at = Instant::now();
            let hot_artifact = runtime
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    latest_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(12),
                        PartitionId(19),
                    ],
                )
                .expect("hot chip compiled artifact");
            let hot_compile_micros = hot_compile_started_at.elapsed().as_micros();

            let checkpoint_started_at = Instant::now();
            runtime
                .durability_authority()
                .checkpoint()
                .expect("chip hot/cold checkpoint");
            let checkpoint_micros = checkpoint_started_at.elapsed().as_micros();

            let plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            recovered.performance_access().reset_counters();
            let recover_started_at = Instant::now();
            recovered
                .durability_authority()
                .recover(plan)
                .expect("chip hot/cold recovery");
            let recover_micros = recover_started_at.elapsed().as_micros();

            let replay_started_at = Instant::now();
            let replay = recovered
                .replay_authority()
                .replay_commit(RelationalReplayRequest {
                    commit_id: hot_commit.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                    verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();

            let recovered_commit = recovered
                .history()
                .latest_commit()
                .expect("recovered chip latest commit")
                .clone();
            let cold_compile_started_at = Instant::now();
            let cold_artifact = recovered
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    recovered_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(12),
                        PartitionId(19),
                    ],
                )
                .expect("cold chip compiled artifact");
            let cold_compile_micros = cold_compile_started_at.elapsed().as_micros();

            let elapsed_micros = hot_commit_micros
                + hot_compile_micros
                + checkpoint_micros
                + recover_micros
                + replay_commit_micros
                + cold_compile_micros;
            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "hot_changed_records": hot_commit.changed_records.len(),
                    "replay_mismatch_count": replay.mismatches.len(),
                    "replay_failure": replay.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "hot_compatibility": format!(
                        "{:?}",
                        runtime.compiled_artifacts().compiled_artifact_compatibility(hot_artifact.artifact_id)
                    ),
                    "cold_compatibility": format!(
                        "{:?}",
                        recovered.compiled_artifacts().compiled_artifact_compatibility(cold_artifact.artifact_id)
                    ),
                    "phase_timing": {
                        "hot_commit_micros": hot_commit_micros,
                        "hot_compile_micros": hot_compile_micros,
                        "checkpoint_micros": checkpoint_micros,
                        "recover_micros": recover_micros,
                        "replay_commit_micros": replay_commit_micros,
                        "cold_compile_micros": cold_compile_micros,
                    },
                    "hot_counters": runtime.performance_access().counters(),
                    "cold_counters": recovered.performance_access().counters(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "chip_hot_compile_vs_recovery_compile",
        &chip_hot_vs_recovery_samples,
        &[
            ("hot_commit_micros", &["phase_timing", "hot_commit_micros"]),
            (
                "hot_compile_micros",
                &["phase_timing", "hot_compile_micros"],
            ),
            ("checkpoint_micros", &["phase_timing", "checkpoint_micros"]),
            ("recover_micros", &["phase_timing", "recover_micros"]),
            (
                "replay_commit_micros",
                &["phase_timing", "replay_commit_micros"],
            ),
            (
                "cold_compile_micros",
                &["phase_timing", "cold_compile_micros"],
            ),
        ],
    );
    assert!(chip_hot_vs_recovery_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &chip_hot_vs_recovery_samples,
        "chip hot/cold certification should keep compile-ready stepping narrow while preserving recovery-compile equivalence on the cold path",
        |metrics| {
            metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["replay_failure"].is_null()
                && metrics["replay_mismatch_count"].as_u64() == Some(0)
                && metrics["hot_compatibility"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactCompatibility::Compatible))
                && metrics["cold_compatibility"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactCompatibility::Compatible))
                && metrics["phase_timing"]["hot_compile_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["recover_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["cold_compile_micros"].as_u64().unwrap_or(0) > 0
                && metrics["hot_counters"]["full_state_clones"].as_u64() == Some(0)
                && metrics["cold_counters"]["full_state_clones"].as_u64() == Some(0)
        },
    );

    let geometry_rich_publication_samples = capture_perf_samples(
        suite,
        "geometry_rich_publication_hot_vs_replay_truth",
        || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::GeometryKernel,
            );
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();

            let source = create_entity_outcome(&mut runtime, "hot-cold-geometry-rich-source");
            let middle = create_entity_outcome(&mut runtime, "hot-cold-geometry-rich-middle");
            let target = create_entity_outcome(&mut runtime, "hot-cold-geometry-rich-target");
            let source_entity = changed_entities(&source)[0];
            let middle_entity = changed_entities(&middle)[0];
            let target_entity = changed_entities(&target)[0];
            create_relation_outcome(
                &mut runtime,
                source_entity,
                middle_entity,
                "hot-cold-geometry-rich-link-a",
            );
            create_relation_outcome(
                &mut runtime,
                middle_entity,
                target_entity,
                "hot-cold-geometry-rich-link-b",
            );

            runtime.performance_access().reset_counters();
            let hot_commit_started_at = Instant::now();
            let hot_commit = update_entity(
                &mut runtime,
                middle_entity,
                "hot-cold-geometry-rich-middle-updated",
            );
            let hot_commit_micros = hot_commit_started_at.elapsed().as_micros();
            let hot_phase_timing = hot_commit.execution.phase_timing.clone();

            let snapshot = runtime.visibility_authority().snapshot();
            let hot_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "hot-cold-geometry-rich-hot-query",
                vec![
                    RecordRef::Entity(source_entity),
                    RecordRef::Entity(middle_entity),
                ],
            );
            let hot_query_started_at = Instant::now();
            let hot_query = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, hot_packet)
                        .expect("planned hot rich geometry query"),
                )
                .expect("hot rich geometry query outcome");
            let hot_query_micros = hot_query_started_at.elapsed().as_micros();
            let (hot_diagnostic_artifact_count, hot_detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            let checkpoint_started_at = Instant::now();
            runtime
                .durability_authority()
                .checkpoint()
                .expect("geometry rich hot/cold checkpoint");
            let checkpoint_micros = checkpoint_started_at.elapsed().as_micros();

            let plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::GeometryKernel,
            );
            recovered.performance_access().reset_counters();
            let recover_started_at = Instant::now();
            recovered
                .durability_authority()
                .recover(plan)
                .expect("geometry rich hot/cold recovery");
            let recover_micros = recover_started_at.elapsed().as_micros();

            let replay_started_at = Instant::now();
            let replay = recovered
                .replay_authority()
                .replay_commit(RelationalReplayRequest {
                    commit_id: hot_commit.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                    verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();

            let recovered_snapshot = recovered.visibility_authority().snapshot();
            let cold_packet = explicit_query_packet(
                &recovered,
                &recovered_snapshot,
                "hot-cold-geometry-rich-cold-query",
                vec![
                    RecordRef::Entity(source_entity),
                    RecordRef::Entity(middle_entity),
                ],
            );
            let cold_query_started_at = Instant::now();
            let cold_query = recovered
                .read_truth()
                .execute_query_plan(
                    recovered
                        .read_truth()
                        .plan_query_packet(&recovered_snapshot, cold_packet)
                        .expect("planned cold rich geometry query"),
                )
                .expect("cold rich geometry query outcome");
            let cold_query_micros = cold_query_started_at.elapsed().as_micros();

            let elapsed_micros = hot_commit_micros
                + hot_query_micros
                + checkpoint_micros
                + recover_micros
                + replay_commit_micros
                + cold_query_micros;
            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "hot_changed_records": hot_commit.changed_records.len(),
                    "hot_result_entities": hot_query.result.entities.len(),
                    "cold_result_entities": cold_query.result.entities.len(),
                    "hot_diagnostic_artifact_count": hot_diagnostic_artifact_count,
                    "hot_detailed_trace_entries": hot_detailed_trace_entries,
                    "replay_mismatch_count": replay.mismatches.len(),
                    "replay_failure": replay.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "phase_timing": {
                        "hot_commit_micros": hot_commit_micros,
                        "hot_query_micros": hot_query_micros,
                        "artifact_assembly_micros": hot_phase_timing.artifact_assembly_micros,
                        "durable_append_micros": hot_phase_timing.durable_append_micros,
                        "publication_micros": hot_phase_timing.publication_micros,
                        "checkpoint_micros": checkpoint_micros,
                        "recover_micros": recover_micros,
                        "replay_commit_micros": replay_commit_micros,
                        "cold_query_micros": cold_query_micros,
                    },
                    "hot_counters": runtime.performance_access().counters(),
                    "cold_counters": recovered.performance_access().counters(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "geometry_rich_publication_hot_vs_replay_truth",
        &geometry_rich_publication_samples,
        &[
            ("hot_commit_micros", &["phase_timing", "hot_commit_micros"]),
            ("hot_query_micros", &["phase_timing", "hot_query_micros"]),
            (
                "artifact_assembly_micros",
                &["phase_timing", "artifact_assembly_micros"],
            ),
            (
                "durable_append_micros",
                &["phase_timing", "durable_append_micros"],
            ),
            (
                "publication_micros",
                &["phase_timing", "publication_micros"],
            ),
            ("checkpoint_micros", &["phase_timing", "checkpoint_micros"]),
            ("recover_micros", &["phase_timing", "recover_micros"]),
            (
                "replay_commit_micros",
                &["phase_timing", "replay_commit_micros"],
            ),
            ("cold_query_micros", &["phase_timing", "cold_query_micros"]),
            (
                "hot_diagnostic_artifact_count",
                &["hot_diagnostic_artifact_count"],
            ),
            (
                "hot_detailed_trace_entries",
                &["hot_detailed_trace_entries"],
            ),
        ],
    );
    assert!(geometry_rich_publication_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &geometry_rich_publication_samples,
        "geometry rich hot/cold certification should isolate summaries on the hot side while proving replay-recoverable truth on the cold side",
        |metrics| {
            metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["hot_result_entities"].as_u64() == Some(2)
                && metrics["cold_result_entities"].as_u64() == Some(2)
                && metrics["hot_diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["hot_detailed_trace_entries"].as_u64() == Some(0)
                && metrics["replay_failure"].is_null()
                && metrics["replay_mismatch_count"].as_u64() == Some(0)
                && metrics["phase_timing"]["artifact_assembly_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["publication_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["recover_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["replay_commit_micros"].as_u64().unwrap_or(0) > 0
                && metrics["hot_counters"]["full_state_clones"].as_u64() == Some(0)
                && metrics["cold_counters"]["full_state_clones"].as_u64() == Some(0)
        },
    );

    let chip_rich_compile_samples = capture_perf_samples(
        suite,
        "chip_rich_compile_hot_vs_recovery_compile",
        || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::ChipRichCertification,
            );
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();

            let source = create_entity_in_partition(
                &mut runtime,
                "chip-rich-hot-cold-source",
                PartitionId(7),
            );
            let sinks = (0..8)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("chip-rich-hot-cold-sink-{index}"),
                        if index % 2 == 0 {
                            PartitionId(11)
                        } else {
                            PartitionId(12)
                        },
                    )
                })
                .collect::<Vec<_>>();
            for (index, sink) in sinks.iter().enumerate() {
                create_relation_in_partition(
                    &mut runtime,
                    source,
                    *sink,
                    &format!("chip-rich-hot-cold-link-{index}"),
                    PartitionId(19),
                );
            }

            runtime.performance_access().reset_counters();
            let hot_commit_started_at = Instant::now();
            let hot_commit = update_entity(&mut runtime, source, "chip-rich-hot-cold-updated");
            let hot_commit_micros = hot_commit_started_at.elapsed().as_micros();
            let latest_commit = runtime
                .history()
                .latest_commit()
                .expect("chip rich hot/cold latest commit")
                .clone();
            let hot_compile_started_at = Instant::now();
            let hot_artifact = runtime
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    latest_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(12),
                        PartitionId(19),
                    ],
                )
                .expect("hot rich chip compiled artifact");
            let hot_compile_micros = hot_compile_started_at.elapsed().as_micros();
            let (hot_diagnostic_artifact_count, hot_detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            let checkpoint_started_at = Instant::now();
            runtime
                .durability_authority()
                .checkpoint()
                .expect("chip rich hot/cold checkpoint");
            let checkpoint_micros = checkpoint_started_at.elapsed().as_micros();

            let plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            apply_perf_diagnostics_policy(
                &mut recovered,
                PerfDiagnosticsPolicy::ChipRichCertification,
            );
            recovered.performance_access().reset_counters();
            let recover_started_at = Instant::now();
            recovered
                .durability_authority()
                .recover(plan)
                .expect("chip rich hot/cold recovery");
            let recover_micros = recover_started_at.elapsed().as_micros();

            let replay_started_at = Instant::now();
            let replay = recovered
                .replay_authority()
                .replay_commit(RelationalReplayRequest {
                    commit_id: hot_commit.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                    verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();

            let recovered_commit = recovered
                .history()
                .latest_commit()
                .expect("recovered rich chip latest commit")
                .clone();
            let cold_compile_started_at = Instant::now();
            let cold_artifact = recovered
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    recovered_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(12),
                        PartitionId(19),
                    ],
                )
                .expect("cold rich chip compiled artifact");
            let cold_compile_micros = cold_compile_started_at.elapsed().as_micros();

            let elapsed_micros = hot_commit_micros
                + hot_compile_micros
                + checkpoint_micros
                + recover_micros
                + replay_commit_micros
                + cold_compile_micros;
            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "hot_changed_records": hot_commit.changed_records.len(),
                    "hot_diagnostic_artifact_count": hot_diagnostic_artifact_count,
                    "hot_detailed_trace_entries": hot_detailed_trace_entries,
                    "replay_mismatch_count": replay.mismatches.len(),
                    "replay_failure": replay.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "hot_compatibility": format!(
                        "{:?}",
                        runtime.compiled_artifacts().compiled_artifact_compatibility(hot_artifact.artifact_id)
                    ),
                    "cold_compatibility": format!(
                        "{:?}",
                        recovered.compiled_artifacts().compiled_artifact_compatibility(cold_artifact.artifact_id)
                    ),
                    "phase_timing": {
                        "hot_commit_micros": hot_commit_micros,
                        "hot_compile_micros": hot_compile_micros,
                        "checkpoint_micros": checkpoint_micros,
                        "recover_micros": recover_micros,
                        "replay_commit_micros": replay_commit_micros,
                        "cold_compile_micros": cold_compile_micros,
                    },
                    "hot_counters": runtime.performance_access().counters(),
                    "cold_counters": recovered.performance_access().counters(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "chip_rich_compile_hot_vs_recovery_compile",
        &chip_rich_compile_samples,
        &[
            ("hot_commit_micros", &["phase_timing", "hot_commit_micros"]),
            (
                "hot_compile_micros",
                &["phase_timing", "hot_compile_micros"],
            ),
            ("checkpoint_micros", &["phase_timing", "checkpoint_micros"]),
            ("recover_micros", &["phase_timing", "recover_micros"]),
            (
                "replay_commit_micros",
                &["phase_timing", "replay_commit_micros"],
            ),
            (
                "cold_compile_micros",
                &["phase_timing", "cold_compile_micros"],
            ),
            (
                "hot_diagnostic_artifact_count",
                &["hot_diagnostic_artifact_count"],
            ),
            (
                "hot_detailed_trace_entries",
                &["hot_detailed_trace_entries"],
            ),
        ],
    );
    assert!(chip_rich_compile_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &chip_rich_compile_samples,
        "chip rich hot/cold certification should isolate compile and diagnostics on the hot side while preserving recovery-compile equivalence",
        |metrics| {
            metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["hot_diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["hot_detailed_trace_entries"].as_u64().unwrap_or(0) >= 1
                && metrics["replay_failure"].is_null()
                && metrics["replay_mismatch_count"].as_u64() == Some(0)
                && metrics["hot_compatibility"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactCompatibility::Compatible))
                && metrics["cold_compatibility"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactCompatibility::Compatible))
                && metrics["phase_timing"]["hot_compile_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["recover_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["cold_compile_micros"].as_u64().unwrap_or(0) > 0
                && metrics["hot_counters"]["full_state_clones"].as_u64() == Some(0)
                && metrics["cold_counters"]["full_state_clones"].as_u64() == Some(0)
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_artifact_recoverability_matrix() {
    let suite = "artifact_recoverability_matrix";

    let geometry_recoverability_samples = capture_perf_samples(
        suite,
        "geometry_diagnostics_summary_vs_trace_recoverability",
        || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::GeometryKernel,
            );
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();

            let source = create_entity_outcome(&mut runtime, "recover-geometry-source");
            let middle = create_entity_outcome(&mut runtime, "recover-geometry-middle");
            let target = create_entity_outcome(&mut runtime, "recover-geometry-target");
            let source_entity = changed_entities(&source)[0];
            let middle_entity = changed_entities(&middle)[0];
            let target_entity = changed_entities(&target)[0];
            create_relation_outcome(
                &mut runtime,
                source_entity,
                middle_entity,
                "recover-geometry-link-a",
            );
            create_relation_outcome(
                &mut runtime,
                middle_entity,
                target_entity,
                "recover-geometry-link-b",
            );

            let hot_commit_started_at = Instant::now();
            let hot_commit = update_entity(
                &mut runtime,
                middle_entity,
                "recover-geometry-middle-updated",
            );
            let hot_commit_micros = hot_commit_started_at.elapsed().as_micros();
            let hot_bundle = runtime
                .publication()
                .latest_bundle()
                .expect("geometry hot publication bundle")
                .clone();
            let hot_artifacts = runtime.publication().diagnostics_since(diagnostics_start);

            let checkpoint_started_at = Instant::now();
            runtime
                .durability_authority()
                .checkpoint()
                .expect("geometry recoverability checkpoint");
            let checkpoint_micros = checkpoint_started_at.elapsed().as_micros();

            let plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::GeometryKernel,
            );
            let recover_started_at = Instant::now();
            recovered
                .durability_authority()
                .recover(plan)
                .expect("geometry recoverability recovery");
            let recover_micros = recover_started_at.elapsed().as_micros();

            let replay_started_at = Instant::now();
            let replay = recovered
                .replay_authority()
                .replay_commit(RelationalReplayRequest {
                    commit_id: hot_commit.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                    verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
                });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();
            let recovered_envelope = recovered
                .replay()
                .canonical_commit_envelope(hot_commit.commit.commit_id)
                .cloned()
                .expect("recovered canonical geometry envelope");

            PerfMeasurement {
                elapsed_micros: hot_commit_micros
                    + checkpoint_micros
                    + recover_micros
                    + replay_commit_micros,
                metrics: json!({
                    "hot_summary_entry_count": hot_bundle.diagnostics_summary.entries.len(),
                    "hot_total_artifacts": hot_artifacts.len(),
                    "hot_total_entries": diagnostic_entry_count(&hot_artifacts),
                    "hot_detailed_trace_artifact_count": diagnostic_artifact_kind_count(
                        &hot_artifacts,
                        DiagnosticsArtifactKind::DetailedTrace,
                    ),
                    "hot_detailed_trace_entry_count": hot_artifacts
                        .iter()
                        .filter(|artifact| artifact.kind == DiagnosticsArtifactKind::DetailedTrace)
                        .map(|artifact| artifact.entries.len())
                        .sum::<usize>(),
                    "hot_history_scope_artifact_count": diagnostic_artifact_scope_count(
                        &hot_artifacts,
                        DiagnosticsScope::History,
                    ),
                    "hot_query_scope_artifact_count": diagnostic_artifact_scope_count(
                        &hot_artifacts,
                        DiagnosticsScope::QueryPlanning,
                    ),
                    "hot_commit_published_entries": diagnostic_entry_code_count(
                        &hot_artifacts,
                        DiagnosticCode::CommitPublished,
                    ),
                    "recovered_summary_entry_count": recovered_envelope.diagnostics_summary.entries.len(),
                    "summary_digest_match": certification_digest(&hot_bundle.diagnostics_summary)
                        == certification_digest(&recovered_envelope.diagnostics_summary),
                    "replay_compared_diagnostics_surface": replay
                        .compared_surfaces
                        .contains(&crate::facade::replay::ReplayObservableSurface::Diagnostics),
                    "replay_mismatch_count": replay.mismatches.len(),
                    "replay_failure": replay.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "phase_timing": {
                        "hot_commit_micros": hot_commit_micros,
                        "checkpoint_micros": checkpoint_micros,
                        "recover_micros": recover_micros,
                        "replay_commit_micros": replay_commit_micros,
                    },
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "geometry_diagnostics_summary_vs_trace_recoverability",
        &geometry_recoverability_samples,
        &[
            ("hot_commit_micros", &["phase_timing", "hot_commit_micros"]),
            ("checkpoint_micros", &["phase_timing", "checkpoint_micros"]),
            ("recover_micros", &["phase_timing", "recover_micros"]),
            (
                "replay_commit_micros",
                &["phase_timing", "replay_commit_micros"],
            ),
            ("hot_summary_entry_count", &["hot_summary_entry_count"]),
            ("hot_total_artifacts", &["hot_total_artifacts"]),
            ("hot_total_entries", &["hot_total_entries"]),
            (
                "hot_detailed_trace_artifact_count",
                &["hot_detailed_trace_artifact_count"],
            ),
            (
                "hot_detailed_trace_entry_count",
                &["hot_detailed_trace_entry_count"],
            ),
            (
                "recovered_summary_entry_count",
                &["recovered_summary_entry_count"],
            ),
        ],
    );
    assert_budget(
        &geometry_recoverability_samples,
        "geometry diagnostics recoverability should prove canonical summary replay parity while treating detailed traces as deferred hot-path richness rather than required replay truth",
        |metrics| {
            metrics["hot_summary_entry_count"].as_u64().unwrap_or(0) >= 1
                && metrics["hot_detailed_trace_artifact_count"].as_u64() == Some(0)
                && metrics["hot_detailed_trace_entry_count"].as_u64() == Some(0)
                && metrics["summary_digest_match"].as_bool() == Some(true)
                && metrics["replay_compared_diagnostics_surface"].as_bool() == Some(true)
                && metrics["replay_failure"].is_null()
                && metrics["replay_mismatch_count"].as_u64() == Some(0)
        },
    );

    let chip_recoverability_samples = capture_perf_samples(
        suite,
        "chip_compiled_artifact_recoverability",
        || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::ChipOperationalHotPath,
            );

            let source =
                create_entity_in_partition(&mut runtime, "recover-chip-source", PartitionId(7));
            let sinks = (0..8)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("recover-chip-sink-{index}"),
                        if index % 2 == 0 {
                            PartitionId(11)
                        } else {
                            PartitionId(12)
                        },
                    )
                })
                .collect::<Vec<_>>();
            for (index, sink) in sinks.iter().enumerate() {
                create_relation_in_partition(
                    &mut runtime,
                    source,
                    *sink,
                    &format!("recover-chip-link-{index}"),
                    PartitionId(19),
                );
            }

            let hot_commit_started_at = Instant::now();
            let hot_commit = update_entity(&mut runtime, source, "recover-chip-updated");
            let hot_commit_micros = hot_commit_started_at.elapsed().as_micros();
            let latest_commit = runtime
                .history()
                .latest_commit()
                .expect("recoverability latest commit")
                .clone();
            let hot_compile_started_at = Instant::now();
            let hot_artifact = runtime
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    latest_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(12),
                        PartitionId(19),
                    ],
                )
                .expect("hot recoverable compiled artifact");
            let hot_compile_micros = hot_compile_started_at.elapsed().as_micros();

            let checkpoint_started_at = Instant::now();
            runtime
                .durability_authority()
                .checkpoint()
                .expect("chip recoverability checkpoint");
            let checkpoint_micros = checkpoint_started_at.elapsed().as_micros();

            let plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            let recover_started_at = Instant::now();
            recovered
                .durability_authority()
                .recover(plan)
                .expect("chip recoverability recovery");
            let recover_micros = recover_started_at.elapsed().as_micros();

            let replay_started_at = Instant::now();
            let replay = recovered
                .replay_authority()
                .replay_commit(RelationalReplayRequest {
                    commit_id: hot_commit.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                    verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();
            let recovered_commit = recovered
                .history()
                .latest_commit()
                .expect("recovered chip latest commit")
                .clone();
            let cold_compile_started_at = Instant::now();
            let cold_artifact = recovered
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    recovered_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(12),
                        PartitionId(19),
                    ],
                )
                .expect("cold recoverable compiled artifact");
            let cold_compile_micros = cold_compile_started_at.elapsed().as_micros();

            PerfMeasurement {
                elapsed_micros: hot_commit_micros
                    + hot_compile_micros
                    + checkpoint_micros
                    + recover_micros
                    + replay_commit_micros
                    + cold_compile_micros,
                metrics: json!({
                    "hot_compiled_record_count": hot_artifact.compiled_record_count,
                    "cold_compiled_record_count": cold_artifact.compiled_record_count,
                    "hot_partition_count": hot_artifact.partition_ids.len(),
                    "cold_partition_count": cold_artifact.partition_ids.len(),
                    "hot_compatibility": format!(
                        "{:?}",
                        runtime.compiled_artifacts().compiled_artifact_compatibility(hot_artifact.artifact_id)
                    ),
                    "cold_compatibility": format!(
                        "{:?}",
                        recovered.compiled_artifacts().compiled_artifact_compatibility(cold_artifact.artifact_id)
                    ),
                    "replay_failure": replay.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "replay_mismatch_count": replay.mismatches.len(),
                    "phase_timing": {
                        "hot_commit_micros": hot_commit_micros,
                        "hot_compile_micros": hot_compile_micros,
                        "checkpoint_micros": checkpoint_micros,
                        "recover_micros": recover_micros,
                        "replay_commit_micros": replay_commit_micros,
                        "cold_compile_micros": cold_compile_micros,
                    },
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "chip_compiled_artifact_recoverability",
        &chip_recoverability_samples,
        &[
            ("hot_commit_micros", &["phase_timing", "hot_commit_micros"]),
            (
                "hot_compile_micros",
                &["phase_timing", "hot_compile_micros"],
            ),
            ("checkpoint_micros", &["phase_timing", "checkpoint_micros"]),
            ("recover_micros", &["phase_timing", "recover_micros"]),
            (
                "replay_commit_micros",
                &["phase_timing", "replay_commit_micros"],
            ),
            (
                "cold_compile_micros",
                &["phase_timing", "cold_compile_micros"],
            ),
            ("hot_compiled_record_count", &["hot_compiled_record_count"]),
            (
                "cold_compiled_record_count",
                &["cold_compiled_record_count"],
            ),
        ],
    );
    assert_budget(
        &chip_recoverability_samples,
        "chip compiled artifacts should be safely reconstructable after recovery and replay rather than requiring hot-path persistence",
        |metrics| {
            metrics["hot_compiled_record_count"] == metrics["cold_compiled_record_count"]
                && metrics["hot_partition_count"] == metrics["cold_partition_count"]
                && metrics["hot_compatibility"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactCompatibility::Compatible))
                && metrics["cold_compatibility"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactCompatibility::Compatible))
                && metrics["replay_failure"].is_null()
                && metrics["replay_mismatch_count"].as_u64() == Some(0)
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_geometry_artifact_decomposition_matrix() {
    let suite = "geometry_artifact_decomposition_matrix";
    let node_count = rocketship_node_count();
    let query_target_count = rocketship_query_target_count(node_count);

    let artifact_decomposition_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_pseudorealistic_rich_artifact_classes",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 4;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded =
                seed_pseudorealistic_rocketship_world(&mut runtime, node_count, query_target_count);

            let hot_update_started_at = Instant::now();
            let update = update_entity(
                &mut runtime,
                seeded.hot_update_target,
                "rocketship-rich-artifact-update",
            );
            let hot_update_micros = hot_update_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_targets = seeded
                .mixed_query_targets
                .iter()
                .take(12)
                .cloned()
                .collect::<Vec<_>>();
            let explicit_started_at = Instant::now();
            let explicit_outcome = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(
                            &snapshot,
                            explicit_query_packet(
                                &runtime,
                                &snapshot,
                                "rocketship-rich-artifact-explicit",
                                explicit_targets.clone(),
                            ),
                        )
                        .expect("planned artifact decomposition explicit query"),
                )
                .expect("artifact decomposition explicit query");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();

            let diagnostics = runtime.publication().diagnostics_since(diagnostics_start);
            let distinct_scopes = diagnostics
                .iter()
                .map(|artifact| format!("{:?}", artifact.scope))
                .collect::<BTreeSet<_>>()
                .len();

            PerfMeasurement {
                elapsed_micros: seeded.entity_commit_micros
                    + seeded.relation_commit_micros
                    + hot_update_micros
                    + explicit_query_micros,
                metrics: json!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "subsystem_count": seeded.subsystem_count,
                    "hot_update_micros": hot_update_micros,
                    "explicit_query_micros": explicit_query_micros,
                    "hot_changed_records": update.changed_records.len(),
                    "explicit_result_entities": explicit_outcome.result.entities.len(),
                    "artifact_count_total": diagnostics.len(),
                    "artifact_entry_count_total": diagnostic_entry_count(&diagnostics),
                    "artifact_kind_minimal_summary_count": diagnostic_artifact_kind_count(
                        &diagnostics,
                        DiagnosticsArtifactKind::MinimalSummary,
                    ),
                    "artifact_kind_detailed_trace_count": diagnostic_artifact_kind_count(
                        &diagnostics,
                        DiagnosticsArtifactKind::DetailedTrace,
                    ),
                    "artifact_scope_history_count": diagnostic_artifact_scope_count(
                        &diagnostics,
                        DiagnosticsScope::History,
                    ),
                    "artifact_scope_query_planning_count": diagnostic_artifact_scope_count(
                        &diagnostics,
                        DiagnosticsScope::QueryPlanning,
                    ),
                    "artifact_scope_snapshot_count": diagnostic_artifact_scope_count(
                        &diagnostics,
                        DiagnosticsScope::Snapshot,
                    ),
                    "artifact_scope_count_distinct": distinct_scopes,
                    "entry_code_commit_published_count": diagnostic_entry_code_count(
                        &diagnostics,
                        DiagnosticCode::CommitPublished,
                    ),
                    "entry_code_snapshot_read_path_count": diagnostic_entry_code_count(
                        &diagnostics,
                        DiagnosticCode::SnapshotReadPathInspected,
                    ),
                    "entry_code_visibility_cache_hit_count": diagnostic_entry_code_count(
                        &diagnostics,
                        DiagnosticCode::VisibilityCacheHit,
                    ),
                    "entry_code_visibility_cache_miss_count": diagnostic_entry_code_count(
                        &diagnostics,
                        DiagnosticCode::VisibilityCacheMissReconstructed,
                    ),
                    "counters": runtime.performance_access().counters(),
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_pseudorealistic_rich_artifact_classes",
        &artifact_decomposition_samples,
        &[
            ("resident_node_count", &["resident_node_count"]),
            ("resident_relation_count", &["resident_relation_count"]),
            ("subsystem_count", &["subsystem_count"]),
            ("hot_update_micros", &["hot_update_micros"]),
            ("explicit_query_micros", &["explicit_query_micros"]),
            ("artifact_count_total", &["artifact_count_total"]),
            (
                "artifact_entry_count_total",
                &["artifact_entry_count_total"],
            ),
            (
                "artifact_kind_minimal_summary_count",
                &["artifact_kind_minimal_summary_count"],
            ),
            (
                "artifact_kind_detailed_trace_count",
                &["artifact_kind_detailed_trace_count"],
            ),
            (
                "artifact_scope_count_distinct",
                &["artifact_scope_count_distinct"],
            ),
        ],
    );
    assert_budget(
        &artifact_decomposition_samples,
        "rich geometry scale decomposition should prove hot-path traces are deferred at rocketship size instead of hiding them behind one giant total",
        |metrics| {
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && metrics["subsystem_count"].as_u64() == Some(12)
                && metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["explicit_result_entities"].as_u64() == Some(12)
                && metrics["artifact_kind_minimal_summary_count"].as_u64().unwrap_or(0) >= 1
                && metrics["artifact_kind_detailed_trace_count"].as_u64() == Some(0)
                && metrics["artifact_scope_count_distinct"].as_u64().unwrap_or(0) >= 2
                && metrics["entry_code_commit_published_count"].as_u64().unwrap_or(0) >= 1
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_runtime_bridge_mock_matrix() {
    let suite = "runtime_bridge_mock_matrix";

    for (case, development_profile) in [
        ("geometry_commit_bridge_wave_operational", false),
        ("geometry_commit_bridge_wave_development", true),
    ] {
        let samples = capture_perf_samples(suite, case, || {
            let mut relational =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
            relational
                .config
                .diagnostics
                .profile
                .detailed_traces_enabled = development_profile;
            relational
                .config
                .diagnostics
                .profile
                .max_entries_per_artifact = if development_profile { 256 } else { 0 };

            let source = create_entity_outcome(&mut relational, "merged-geometry-source");
            let middle = create_entity_outcome(&mut relational, "merged-geometry-middle");
            let target = create_entity_outcome(&mut relational, "merged-geometry-target");
            let source_entity = changed_entities(&source)[0];
            let middle_entity = changed_entities(&middle)[0];
            let target_entity = changed_entities(&target)[0];
            create_relation_outcome(
                &mut relational,
                source_entity,
                middle_entity,
                "merged-geometry-link-a",
            );
            create_relation_outcome(
                &mut relational,
                middle_entity,
                target_entity,
                "merged-geometry-link-b",
            );

            let mut bridge_runtime = build_mock_bridge_runtime(development_profile, 4);

            let relational_commit_started_at = Instant::now();
            let update = update_entity(
                &mut relational,
                middle_entity,
                "merged-geometry-middle-updated",
            );
            let relational_commit_micros = relational_commit_started_at.elapsed().as_micros();

            let snapshot = relational.visibility_authority().snapshot();
            let traversal_packet = PlannedQueryPacket {
                label: "merged-relational-signal-traversal".to_string(),
                context_id: relational
                    .read_truth()
                    .query_plan_context(&snapshot)
                    .expect("merged query plan context"),
                scope: QueryScope::ConnectivityTraversal {
                    seeds: Arc::from([source_entity, middle_entity]),
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                    max_depth: Some(2),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                fallback: QueryFallbackContract::StorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(92_001),
                target_count_hint: 2,
            };
            let relational_query_started_at = Instant::now();
            let traversal = relational
                .read_truth()
                .execute_query_plan(
                    relational
                        .read_truth()
                        .plan_query_packet(&snapshot, traversal_packet)
                        .expect("merged traversal plan"),
                )
                .expect("merged traversal outcome");
            let relational_query_micros = relational_query_started_at.elapsed().as_micros();

            let affected_sources = traversal
                .result
                .entities
                .len()
                .min(bridge_runtime.source_versions.len())
                .max(1);
            let bridge_before = bridge_runtime.observe();
            let bridge_started_at = Instant::now();
            bridge_runtime.apply_changes(affected_sources);
            let bridge_micros = bridge_started_at.elapsed().as_micros();
            let bridge_after = bridge_runtime.observe();
            let bridge_history_entries = bridge_runtime.recent_history_len();

            PerfMeasurement {
                elapsed_micros: relational_commit_micros + relational_query_micros + bridge_micros,
                metrics: json!({
                    "relational_changed_records": update.changed_records.len(),
                    "relational_result_entities": traversal.result.entities.len(),
                    "affected_bridge_sources": affected_sources,
                    "bridge_nodes_evaluated": bridge_after.evaluation.nodes_evaluated
                        - bridge_before.evaluation.nodes_evaluated,
                    "bridge_nodes_recomputed": bridge_after.evaluation.nodes_recomputed
                        - bridge_before.evaluation.nodes_recomputed,
                    "bridge_tasks_scheduled": bridge_after.planner.tasks_scheduled
                        - bridge_before.planner.tasks_scheduled,
                    "bridge_tasks_pruned": bridge_after.planner.tasks_pruned_before_execution
                        - bridge_before.planner.tasks_pruned_before_execution,
                    "bridge_suppressed_downstream": bridge_after.evaluation.suppressed_downstream_propagations
                        - bridge_before.evaluation.suppressed_downstream_propagations,
                    "bridge_history_entries": bridge_history_entries,
                    "bridge_has_latest_flow": bridge_runtime.latest_flow_diagnostics().is_some(),
                    "phase_timing": {
                        "relational_commit_micros": relational_commit_micros,
                        "relational_query_micros": relational_query_micros,
                        "bridge_micros": bridge_micros,
                    },
                }),
            }
        });
        emit_metric_summaries(
            suite,
            case,
            &samples,
            &[
                (
                    "relational_commit_micros",
                    &["phase_timing", "relational_commit_micros"],
                ),
                (
                    "relational_query_micros",
                    &["phase_timing", "relational_query_micros"],
                ),
                ("bridge_micros", &["phase_timing", "bridge_micros"]),
                ("affected_bridge_sources", &["affected_bridge_sources"]),
                ("bridge_nodes_evaluated", &["bridge_nodes_evaluated"]),
                ("bridge_nodes_recomputed", &["bridge_nodes_recomputed"]),
                ("bridge_tasks_scheduled", &["bridge_tasks_scheduled"]),
                ("bridge_history_entries", &["bridge_history_entries"]),
            ],
        );
        assert_budget(
            &samples,
            "mocked bridge certification should keep truth updates narrow while surfacing downstream invalidation and recomputation work without crossing the crate boundary",
            |metrics| {
                let affected = metrics["affected_bridge_sources"].as_u64().unwrap_or(0);
                metrics["relational_changed_records"].as_u64() == Some(1)
                    && metrics["relational_result_entities"].as_u64().unwrap_or(0) >= 2
                    && affected >= 1
                    && metrics["bridge_nodes_recomputed"].as_u64().unwrap_or(0) >= affected
                    && metrics["bridge_tasks_scheduled"].as_u64().unwrap_or(0) >= affected
                    && metrics["bridge_history_entries"].as_u64().unwrap_or(0) >= 1
                    && metrics["bridge_has_latest_flow"].as_bool() == Some(true)
            },
        );
    }

    for (case, development_profile) in [
        (
            "geometry_commit_bridge_wave_medium_region_operational",
            false,
        ),
        (
            "geometry_commit_bridge_wave_medium_region_development",
            true,
        ),
    ] {
        let samples = capture_perf_samples(suite, case, || {
            let mut relational =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
            relational
                .config
                .diagnostics
                .profile
                .detailed_traces_enabled = development_profile;
            relational
                .config
                .diagnostics
                .profile
                .max_entries_per_artifact = if development_profile { 256 } else { 0 };

            let entities = seed_bridge_region_world(&mut relational, "bridge-medium", 24, 4);
            let updated = entities[10];
            let seeds = Arc::from([entities[8], entities[10], entities[12], entities[14]]);
            let mut bridge_runtime = build_mock_bridge_runtime(development_profile, entities.len());

            let relational_commit_started_at = Instant::now();
            let update = update_entity(&mut relational, updated, "bridge-medium-updated");
            let relational_commit_micros = relational_commit_started_at.elapsed().as_micros();

            let snapshot = relational.visibility_authority().snapshot();
            let traversal_packet = PlannedQueryPacket {
                label: "bridge-medium-traversal".to_string(),
                context_id: relational
                    .read_truth()
                    .query_plan_context(&snapshot)
                    .expect("bridge medium query plan context"),
                scope: QueryScope::ConnectivityTraversal {
                    seeds,
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                    max_depth: Some(3),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                fallback: QueryFallbackContract::StorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(92_101),
                target_count_hint: 4,
            };
            let relational_query_started_at = Instant::now();
            let traversal = relational
                .read_truth()
                .execute_query_plan(
                    relational
                        .read_truth()
                        .plan_query_packet(&snapshot, traversal_packet)
                        .expect("bridge medium traversal plan"),
                )
                .expect("bridge medium traversal outcome");
            let relational_query_micros = relational_query_started_at.elapsed().as_micros();

            let affected_sources = traversal
                .result
                .entities
                .len()
                .min(bridge_runtime.source_versions.len())
                .max(4);
            let bridge_before = bridge_runtime.observe();
            let bridge_started_at = Instant::now();
            bridge_runtime.apply_changes(affected_sources);
            let bridge_micros = bridge_started_at.elapsed().as_micros();
            let bridge_after = bridge_runtime.observe();

            PerfMeasurement {
                elapsed_micros: relational_commit_micros + relational_query_micros + bridge_micros,
                metrics: json!({
                    "resident_entities": entities.len(),
                    "relational_changed_records": update.changed_records.len(),
                    "relational_result_entities": traversal.result.entities.len(),
                    "affected_bridge_sources": affected_sources,
                    "bridge_nodes_evaluated": bridge_after.evaluation.nodes_evaluated
                        - bridge_before.evaluation.nodes_evaluated,
                    "bridge_nodes_recomputed": bridge_after.evaluation.nodes_recomputed
                        - bridge_before.evaluation.nodes_recomputed,
                    "bridge_tasks_scheduled": bridge_after.planner.tasks_scheduled
                        - bridge_before.planner.tasks_scheduled,
                    "bridge_tasks_pruned": bridge_after.planner.tasks_pruned_before_execution
                        - bridge_before.planner.tasks_pruned_before_execution,
                    "bridge_history_entries": bridge_runtime.recent_history_len(),
                    "phase_timing": {
                        "relational_commit_micros": relational_commit_micros,
                        "relational_query_micros": relational_query_micros,
                        "bridge_micros": bridge_micros,
                    },
                }),
            }
        });
        emit_metric_summaries(
            suite,
            case,
            &samples,
            &[
                (
                    "relational_commit_micros",
                    &["phase_timing", "relational_commit_micros"],
                ),
                (
                    "relational_query_micros",
                    &["phase_timing", "relational_query_micros"],
                ),
                ("bridge_micros", &["phase_timing", "bridge_micros"]),
                ("resident_entities", &["resident_entities"]),
                ("affected_bridge_sources", &["affected_bridge_sources"]),
                ("bridge_nodes_recomputed", &["bridge_nodes_recomputed"]),
                ("bridge_tasks_scheduled", &["bridge_tasks_scheduled"]),
            ],
        );
        assert_budget(
            &samples,
            "medium bridge region certification should scale recompute with the affected region instead of the whole resident world",
            |metrics| {
                let affected = metrics["affected_bridge_sources"].as_u64().unwrap_or(0);
                let resident = metrics["resident_entities"].as_u64().unwrap_or(0);
                metrics["relational_changed_records"].as_u64() == Some(1)
                    && metrics["relational_result_entities"].as_u64().unwrap_or(0) >= 8
                    && affected >= 8
                    && affected < resident
                    && metrics["bridge_nodes_recomputed"].as_u64().unwrap_or(0) >= affected
                    && metrics["bridge_nodes_recomputed"].as_u64().unwrap_or(0) <= affected * 4
                    && metrics["bridge_tasks_scheduled"].as_u64().unwrap_or(0) >= affected
                    && metrics["bridge_tasks_scheduled"].as_u64().unwrap_or(0) <= affected * 3
            },
        );
    }

    let mixed_locality_samples = capture_perf_samples(
        suite,
        "geometry_commit_bridge_wave_mixed_locality_operational",
        || {
            let mut relational =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
            let entities = seed_bridge_region_world(&mut relational, "bridge-mixed", 20, 5);
            let updated = entities[9];
            let query_targets = [
                "bridge-mixed-node-2",
                "bridge-mixed-node-7",
                "bridge-mixed-node-11",
                "bridge-mixed-node-16",
            ];
            let traversal_seeds = Arc::from([entities[7], entities[9]]);
            let mut bridge_runtime = build_mock_bridge_runtime(false, entities.len());

            let relational_commit_started_at = Instant::now();
            let update = update_entity(&mut relational, updated, "bridge-mixed-updated");
            let relational_commit_micros = relational_commit_started_at.elapsed().as_micros();

            let snapshot = relational.visibility_authority().snapshot();
            let traversal_packet = PlannedQueryPacket {
                label: "bridge-mixed-traversal".to_string(),
                context_id: relational
                    .read_truth()
                    .query_plan_context(&snapshot)
                    .expect("bridge mixed query plan context"),
                scope: QueryScope::ConnectivityTraversal {
                    seeds: traversal_seeds,
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                    max_depth: Some(2),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                fallback: QueryFallbackContract::StorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(92_201),
                target_count_hint: 2,
            };
            let relational_query_started_at = Instant::now();
            let traversal = relational
                .read_truth()
                .execute_query_plan(
                    relational
                        .read_truth()
                        .plan_query_packet(&snapshot, traversal_packet)
                        .expect("bridge mixed traversal plan"),
                )
                .expect("bridge mixed traversal outcome");
            let explicit_hits = query_targets
                .iter()
                .map(|name| {
                    relational
                        .read_truth()
                        .execute_query_plan(
                            relational
                                .read_truth()
                                .plan_query_packet(
                                    &snapshot,
                                    entity_name_index_packet(
                                        &relational,
                                        &snapshot,
                                        "bridge-mixed-explicit",
                                        name,
                                    ),
                                )
                                .expect("bridge mixed explicit plan"),
                        )
                        .expect("bridge mixed explicit outcome")
                        .result
                        .entities
                        .len()
                })
                .sum::<usize>();
            let relational_query_micros = relational_query_started_at.elapsed().as_micros();

            let affected_sources = (traversal.result.entities.len() + explicit_hits)
                .min(bridge_runtime.source_versions.len())
                .max(4);
            let bridge_before = bridge_runtime.observe();
            let bridge_started_at = Instant::now();
            bridge_runtime.apply_changes(affected_sources);
            let bridge_micros = bridge_started_at.elapsed().as_micros();
            let bridge_after = bridge_runtime.observe();

            PerfMeasurement {
                elapsed_micros: relational_commit_micros + relational_query_micros + bridge_micros,
                metrics: json!({
                    "resident_entities": entities.len(),
                    "relational_changed_records": update.changed_records.len(),
                    "traversal_result_entities": traversal.result.entities.len(),
                    "explicit_result_entities": explicit_hits,
                    "affected_bridge_sources": affected_sources,
                    "bridge_nodes_evaluated": bridge_after.evaluation.nodes_evaluated
                        - bridge_before.evaluation.nodes_evaluated,
                    "bridge_nodes_recomputed": bridge_after.evaluation.nodes_recomputed
                        - bridge_before.evaluation.nodes_recomputed,
                    "bridge_tasks_scheduled": bridge_after.planner.tasks_scheduled
                        - bridge_before.planner.tasks_scheduled,
                    "bridge_tasks_pruned": bridge_after.planner.tasks_pruned_before_execution
                        - bridge_before.planner.tasks_pruned_before_execution,
                    "bridge_history_entries": bridge_runtime.recent_history_len(),
                    "phase_timing": {
                        "relational_commit_micros": relational_commit_micros,
                        "relational_query_micros": relational_query_micros,
                        "bridge_micros": bridge_micros,
                    },
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "geometry_commit_bridge_wave_mixed_locality_operational",
        &mixed_locality_samples,
        &[
            (
                "relational_commit_micros",
                &["phase_timing", "relational_commit_micros"],
            ),
            (
                "relational_query_micros",
                &["phase_timing", "relational_query_micros"],
            ),
            ("bridge_micros", &["phase_timing", "bridge_micros"]),
            ("traversal_result_entities", &["traversal_result_entities"]),
            ("explicit_result_entities", &["explicit_result_entities"]),
            ("affected_bridge_sources", &["affected_bridge_sources"]),
            ("bridge_tasks_scheduled", &["bridge_tasks_scheduled"]),
        ],
    );
    assert_budget(
        &mixed_locality_samples,
        "mixed locality bridge certification should keep explicit and traversal reads additive without exploding downstream recompute",
        |metrics| {
            let traversal = metrics["traversal_result_entities"].as_u64().unwrap_or(0);
            let explicit = metrics["explicit_result_entities"].as_u64().unwrap_or(0);
            let affected = metrics["affected_bridge_sources"].as_u64().unwrap_or(0);
            metrics["relational_changed_records"].as_u64() == Some(1)
                && traversal >= 4
                && explicit >= 4
                && affected >= explicit
                && metrics["bridge_tasks_scheduled"].as_u64().unwrap_or(0) >= affected
                && metrics["bridge_nodes_recomputed"].as_u64().unwrap_or(0) >= affected
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_game_engine_matrix() {
    let suite = "game_engine_matrix";

    let local_scene_wave_samples =
        capture_perf_samples(suite, "local_scene_graph_propagation_wave", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore);
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            let seeded = seed_game_engine_frame_world(&mut runtime, "scene-local", 8, 24);
            let updated = seeded.frame_targets[3];
            let explicit_targets = seeded
                .explicit_targets
                .iter()
                .take(12)
                .map(|entity| RecordRef::Entity(*entity))
                .collect::<Vec<_>>();
            let traversal_seeds = Arc::from([
                seeded.propagation_seeds[1],
                seeded.propagation_seeds[2],
                seeded.propagation_seeds[3],
                seeded.propagation_seeds[4],
            ]);
            let mut bridge_runtime = build_mock_bridge_runtime(false, 32);

            runtime.performance_access().reset_counters();
            let update_started_at = Instant::now();
            let update = update_entity(&mut runtime, updated, "scene-local-updated");
            let update_micros = update_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let propagation_packet = PlannedQueryPacket {
                label: "scene-local-propagation".to_string(),
                context_id: runtime
                    .read_truth()
                    .query_plan_context(&snapshot)
                    .expect("scene local query context"),
                scope: QueryScope::ConnectivityTraversal {
                    seeds: traversal_seeds,
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                    max_depth: Some(3),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                fallback: QueryFallbackContract::StorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(93_001),
                target_count_hint: 4,
            };
            let propagation_started_at = Instant::now();
            let propagation = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, propagation_packet)
                        .expect("scene local propagation plan"),
                )
                .expect("scene local propagation outcome");
            let propagation_micros = propagation_started_at.elapsed().as_micros();

            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "scene-local-explicit",
                explicit_targets,
            );
            let explicit_started_at = Instant::now();
            let explicit = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("scene local explicit plan"),
                )
                .expect("scene local explicit outcome");
            let explicit_micros = explicit_started_at.elapsed().as_micros();
            assert!(runtime.visibility_authority().release_snapshot(&snapshot));

            let affected_sources = (propagation.result.entities.len()
                + explicit.result.entities.len())
            .min(bridge_runtime.source_versions.len())
            .max(4);
            let bridge_before = bridge_runtime.observe();
            let bridge_started_at = Instant::now();
            bridge_runtime.apply_changes(affected_sources);
            let bridge_micros = bridge_started_at.elapsed().as_micros();
            let bridge_after = bridge_runtime.observe();

            measurement_with_elapsed(
                update_micros + propagation_micros + explicit_micros + bridge_micros,
                || {
                    json!({
                        "region_count": seeded.region_count,
                        "resident_entities": seeded.entities.len(),
                        "resident_relations": seeded.relation_count,
                        "changed_records": update.changed_records.len(),
                        "update_micros": update_micros,
                        "propagation_micros": propagation_micros,
                        "explicit_query_micros": explicit_micros,
                        "bridge_micros": bridge_micros,
                        "propagation_result_entities": propagation.result.entities.len(),
                        "explicit_result_entities": explicit.result.entities.len(),
                        "affected_bridge_sources": affected_sources,
                        "bridge_nodes_recomputed": bridge_after.evaluation.nodes_recomputed
                            - bridge_before.evaluation.nodes_recomputed,
                        "bridge_tasks_scheduled": bridge_after.planner.tasks_scheduled
                            - bridge_before.planner.tasks_scheduled,
                        "counters": runtime.performance_access().counters(),
                    })
                },
            )
        });
    emit_metric_summaries(
        suite,
        "local_scene_graph_propagation_wave",
        &local_scene_wave_samples,
        &[
            ("update_micros", &["update_micros"]),
            ("propagation_micros", &["propagation_micros"]),
            ("explicit_query_micros", &["explicit_query_micros"]),
            ("bridge_micros", &["bridge_micros"]),
            (
                "propagation_result_entities",
                &["propagation_result_entities"],
            ),
            ("explicit_result_entities", &["explicit_result_entities"]),
            ("affected_bridge_sources", &["affected_bridge_sources"]),
            ("bridge_tasks_scheduled", &["bridge_tasks_scheduled"]),
        ],
    );
    assert_budget(
        &local_scene_wave_samples,
        "game-engine local scene waves should keep frame-local propagation and derived work region-bounded",
        |metrics| {
            let affected = metrics["affected_bridge_sources"].as_u64().unwrap_or(0);
            metrics["region_count"].as_u64() == Some(8)
                && metrics["resident_entities"].as_u64() == Some(192)
                && metrics["changed_records"].as_u64() == Some(1)
                && metrics["propagation_result_entities"].as_u64().unwrap_or(0) >= 8
                && metrics["explicit_result_entities"].as_u64().unwrap_or(0) == 12
                && affected >= 8
                && affected <= 32
                && metrics["bridge_tasks_scheduled"].as_u64().unwrap_or(0) >= affected
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );

    let flat_batch_wave_samples =
        capture_perf_samples(suite, "flat_entity_batch_region_wave", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore);
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            let seeded = seed_game_engine_frame_world(&mut runtime, "scene-batch", 8, 24);

            let mut partition_targets = BTreeMap::new();
            for entity in &seeded.entities {
                let targets = partition_targets
                    .entry(entity.partition_id)
                    .or_insert_with(Vec::new);
                if targets.len() < 8 {
                    targets.push(*entity);
                }
                if partition_targets.len() >= 4
                    && partition_targets.values().all(|targets| targets.len() >= 6)
                {
                    break;
                }
            }
            let batch_targets = partition_targets
                .values()
                .flat_map(|targets| targets.iter().take(6).copied())
                .collect::<Vec<_>>();
            assert!(
                batch_targets.len() >= 24,
                "batch wave should gather a multi-partition entity batch"
            );

            runtime.performance_access().reset_counters();
            let update_started_at = Instant::now();
            let update = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                let mut batch = WorkerIntentBatch::new("scene-batch-flat-entity-wave");
                for (index, entity) in batch_targets.iter().enumerate() {
                    batch = batch.push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                        UpdateEntityFieldsIntent {
                            entity_id: *entity,
                            fields: crate::tests::support::aspect_field_patch_from_values([
                                (
                                    "name",
                                    crate::tests::support::string_aspect_value(&format!(
                                        "scene-batch-updated-{index}"
                                    )),
                                ),
                                (
                                    "phase",
                                    crate::tests::support::string_aspect_value("batch-wave"),
                                ),
                                (
                                    "partition",
                                    crate::tests::support::u64_aspect_value(
                                        entity.partition_id.0 as u64,
                                    ),
                                ),
                            ]),
                        },
                    )));
                }
                txn.push_batch(batch);
                txn.commit().expect("scene batch flat entity wave commit")
            };
            let update_micros = update_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_targets = batch_targets
                .iter()
                .take(12)
                .map(|entity| RecordRef::Entity(*entity))
                .collect::<Vec<_>>();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "scene-batch-explicit",
                explicit_targets,
            );
            let explicit_started_at = Instant::now();
            let explicit = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("scene batch explicit plan"),
                )
                .expect("scene batch explicit outcome");
            let explicit_micros = explicit_started_at.elapsed().as_micros();
            assert!(runtime.visibility_authority().release_snapshot(&snapshot));

            measurement_with_elapsed(update_micros + explicit_micros, || {
                json!({
                    "region_count": seeded.region_count,
                    "resident_entities": seeded.entities.len(),
                    "resident_relations": seeded.relation_count,
                    "batch_target_count": batch_targets.len(),
                    "batch_partition_count": partition_targets.len(),
                    "changed_records": update.changed_records.len(),
                    "update_micros": update_micros,
                    "explicit_query_micros": explicit_micros,
                    "explicit_result_entities": explicit.result.entities.len(),
                    "counters": runtime.performance_access().counters(),
                })
            })
        });
    emit_metric_summaries(
        suite,
        "flat_entity_batch_region_wave",
        &flat_batch_wave_samples,
        &[
            ("update_micros", &["update_micros"]),
            ("explicit_query_micros", &["explicit_query_micros"]),
            ("batch_target_count", &["batch_target_count"]),
            ("batch_partition_count", &["batch_partition_count"]),
            (
                "aosoa_entity_chunk_slots_materialized",
                &["counters", "aosoa_entity_chunk_slots_materialized"],
            ),
            (
                "aosoa_entity_chunks_published",
                &["counters", "aosoa_entity_chunks_published"],
            ),
        ],
    );
    assert_budget(
        &flat_batch_wave_samples,
        "game-engine flat entity batches should stay on the sparse AoSoA path across a few touched partitions",
        |metrics| {
            let batch_target_count = metrics["batch_target_count"].as_u64().unwrap_or(0);
            let batch_partition_count = metrics["batch_partition_count"].as_u64().unwrap_or(0);
            metrics["region_count"].as_u64() == Some(8)
                && batch_target_count >= 24
                && batch_partition_count >= 4
                && metrics["changed_records"].as_u64() == Some(batch_target_count)
                && counter_u64(metrics, "entity_slots_touched_by_commit") == batch_target_count
                && counter_u64(metrics, "partitions_touched_by_commit") >= batch_partition_count
                && counter_u64(metrics, "aosoa_entity_chunk_slots_materialized")
                    == batch_target_count
                && counter_u64(metrics, "aosoa_entity_chunks_published") >= batch_partition_count
                && counter_u64(metrics, "aosoa_publish_fallback_count") == 0
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );

    let mixed_frame_churn_samples = capture_perf_samples(
        suite,
        "mixed_read_write_frame_churn_window",
        || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore);
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            let seeded = seed_game_engine_frame_world(&mut runtime, "scene-frame", 8, 24);
            let mut bridge_runtime = build_mock_bridge_runtime(false, 48);

            const ITERATIONS: usize = 48;
            const WINDOW: usize = 12;
            let mut cycle_samples = Vec::with_capacity(ITERATIONS);
            let mut total_update_micros = 0u128;
            let mut total_propagation_micros = 0u128;
            let mut total_explicit_query_micros = 0u128;
            let mut total_bridge_micros = 0u128;
            let mut max_packets_per_iteration = 0usize;
            let mut max_scope_units_per_iteration = 0usize;
            let mut max_bridge_tasks_scheduled = 0u64;
            let mut previous_packets = 0usize;
            let mut previous_scope_units = 0usize;

            runtime.performance_access().reset_counters();
            for frame in 0..ITERATIONS {
                let actor = seeded.frame_targets[frame % seeded.frame_targets.len()];
                let update_started_at = Instant::now();
                let _ = update_entity(&mut runtime, actor, &format!("scene-frame-step-{frame}"));
                let update_micros = update_started_at.elapsed().as_micros();
                total_update_micros += update_micros;

                let snapshot = runtime.visibility_authority().snapshot();
                let propagation_packet = PlannedQueryPacket {
                    label: "scene-frame-propagation".to_string(),
                    context_id: runtime
                        .read_truth()
                        .query_plan_context(&snapshot)
                        .expect("scene frame query context"),
                    scope: QueryScope::ConnectivityTraversal {
                        seeds: Arc::from([
                            seeded.propagation_seeds[frame % seeded.propagation_seeds.len()],
                            seeded.propagation_seeds[(frame + 1) % seeded.propagation_seeds.len()],
                        ]),
                        relation_kind_scope: Some(Arc::from([KindId(2)])),
                        max_depth: Some(2),
                    },
                    locality: QueryLocalityClass::CrossPartitionTraversal,
                    ordering: QueryOrderingContract::CanonicalTraversalOrder,
                    fallback: QueryFallbackContract::StorageOnly,
                    execution_shape: QueryExecutionShape::BulkPacketized,
                    reduction: ReductionDiscipline::DeterministicMerge,
                    plan_key: DeterministicQueryPlanKey(93_101),
                    target_count_hint: 2,
                };
                let propagation_started_at = Instant::now();
                let propagation = runtime
                    .read_truth()
                    .execute_query_plan(
                        runtime
                            .read_truth()
                            .plan_query_packet(&snapshot, propagation_packet)
                            .expect("scene frame propagation plan"),
                    )
                    .expect("scene frame propagation outcome");
                let propagation_micros = propagation_started_at.elapsed().as_micros();
                total_propagation_micros += propagation_micros;

                let explicit_targets = seeded
                    .explicit_targets
                    .iter()
                    .cycle()
                    .skip(frame)
                    .take(8)
                    .map(|entity| RecordRef::Entity(*entity))
                    .collect::<Vec<_>>();
                let explicit_packet = explicit_query_packet(
                    &runtime,
                    &snapshot,
                    "scene-frame-explicit",
                    explicit_targets,
                );
                let explicit_started_at = Instant::now();
                let explicit = runtime
                    .read_truth()
                    .execute_query_plan(
                        runtime
                            .read_truth()
                            .plan_query_packet(&snapshot, explicit_packet)
                            .expect("scene frame explicit plan"),
                    )
                    .expect("scene frame explicit outcome");
                let explicit_micros = explicit_started_at.elapsed().as_micros();
                total_explicit_query_micros += explicit_micros;
                assert!(runtime.visibility_authority().release_snapshot(&snapshot));

                let affected_sources = (propagation.result.entities.len()
                    + explicit.result.entities.len())
                .min(bridge_runtime.source_versions.len())
                .max(4);
                let bridge_before = bridge_runtime.observe();
                let bridge_started_at = Instant::now();
                bridge_runtime.apply_changes(affected_sources);
                let bridge_micros = bridge_started_at.elapsed().as_micros();
                total_bridge_micros += bridge_micros;
                let bridge_after = bridge_runtime.observe();
                max_bridge_tasks_scheduled = max_bridge_tasks_scheduled.max(
                    bridge_after.planner.tasks_scheduled - bridge_before.planner.tasks_scheduled,
                );

                cycle_samples
                    .push(update_micros + propagation_micros + explicit_micros + bridge_micros);
                let counters = runtime.performance_access().counters();
                max_packets_per_iteration = max_packets_per_iteration
                    .max(counters.query_packet_count.saturating_sub(previous_packets));
                max_scope_units_per_iteration = max_scope_units_per_iteration.max(
                    counters
                        .query_scope_unit_count
                        .saturating_sub(previous_scope_units),
                );
                previous_packets = counters.query_packet_count;
                previous_scope_units = counters.query_scope_unit_count;
            }

            let first_window_average_cycle_micros =
                cycle_samples.iter().take(WINDOW).copied().sum::<u128>() / WINDOW as u128;
            let last_window_average_cycle_micros = cycle_samples
                .iter()
                .rev()
                .take(WINDOW)
                .copied()
                .sum::<u128>()
                / WINDOW as u128;
            let elapsed_micros = total_update_micros
                + total_propagation_micros
                + total_explicit_query_micros
                + total_bridge_micros;
            measurement_with_elapsed(elapsed_micros, || {
                json!({
                    "iterations": ITERATIONS,
                    "region_count": seeded.region_count,
                    "resident_entities": seeded.entities.len(),
                    "resident_relations": seeded.relation_count,
                    "average_update_micros": total_update_micros / ITERATIONS as u128,
                    "average_propagation_micros": total_propagation_micros / ITERATIONS as u128,
                    "average_explicit_query_micros": total_explicit_query_micros / ITERATIONS as u128,
                    "average_bridge_micros": total_bridge_micros / ITERATIONS as u128,
                    "first_window_average_cycle_micros": first_window_average_cycle_micros,
                    "last_window_average_cycle_micros": last_window_average_cycle_micros,
                    "max_packets_per_iteration": max_packets_per_iteration,
                    "max_scope_units_per_iteration": max_scope_units_per_iteration,
                    "max_bridge_tasks_scheduled": max_bridge_tasks_scheduled,
                    "counters": runtime.performance_access().counters(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "mixed_read_write_frame_churn_window",
        &mixed_frame_churn_samples,
        &[
            ("iterations", &["iterations"]),
            ("average_update_micros", &["average_update_micros"]),
            (
                "average_propagation_micros",
                &["average_propagation_micros"],
            ),
            (
                "average_explicit_query_micros",
                &["average_explicit_query_micros"],
            ),
            ("average_bridge_micros", &["average_bridge_micros"]),
            (
                "first_window_average_cycle_micros",
                &["first_window_average_cycle_micros"],
            ),
            (
                "last_window_average_cycle_micros",
                &["last_window_average_cycle_micros"],
            ),
            ("max_packets_per_iteration", &["max_packets_per_iteration"]),
            (
                "max_scope_units_per_iteration",
                &["max_scope_units_per_iteration"],
            ),
            (
                "max_bridge_tasks_scheduled",
                &["max_bridge_tasks_scheduled"],
            ),
        ],
    );
    assert_budget(
        &mixed_frame_churn_samples,
        "game-engine frame churn should keep repeated mixed read/write cycles local and stable across a bounded frame window",
        |metrics| {
            let first_window = metrics["first_window_average_cycle_micros"]
                .as_u64()
                .unwrap_or(0);
            let last_window = metrics["last_window_average_cycle_micros"]
                .as_u64()
                .unwrap_or(0);
            metrics["iterations"].as_u64() == Some(48)
                && metrics["region_count"].as_u64() == Some(8)
                && metrics["resident_entities"].as_u64() == Some(192)
                && metrics["max_packets_per_iteration"].as_u64().unwrap_or(0) <= 16
                && metrics["max_scope_units_per_iteration"].as_u64().unwrap_or(0) <= 16
                && metrics["max_bridge_tasks_scheduled"].as_u64().unwrap_or(0) <= 64
                && last_window <= first_window.saturating_mul(2).max(1)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "bulk_mutation_batch_count") == 48
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_recoverability_policy_matrix() {
    let suite = "recoverability_policy_matrix";

    let geometry_policy_samples =
        capture_perf_samples(suite, "geometry_hot_truth_vs_deferred_trace_policy", || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::GeometryKernel,
            );
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();

            let source = create_entity_outcome(&mut runtime, "policy-geometry-source");
            let middle = create_entity_outcome(&mut runtime, "policy-geometry-middle");
            let target = create_entity_outcome(&mut runtime, "policy-geometry-target");
            let source_entity = changed_entities(&source)[0];
            let middle_entity = changed_entities(&middle)[0];
            let target_entity = changed_entities(&target)[0];
            create_relation_outcome(&mut runtime, source_entity, middle_entity, "policy-link-a");
            create_relation_outcome(&mut runtime, middle_entity, target_entity, "policy-link-b");

            let hot_commit_started_at = Instant::now();
            let hot_commit = update_entity(&mut runtime, middle_entity, "policy-middle-updated");
            let hot_commit_micros = hot_commit_started_at.elapsed().as_micros();
            let hot_bundle = runtime
                .publication()
                .latest_bundle()
                .expect("policy geometry latest bundle")
                .clone();
            let hot_artifacts = runtime.publication().diagnostics_since(diagnostics_start);

            runtime
                .durability_authority()
                .checkpoint()
                .expect("policy geometry checkpoint");
            let plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::GeometryKernel,
            );
            recovered
                .durability_authority()
                .recover(plan)
                .expect("policy geometry recovery");
            let replay_started_at = Instant::now();
            let replay = recovered
                .replay_authority()
                .replay_commit(RelationalReplayRequest {
                    commit_id: hot_commit.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                    verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
                });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();
            let recovered_envelope = recovered
                .replay()
                .canonical_commit_envelope(hot_commit.commit.commit_id)
                .cloned()
                .expect("policy recovered geometry envelope");

            PerfMeasurement {
                elapsed_micros: hot_commit_micros + replay_commit_micros,
                metrics: json!({
                    "must_be_hot_changed_records": hot_commit.changed_records.len(),
                    "reconstructable_summary_entries": hot_bundle.diagnostics_summary.entries.len(),
                    "deferred_trace_entries": hot_artifacts
                        .iter()
                        .filter(|artifact| artifact.kind == DiagnosticsArtifactKind::DetailedTrace)
                        .map(|artifact| artifact.entries.len())
                        .sum::<usize>(),
                    "summary_reconstructed": certification_digest(&hot_bundle.diagnostics_summary)
                        == certification_digest(&recovered_envelope.diagnostics_summary),
                    "replay_mismatch_count": replay.mismatches.len(),
                    "replay_failure": replay.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "phase_timing": {
                        "hot_commit_micros": hot_commit_micros,
                        "replay_commit_micros": replay_commit_micros,
                    },
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "geometry_hot_truth_vs_deferred_trace_policy",
        &geometry_policy_samples,
        &[
            ("hot_commit_micros", &["phase_timing", "hot_commit_micros"]),
            (
                "replay_commit_micros",
                &["phase_timing", "replay_commit_micros"],
            ),
            (
                "must_be_hot_changed_records",
                &["must_be_hot_changed_records"],
            ),
            (
                "reconstructable_summary_entries",
                &["reconstructable_summary_entries"],
            ),
            ("deferred_trace_entries", &["deferred_trace_entries"]),
        ],
    );
    assert_budget(
        &geometry_policy_samples,
        "geometry policy budgets should keep truth updates hot, canonical summaries reconstructable, and detailed traces explicitly deferrable",
        |metrics| {
            metrics["must_be_hot_changed_records"].as_u64() == Some(1)
                && metrics["reconstructable_summary_entries"].as_u64().unwrap_or(0) >= 1
                && metrics["deferred_trace_entries"].as_u64() == Some(0)
                && metrics["summary_reconstructed"].as_bool() == Some(true)
                && metrics["replay_failure"].is_null()
                && metrics["replay_mismatch_count"].as_u64() == Some(0)
        },
    );

    let chip_policy_samples = capture_perf_samples(
        suite,
        "chip_compile_reconstructable_policy",
        || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            runtime.config.diagnostics.profile.detailed_traces_enabled = false;
            runtime.config.diagnostics.profile.max_entries_per_artifact = 0;

            let source =
                create_entity_in_partition(&mut runtime, "policy-chip-source", PartitionId(7));
            let sinks = (0..4)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("policy-chip-sink-{index}"),
                        PartitionId(11 + index as u32),
                    )
                })
                .collect::<Vec<_>>();
            for (index, sink) in sinks.iter().enumerate() {
                create_relation_in_partition(
                    &mut runtime,
                    source,
                    *sink,
                    &format!("policy-chip-link-{index}"),
                    PartitionId(19),
                );
            }

            let hot_commit_started_at = Instant::now();
            let hot_commit = update_entity(&mut runtime, source, "policy-chip-updated");
            let hot_commit_micros = hot_commit_started_at.elapsed().as_micros();
            let latest_commit = runtime
                .history()
                .latest_commit()
                .expect("policy chip latest commit")
                .clone();
            let hot_compile_started_at = Instant::now();
            let hot_artifact = runtime
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    latest_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(12),
                        PartitionId(13),
                        PartitionId(14),
                        PartitionId(19),
                    ],
                )
                .expect("policy hot chip compile");
            let hot_compile_micros = hot_compile_started_at.elapsed().as_micros();

            runtime
                .durability_authority()
                .checkpoint()
                .expect("policy chip checkpoint");
            let plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            recovered
                .durability_authority()
                .recover(plan)
                .expect("policy chip recovery");
            let replay_started_at = Instant::now();
            let replay = recovered
                .replay_authority()
                .replay_commit(RelationalReplayRequest {
                    commit_id: hot_commit.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                    verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();
            let recovered_commit = recovered
                .history()
                .latest_commit()
                .expect("policy recovered chip latest commit")
                .clone();
            let cold_compile_started_at = Instant::now();
            let cold_artifact = recovered
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    recovered_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(12),
                        PartitionId(13),
                        PartitionId(14),
                        PartitionId(19),
                    ],
                )
                .expect("policy cold chip compile");
            let cold_compile_micros = cold_compile_started_at.elapsed().as_micros();

            PerfMeasurement {
                elapsed_micros: hot_commit_micros
                    + hot_compile_micros
                    + replay_commit_micros
                    + cold_compile_micros,
                metrics: json!({
                    "must_be_hot_changed_records": hot_commit.changed_records.len(),
                    "reconstructable_compiled_record_count": cold_artifact.compiled_record_count,
                    "hot_compiled_record_count": hot_artifact.compiled_record_count,
                    "hot_compatibility": format!(
                        "{:?}",
                        runtime.compiled_artifacts().compiled_artifact_compatibility(hot_artifact.artifact_id)
                    ),
                    "cold_compatibility": format!(
                        "{:?}",
                        recovered.compiled_artifacts().compiled_artifact_compatibility(cold_artifact.artifact_id)
                    ),
                    "replay_mismatch_count": replay.mismatches.len(),
                    "replay_failure": replay.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "phase_timing": {
                        "hot_commit_micros": hot_commit_micros,
                        "hot_compile_micros": hot_compile_micros,
                        "replay_commit_micros": replay_commit_micros,
                        "cold_compile_micros": cold_compile_micros,
                    },
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "chip_compile_reconstructable_policy",
        &chip_policy_samples,
        &[
            ("hot_commit_micros", &["phase_timing", "hot_commit_micros"]),
            (
                "hot_compile_micros",
                &["phase_timing", "hot_compile_micros"],
            ),
            (
                "replay_commit_micros",
                &["phase_timing", "replay_commit_micros"],
            ),
            (
                "cold_compile_micros",
                &["phase_timing", "cold_compile_micros"],
            ),
            (
                "must_be_hot_changed_records",
                &["must_be_hot_changed_records"],
            ),
            (
                "reconstructable_compiled_record_count",
                &["reconstructable_compiled_record_count"],
            ),
        ],
    );
    assert_budget(
        &chip_policy_samples,
        "chip policy budgets should keep commit truth hot while treating compiled execution artifacts as reconstructable cold-path products",
        |metrics| {
            metrics["must_be_hot_changed_records"].as_u64() == Some(1)
                && metrics["hot_compiled_record_count"] == metrics["reconstructable_compiled_record_count"]
                && metrics["hot_compatibility"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactCompatibility::Compatible))
                && metrics["cold_compatibility"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactCompatibility::Compatible))
                && metrics["replay_failure"].is_null()
                && metrics["replay_mismatch_count"].as_u64() == Some(0)
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_merge_lineage_matrix() {
    let suite = "merge_lineage_matrix";

    let merge_planning_samples =
        capture_perf_samples(suite, "merge_planning_divergent_update", || {
            let mut runtime = persisted_runtime_with_test_schema();
            let shared = create_entity(&mut runtime, "shared");
            create_branch_from_main(&mut runtime, "feature");
            let _ = update_entity(&mut runtime, shared, "main-value");
            let _ = update_entity_on_branch(
                &mut runtime,
                shared,
                "feature-value",
                BranchId("feature".to_string()),
            );

            runtime.performance_access().reset_counters();
            let started_at = Instant::now();
            let artifact = runtime
                .merge()
                .inspect_planning_scope(crate::merge::data::MergePlanningRequest::new(
                    BranchId("main".to_string()),
                    BranchId("feature".to_string()),
                    MergeIntent::ReconcileIntoTarget,
                ))
                .expect("merge planning artifact");
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "candidate_count": artifact.identity_discovery.candidate_count,
                    "classified_records": artifact.conflict_classification.classified_record_count,
                    "resolved_records": artifact.policy_resolution.resolved_record_count,
                    "decision_count": artifact.decision_log.decisions.len(),
                    "counters": counters,
                }),
            }
        });
    assert!(merge_planning_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &merge_planning_samples,
        "merge planning should stay request-shaped and artifact-accounted",
        |metrics| {
            counter_u64(metrics, "merge_planning_requests") == 1
                && counter_u64(metrics, "merge_identity_candidates_discovered")
                    == metrics["candidate_count"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "merge_conflict_records_classified")
                    == metrics["classified_records"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "merge_decision_log_width")
                    == metrics["decision_count"].as_u64().unwrap_or(0)
        },
    );

    let merge_execution_samples = capture_perf_samples(
        suite,
        "merge_execution_feature_adoption",
        || {
            let mut runtime = persisted_runtime_with_test_schema();
            create_entity(&mut runtime, "main-anchor");
            create_branch_from_main(&mut runtime, "feature");
            let mut txn = runtime.begin_transaction(TransactionOptions {
                target_branch: Some(BranchId("feature".to_string())),
                ..TransactionOptions::default()
            });
            txn.push_batch(
                WorkerIntentBatch::new("create-feature-only").push(
                    MutationIntent::Create(CreateIntent::Entity(
                        crate::transactions::data::EntitySpec {
                            partition_id: PartitionId::main(),
                            kind_id: KindId(1),
                            client_key: crate::symbols::data::ClientKey::raw("feature-only"),
                            fields: crate::tests::support::single_string_aspect_field_patch(
                                "name",
                                "feature-only",
                            ),
                        },
                    ))
                    .into(),
                ),
            );
            let _feature_only = changed_entities(&txn.commit().expect("feature create"))[0];

            let prepared = runtime
                .prepare_merge_execution(MergeExecutionRequest {
                    target_branch: BranchId("main".to_string()),
                    source_branch: BranchId("feature".to_string()),
                    merge_intent: MergeIntent::ReconcileIntoTarget,
                })
                .expect("prepared merge");

            runtime.performance_access().reset_counters();
            let started_at = Instant::now();
            let outcome = runtime
                .execute_prepared_merge(prepared)
                .expect("execute merge");
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "executed_record_count": outcome.structural_summary.executed_record_count,
                    "emitted_mutation_intent_count": outcome.structural_summary.emitted_mutation_intent_count,
                    "adopted_source_record_count": outcome.structural_summary.adopted_source_record_count,
                    "changed_entities": changed_entities(&outcome.commit).len(),
                    "counters": counters,
                }),
            }
        },
    );
    assert!(merge_execution_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &merge_execution_samples,
        "merge execution should admit and emit exactly the scoped merge work",
        |metrics| {
            counter_u64(metrics, "merge_execution_attempts") == 1
                && counter_u64(metrics, "merge_execution_records_admitted")
                    == metrics["executed_record_count"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "merge_execution_mutation_intents_emitted")
                    == metrics["emitted_mutation_intent_count"]
                        .as_u64()
                        .unwrap_or(0)
                && metrics["adopted_source_record_count"].as_u64() == Some(1)
                && metrics["changed_entities"].as_u64() == Some(1)
        },
    );

    let merge_execution_zero_diag_samples = capture_perf_samples(
        suite,
        "merge_execution_feature_adoption_zero_diagnostics_budget",
        || {
            let mut runtime = persisted_runtime_with_test_schema();
            runtime.config.diagnostics.profile.max_entries_per_artifact = 0;
            create_entity(&mut runtime, "main-anchor");
            create_branch_from_main(&mut runtime, "feature");
            let mut txn = runtime.begin_transaction(TransactionOptions {
                target_branch: Some(BranchId("feature".to_string())),
                ..TransactionOptions::default()
            });
            txn.push_batch(
                WorkerIntentBatch::new("create-feature-only").push(
                    MutationIntent::Create(CreateIntent::Entity(
                        crate::transactions::data::EntitySpec {
                            partition_id: PartitionId::main(),
                            kind_id: KindId(1),
                            client_key: crate::symbols::data::ClientKey::raw("feature-only"),
                            fields: crate::tests::support::single_string_aspect_field_patch(
                                "name",
                                "feature-only",
                            ),
                        },
                    ))
                    .into(),
                ),
            );
            let _feature_only = changed_entities(&txn.commit().expect("feature create"))[0];

            let prepared = runtime
                .prepare_merge_execution(MergeExecutionRequest {
                    target_branch: BranchId("main".to_string()),
                    source_branch: BranchId("feature".to_string()),
                    merge_intent: MergeIntent::ReconcileIntoTarget,
                })
                .expect("prepared merge");

            runtime.performance_access().reset_counters();
            let started_at = Instant::now();
            let outcome = runtime
                .execute_prepared_merge(prepared)
                .expect("execute merge");
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "executed_record_count": outcome.structural_summary.executed_record_count,
                    "emitted_mutation_intent_count": outcome.structural_summary.emitted_mutation_intent_count,
                    "adopted_source_record_count": outcome.structural_summary.adopted_source_record_count,
                    "changed_entities": changed_entities(&outcome.commit).len(),
                    "diagnostic_artifact_entries": runtime
                        .publication()
                        .diagnostics()
                        .artifacts()
                        .iter()
                        .rev()
                        .find(|artifact| {
                            artifact.scope == crate::facade::diagnostics::DiagnosticsScope::History
                                && artifact.kind
                                    == crate::facade::diagnostics::DiagnosticsArtifactKind::DetailedTrace
                        })
                        .map(|artifact| artifact.entries.len())
                        .unwrap_or(0),
                    "counters": counters,
                }),
            }
        },
    );
    assert!(merge_execution_zero_diag_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &merge_execution_zero_diag_samples,
        "merge execution should preserve structural truth even when detailed diagnostics are budget-zero",
        |metrics| {
            counter_u64(metrics, "merge_execution_attempts") == 1
                && counter_u64(metrics, "merge_execution_records_admitted")
                    == metrics["executed_record_count"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "merge_execution_mutation_intents_emitted")
                    == metrics["emitted_mutation_intent_count"].as_u64().unwrap_or(0)
                && metrics["adopted_source_record_count"].as_u64() == Some(1)
                && metrics["changed_entities"].as_u64() == Some(1)
                && metrics["diagnostic_artifact_entries"].as_u64() == Some(0)
        },
    );

    let merge_prepare_execute_split_samples = capture_perf_samples(
        suite,
        "merge_prepare_vs_execute_feature_adoption",
        || {
            let mut runtime = persisted_runtime_with_test_schema();
            create_entity(&mut runtime, "main-anchor");
            create_branch_from_main(&mut runtime, "feature");
            let mut txn = runtime.begin_transaction(TransactionOptions {
                target_branch: Some(BranchId("feature".to_string())),
                ..TransactionOptions::default()
            });
            txn.push_batch(
                WorkerIntentBatch::new("create-feature-only").push(
                    MutationIntent::Create(CreateIntent::Entity(
                        crate::transactions::data::EntitySpec {
                            partition_id: PartitionId::main(),
                            kind_id: KindId(1),
                            client_key: crate::symbols::data::ClientKey::raw("feature-only"),
                            fields: crate::tests::support::single_string_aspect_field_patch(
                                "name",
                                "feature-only",
                            ),
                        },
                    ))
                    .into(),
                ),
            );
            let _feature_only = changed_entities(&txn.commit().expect("feature create"))[0];

            runtime.performance_access().reset_counters();
            let prepare_started_at = Instant::now();
            let prepared = runtime
                .prepare_merge_execution(MergeExecutionRequest {
                    target_branch: BranchId("main".to_string()),
                    source_branch: BranchId("feature".to_string()),
                    merge_intent: MergeIntent::ReconcileIntoTarget,
                })
                .expect("prepared merge");
            let prepare_elapsed_micros = prepare_started_at.elapsed().as_micros();

            runtime.performance_access().reset_counters();
            let execute_started_at = Instant::now();
            let outcome = runtime
                .execute_prepared_merge(prepared)
                .expect("execute merge");
            let execute_elapsed_micros = execute_started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros: prepare_elapsed_micros + execute_elapsed_micros,
                metrics: json!({
                    "prepare_elapsed_micros": prepare_elapsed_micros,
                    "execute_elapsed_micros": execute_elapsed_micros,
                    "executed_record_count": outcome.structural_summary.executed_record_count,
                    "emitted_mutation_intent_count": outcome.structural_summary.emitted_mutation_intent_count,
                    "changed_entities": changed_entities(&outcome.commit).len(),
                    "counters": counters,
                }),
            }
        },
    );
    assert!(merge_prepare_execute_split_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &merge_prepare_execute_split_samples,
        "merge prepare/execute split should preserve the same single-record structural truth",
        |metrics| {
            metrics["prepare_elapsed_micros"].as_u64().unwrap_or(0) > 0
                && metrics["execute_elapsed_micros"].as_u64().unwrap_or(0) > 0
                && counter_u64(metrics, "merge_execution_attempts") == 1
                && counter_u64(metrics, "merge_execution_records_admitted")
                    == metrics["executed_record_count"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "merge_execution_mutation_intents_emitted")
                    == metrics["emitted_mutation_intent_count"]
                        .as_u64()
                        .unwrap_or(0)
                && metrics["changed_entities"].as_u64() == Some(1)
        },
    );

    let merge_vs_commit_floor_samples = capture_perf_samples(
        suite,
        "merge_execution_vs_persisted_commit_floor",
        || {
            let mut merge_runtime = persisted_runtime_with_test_schema();
            create_entity(&mut merge_runtime, "main-anchor");
            create_branch_from_main(&mut merge_runtime, "feature");
            let mut txn = merge_runtime.begin_transaction(TransactionOptions {
                target_branch: Some(BranchId("feature".to_string())),
                ..TransactionOptions::default()
            });
            txn.push_batch(
                WorkerIntentBatch::new("create-feature-only").push(
                    MutationIntent::Create(CreateIntent::Entity(
                        crate::transactions::data::EntitySpec {
                            partition_id: PartitionId::main(),
                            kind_id: KindId(1),
                            client_key: crate::symbols::data::ClientKey::raw("feature-only"),
                            fields: crate::tests::support::single_string_aspect_field_patch(
                                "name",
                                "feature-only",
                            ),
                        },
                    ))
                    .into(),
                ),
            );
            let _feature_only = changed_entities(&txn.commit().expect("feature create"))[0];

            let prepared = merge_runtime
                .prepare_merge_execution(MergeExecutionRequest {
                    target_branch: BranchId("main".to_string()),
                    source_branch: BranchId("feature".to_string()),
                    merge_intent: MergeIntent::ReconcileIntoTarget,
                })
                .expect("prepared merge");

            merge_runtime.performance_access().reset_counters();
            let merge_started_at = Instant::now();
            let merge_outcome = merge_runtime
                .execute_prepared_merge(prepared)
                .expect("execute merge");
            let merge_elapsed_micros = merge_started_at.elapsed().as_micros();
            let merge_counters = merge_runtime.performance_access().counters();

            let mut control_runtime = persisted_runtime_with_test_schema();
            control_runtime.performance_access().reset_counters();
            let control_started_at = Instant::now();
            let control_outcome = {
                let mut txn = control_runtime.begin_transaction(TransactionOptions::default());
                txn.push_batch(batch_create("control-single"));
                txn.commit().expect("control persisted single create")
            };
            let control_elapsed_micros = control_started_at.elapsed().as_micros();
            let control_counters = control_runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros: merge_elapsed_micros,
                metrics: json!({
                    "merge_elapsed_micros": merge_elapsed_micros,
                    "control_commit_elapsed_micros": control_elapsed_micros,
                    "merge_over_control_delta_micros": merge_elapsed_micros as i128 - control_elapsed_micros as i128,
                    "merge_control_ratio": merge_elapsed_micros as f64 / control_elapsed_micros.max(1) as f64,
                    "executed_record_count": merge_outcome.structural_summary.executed_record_count,
                    "emitted_mutation_intent_count": merge_outcome.structural_summary.emitted_mutation_intent_count,
                    "merge_changed_entities": changed_entities(&merge_outcome.commit).len(),
                    "control_changed_records": control_outcome.changed_records.len(),
                    "merge_counters": merge_counters,
                    "control_counters": control_counters,
                }),
            }
        },
    );
    assert!(merge_vs_commit_floor_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &merge_vs_commit_floor_samples,
        "merge execute vs persisted commit floor should preserve single-record structural truth on both paths",
        |metrics| {
            metrics["merge_elapsed_micros"].as_u64().unwrap_or(0) > 0
                && metrics["control_commit_elapsed_micros"].as_u64().unwrap_or(0) > 0
                && metrics["merge_changed_entities"].as_u64() == Some(1)
                && metrics["control_changed_records"].as_u64() == Some(1)
                && metrics["merge_counters"]["merge_execution_attempts"].as_u64() == Some(1)
                && metrics["merge_counters"]["merge_execution_records_admitted"].as_u64()
                    == metrics["executed_record_count"].as_u64()
                && metrics["merge_counters"]["merge_execution_mutation_intents_emitted"].as_u64()
                    == metrics["emitted_mutation_intent_count"].as_u64()
                && metrics["control_counters"]["full_state_clones"].as_u64() == Some(0)
                && metrics["control_counters"]["snapshot_pin_full_rebuilds"].as_u64() == Some(0)
                && metrics["control_counters"]["partitions_touched_by_commit"].as_u64() == Some(1)
        },
    );

    let merge_verify_execute_split_samples = capture_perf_samples(
        suite,
        "merge_verify_vs_execute_feature_adoption",
        || {
            let mut runtime = persisted_runtime_with_test_schema();
            create_entity(&mut runtime, "main-anchor");
            create_branch_from_main(&mut runtime, "feature");
            let mut txn = runtime.begin_transaction(TransactionOptions {
                target_branch: Some(BranchId("feature".to_string())),
                ..TransactionOptions::default()
            });
            txn.push_batch(
                WorkerIntentBatch::new("create-feature-only").push(
                    MutationIntent::Create(CreateIntent::Entity(
                        crate::transactions::data::EntitySpec {
                            partition_id: PartitionId::main(),
                            kind_id: KindId(1),
                            client_key: crate::symbols::data::ClientKey::raw("feature-only"),
                            fields: crate::tests::support::single_string_aspect_field_patch(
                                "name",
                                "feature-only",
                            ),
                        },
                    ))
                    .into(),
                ),
            );
            let _feature_only = changed_entities(&txn.commit().expect("feature create"))[0];

            let prepared = runtime
                .prepare_merge_execution(MergeExecutionRequest {
                    target_branch: BranchId("main".to_string()),
                    source_branch: BranchId("feature".to_string()),
                    merge_intent: MergeIntent::ReconcileIntoTarget,
                })
                .expect("prepared merge");

            runtime.performance_access().reset_counters();
            let verify_started_at = Instant::now();
            runtime
                .merge()
                .verify_prepared_merge_execution(&prepared)
                .expect("verify prepared merge");
            let verify_elapsed_micros = verify_started_at.elapsed().as_micros();
            let verify_counters = runtime.performance_access().counters();

            runtime.performance_access().reset_counters();
            let execute_started_at = Instant::now();
            let outcome = runtime
                .execute_prepared_merge(prepared)
                .expect("execute merge");
            let execute_elapsed_micros = execute_started_at.elapsed().as_micros();
            let execute_counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros: verify_elapsed_micros + execute_elapsed_micros,
                metrics: json!({
                    "verify_elapsed_micros": verify_elapsed_micros,
                    "execute_elapsed_micros": execute_elapsed_micros,
                    "executed_record_count": outcome.structural_summary.executed_record_count,
                    "emitted_mutation_intent_count": outcome.structural_summary.emitted_mutation_intent_count,
                    "changed_entities": changed_entities(&outcome.commit).len(),
                    "verify_counters": verify_counters,
                    "execute_counters": execute_counters,
                }),
            }
        },
    );
    assert!(merge_verify_execute_split_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &merge_verify_execute_split_samples,
        "merge verify/execute split should show certified verification and preserve single-record execute truth",
        |metrics| {
            metrics["verify_elapsed_micros"].as_u64().unwrap_or(0) > 0
                && metrics["execute_elapsed_micros"].as_u64().unwrap_or(0) > 0
                && metrics["changed_entities"].as_u64() == Some(1)
                && metrics["verify_counters"]["merge_execution_verification_requests"].as_u64()
                    == Some(1)
                && metrics["verify_counters"]["merge_execution_branch_head_checks"].as_u64()
                    == Some(2)
                && metrics["verify_counters"]["merge_execution_merge_base_checks"].as_u64()
                    == Some(1)
                && metrics["verify_counters"]["merge_execution_compiled_plan_digest_checks"]
                    .as_u64()
                    == Some(1)
                && metrics["execute_counters"]["merge_execution_attempts"].as_u64() == Some(1)
                && metrics["execute_counters"]["merge_execution_records_admitted"].as_u64()
                    == metrics["executed_record_count"].as_u64()
                && metrics["execute_counters"]["merge_execution_mutation_intents_emitted"].as_u64()
                    == metrics["emitted_mutation_intent_count"].as_u64()
        },
    );

    let merge_phase_timing_samples = capture_perf_samples(
        suite,
        "merge_execute_phase_timing_feature_adoption",
        || {
            let mut runtime = persisted_runtime_with_test_schema();
            create_entity(&mut runtime, "main-anchor");
            create_branch_from_main(&mut runtime, "feature");
            let mut txn = runtime.begin_transaction(TransactionOptions {
                target_branch: Some(BranchId("feature".to_string())),
                ..TransactionOptions::default()
            });
            txn.push_batch(
                WorkerIntentBatch::new("create-feature-only").push(
                    MutationIntent::Create(CreateIntent::Entity(
                        crate::transactions::data::EntitySpec {
                            partition_id: PartitionId::main(),
                            kind_id: KindId(1),
                            client_key: crate::symbols::data::ClientKey::raw("feature-only"),
                            fields: crate::tests::support::single_string_aspect_field_patch(
                                "name",
                                "feature-only",
                            ),
                        },
                    ))
                    .into(),
                ),
            );
            let _feature_only = changed_entities(&txn.commit().expect("feature create"))[0];

            let prepared = runtime
                .prepare_merge_execution(MergeExecutionRequest {
                    target_branch: BranchId("main".to_string()),
                    source_branch: BranchId("feature".to_string()),
                    merge_intent: MergeIntent::ReconcileIntoTarget,
                })
                .expect("prepared merge");

            runtime.performance_access().reset_counters();
            let started_at = Instant::now();
            let outcome = runtime
                .execute_prepared_merge(prepared)
                .expect("execute merge");
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();
            let phase_timing = outcome.commit.execution.phase_timing.clone();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "executed_record_count": outcome.structural_summary.executed_record_count,
                    "changed_entities": changed_entities(&outcome.commit).len(),
                    "phase_timing": {
                        "working_state_preparation_micros": phase_timing.working_state_preparation_micros,
                        "invariant_pre_check_micros": phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": phase_timing.invariant_post_check_micros,
                        "artifact_assembly_micros": phase_timing.artifact_assembly_micros,
                        "durable_append_micros": phase_timing.durable_append_micros,
                        "publication_micros": phase_timing.publication_micros
                    },
                    "counters": counters,
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "merge_execute_phase_timing_feature_adoption",
        &merge_phase_timing_samples,
        &[
            (
                "working_state_preparation_micros",
                &["phase_timing", "working_state_preparation_micros"],
            ),
            (
                "authoritative_mutation_micros",
                &["phase_timing", "authoritative_mutation_micros"],
            ),
            (
                "history_resolution_micros",
                &["phase_timing", "history_resolution_micros"],
            ),
            (
                "artifact_assembly_micros",
                &["phase_timing", "artifact_assembly_micros"],
            ),
            (
                "durable_append_micros",
                &["phase_timing", "durable_append_micros"],
            ),
            (
                "publication_micros",
                &["phase_timing", "publication_micros"],
            ),
        ],
    );
    assert!(merge_phase_timing_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &merge_phase_timing_samples,
        "merge execute phase timing should preserve single-record truth and expose nonzero tail-phase timings",
        |metrics| {
            metrics["changed_entities"].as_u64() == Some(1)
                && metrics["executed_record_count"].as_u64() == Some(1)
                && metrics["phase_timing"]["authoritative_mutation_micros"]
                    .as_u64()
                    .unwrap_or(0)
                    > 0
                && metrics["phase_timing"]["artifact_assembly_micros"]
                    .as_u64()
                    .unwrap_or(0)
                    > 0
                && metrics["phase_timing"]["durable_append_micros"]
                    .as_u64()
                    .unwrap_or(0)
                    > 0
                && metrics["phase_timing"]["publication_micros"]
                    .as_u64()
                    .unwrap_or(0)
                    > 0
                && metrics["counters"]["merge_execution_attempts"].as_u64() == Some(1)
        },
    );

    let lineage_divergence_samples =
        capture_perf_samples(suite, "lineage_branch_divergence_breadth", || {
            let mut runtime = runtime_with_test_schema();
            let created = create_entity_outcome(&mut runtime, "main");
            let start_lineage = runtime
                .lineage_access()
                .for_record(changed_entities(&created)[0])
                .expect("start lineage")
                .lineage_id;
            create_branch_from_main(&mut runtime, "feature");
            let _ = create_entity_outcome_on_branch(
                &mut runtime,
                "feature",
                BranchId("feature".to_string()),
            );

            runtime.performance_access().reset_counters();
            let started_at = Instant::now();
            let divergence =
                runtime
                    .lineage_access()
                    .divergence_between_branches(LineageDivergenceRequest {
                        left_branch: BranchId("main".to_string()),
                        right_branch: BranchId("feature".to_string()),
                        traversal_basis: LineageDivergenceTraversalBasis::FullBranchGraphComparison,
                    });
            let resolution =
                runtime
                    .lineage_access()
                    .resolve_historical_lineage(HistoricalResolutionRequest {
                        branch_id: BranchId("main".to_string()),
                        lineage_id: start_lineage,
                        boundedness_basis:
                            HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed,
                    });
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: json!({
                    "left_event_count": divergence.metrics.left_event_count,
                    "right_event_count": divergence.metrics.right_event_count,
                    "left_node_count": divergence.metrics.left_node_count,
                    "right_node_count": divergence.metrics.right_node_count,
                    "resolution_event_scans": resolution.metrics.branch_event_scan_count,
                    "resolution_traversed_events": resolution.metrics.traversed_event_count,
                    "counters": counters,
                }),
            }
        });
    assert!(lineage_divergence_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &lineage_divergence_samples,
        "lineage divergence and branch-scoped resolution should report their true breadths",
        |metrics| {
            counter_u64(metrics, "lineage_branch_divergence_requests") == 1
                && counter_u64(metrics, "lineage_historical_resolution_requests") == 1
                && counter_u64(metrics, "lineage_branch_divergence_event_scans")
                    == metrics["left_event_count"].as_u64().unwrap_or(0)
                        + metrics["right_event_count"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "lineage_branch_divergence_node_scans")
                    == metrics["left_node_count"].as_u64().unwrap_or(0)
                        + metrics["right_node_count"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "lineage_historical_resolution_branch_event_scans")
                    == metrics["resolution_event_scans"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "lineage_historical_resolution_traversed_events")
                    == metrics["resolution_traversed_events"].as_u64().unwrap_or(0)
        },
    );
}
