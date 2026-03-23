use crate::data::telemetry::{
    CheckpointTelemetry, EvaluationTelemetry, ExecutionTelemetry, InvalidationTelemetry,
    PlannerTelemetry, StorageTelemetry, TransactionTelemetry,
};
use crate::easy::ReactiveGraph;
use crate::facade::*;
use crate::tests::support::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Domain {
    Cache,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Impact {
    One,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ev {
    Tick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tier {
    Feature,
}

#[test]
fn runtime_builder_uses_expected_defaults() {
    let graph = SignalGraph::new();
    let runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();

    assert_eq!(
        runtime.checkpoint().policy().barrier_for(()),
        CheckpointBarrier::PerOperation
    );
    assert_eq!(
        *runtime.config().fallback_comparator(),
        VersionComparatorPolicy::Exact
    );
}

#[test]
fn runtime_builder_supports_typed_runtime_configuration() {
    let graph = SignalGraph::new();
    let _ = Impact::One;
    let _ = Ev::Tick;
    let _ = Tier::Feature;
    let runtime = SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .with_domains::<Domain>()
        .with_impacts::<Impact>()
        .with_events::<Ev>()
        .with_tiers::<Tier>()
        .checkpoint_barrier(CheckpointBarrier::PerOperation)
        .fallback_comparator(VersionComparatorPolicy::Exact)
        .build();

    assert_eq!(
        runtime.checkpoint().policy().barrier_for(Domain::Cache),
        CheckpointBarrier::PerOperation
    );
}

#[test]
fn transaction_helper_commits_on_success() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let outcome = runtime
        .transaction(&mut (), |transaction| {
            transaction.mark_dirty(source, ASPECT_A)?;
            Ok(())
        })
        .unwrap();

    assert_eq!(outcome.outcome, TransactionOutcome::Committed);
    assert_eq!(
        outcome.reconstructability.authority_branch_id,
        runtime.observe().current_branch().id
    );
    assert_eq!(
        outcome.reconstructability.authority_snapshot_id,
        runtime.observe().current_branch().head_snapshot_id
    );
    assert!(
        outcome
            .reconstructability
            .journal
            .replay_event_count
            >= 1
    );
    assert_eq!(
        outcome.reconstructability.checkpoint.journal_replay_span,
        outcome
            .reconstructability
            .journal
            .replay_event_count as u64
    );
    let metrics = runtime.observe().metrics();
    let graph_metrics = runtime.observe().graph().metrics();
    assert!(metrics.transaction.decision_log_event_count >= 1);
    assert!(graph_metrics.invalidation.batch_width >= 1);
    assert!(outcome.performance_accounting.transaction.decision_log_event_count >= 1);
    assert!(
        metrics.checkpoint.journal_replay_span
            >= outcome
                .reconstructability
                .journal
                .replay_event_count as u64
    );
    assert!(
        outcome.performance_accounting.checkpoint.journal_replay_span
            >= outcome
                .reconstructability
                .journal
                .replay_event_count as u64
    );
    assert_eq!(
        outcome.reconstructability.checkpoint.checkpoint_size,
        outcome.performance_accounting.checkpoint.checkpoint_size
    );
    let proof = outcome
        .reconstructability
        .proof();
    assert_eq!(
        proof.checkpoint.authority_branch_id,
        outcome.reconstructability.authority_branch_id
    );
    assert!(
        proof.required_rebuild.len() >= 2,
        "transaction proof should classify semantically required derived rebuild surfaces"
    );
    assert_eq!(
        runtime.graph().get_state(dependent).unwrap(),
        NodeState::Dirty
    );
}

#[test]
fn transaction_helper_rolls_back_on_error() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();
    let before = graph.get_state(dependent).unwrap();

    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let err = runtime
        .transaction(&mut (), |transaction| {
            transaction.mark_dirty(source, ASPECT_A)?;
            Err(SignalError::internal("fail the transaction"))
        })
        .unwrap_err();

    assert!(format!("{err}").contains("fail the transaction"));
    assert_eq!(runtime.graph().get_state(dependent).unwrap(), before);
}

#[test]
fn graph_node_builder_sets_accessible_configuration() {
    let mut graph = SignalGraph::new();
    let node = graph
        .node()
        .reads_aspects([ASPECT_A, ASPECT_B])
        .produces_aspects([ASPECT_B])
        .requires_context(ContextRequirement::DomainContext)
        .path_class(PathClass::Rich)
        .maintenance_mode(MaintenanceMode::RebuildAllowed)
        .artifact_policy(ArtifactPolicyClass::DevelopmentRetained)
        .on_demand()
        .tolerance(2)
        .build();

    let config = graph.get_entry(node).unwrap().get_eval_config().clone();
    assert_eq!(
        config.contract.semantics.reads,
        AspectMask::from([ASPECT_A, ASPECT_B])
    );
    assert_eq!(
        config.contract.semantics.produces,
        AspectMask::from([ASPECT_B])
    );
    assert_eq!(
        config.contract.semantics.required_context,
        ContextRequirement::DomainContext
    );
    assert_eq!(
        config.contract.projection.consumes,
        AspectMask::from([ASPECT_A, ASPECT_B])
    );
    assert_eq!(config.contract.execution.path_class, PathClass::Rich);
    assert_eq!(
        config.contract.execution.maintenance_mode,
        MaintenanceMode::RebuildAllowed
    );
    assert_eq!(
        config.contract.execution.artifact_policy,
        ArtifactPolicyClass::DevelopmentRetained
    );
    assert_eq!(
        config.contract.execution.equivalence,
        EquivalenceContract::for_comparator_override(&VersionComparatorPolicy::Tolerance {
            epsilon: 2,
        })
    );
    assert_eq!(
        config.contract.authority.policy,
        AuthorityPolicy::SpeculativeThenReconcile
    );
    assert_eq!(config.condition, EvaluationCondition::OnDemand);
    assert_eq!(
        config.comparator,
        Some(VersionComparatorPolicy::Tolerance { epsilon: 2 })
    );
}

#[test]
fn node_contract_uses_explicit_performance_defaults() {
    let contract = NodeContract::default();

    assert_eq!(
        contract.execution.equivalence,
        EquivalenceContract::default()
    );
    assert_eq!(contract.execution.path_class, PathClass::Operational);
    assert_eq!(
        contract.execution.maintenance_mode,
        MaintenanceMode::DensityAdaptive
    );
    assert_eq!(
        contract.execution.artifact_policy,
        ArtifactPolicyClass::OperationalMinimal
    );
    assert_eq!(
        contract.authority.policy,
        AuthorityPolicy::SpeculativeThenReconcile
    );
    assert_eq!(
        contract.reuse,
        NodeReuseContract {
            equivalence: ArtifactEquivalenceContract::strict(),
            retain_certification: true,
        }
    );
    assert_eq!(contract.projection.consumes, AspectMask::ALL);
}

#[test]
fn graph_node_builder_sets_reuse_contract_accessibly() {
    let mut graph = SignalGraph::new();
    let node = graph
        .node()
        .artifact_equivalence_contract(ArtifactEquivalenceContract {
            required_boundaries: vec![
                ArtifactSemanticBoundary::TopologyRegime,
                ArtifactSemanticBoundary::AuthorityLane,
            ],
            supported_strategies: vec![
                crate::data::reuse::ReuseStrategy::OutputSuppression,
                crate::data::reuse::ReuseStrategy::MemoizedArtifactReuse,
                crate::data::reuse::ReuseStrategy::SnapshotRestoreReuse,
                crate::data::reuse::ReuseStrategy::ReconciliationAdoption,
                crate::data::reuse::ReuseStrategy::CrossIdentityPersistentMatch,
                crate::data::reuse::ReuseStrategy::PartialArtifactSplicing,
            ],
            allows_snapshot_restore_reuse: true,
            allows_authority_reconciliation_reuse: false,
        })
        .retain_reuse_certification(false)
        .build();

    let config = graph.get_entry(node).unwrap().get_eval_config().clone();
    assert_eq!(
        config.contract.reuse.equivalence.required_boundaries,
        vec![
            ArtifactSemanticBoundary::TopologyRegime,
            ArtifactSemanticBoundary::AuthorityLane,
        ]
    );
    assert!(
        config
            .contract
            .reuse
            .equivalence
            .allows_snapshot_restore_reuse
    );
    assert!(
        !config
            .contract
            .reuse
            .equivalence
            .allows_authority_reconciliation_reuse
    );
    assert!(!config.contract.reuse.retain_certification);
}

#[test]
fn reuse_domain_types_are_publicly_reachable() {
    let basis = ReuseBasis::strategy(
        crate::data::reuse::ReuseStrategy::MemoizedArtifactReuse,
        ReuseSource::MemoizedArtifact,
        ReuseCrossing::None,
    );
    let record = ReuseCertificationRecord {
        strategy: crate::data::reuse::ReuseStrategy::SnapshotRestoreReuse,
        origin: crate::data::reuse::ReuseOrigin::SnapshotRestore,
        source: ReuseSource::SnapshotArtifact,
        crossing: ReuseCrossing::SnapshotRestore,
        proofs: vec![ReuseBoundaryProof {
            boundary: ArtifactSemanticBoundary::SnapshotLineage,
            satisfied: true,
        }],
    };

    assert_eq!(
        basis,
        ReuseBasis::strategy(
            crate::data::reuse::ReuseStrategy::MemoizedArtifactReuse,
            ReuseSource::MemoizedArtifact,
            ReuseCrossing::None,
        )
    );
    assert_eq!(record.proofs.len(), 1);
    assert_eq!(
        record.proofs[0].boundary,
        ArtifactSemanticBoundary::SnapshotLineage
    );
}

#[test]
fn runtime_policy_maps_into_s9_contract_and_strategy_defaults() {
    let operational = SignalRuntimePolicy::operational();
    let development = SignalRuntimePolicy::development();
    let forensic = SignalRuntimePolicy::forensic();

    assert_eq!(operational.default_path_class(), PathClass::Operational);
    assert_eq!(
        operational.default_artifact_policy_class(),
        ArtifactPolicyClass::OperationalMinimal
    );
    assert_eq!(
        operational.default_execution_strategy(),
        ResolvedExecutionStrategy::SparseIncremental
    );
    assert_eq!(
        operational.default_maintenance_strategy(),
        ResolvedMaintenanceStrategy::DensityAdaptive
    );
    assert_eq!(
        operational.default_authority_policy(),
        AuthorityPolicy::SpeculativeThenReconcile
    );

    assert_eq!(development.default_path_class(), PathClass::Rich);
    assert_eq!(
        development.default_artifact_policy_class(),
        ArtifactPolicyClass::DevelopmentRetained
    );
    assert_eq!(
        development.default_execution_strategy(),
        ResolvedExecutionStrategy::DenseStageBatched
    );
    assert_eq!(
        development.default_maintenance_strategy(),
        ResolvedMaintenanceStrategy::Incremental
    );
    assert_eq!(
        development.default_authority_policy(),
        AuthorityPolicy::SpeculativeThenReconcile
    );

    assert_eq!(forensic.default_path_class(), PathClass::Rich);
    assert_eq!(
        forensic.default_artifact_policy_class(),
        ArtifactPolicyClass::ForensicReconstructable
    );
    assert_eq!(
        forensic.default_execution_strategy(),
        ResolvedExecutionStrategy::DenseStageBatched
    );
    assert_eq!(
        forensic.default_maintenance_strategy(),
        ResolvedMaintenanceStrategy::Rebuild
    );
    assert_eq!(
        forensic.default_authority_policy(),
        AuthorityPolicy::SpeculativeThenReconcile
    );
}

#[test]
fn node_contract_and_runtime_policy_expose_s9_1_enforcement_surfaces() {
    let contract = NodeContract::reads([ASPECT_A])
        .with_equivalence(EquivalenceContract::for_comparator_override(
            &VersionComparatorPolicy::Exact,
        ))
        .with_path_class(PathClass::Rich)
        .with_maintenance_mode(MaintenanceMode::RebuildAllowed)
        .with_artifact_policy(ArtifactPolicyClass::DevelopmentRetained);
    let compile_time = contract.compile_time_performance_contract();
    let resolved = SignalRuntimePolicy::development().resolve_performance_policy();

    assert_eq!(PerformanceEnforcementLayer::CompileTime as u8, 0);
    assert_eq!(PerformanceEnforcementLayer::RuntimePolicy as u8, 1);
    assert_eq!(PerformanceEnforcementLayer::CounterTest as u8, 2);

    assert_eq!(compile_time.equivalence, contract.execution.equivalence);
    assert_eq!(compile_time.path_class, PathClass::Rich);
    assert_eq!(
        compile_time.maintenance_mode,
        MaintenanceMode::RebuildAllowed
    );
    assert_eq!(
        compile_time.artifact_policy,
        ArtifactPolicyClass::DevelopmentRetained
    );
    assert_eq!(
        compile_time.authority_policy,
        AuthorityPolicy::SpeculativeThenReconcile
    );

    assert_eq!(resolved.path_class, PathClass::Rich);
    assert_eq!(
        resolved.artifact_policy,
        ArtifactPolicyClass::DevelopmentRetained
    );
    assert_eq!(
        resolved.execution_strategy,
        ResolvedExecutionStrategy::DenseStageBatched
    );
    assert_eq!(
        resolved.maintenance_strategy,
        ResolvedMaintenanceStrategy::Incremental
    );
    assert_eq!(
        resolved.authority_policy,
        AuthorityPolicy::SpeculativeThenReconcile
    );
}

#[test]
fn runtime_telemetry_exposes_performance_counter_surface() {
    let telemetry = RuntimeTelemetry {
        evaluation: EvaluationTelemetry {
            evaluation_calls: 3,
            ..EvaluationTelemetry::default()
        },
        invalidation: InvalidationTelemetry {
            invalidation_nodes_visited: 5,
            ..InvalidationTelemetry::default()
        },
        transaction: TransactionTelemetry {
            transaction_commit_count: 2,
            ..TransactionTelemetry::default()
        },
        planner: PlannerTelemetry {
            stages_built: 7,
            ..PlannerTelemetry::default()
        },
        execution: ExecutionTelemetry {
            rewiring_apply_count: 11,
            ..ExecutionTelemetry::default()
        },
        storage: StorageTelemetry {
            graph_storage_snapshot_rewrites: 13,
            ..StorageTelemetry::default()
        },
        checkpoint: CheckpointTelemetry {
            checkpoint_flushes: 17,
            ..CheckpointTelemetry::default()
        },
    };
    let counters = telemetry.performance_counter_surface();

    assert_eq!(counters.evaluation.evaluation_calls, 3);
    assert_eq!(counters.invalidation.invalidation_nodes_visited, 5);
    assert_eq!(counters.transaction.transaction_commit_count, 2);
    assert_eq!(counters.planner.stages_built, 7);
    assert_eq!(counters.execution.rewiring_apply_count, 11);
    assert_eq!(counters.storage.graph_storage_snapshot_rewrites, 13);
    assert_eq!(counters.checkpoint.checkpoint_flushes, 17);
}

#[test]
fn proof_bearing_form_families_exist_as_real_types() {
    fn assert_canonical<T: CanonicalForm>() {}
    fn assert_resolved<T: ResolvedForm>() {}
    fn assert_delta<T: DeltaForm>() {}
    fn assert_summary<T: SummaryForm>() {}

    assert_canonical::<CanonicalDependencies>();
    assert_canonical::<CanonicalChangedRegions>();
    assert_canonical::<DedupedNodeBatch>();
    assert_canonical::<DependencyBatchEdit>();
    assert_canonical::<PartitionScopeSet>();
    assert_canonical::<SortedSourceBatch>();
    assert_resolved::<ResolvedExecutionStrategy>();
    assert_resolved::<ResolvedMaintenanceStrategy>();
    assert_resolved::<ResolvedPerformancePolicy>();
    assert_delta::<DirtyBatch>();
    assert_delta::<DirtyDelta>();
    assert_delta::<StructuralDelta>();
    assert_delta::<PatchPlan>();
    assert_summary::<LocalityFootprint>();
    assert_summary::<NarrowedPropagationSet>();
    assert_summary::<FrontierWave>();
    assert_summary::<InvalidationFrontier>();
    assert_summary::<InvalidationSeedBatch>();
    assert_summary::<FrontierPlan>();
    assert_summary::<FrontierExecutionSummary>();
    assert_summary::<SemanticBatchCommit>();
    assert_summary::<TouchedScopeSummary>();
    assert_summary::<PendingSnapshotBatch>();
    assert_summary::<SnapshotBatchCommit>();
    assert_summary::<SubscriberRepairBatch>();
}

#[test]
fn single_consumer_preserves_one_way_packet_flow() {
    let packet = SingleConsumer::new(vec![1_u32, 2, 3]);

    assert_eq!(packet.as_ref(), &[1, 2, 3]);
    assert_eq!(packet.into_inner(), vec![1, 2, 3]);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrderedTestItem(u32);

impl OrderedStreamItem for OrderedTestItem {
    type OrderKey = u32;

    fn order_key(&self) -> Self::OrderKey {
        self.0
    }
}

#[test]
fn mergeable_ordered_stream_merges_locally_ordered_shards_without_global_sort() {
    let left = LocallyOrderedShard::new(vec![OrderedTestItem(0), OrderedTestItem(2)]);
    let right = LocallyOrderedShard::new(vec![OrderedTestItem(1), OrderedTestItem(3)]);

    let merged = MergeableOrderedStream::new(vec![left, right])
        .try_into_vec()
        .unwrap();

    assert_eq!(
        merged,
        vec![
            OrderedTestItem(0),
            OrderedTestItem(1),
            OrderedTestItem(2),
            OrderedTestItem(3)
        ]
    );
}

#[test]
fn unordered_canonicalization_is_explicit_fallback_for_ordered_shards() {
    let shard = LocallyOrderedShard::canonicalize_unordered(vec![
        OrderedTestItem(3),
        OrderedTestItem(1),
        OrderedTestItem(2),
    ]);

    assert_eq!(
        shard.into_vec(),
        vec![OrderedTestItem(1), OrderedTestItem(2), OrderedTestItem(3)]
    );
}

#[test]
fn prepared_dependency_capture_recording_preserves_sorted_unique_order_without_resort() {
    let mut capture = crate::logic::prepared::PreparedDependencyCapture::new();
    let source_a = NodeId::new(9, 0);
    let source_b = NodeId::new(3, 1);

    capture.record(source_a, ASPECT_B, None);
    capture.record(source_b, ASPECT_A, None);
    capture.record(source_a, ASPECT_B, None);

    let capture = capture.into_sorted_unique();
    assert_eq!(capture.as_slice().len(), 2);
    assert!(capture.as_slice().windows(2).all(|pair| {
        (
            pair[0].source.index(),
            pair[0].source.generation(),
            pair[0].aspect.index(),
            pair[0].scope.as_ref(),
        ) < (
            pair[1].source.index(),
            pair[1].source.generation(),
            pair[1].aspect.index(),
            pair[1].scope.as_ref(),
        )
    }));
    assert_eq!(capture.as_slice()[0].source, source_b);
    assert_eq!(capture.as_slice()[1].source, source_a);
}

#[test]
fn proof_bearing_batches_and_summaries_canonicalize_their_inputs() {
    let node_a = NodeId::new(7, 1);
    let node_b = NodeId::new(3, 2);
    let changed_regions = CanonicalChangedRegions::new(vec![
        ChangedRegion {
            partition: "wing".into(),
            detail: Some("spar".into()),
        },
        ChangedRegion {
            partition: "wing".into(),
            detail: Some("spar".into()),
        },
        ChangedRegion {
            partition: "fuselage".into(),
            detail: None,
        },
    ]);
    let touched_nodes = DedupedNodeBatch::new([node_a, node_b, node_a]);
    let touched_sources = SortedSourceBatch::new([node_a, node_b, node_b]);
    let dirty_delta = DirtyDelta::new(AspectMask::from([ASPECT_A]), changed_regions, touched_nodes);
    let structural_delta = StructuralDelta::new(Some(dirty_delta.clone()), None);
    let patch_plan = PatchPlan::new(vec![node_a, node_b, node_a], structural_delta.clone());
    let touched_scope_summary = TouchedScopeSummary::new(
        vec![
            PartitionSubscription::partition_and_detail("wing", "spar"),
            PartitionSubscription::whole_partition("fuselage"),
            PartitionSubscription::partition_and_detail("wing", "spar"),
        ],
        vec![node_a, node_b, node_a],
        vec![node_a, node_b, node_b],
    );
    let snapshot_batch = PendingSnapshotBatch::from_pairs(vec![
        (node_a, crate::data::dependency::DependencySnapshot::empty()),
        (node_b, crate::data::dependency::DependencySnapshot::empty()),
        (node_a, crate::data::dependency::DependencySnapshot::empty()),
    ]);
    let subscriber_repairs = SubscriberRepairBatch::new(vec![
        SubscriberRepair {
            source: node_a,
            subscribers: DedupedNodeBatch::new([node_b, node_b]),
        },
        SubscriberRepair {
            source: node_b,
            subscribers: DedupedNodeBatch::new([node_a, node_a]),
        },
        SubscriberRepair {
            source: node_a,
            subscribers: DedupedNodeBatch::new([node_a, node_b]),
        },
    ]);
    let desired = DesiredState::new(AspectMask::from([ASPECT_A, ASPECT_B]));
    let dependency_batch = DependencyBatchEdit::from_pairs(vec![
        (
            node_a,
            CanonicalDependencies::new([DependencyEdge::new(node_b, ASPECT_A)]),
        ),
        (
            node_b,
            CanonicalDependencies::new([DependencyEdge::new(node_a, ASPECT_B)]),
        ),
    ]);
    let dirty_batch = DirtyBatch::new(vec![
        DirtyBatchEntry::new(node_a, ASPECT_A, vec![ChangedRegion::new("wing")]),
        DirtyBatchEntry::new(
            node_a,
            ASPECT_A,
            vec![ChangedRegion::new("wing"), ChangedRegion::new("fuselage")],
        ),
        DirtyBatchEntry::without_regions(node_b, ASPECT_B),
    ]);
    let semantic_batch_commit = SemanticBatchCommit::new(dirty_batch.clone());
    let locality = LocalityFootprint::new(
        vec![
            PartitionSubscription::partition_and_detail("wing", "spar"),
            PartitionSubscription::whole_partition("fuselage"),
            PartitionSubscription::partition_and_detail("wing", "spar"),
        ],
        vec![node_a, node_b, node_a],
        vec![node_a, node_b, node_b],
    );
    let snapshot_commit = SnapshotBatchCommit::from_pairs(vec![
        (node_a, crate::data::dependency::DependencySnapshot::empty()),
        (node_b, crate::data::dependency::DependencySnapshot::empty()),
        (node_a, crate::data::dependency::DependencySnapshot::empty()),
    ]);

    assert_eq!(dirty_delta.changed_regions.as_slice().len(), 2);
    assert_eq!(dirty_delta.touched_nodes.as_slice(), &[node_b, node_a]);
    assert!(!structural_delta.is_empty());
    assert!(!patch_plan.is_empty());
    assert_eq!(patch_plan.target_nodes.as_slice(), &[node_b, node_a]);
    assert_eq!(touched_sources.as_slice(), &[node_b, node_a]);
    assert_eq!(touched_scope_summary.seed_scopes.len(), 2);
    assert_eq!(
        touched_scope_summary.touched_nodes.as_slice(),
        &[node_b, node_a]
    );
    assert_eq!(
        touched_scope_summary.touched_sources.as_slice(),
        &[node_b, node_a]
    );
    assert_eq!(snapshot_batch.as_slice().len(), 2);
    assert_eq!(dependency_batch.as_slice().len(), 2);
    assert_eq!(dirty_batch.as_slice().len(), 2);
    assert_eq!(dirty_batch.changed_regions().as_slice().len(), 2);
    assert_eq!(dirty_batch.locality_footprint().partitions.len(), 2);
    assert_eq!(dirty_batch.touched_sources().as_slice(), &[node_b, node_a]);
    assert_eq!(locality.partitions.len(), 2);
    assert_eq!(locality.nodes.as_slice(), &[node_b, node_a]);
    assert_eq!(
        semantic_batch_commit.changed_aspects.bits(),
        AspectMask::from([ASPECT_A, ASPECT_B]).bits()
    );
    assert_eq!(semantic_batch_commit.locality.partitions.len(), 2);
    assert_eq!(snapshot_commit.target_nodes().as_slice(), &[node_b, node_a]);
    assert_eq!(subscriber_repairs.as_slice().len(), 2);
    assert_eq!(
        desired.value().bits(),
        AspectMask::from([ASPECT_A, ASPECT_B]).bits()
    );
}

#[test]
fn observer_exposes_runtime_and_retained_artifacts_separately() {
    let mut graph = SignalGraph::new();
    let node = graph.node().output_identity().build();
    let runtime_only = graph.node().build();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(7, 0))
            .with_output_identity("wing-surface")
            .with_continuity_token("wing-lineage")
            .with_label("forensic"))
    };
    evaluate(&mut graph, node, &mut compute).unwrap();
    let mut runtime_only_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(8, 0));
    evaluate(&mut graph, runtime_only, &mut runtime_only_compute).unwrap();
    graph
        .get_entry_mut(node)
        .unwrap()
        .set_causality(Some(CausalityMetadata {
            kind: "host_patch".to_string(),
            fields: std::collections::BTreeMap::from([(
                "patch_id".to_string(),
                "wing-42".to_string(),
            )]),
        }));

    let observer = graph.observe();
    let runtime = observer.runtime_artifact_state(node).unwrap().unwrap();
    let retained = observer
        .retained_diagnostic_artifact(node)
        .unwrap()
        .unwrap();
    let materializer = observer.materialize();
    let historical = materializer
        .materialize_historical_artifact_record(node)
        .unwrap()
        .unwrap();
    let trace = materializer.materialize_trace_summary(node).unwrap().unwrap();

    assert_eq!(
        runtime.output_identity.as_ref().map(|id| id.as_str()),
        Some("wing-surface")
    );
    assert_eq!(
        runtime
            .continuity_token
            .as_ref()
            .map(|token| token.as_str()),
        Some("wing-lineage")
    );
    assert_eq!(runtime.memoized_origin, MemoizedResultOrigin::DirectCompute);
    assert_eq!(runtime.reuse_basis, ReuseBasis::fresh_compute());
    assert_eq!(retained.labels, vec!["forensic".to_owned()]);
    assert_eq!(historical.node, node);
    assert_eq!(historical.runtime.output_identity, runtime.output_identity);
    assert_eq!(historical.runtime.reuse_basis, runtime.reuse_basis);
    assert_eq!(
        historical.retained.as_ref().unwrap().labels,
        retained.labels
    );
    assert_eq!(trace.reuse_basis, runtime.reuse_basis);
    assert_eq!(
        historical
            .causality
            .as_ref()
            .and_then(|causality| causality.fields.get("patch_id"))
            .map(|value| value.as_str()),
        Some("wing-42")
    );
    assert_eq!(trace.labels, vec!["forensic".to_owned()]);
    assert_eq!(
        trace.output_identity.as_ref().map(|id| id.as_str()),
        Some("wing-surface")
    );

    let runtime_only_state = observer.runtime_artifact_state(runtime_only).unwrap().unwrap();
    assert!(
        observer
            .retained_diagnostic_artifact(runtime_only)
            .unwrap()
            .is_none(),
        "runtime-only artifacts must not require retained richness"
    );
    let runtime_only_historical = materializer
        .materialize_historical_artifact_record(runtime_only)
        .unwrap()
        .unwrap();
    assert!(
        runtime_only_historical.retained.is_none(),
        "cold historical assembly should remain available without retained payload"
    );
    let runtime_only_trace = materializer
        .materialize_trace_summary(runtime_only)
        .unwrap()
        .unwrap();
    assert_eq!(
        runtime_only_trace.output_hash,
        runtime_only_state.output_hash,
        "cold trace assembly should derive from runtime truth even when retained richness is absent"
    );
}

