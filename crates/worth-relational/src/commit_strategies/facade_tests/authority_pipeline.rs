use super::native_strategy_fixtures::*;

#[test]
fn execute_lowered_commit_routes_strategy_plan_through_authoritative_pipeline() {
    let mut runtime = RelationalRuntimeBuilder::new()
        .schema_registry(strategy_schema_registry())
        .commit_strategy(strategy_registration())
        .build();
    let request = canonical_request();
    let execution = execution_draft(&request);
    let lowered = {
        let mut authority = runtime.commit_strategies_authority();
        authority
            .lower_execution(&request, &execution, TransactionOptions::default())
            .expect("lowered strategy plan")
    };

    let commit = {
        let mut authority = CommitStrategiesAuthorityFacade::new(&mut runtime);
        authority
            .execute_lowered_commit(lowered)
            .expect("strategy commit executed")
    };

    assert_eq!(commit.commit.commit_id.0, 1);
    assert_eq!(commit.version_id.0, 1);
    assert_eq!(runtime.current_version_id().0, 1);
    assert!(commit.publication().strategy_artifacts.is_some());
    assert!(commit.publication().envelope.strategy_artifacts.is_some());
    assert_eq!(
        commit
            .publication()
            .strategy_artifacts
            .as_ref()
            .expect("strategy artifacts")
            .merge_descriptor()
            .semantic_name()
            .as_str(),
        "strategy.intent.reconcile"
    );
}

#[test]
fn validate_lowered_plan_preserves_strategy_provenance_and_commit_boundary_summary() {
    let mut runtime = RelationalRuntimeBuilder::new()
        .schema_registry(strategy_schema_registry())
        .commit_strategy(strategy_registration())
        .build();
    let request = canonical_request();
    let execution = execution_draft(&request);
    let lowered = {
        let mut authority = runtime.commit_strategies_authority();
        authority
            .lower_execution(&request, &execution, TransactionOptions::default())
            .expect("lowered strategy plan")
    };

    let validated = {
        let mut authority = CommitStrategiesAuthorityFacade::new(&mut runtime);
        authority
            .validate_lowered_plan(lowered)
            .expect("validated lowered strategy plan")
    };

    assert_eq!(
        validated.lowered_plan().lowering_provenance().strategy_id(),
        CommitStrategyId(41)
    );
    assert_eq!(validated.validated_against_version_id().0, 0);
    assert!(validated.validation_summary().commit_boundary_seen);
    assert!(validated.validation_summary().mutation_sensitive_seen);
    assert!(validated.validation_summary().snapshot_publication_seen);
    assert_eq!(validated.validation_summary().execution_count, 3);
    assert!(validated.validation_summary().plan_backed_execution_count >= 1);
    assert_eq!(validated.preview_validation_cost().merged_intent_count(), 1);
    assert_eq!(
        validated
            .preview_validation_cost()
            .post_mutation_preview_pass_count(),
        2
    );
    assert!(validated
        .preview_mutation_sensitive_invariants()
        .metadata()
        .has_merged_plan());
}

#[test]
fn execute_validated_commit_routes_prevalidated_strategy_plan_through_authoritative_pipeline() {
    let mut runtime = RelationalRuntimeBuilder::new()
        .schema_registry(strategy_schema_registry())
        .commit_strategy(strategy_registration())
        .build();
    let request = canonical_request();
    let execution = execution_draft(&request);
    let validated = {
        let mut authority = runtime.commit_strategies_authority();
        let lowered = authority
            .lower_execution(&request, &execution, TransactionOptions::default())
            .expect("lowered strategy plan");
        authority
            .validate_lowered_plan(lowered)
            .expect("validated lowered strategy plan")
    };

    let commit = {
        let mut authority = CommitStrategiesAuthorityFacade::new(&mut runtime);
        authority
            .execute_validated_commit(validated)
            .expect("validated strategy commit executed")
    };

    assert_eq!(commit.commit.commit_id.0, 1);
    assert!(commit.validation().summary.execution_count >= 3);
    assert!(commit.validation().summary.commit_boundary_seen);
    let strategy_artifacts = commit
        .publication()
        .strategy_artifacts
        .as_ref()
        .expect("strategy artifacts on publication");
    assert!(strategy_artifacts.preview_validation_summary().is_some());
    assert_eq!(
        strategy_artifacts
            .preview_validation_cost()
            .expect("preview validation cost")
            .post_mutation_preview_pass_count(),
        2
    );
    assert_eq!(
        strategy_artifacts
            .merge_descriptor()
            .merge_semantics()
            .conflict_class(),
        crate::commit_strategies::data::StrategyMergeConflictClass::IntentReconciliation
    );
    assert_eq!(
        commit.publication().envelope.strategy_artifacts.as_ref(),
        Some(strategy_artifacts)
    );
}

#[test]
fn execute_validated_commit_rejects_stale_validation_basis_after_intervening_commit() {
    let mut runtime = RelationalRuntimeBuilder::new()
        .schema_registry(strategy_schema_registry())
        .commit_strategy(strategy_registration())
        .build();
    let request = canonical_request();
    let execution = execution_draft(&request);
    let validated = {
        let mut authority = runtime.commit_strategies_authority();
        let lowered = authority
            .lower_execution(&request, &execution, TransactionOptions::default())
            .expect("lowered strategy plan");
        authority
            .validate_lowered_plan(lowered)
            .expect("validated lowered strategy plan")
    };

    let mut ordinary_txn = runtime.begin_transaction(TransactionOptions::default());
    ordinary_txn.push_batch(WorkerIntentBatch::new("ordinary-create").push(
        MutationIntent::Create(CreateIntent::Entity(EntitySpec {
            partition_id: PartitionId(1),
            kind_id: KindId(1),
            client_key: ClientKey::from("ordinary-a"),
            fields: strategy_name_and_replicas_patch("ordinary-a", 1),
        })),
    ));
    ordinary_txn.commit().expect("ordinary commit succeeds");

    let error = {
        let mut authority = CommitStrategiesAuthorityFacade::new(&mut runtime);
        authority
            .execute_validated_commit(validated)
            .expect_err("stale validated strategy plan should be rejected")
    };

    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert!(matches!(
                error.class,
                crate::transactions::data::ConflictClass::StaleValidationBasis { .. }
            ));
        }
        other => panic!("expected conflict rejection, got {other:?}"),
    }
}
