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
    assert!(outcome.reconstructability.journal.replay_event_count >= 1);
    assert_eq!(
        outcome.reconstructability.checkpoint.journal_replay_span,
        outcome.reconstructability.journal.replay_event_count as u64
    );
    let metrics = runtime.observe().metrics();
    let graph_metrics = runtime.observe().graph().metrics();
    assert!(metrics.transaction.decision_log_event_count >= 1);
    assert!(graph_metrics.invalidation.batch_width >= 1);
    assert!(
        outcome
            .performance_accounting
            .transaction
            .decision_log_event_count
            >= 1
    );
    assert!(
        metrics.checkpoint.journal_replay_span
            >= outcome.reconstructability.journal.replay_event_count as u64
    );
    assert!(
        outcome
            .performance_accounting
            .checkpoint
            .journal_replay_span
            >= outcome.reconstructability.journal.replay_event_count as u64
    );
    assert_eq!(
        outcome.reconstructability.checkpoint.checkpoint_size,
        outcome.performance_accounting.checkpoint.checkpoint_size
    );
    let proof = outcome.reconstructability.proof();
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