#[test]
fn dependency_snapshot_clone_shares_backing_storage() {
    let mut snapshot = crate::data::dependency::DependencySnapshot::empty();
    snapshot.record(NodeId::new(1, 0), ASPECT_A, 7, None);
    snapshot.record(NodeId::new(2, 0), ASPECT_B, 11, None);

    let cloned = snapshot.clone();

    assert!(std::sync::Arc::ptr_eq(
        &snapshot.shared_entries(),
        &cloned.shared_entries()
    ));
    assert_eq!(snapshot.entries(), cloned.entries());
}

#[test]
fn replacing_dependency_snapshot_reports_delta() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let source = graph.node().build();

    let mut baseline = crate::data::dependency::DependencySnapshot::empty();
    baseline.record(source, ASPECT_A, 3, None);
    graph.set_dep_snapshot(node, baseline).unwrap();

    let mut updated = crate::data::dependency::DependencySnapshot::empty();
    updated.record(source, ASPECT_A, 5, None);
    updated.record(source, ASPECT_B, 7, None);

    let delta = graph
        .replace_dep_snapshot_shared(
            node,
            crate::data::dependency::DependencySnapshotUpdate::Replace(
                crate::data::dependency::SharedDependencySnapshot::new(updated),
            ),
        )
        .unwrap();

    assert_eq!(delta.node, node);
    assert_eq!(delta.previous_entry_count, 1);
    assert_eq!(delta.next_entry_count, 2);
    assert_eq!(delta.changed_entry_count, 2);
    assert!(delta.changed());
}

