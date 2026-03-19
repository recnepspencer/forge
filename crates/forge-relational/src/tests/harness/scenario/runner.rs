use super::operation::ScenarioOperation;
use super::profiles::CertificationPressureProfile;
use super::seed::DeterministicGenerator;
use crate::facade::config::RelationalRuntimeProfile;
use crate::facade::config::{
    MvccConfig, RetentionBackend, SnapshotReleasePolicy, VisibilityCachePolicy,
};
use crate::facade::durability::{DurabilityMode, DurableStoreLayout};
use crate::facade::history::BranchId;
use crate::facade::identity::{EntityId, PartitionId, RelationId};
use crate::facade::publication::SubscriberCheckpoint;
use crate::facade::runtime::RelationalRuntime;
use crate::facade::snapshots::SnapshotHandle;
use crate::facade::transactions::{
    DeleteEntityIntent, DeleteRelationIntent, EntityMutationIntent, MutationIntent,
    RelationMutationIntent, ReplaceEntityIntent, TransactionOptions, WorkerIntentBatch,
};
use crate::tests::harness::fixtures::runtime::{build_runtime, RuntimeHarnessMode};
use crate::tests::support::{
    checkpoint_for_schema_version, create_entity_in_partition, create_relation_in_partition,
    test_schema_registry, update_entity_on_branch, InternedString, KindId, RecordPayload,
    SchemaVersionId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ActiveRelation {
    relation_id: RelationId,
    source: EntityId,
    target: EntityId,
    partition: PartitionId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SeededScenarioConfig {
    pub(crate) seed: u64,
    pub(crate) steps: usize,
    pub(crate) checkpoint_stride: usize,
    pub(crate) runtime_mode: RuntimeHarnessMode,
    pub(crate) relation_pressure: bool,
    pub(crate) durable_checkpoint_every: Option<usize>,
    pub(crate) durable_compact_every: Option<usize>,
    pub(crate) retention_pass_every: Option<usize>,
    pub(crate) branch_pressure: bool,
    pub(crate) replacement_pressure: bool,
}

impl SeededScenarioConfig {
    pub(crate) fn geometry_kernel(seed: u64, profile: CertificationPressureProfile) -> Self {
        Self {
            seed,
            steps: profile.steps(),
            checkpoint_stride: 16,
            runtime_mode: RuntimeHarnessMode::InMemory(RelationalRuntimeProfile::GeometryKernel),
            relation_pressure: true,
            durable_checkpoint_every: None,
            durable_compact_every: None,
            retention_pass_every: None,
            branch_pressure: false,
            replacement_pressure: false,
        }
    }

    pub(crate) fn persisted_geometry(seed: u64, profile: CertificationPressureProfile) -> Self {
        Self {
            seed,
            steps: profile.steps(),
            checkpoint_stride: 16,
            runtime_mode: RuntimeHarnessMode::Persisted,
            relation_pressure: true,
            durable_checkpoint_every: Some(32),
            durable_compact_every: Some(64),
            retention_pass_every: Some(8),
            branch_pressure: false,
            replacement_pressure: true,
        }
    }

    pub(crate) fn hostile_geometry(seed: u64, profile: CertificationPressureProfile) -> Self {
        Self {
            seed,
            steps: profile.steps(),
            checkpoint_stride: 8,
            runtime_mode: RuntimeHarnessMode::InMemory(RelationalRuntimeProfile::GeometryKernel),
            relation_pressure: true,
            durable_checkpoint_every: None,
            durable_compact_every: None,
            retention_pass_every: Some(4),
            branch_pressure: true,
            replacement_pressure: true,
        }
    }
}

#[derive(Debug)]
pub(crate) struct ScenarioTrace {
    pub(crate) seed: u64,
    pub(crate) operations: Vec<ScenarioOperation>,
}

#[derive(Debug)]
pub(crate) struct SeededScenarioWorld {
    pub(crate) runtime: RelationalRuntime,
    pub(crate) baseline_checkpoint: SubscriberCheckpoint,
    pub(crate) checkpoints: Vec<SubscriberCheckpoint>,
    pub(crate) trace: ScenarioTrace,
}

pub(crate) fn run_seeded_scenario(config: SeededScenarioConfig) -> SeededScenarioWorld {
    let mut runtime = build_runtime(config.runtime_mode);
    let mut entities = vec![
        create_entity_in_partition(&mut runtime, "seed-left", PartitionId(7)),
        create_entity_in_partition(&mut runtime, "seed-right", PartitionId(11)),
        create_entity_in_partition(&mut runtime, "seed-center", PartitionId(29)),
    ];
    let baseline_checkpoint = checkpoint_for_schema_version(
        runtime
            .publication_access()
            .latest_patch()
            .unwrap()
            .position,
        SchemaVersionId(1),
    );
    let mut checkpoints = vec![baseline_checkpoint.clone()];
    let mut snapshots: Vec<SnapshotHandle> = Vec::new();
    let mut relations: Vec<ActiveRelation> = Vec::new();
    let mut branches = vec![BranchId("main".to_string())];
    let mut generator = DeterministicGenerator::new(config.seed);
    let mut operations = Vec::with_capacity(config.steps);
    let mut name_counter = 0_u64;

    for step in 0..config.steps {
        let operation = choose_operation(
            &mut generator,
            &entities,
            &snapshots,
            &relations,
            &branches,
            &config,
        );
        apply_operation(
            &mut runtime,
            &mut entities,
            &mut snapshots,
            &mut relations,
            &mut branches,
            &mut generator,
            &mut name_counter,
            config.seed,
            step,
            &operation,
        );
        operations.push(operation);

        if config
            .durable_checkpoint_every
            .map(|interval| (step + 1) % interval == 0)
            .unwrap_or(false)
        {
            runtime.durability_authority().checkpoint().unwrap();
            operations.push(ScenarioOperation::DurableCheckpoint);
        }

        if config
            .durable_compact_every
            .map(|interval| (step + 1) % interval == 0)
            .unwrap_or(false)
        {
            let _ = runtime.durability_authority().compact_store().unwrap();
            operations.push(ScenarioOperation::CompactDurableStore);
        }

        if config
            .retention_pass_every
            .map(|interval| (step + 1) % interval == 0)
            .unwrap_or(false)
        {
            let _ = runtime.retention_access().run_pass();
            operations.push(ScenarioOperation::RunRetentionPass);
        }

        let latest_position = runtime
            .publication_access()
            .latest_patch()
            .unwrap()
            .position;
        if latest_position.0 > checkpoints.last().unwrap().position().0
            && latest_position.0 as usize % config.checkpoint_stride == 0
        {
            checkpoints.push(checkpoint_for_schema_version(
                latest_position,
                SchemaVersionId(1),
            ));
        }
    }

    let latest_position = runtime
        .publication_access()
        .latest_patch()
        .unwrap()
        .position;
    if checkpoints
        .last()
        .map(|checkpoint| checkpoint.position() != latest_position)
        .unwrap_or(true)
    {
        checkpoints.push(checkpoint_for_schema_version(
            latest_position,
            SchemaVersionId(1),
        ));
    }

    SeededScenarioWorld {
        runtime,
        baseline_checkpoint,
        checkpoints,
        trace: ScenarioTrace {
            seed: config.seed,
            operations,
        },
    }
}

pub(crate) fn run_property_scenario(
    operations: Vec<ScenarioOperation>,
    runtime_mode: RuntimeHarnessMode,
) -> SeededScenarioWorld {
    let mut runtime = build_property_runtime(runtime_mode);
    let mut entities = vec![
        create_entity_in_partition(&mut runtime, "property-left", PartitionId(7)),
        create_entity_in_partition(&mut runtime, "property-right", PartitionId(11)),
        create_entity_in_partition(&mut runtime, "property-center", PartitionId(29)),
    ];
    let baseline_checkpoint = checkpoint_for_schema_version(
        runtime
            .publication_access()
            .latest_patch()
            .unwrap()
            .position,
        SchemaVersionId(1),
    );
    let mut checkpoints = vec![baseline_checkpoint.clone()];
    let mut snapshots: Vec<SnapshotHandle> = Vec::new();
    let mut relations: Vec<ActiveRelation> = Vec::new();
    let mut branches = vec![BranchId("main".to_string())];
    let mut generator = DeterministicGenerator::new(0xC0FFEE);
    let mut name_counter = 0_u64;

    for (step, operation) in operations.iter().enumerate() {
        apply_operation(
            &mut runtime,
            &mut entities,
            &mut snapshots,
            &mut relations,
            &mut branches,
            &mut generator,
            &mut name_counter,
            0xC0FFEE,
            step,
            operation,
        );
        let latest_position = runtime
            .publication_access()
            .latest_patch()
            .unwrap()
            .position;
        if latest_position.0 > checkpoints.last().unwrap().position().0 {
            checkpoints.push(checkpoint_for_schema_version(
                latest_position,
                SchemaVersionId(1),
            ));
        }
    }

    SeededScenarioWorld {
        runtime,
        baseline_checkpoint,
        checkpoints,
        trace: ScenarioTrace {
            seed: 0xC0FFEE,
            operations,
        },
    }
}

fn choose_operation(
    generator: &mut DeterministicGenerator,
    entities: &[EntityId],
    snapshots: &[SnapshotHandle],
    relations: &[ActiveRelation],
    branches: &[BranchId],
    config: &SeededScenarioConfig,
) -> ScenarioOperation {
    match generator.next_u64() % 15 {
        0 => ScenarioOperation::CreateEntity {
            partition: scenario_partition(generator.next_u64()),
            name: String::new(),
        },
        1 | 2 => ScenarioOperation::UpdateEntity {
            entity_slot: random_slot(generator, entities.len()),
            name: String::new(),
            branch_slot: random_slot(generator, branches.len()),
        },
        3 if config.replacement_pressure && !entities.is_empty() => {
            ScenarioOperation::ReplaceEntity {
                entity_slot: random_slot(generator, entities.len()),
                name: String::new(),
                branch_slot: 0,
                partition: scenario_partition(generator.next_u64()),
            }
        }
        4 if !entities.is_empty() => ScenarioOperation::DeleteEntity {
            entity_slot: random_slot(generator, entities.len()),
            branch_slot: 0,
        },
        5 if config.relation_pressure && entities.len() >= 2 => ScenarioOperation::CreateRelation {
            source_slot: random_slot(generator, entities.len()),
            target_slot: random_slot(generator, entities.len()),
            client_key: String::new(),
            partition: scenario_partition(generator.next_u64()),
        },
        6 if config.relation_pressure && !relations.is_empty() => {
            ScenarioOperation::DeleteRelation {
                relation_slot: random_slot(generator, relations.len()),
            }
        }
        7 if config.branch_pressure => ScenarioOperation::CreateBranch {
            branch_name: String::new(),
            from_branch_slot: random_slot(generator, branches.len()),
        },
        8 if config.branch_pressure && branches.len() > 1 => {
            ScenarioOperation::MergeBranchIntoMain {
                branch_slot: random_slot(generator, branches.len() - 1) + 1,
            }
        }
        9 => ScenarioOperation::CaptureSnapshot,
        10 if !snapshots.is_empty() => ScenarioOperation::ReleaseSnapshot {
            snapshot_slot: random_slot(generator, snapshots.len()),
        },
        _ => ScenarioOperation::UpdateEntity {
            entity_slot: random_slot(generator, entities.len()),
            name: String::new(),
            branch_slot: random_slot(generator, branches.len()),
        },
    }
}

fn apply_operation(
    runtime: &mut RelationalRuntime,
    entities: &mut Vec<EntityId>,
    snapshots: &mut Vec<SnapshotHandle>,
    relations: &mut Vec<ActiveRelation>,
    branches: &mut Vec<BranchId>,
    generator: &mut DeterministicGenerator,
    name_counter: &mut u64,
    seed: u64,
    step: usize,
    operation: &ScenarioOperation,
) {
    match operation {
        ScenarioOperation::CreateEntity { partition, .. } => {
            let name = format!("seed-{seed}-create-{step}-{name_counter}");
            *name_counter += 1;
            let entity = create_entity_in_partition(runtime, &name, *partition);
            entities.push(entity);
        }
        ScenarioOperation::UpdateEntity {
            entity_slot,
            branch_slot,
            ..
        } => {
            if entities.is_empty() {
                return;
            }
            let index = (*entity_slot).min(entities.len() - 1);
            let branch = branches[(*branch_slot).min(branches.len() - 1)].clone();
            let name = format!("seed-{seed}-update-{step}-{name_counter}");
            *name_counter += 1;
            let _ = update_entity_on_branch(runtime, entities[index], &name, branch);
        }
        ScenarioOperation::ReplaceEntity {
            entity_slot,
            branch_slot,
            partition,
            ..
        } => {
            if entities.is_empty() {
                return;
            }
            let index = (*entity_slot).min(entities.len() - 1);
            let branch = branches[(*branch_slot).min(branches.len() - 1)].clone();
            let name = format!("seed-{seed}-replace-{step}-{name_counter}");
            *name_counter += 1;
            let mut txn = runtime.begin_transaction(TransactionOptions {
                target_branch: Some(branch),
                ..TransactionOptions::default()
            });
            txn.push_batch(
                WorkerIntentBatch::new("replace").push(MutationIntent::Entity(
                    EntityMutationIntent::Replace(ReplaceEntityIntent {
                        entity_id: entities[index],
                        replacement: crate::transactions::data::EntitySpec {
                            partition_id: *partition,
                            kind_id: KindId(1),
                            client_key: InternedString::Raw(format!(
                                "replace-{seed}-{step}-{name_counter}"
                            )),
                            payload: RecordPayload::StructuredJson(
                                serde_json::json!({ "name": name }),
                            ),
                        },
                    }),
                )),
            );
            if let Ok(outcome) = txn.commit() {
                if let Some(replacement) = crate::tests::support::changed_entities(&outcome).last()
                {
                    entities[index] = *replacement;
                }
                refresh_live_world(runtime, entities, relations);
            }
        }
        ScenarioOperation::CreateRelation {
            source_slot,
            target_slot,
            partition,
            ..
        } => {
            if entities.len() < 2 {
                return;
            }
            let source = entities[(*source_slot).min(entities.len() - 1)];
            let mut target = entities[(*target_slot).min(entities.len() - 1)];
            if target == source {
                target = entities[(generator.next_u64() as usize) % entities.len()];
            }
            if source == target {
                return;
            }
            if relations.iter().any(|relation| {
                relation.source == source
                    && relation.target == target
                    && relation.partition == *partition
            }) {
                return;
            }
            let client_key = format!("seed-{seed}-rel-{step}-{name_counter}");
            *name_counter += 1;
            let relation =
                create_relation_in_partition(runtime, source, target, &client_key, *partition);
            relations.push(ActiveRelation {
                relation_id: relation,
                source,
                target,
                partition: *partition,
            });
        }
        ScenarioOperation::CaptureSnapshot => {
            snapshots.push(runtime.visibility_authority().snapshot())
        }
        ScenarioOperation::ReleaseSnapshot { snapshot_slot } => {
            if snapshots.is_empty() {
                return;
            }
            let index = (*snapshot_slot).min(snapshots.len() - 1);
            let snapshot = snapshots.swap_remove(index);
            assert!(runtime.visibility_authority().release_snapshot(&snapshot));
        }
        ScenarioOperation::DeleteRelation { relation_slot } => {
            if relations.is_empty() {
                return;
            }
            let index = (*relation_slot).min(relations.len() - 1);
            let relation = relations.swap_remove(index);
            let mut txn = runtime.begin_transaction(TransactionOptions::default());
            txn.push_batch(WorkerIntentBatch::new("delete-relation").push(
                MutationIntent::Relation(RelationMutationIntent::Delete(DeleteRelationIntent {
                    relation_id: relation.relation_id,
                })),
            ));
            txn.commit().unwrap();
            refresh_live_world(runtime, entities, relations);
        }
        ScenarioOperation::DeleteEntity {
            entity_slot,
            branch_slot,
        } => {
            if entities.is_empty() {
                return;
            }
            let index = (*entity_slot).min(entities.len() - 1);
            let deleted = entities[index];
            let branch = branches[(*branch_slot).min(branches.len() - 1)].clone();
            let mut txn = runtime.begin_transaction(TransactionOptions {
                target_branch: Some(branch),
                ..TransactionOptions::default()
            });
            txn.push_batch(
                WorkerIntentBatch::new("delete-entity").push(MutationIntent::Entity(
                    EntityMutationIntent::Delete(DeleteEntityIntent { entity_id: deleted }),
                )),
            );
            if txn.commit().is_ok() {
                entities.swap_remove(index);
                relations
                    .retain(|relation| relation.source != deleted && relation.target != deleted);
                refresh_live_world(runtime, entities, relations);
            }
        }
        ScenarioOperation::CreateBranch {
            branch_name,
            from_branch_slot,
        } => {
            let from_branch = branches[(*from_branch_slot).min(branches.len() - 1)].clone();
            let branch_name = if branch_name.is_empty() {
                format!("branch-{seed}-{step}-{name_counter}")
            } else {
                branch_name.clone()
            };
            *name_counter += 1;
            let branch = BranchId(branch_name);
            if runtime
                .history_authority()
                .create_branch(branch.clone(), &from_branch)
                .is_ok()
            {
                branches.push(branch);
            }
        }
        ScenarioOperation::MergeBranchIntoMain { branch_slot } => {
            if branches.len() <= 1 {
                return;
            }
            let branch = branches[(*branch_slot).min(branches.len() - 1)].clone();
            if branch.0 != "main" {
                let txn = runtime.begin_transaction(TransactionOptions {
                    target_branch: Some(BranchId("main".to_string())),
                    merge_parent_branches: vec![branch],
                    ..TransactionOptions::default()
                });
                if txn.commit().is_ok() {
                    refresh_live_world(runtime, entities, relations);
                }
            }
        }
        ScenarioOperation::RunRetentionPass => {
            let _ = runtime.retention_access().run_pass();
            refresh_live_world(runtime, entities, relations);
        }
        ScenarioOperation::DurableCheckpoint | ScenarioOperation::CompactDurableStore => {}
    }
}

fn refresh_live_world(
    runtime: &mut RelationalRuntime,
    entities: &mut Vec<EntityId>,
    relations: &mut Vec<ActiveRelation>,
) {
    let snapshot = runtime.visibility_authority().snapshot();
    let read = runtime.visibility_reads().read_snapshot(&snapshot).unwrap();
    *entities = read
        .entities()
        .iter()
        .map(|record| record.entity_id)
        .collect();
    *relations = read
        .relations()
        .iter()
        .map(|record| ActiveRelation {
            relation_id: record.relation_id,
            source: record.source,
            target: record.target,
            partition: record.relation_id.partition_id,
        })
        .collect();
    drop(read);
    assert!(runtime.visibility_authority().release_snapshot(&snapshot));
}

fn random_slot(generator: &mut DeterministicGenerator, len: usize) -> usize {
    if len == 0 {
        0
    } else {
        (generator.next_u64() as usize) % len
    }
}

fn scenario_partition(value: u64) -> PartitionId {
    match value % 4 {
        0 => PartitionId(7),
        1 => PartitionId(11),
        2 => PartitionId(29),
        _ => PartitionId(31),
    }
}

pub(crate) fn build_property_runtime(mode: RuntimeHarnessMode) -> RelationalRuntime {
    match mode {
        RuntimeHarnessMode::InMemory(profile) => {
            crate::facade::runtime::RelationalRuntimeApi::builder()
                .profile(profile)
                .schema_registry(test_schema_registry())
                .mvcc(MvccConfig {
                    track_visibility_metadata: true,
                    snapshot_release_policy: SnapshotReleasePolicy::ExplicitRelease,
                    auto_reclaim_deleted_records: true,
                    reclaim_batch_size: 8,
                    retention_backend: RetentionBackend::EpochChunkRetention,
                })
                .visibility_cache_policy(VisibilityCachePolicy {
                    enabled: true,
                    protect_branch_heads: false,
                    protect_replay_retained: false,
                    protect_active_snapshots: true,
                    recent_version_window: 2,
                })
                .build()
        }
        RuntimeHarnessMode::Persisted => crate::facade::runtime::RelationalRuntimeApi::builder()
            .profile(RelationalRuntimeProfile::CertificationCore)
            .schema_registry(test_schema_registry())
            .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
            .durable_store_layout(DurableStoreLayout {
                root_path: crate::tests::support::unique_test_store_path(
                    "forge-relational-property",
                ),
                segment_commit_capacity: 2,
            })
            .mvcc(MvccConfig {
                track_visibility_metadata: true,
                snapshot_release_policy: SnapshotReleasePolicy::ExplicitRelease,
                auto_reclaim_deleted_records: true,
                reclaim_batch_size: 8,
                retention_backend: RetentionBackend::EpochChunkRetention,
            })
            .build(),
    }
}
