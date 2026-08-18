use super::*;

pub(super) fn strategy_schema_registry() -> crate::schema::data::RelationalSchemaRegistry {
    AspectSchemaFixture {
        entity_aspects: vec![
            entity_field_aspect(
                crate::tests::support::aspect_key("name"),
                crate::tests::support::field_key("name"),
            ),
            entity_u64_field_aspect(
                crate::tests::support::aspect_key("replicas"),
                crate::tests::support::field_key("replicas"),
            ),
            lifecycle_aspect(),
        ],
        ..AspectSchemaFixture::default()
    }
    .build_registry()
}

pub(super) fn persisted_strategy_runtime(root_path: std::path::PathBuf) -> RelationalRuntime {
    let intent_descriptor = IntentReconciliationStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(881),
    );
    let replica_descriptor = ReplicaConvergenceStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(882),
    );
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(strategy_schema_registry())
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path,
            segment_commit_capacity: 2,
        })
        .commit_strategy(
            crate::facade::commit_strategies::CommitStrategyRegistration::new(
                intent_descriptor.clone(),
            )
            .expect("intent registration"),
        )
        .commit_strategy_executor(IntentReconciliationStrategy::execution_registration(
            &intent_descriptor,
        ))
        .commit_strategy(
            crate::facade::commit_strategies::CommitStrategyRegistration::new(
                replica_descriptor.clone(),
            )
            .expect("replica registration"),
        )
        .commit_strategy_executor(ReplicaConvergenceStrategy::execution_registration(
            &replica_descriptor,
        ))
        .build()
}

pub(super) fn execute_strategy_commit(
    runtime: &mut RelationalRuntime,
    request: NativeStrategyCommitRequest,
    target_branch: Option<BranchId>,
) -> CommitResult {
    let request = runtime
        .commit_strategies()
        .canonicalize_request(&request)
        .expect("canonical strategy request");
    let snapshot = if let Some(branch_id) = target_branch.as_ref() {
        let branch_head = runtime
            .history()
            .branch_head(branch_id)
            .cloned()
            .expect("target branch head for strategy snapshot");
        if branch_head.version_id == runtime.current_version_id() {
            runtime.visibility_authority().snapshot()
        } else {
            runtime
                .visibility_authority()
                .pin_snapshot(branch_head.version_id)
                .expect("pin target branch strategy snapshot")
                .handle()
                .clone()
        }
    } else {
        runtime.visibility_authority().snapshot()
    };
    let execution = runtime
        .commit_strategies()
        .execute(&request, &snapshot)
        .expect("strategy execution");
    let transaction_options = target_branch
        .as_ref()
        .map(|branch| {
            crate::tests::support::test_owner_transaction_options_for_branch(
                &*runtime,
                branch.clone(),
            )
        })
        .unwrap_or_else(|| {
            crate::tests::support::test_owner_transaction_options_for_main(&*runtime)
        });
    let mut authority = runtime.commit_strategies_authority();
    let lowered = authority
        .lower_execution(&request, &execution, transaction_options)
        .expect("lowered strategy plan");
    let validated = authority
        .validate_lowered_plan(lowered)
        .expect("validated strategy plan");
    authority
        .execute_validated_commit(validated)
        .expect("strategy commit")
}