#[test]
fn replacing_identical_dependency_snapshot_is_a_noop() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let source = graph.node().build();

    let mut baseline = crate::data::dependency::DependencySnapshot::empty();
    baseline.record(source, ASPECT_A, 3, None);
    graph.set_dep_snapshot(node, baseline.clone()).unwrap();
    let first_id = graph.get_entry(node).unwrap().get_dep_snapshot_id();

    let delta = graph
        .replace_dep_snapshot_shared(
            node,
            crate::data::dependency::DependencySnapshotUpdate::Replace(
                crate::data::dependency::SharedDependencySnapshot::new(baseline),
            ),
        )
        .unwrap();
    let second_id = graph.get_entry(node).unwrap().get_dep_snapshot_id();

    assert_eq!(first_id, second_id);
    assert_eq!(delta.changed_entry_count, 0);
    assert!(!delta.changed());
}

#[test]
fn dependency_snapshot_version_only_update_preserves_shape() {
    let source_a = NodeId::new(1, 0);
    let source_b = NodeId::new(2, 0);
    let mut baseline = crate::data::dependency::DependencySnapshot::empty();
    baseline.record(source_a, ASPECT_A, 3, None);
    baseline.record(source_b, ASPECT_B, 7, None);

    let updated = baseline.with_updated_versions(&[5, 7]);
    let delta = crate::data::dependency::SnapshotDeltaRecord::between(
        NodeId::new(9, 0),
        &baseline,
        &crate::data::dependency::SharedDependencySnapshot::new(updated.clone()),
    );

    assert_eq!(baseline.entries().len(), updated.entries().len());
    assert_eq!(
        baseline
            .entries()
            .iter()
            .map(|entry| entry.sort_key())
            .collect::<Vec<_>>(),
        updated
            .entries()
            .iter()
            .map(|entry| entry.sort_key())
            .collect::<Vec<_>>()
    );
    assert_eq!(updated.entries()[0].cached_version, 5);
    assert_eq!(updated.entries()[1].cached_version, 7);
    assert_eq!(delta.changed_entry_count, 1);
    assert!(delta.changed());
}

#[test]
fn shared_dependency_snapshot_reports_storage_sharing_without_implying_semantics() {
    let source = NodeId::new(1, 0);
    let mut baseline = crate::data::dependency::DependencySnapshot::empty();
    baseline.record(source, ASPECT_A, 3, None);

    let shared_left = crate::data::dependency::SharedDependencySnapshot::new(baseline.clone());
    let shared_right = crate::data::dependency::SharedDependencySnapshot::new(baseline.clone());

    assert!(
        baseline.shares_storage_with(shared_left.snapshot()),
        "shared snapshot wrapping should preserve shared backing"
    );
    assert!(
        shared_left.shares_storage_with(&shared_right),
        "cloned snapshots should report shared backing explicitly"
    );

    let replace = crate::data::dependency::DependencySnapshotUpdate::Replace(shared_left);
    let version_only = crate::data::dependency::DependencySnapshotUpdate::VersionOnly(
        crate::data::dependency::DependencySnapshotVersionDelta::new([5]),
    );

    assert_eq!(
        replace.storage_strategy(),
        crate::data::dependency::SnapshotStorageStrategy::SharedReplacement
    );
    assert_eq!(
        version_only.storage_strategy(),
        crate::data::dependency::SnapshotStorageStrategy::VersionOnlyDelta
    );
}

#[test]
fn snapshot_storage_telemetry_distinguishes_replacement_from_version_only_delta() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let source = graph.node().build();

    let mut baseline = crate::data::dependency::DependencySnapshot::empty();
    baseline.record(source, ASPECT_A, 3, None);
    graph.set_dep_snapshot(node, baseline.clone()).unwrap();

    let mut replaced = crate::data::dependency::DependencySnapshot::empty();
    replaced.record(source, ASPECT_A, 5, None);
    replaced.record(source, ASPECT_B, 7, None);
    graph.replace_dep_snapshot_shared(
        node,
        crate::data::dependency::DependencySnapshotUpdate::Replace(
            crate::data::dependency::SharedDependencySnapshot::new(replaced),
        ),
    )
    .unwrap();

    graph.replace_dep_snapshot_shared(
        node,
        crate::data::dependency::DependencySnapshotUpdate::VersionOnly(
            crate::data::dependency::DependencySnapshotVersionDelta::new([11, 13]),
        ),
    )
    .unwrap();

    let storage = graph.observe().metrics().storage;
    assert!(
        storage.shared_snapshot_replacement_count >= 2,
        "snapshot telemetry should count full shared replacement boundaries"
    );
    assert!(
        storage.version_only_snapshot_update_count >= 1,
        "snapshot telemetry should count version-only delta boundaries separately"
    );
}

#[test]
fn set_dep_snapshot_uses_version_only_delta_when_snapshot_shape_is_stable() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let source_a = graph.node().build();
    let source_b = graph.node().build();

    let mut baseline = crate::data::dependency::DependencySnapshot::empty();
    baseline.record(source_a, ASPECT_A, 3, None);
    baseline.record(source_b, ASPECT_B, 7, None);
    graph.set_dep_snapshot(node, baseline).unwrap();

    let mut version_only = crate::data::dependency::DependencySnapshot::empty();
    version_only.record(source_a, ASPECT_A, 5, None);
    version_only.record(source_b, ASPECT_B, 11, None);
    graph.set_dep_snapshot(node, version_only).unwrap();

    let storage = graph.observe().metrics().storage;
    assert_eq!(
        storage.shared_snapshot_replacement_count, 1,
        "initial snapshot install should be the only full replacement when shape stays stable"
    );
    assert_eq!(
        storage.version_only_snapshot_update_count, 1,
        "stable-shape snapshot rewrite should narrow to a version-only delta"
    );
}

#[test]
fn derive_dependency_snapshot_restore_batch_uses_version_only_delta_for_shared_shape() {
    let mut current = SignalGraph::new();
    let source_a = current.node().build();
    let source_b = current.node().build();
    let target = current.node().build();

    let mut baseline = crate::data::dependency::DependencySnapshot::empty();
    baseline.record(source_a, ASPECT_A, 3, None);
    baseline.record(source_b, ASPECT_B, 7, None);
    current.set_dep_snapshot(target, baseline).unwrap();

    let mut restored = current.clone();
    let mut updated = crate::data::dependency::DependencySnapshot::empty();
    updated.record(source_a, ASPECT_A, 5, None);
    updated.record(source_b, ASPECT_B, 11, None);
    restored.set_dep_snapshot(target, updated).unwrap();

    let batch = current
        .derive_dependency_snapshot_restore_batch(&restored)
        .unwrap();
    let entries = batch.pending().as_slice();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].node, target);
    assert_eq!(
        entries[0].update.storage_strategy(),
        crate::data::dependency::SnapshotStorageStrategy::VersionOnlyDelta
    );
    assert_eq!(entries[0].delta.changed_entry_count, 2);
}

#[test]
fn locality_footprint_merges_and_detects_conflicts_canonically() {
    let node_a = NodeId::new(7, 1);
    let node_b = NodeId::new(3, 2);
    let node_c = NodeId::new(9, 1);

    let mut left = LocalityFootprint::new(
        vec![
            PartitionSubscription::whole_partition("wing"),
            PartitionSubscription::partition_and_detail("fuselage", "frame-2"),
        ],
        vec![node_a, node_b],
        vec![node_b],
    );
    let right = LocalityFootprint::new(
        vec![
            PartitionSubscription::partition_and_detail("fuselage", "frame-2"),
            PartitionSubscription::whole_partition("tail"),
        ],
        vec![node_b, node_c],
        vec![node_c],
    );

    assert!(left.conflicts_with(&right));
    left.merge(&right);

    assert_eq!(left.partitions.len(), 3);
    assert_eq!(left.nodes.as_slice(), &[node_b, node_a, node_c]);
    assert_eq!(left.sources.as_slice(), &[node_b, node_c]);
}

#[test]
fn graph_node_builder_accepts_explicit_node_contract() {
    let mut graph = SignalGraph::new();
    let contract = NodeContract::reads([ASPECT_A])
        .with_produces([ASPECT_B])
        .with_required_context(ContextRequirement::RelationalSnapshot);
    let node = graph.node().with_contract(contract.clone()).build();

    let stored = graph.get_contract(node).unwrap().clone();
    assert_eq!(stored, contract);
}

#[test]
fn transaction_batch_dirty_is_the_bulk_invalidation_surface() {
    let mut graph = SignalGraph::new();
    let source_a = graph.node().build();
    let source_b = graph.node().build();
    let dependent = graph.node().build();
    graph
        .set_dependencies(
            dependent,
            [
                DependencyEdge::new(source_a, ASPECT_A),
                DependencyEdge::new(source_b, ASPECT_B),
            ],
        )
        .unwrap();

    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    runtime
        .transaction(&mut (), |transaction| {
            transaction.mark_dirty_batch(&DirtyBatch::from_sources([
                (source_a, ASPECT_A),
                (source_b, ASPECT_B),
            ]))?;
            Ok(())
        })
        .unwrap();

    assert_eq!(
        runtime.graph().get_state(source_a).unwrap(),
        NodeState::Dirty
    );
    assert_eq!(
        runtime.graph().get_state(source_b).unwrap(),
        NodeState::Dirty
    );
    assert_eq!(
        runtime.graph().get_state(dependent).unwrap(),
        NodeState::Dirty
    );
}

#[test]
fn dependency_batch_edit_is_the_bulk_dependency_surface() {
    let mut graph = SignalGraph::new();
    let source_a = graph.node().build();
    let source_b = graph.node().build();
    let left = graph.node().build();
    let right = graph.node().build();

    graph
        .apply_dependency_batch_edit(&DependencyBatchEdit::from_pairs([
            (left, vec![DependencyEdge::new(source_a, ASPECT_A)]),
            (right, vec![DependencyEdge::new(source_b, ASPECT_B)]),
        ]))
        .unwrap();

    assert_eq!(graph.dependencies_of(left).unwrap().len(), 1);
    assert_eq!(graph.dependencies_of(right).unwrap().len(), 1);
    assert_eq!(graph.runtime_subscribers_of(source_a).unwrap(), &[left]);
    assert_eq!(graph.runtime_subscribers_of(source_b).unwrap(), &[right]);
}

#[test]
#[should_panic(expected = "dependency batch edit cannot contain multiple edits")]
fn dependency_batch_edit_rejects_duplicate_node_edits() {
    let node = NodeId::new(7, 1);
    let source = NodeId::new(3, 2);
    let _ = DependencyBatchEdit::from_pairs([
        (node, vec![DependencyEdge::new(source, ASPECT_A)]),
        (node, vec![DependencyEdge::new(source, ASPECT_B)]),
    ]);
}

#[test]
fn define_computation_applies_contract_comparator_and_tier_to_created_nodes() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .with_tiers::<Tier>()
        .build();
    let contract = NodeContract::reads([ASPECT_A])
        .with_produces([ASPECT_B])
        .with_required_context(ContextRequirement::DomainContext);
    let computation = runtime
        .define_computation(ComputationSpec {
            family: "geometry".into(),
            contract: contract.clone(),
            tier: Tier::Feature,
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |_ctx: &mut EvaluationContext<'_, ()>| {
                Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(
                    NodeEvaluationResult::from_version(version_ab(1, 0)),
                ))
            },
        })
        .unwrap();

    let node = computation.keyed("bulkhead").node(&mut runtime);
    let stored = runtime
        .graph()
        .get_entry(node)
        .unwrap()
        .get_eval_config()
        .clone();

    assert_eq!(runtime.graph().get_contract(node).unwrap(), &contract);
    assert_eq!(
        stored.comparator,
        Some(VersionComparatorPolicy::OutputIdentity)
    );
    assert_eq!(
        runtime.config().node_meta().tier_for_node(node),
        Some(Tier::Feature)
    );
}

#[test]
fn easy_mode_supports_input_computed_get_set_and_batch() {
    let mut graph = ReactiveGraph::new();
    let price = graph.input(100.0_f64);
    let tax = graph.input(0.08_f64);
    let total = graph.computed(move |context| context.get(price) * (1.0 + context.get(tax)));

    assert_eq!(graph.get(total), 108.0);

    graph.set(price, 200.0);
    assert_eq!(graph.get(total), 216.0);

    graph.batch(|reactive| {
        reactive.set(price, 300.0);
        reactive.set(tax, 0.10);
    });
    assert_eq!(graph.get(total), 330.0);
}

#[test]
fn easy_mode_computed_chains_observe_staged_upstream_values_in_the_same_pass() {
    let mut graph = ReactiveGraph::new();
    let source = graph.input(2_i32);
    let doubled = graph.computed(move |context| context.get(source) * 2);
    let chained = graph.computed(move |context| context.get(doubled) + 1);

    assert_eq!(graph.get(chained), 5);

    graph.set(source, 7);

    assert_eq!(
        graph.get(chained),
        15,
        "downstream computed nodes should see freshly staged upstream values, not the pre-plan cache"
    );
}

#[test]
fn easy_mode_failed_batch_restores_input_values() {
    let mut graph = ReactiveGraph::new();
    let price = graph.input(100_i32);
    let tax = graph.input(5_i32);

    let err = graph.try_batch(|reactive| {
        reactive.try_set(price, 200)?;
        reactive.try_set(tax, 9)?;
        Err(SignalError::invalid_input("force easy-mode rollback"))
    });
    assert!(err.is_err());

    assert_eq!(graph.get(price), 100);
    assert_eq!(graph.get(tax), 5);
}

#[test]
fn easy_mode_failed_batch_restores_downstream_invalidation_state() {
    let mut graph = ReactiveGraph::new();
    let source = graph.input(2_i32);
    let doubled = graph.computed(move |context| context.get(source) * 2);

    assert_eq!(graph.get(doubled), 4);

    let err = graph.try_batch(|reactive| {
        reactive.try_set(source, 9)?;
        reactive.try_get(doubled)?;
        Err(SignalError::invalid_input(
            "force rollback after dirty propagation",
        ))
    });
    assert!(err.is_err());

    assert_eq!(graph.get(source), 2);
    assert_eq!(graph.get(doubled), 4);
}
