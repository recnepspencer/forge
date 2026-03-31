use crate::authority::commit::pipeline::{
    execute_authoritative_commit, AuthoritativeCommitContext,
};
use crate::commit_strategies::data::{
    CanonicalStrategyCommitRequest, LoweredStrategyCommitPlan, RawStrategyCommitRequest,
    StrategyCommitRequestError, StrategyExecutionDraft, StrategyLoweringError,
    ValidatedStrategyCommitPlan,
};
use crate::commit_strategies::logic::{
    bind_execution, canonicalize_request, execute_bound_strategy, lower_execution,
    validate_lowered_plan as validate_lowered_strategy_plan,
};
use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::SnapshotHandle;
use crate::transactions::data::{CommitResult, TransactionCommitError, TransactionOptions};

#[derive(Debug, Clone, Copy)]
pub struct CommitStrategiesFacade<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> CommitStrategiesFacade<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn canonicalize_request(
        &self,
        request: &RawStrategyCommitRequest,
    ) -> Result<CanonicalStrategyCommitRequest, StrategyCommitRequestError> {
        canonicalize_request(self.runtime.commit_strategy_registry(), request)
    }

    pub fn execute(
        &self,
        request: &CanonicalStrategyCommitRequest,
        snapshot: &SnapshotHandle,
    ) -> Result<StrategyExecutionDraft, crate::commit_strategies::StrategyExecutionError> {
        let bound = bind_execution(self.runtime, request, snapshot)?;
        execute_bound_strategy(bound)
    }
}

#[derive(Debug)]
pub struct CommitStrategiesAuthorityFacade<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl<'runtime> CommitStrategiesAuthorityFacade<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn lower_execution(
        &mut self,
        request: &CanonicalStrategyCommitRequest,
        execution: &StrategyExecutionDraft,
        options: TransactionOptions,
    ) -> Result<LoweredStrategyCommitPlan, StrategyLoweringError> {
        lower_execution(self.runtime, request, execution, options)
    }

    pub fn execute_lowered_commit(
        &mut self,
        lowered: LoweredStrategyCommitPlan,
    ) -> Result<CommitResult, TransactionCommitError> {
        execute_authoritative_commit(
            self.runtime,
            AuthoritativeCommitContext::from_strategy(self.runtime, lowered),
        )
    }

    pub fn validate_lowered_plan(
        &mut self,
        lowered: LoweredStrategyCommitPlan,
    ) -> Result<ValidatedStrategyCommitPlan, TransactionCommitError> {
        validate_lowered_strategy_plan(self.runtime, lowered)
    }

    pub fn execute_validated_commit(
        &mut self,
        validated: ValidatedStrategyCommitPlan,
    ) -> Result<CommitResult, TransactionCommitError> {
        execute_authoritative_commit(
            self.runtime,
            AuthoritativeCommitContext::from_validated_strategy(self.runtime, validated),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::CommitStrategiesAuthorityFacade;
    use crate::commit_strategies::data::{
        CanonicalStrategyCommitRequest, CanonicalStrategyInputArtifact,
        CanonicalStrategyInputDigest, CanonicalStrategyOutputArtifact, CommitStrategyDescriptor,
        CommitStrategyExecutionRegistration, CommitStrategyExecutor, CommitStrategyFamilyName,
        CommitStrategyId, CommitStrategyRegistration, CommitStrategySemanticName,
        CommitStrategyVersion, PersistentArtifactName, StrategyCallerProvenance,
        StrategyExecutionDraft, StrategyExecutionResult, StrategyExecutionSummary,
        StrategyExecutorFailure, StrategyInputSchemaName, StrategyInputSchemaVersion,
        StrategyIntentName, StrategyMutationProgram, StrategyOutputSchemaName,
        StrategyPacketContract, StrategyReadContract, StrategyReadCostClass,
        StrategyReadLocalityClass, StrategyReadScopeClass, StrategyRequestCanonicalization,
        StrategyRequestOrigin, StrategyTraversalBasis,
    };
    use crate::commit_strategies::strategies::{
        IntentReconciliationInput, IntentReconciliationStrategy,
    };
    use crate::facade::history::BranchId;
    use crate::facade::merge::{MergeIntent, MergePlanningRequest};
    use crate::facade::replay::{
        RelationalReplayRequest, ReplayExecutionMode, ReplayObservableSurface,
        ReplayVerificationMode,
    };
    use crate::facade::transactions::{
        CreateIntent, EntityMutationIntent, MutationIntent, TransactionOptions, UpdateEntityIntent,
        WorkerIntentBatch,
    };
    use crate::identity::data::{EntityId, KindId, PartitionId};
    use crate::logic::builder::RelationalRuntimeBuilder;
    use crate::payloads::data::RecordPayload;
    use crate::snapshots::data::SnapshotHandle;
    use crate::symbols::data::InternedString;
    use crate::tests::support::read_entity_name;
    use crate::transactions::data::{EntitySpec, TransactionCommitError};
    use serde_json::json;

    fn strategy_descriptor() -> CommitStrategyDescriptor {
        strategy_descriptor_named(
            CommitStrategyId(41),
            "strategy.intent.reconcile",
            "strategy.intent",
            "reconcile.desired.state",
        )
    }

    fn strategy_descriptor_named(
        id: CommitStrategyId,
        semantic_name: &str,
        family_name: &str,
        intent_name: &str,
    ) -> CommitStrategyDescriptor {
        CommitStrategyDescriptor::new(
            id,
            CommitStrategySemanticName::new(semantic_name),
            CommitStrategyFamilyName::new(family_name),
            CommitStrategyVersion::new(1, 0),
            StrategyIntentName::new(intent_name),
            StrategyInputSchemaName::new("intent.reconcile.input.v1"),
            StrategyInputSchemaVersion(1),
            StrategyOutputSchemaName::new("intent.reconcile.output.v1"),
            StrategyRequestCanonicalization::JsonStableObjectOrderV1,
            StrategyReadContract {
                scope_class: StrategyReadScopeClass::ExplicitTargetsOnly,
                locality_class: StrategyReadLocalityClass::SinglePartition,
                traversal_basis: StrategyTraversalBasis::NoTraversal,
                packet_contract: StrategyPacketContract::ProjectionOnly,
                cost_class: StrategyReadCostClass::ORequestedSurface,
            },
            PersistentArtifactName::new(semantic_name),
        )
    }

    fn strategy_registration() -> CommitStrategyRegistration {
        CommitStrategyRegistration::new(strategy_descriptor()).expect("valid strategy registration")
    }

    #[derive(Clone, Copy)]
    struct PlanningExecutor;

    impl CommitStrategyExecutor for PlanningExecutor {
        fn execute(
            &self,
            request: &CanonicalStrategyCommitRequest,
            _observation: &crate::commit_strategies::data::StrategyObservationContext<'_>,
        ) -> Result<StrategyExecutionResult, StrategyExecutorFailure> {
            Ok(execution_result(request))
        }
    }

    fn canonical_request() -> CanonicalStrategyCommitRequest {
        let descriptor = strategy_descriptor();
        CanonicalStrategyCommitRequest::new(
            CommitStrategyId(41),
            descriptor.digest(),
            CanonicalStrategyInputArtifact::new(
                StrategyInputSchemaName::new("intent.reconcile.input.v1"),
                StrategyInputSchemaVersion(1),
                StrategyRequestCanonicalization::JsonStableObjectOrderV1,
                br#"{"replicas":3}"#.to_vec().into(),
                CanonicalStrategyInputDigest([9; 32]),
                PersistentArtifactName::new("strategy.intent.reconcile.input"),
            ),
            StrategyCallerProvenance {
                request_origin: StrategyRequestOrigin::Test,
                actor_identity: None,
                correlation_id: None,
            },
        )
    }

    fn execution_result(_request: &CanonicalStrategyCommitRequest) -> StrategyExecutionResult {
        let batch = WorkerIntentBatch::new("reconcile-deployment").push(MutationIntent::Create(
            CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId(1),
                kind_id: KindId(1),
                client_key: InternedString::from("deployment-a"),
                payload: RecordPayload::from(json!({"replicas": 3})),
            }),
        ));

        StrategyExecutionResult::new(
            CanonicalStrategyOutputArtifact::new(
                StrategyOutputSchemaName::new("intent.reconcile.output.v1"),
                br#"{"status":"planned"}"#.to_vec(),
                PersistentArtifactName::new("strategy.intent.reconcile.output"),
            ),
            StrategyMutationProgram::new(vec![batch]),
        )
    }

    fn update_execution_draft(
        request: &CanonicalStrategyCommitRequest,
        entity_id: EntityId,
        name: &str,
    ) -> StrategyExecutionDraft {
        let batch = WorkerIntentBatch::new("reconcile-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id,
                payload: RecordPayload::from(json!({ "name": name })),
            }),
        ));

        StrategyExecutionDraft::from_measured_result(
            request,
            StrategyExecutionResult::new(
                CanonicalStrategyOutputArtifact::new(
                    StrategyOutputSchemaName::new("intent.reconcile.output.v1"),
                    format!(r#"{{"status":"planned","name":"{name}"}}"#).into_bytes(),
                    PersistentArtifactName::new("strategy.intent.reconcile.output"),
                ),
                StrategyMutationProgram::new(vec![batch]),
            ),
            StrategyExecutionSummary::default(),
        )
    }

    fn execution_draft(request: &CanonicalStrategyCommitRequest) -> StrategyExecutionDraft {
        StrategyExecutionDraft::from_measured_result(
            request,
            execution_result(request),
            StrategyExecutionSummary::default(),
        )
    }

    #[test]
    fn execute_lowered_commit_routes_strategy_plan_through_authoritative_pipeline() {
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(crate::tests::support::test_schema_registry())
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
        assert!(commit.publication.strategy_artifacts.is_some());
        assert!(commit.publication.envelope.strategy_artifacts.is_some());
        assert_eq!(
            commit
                .publication
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
            .schema_registry(crate::tests::support::test_schema_registry())
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
            .schema_registry(crate::tests::support::test_schema_registry())
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
        assert!(commit.validation.summary.execution_count >= 3);
        assert!(commit.validation.summary.commit_boundary_seen);
        let strategy_artifacts = commit
            .publication
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
            strategy_artifacts.merge_descriptor().conflict_class(),
            crate::commit_strategies::data::StrategyMergeConflictClass::IntentReconciliation
        );
        assert_eq!(
            commit.publication.envelope.strategy_artifacts.as_ref(),
            Some(strategy_artifacts)
        );
    }

    #[test]
    fn execute_validated_commit_rejects_stale_validation_basis_after_intervening_commit() {
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(crate::tests::support::test_schema_registry())
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
                client_key: InternedString::from("ordinary-a"),
                payload: RecordPayload::from(json!({"replicas": 1})),
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

    #[test]
    fn replay_commit_certifies_strategy_surface_for_strategy_bearing_commit() {
        let descriptor = strategy_descriptor();
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(crate::tests::support::test_schema_registry())
            .commit_strategy(strategy_registration())
            .commit_strategy_executor(CommitStrategyExecutionRegistration::new(
                &descriptor,
                PlanningExecutor,
            ))
            .build();
        let request = runtime
            .commit_strategies()
            .canonicalize_request(
                &crate::commit_strategies::data::RawStrategyCommitRequest::new(
                    CommitStrategySemanticName::new("strategy.intent.reconcile"),
                    br#"{"replicas":3}"#.to_vec(),
                    StrategyCallerProvenance {
                        request_origin: StrategyRequestOrigin::Test,
                        actor_identity: None,
                        correlation_id: None,
                    },
                ),
            )
            .expect("canonical strategy request");
        let snapshot: SnapshotHandle = runtime.visibility_authority().snapshot();
        let execution = runtime
            .commit_strategies()
            .execute(&request, &snapshot)
            .expect("strategy executes against committed basis");
        let commit = {
            let mut authority = runtime.commit_strategies_authority();
            let lowered = authority
                .lower_execution(&request, &execution, TransactionOptions::default())
                .expect("lowered strategy plan");
            let validated = authority
                .validate_lowered_plan(lowered)
                .expect("validated lowered strategy plan");
            authority
                .execute_validated_commit(validated)
                .expect("validated strategy commit executed")
        };

        let replay = runtime
            .replay_authority()
            .replay_commit(RelationalReplayRequest {
                commit_id: commit.commit.commit_id,
                branch_id: commit.publication.envelope.branch_context.clone(),
                execution_mode: ReplayExecutionMode::SerialDeterministic,
                verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
            });

        assert!(
            replay.failure.is_none(),
            "unexpected replay failure: {replay:?}"
        );
        assert!(replay
            .compared_surfaces
            .contains(&ReplayObservableSurface::Strategy));
        assert!(replay
            .mismatches
            .iter()
            .all(|mismatch| mismatch.surface != ReplayObservableSurface::Strategy));
    }

    #[test]
    fn intent_reconciliation_strategy_commits_and_replays_end_to_end() {
        let descriptor = IntentReconciliationStrategy::descriptor(CommitStrategyId(61));
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(crate::tests::support::test_schema_registry())
            .commit_strategy(
                CommitStrategyRegistration::new(descriptor.clone())
                    .expect("intent strategy registration"),
            )
            .commit_strategy_executor(IntentReconciliationStrategy::execution_registration(
                &descriptor,
            ))
            .build();
        let entity = crate::tests::support::create_entity(&mut runtime, "before");
        let request = runtime
            .commit_strategies()
            .canonicalize_request(
                &crate::commit_strategies::data::RawStrategyCommitRequest::new(
                    crate::commit_strategies::data::CommitStrategySemanticName::new(
                        IntentReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
                    ),
                    serde_json::to_vec(&IntentReconciliationInput {
                        entity_id: entity,
                        desired_payload: json!({"name":"after"}),
                    })
                    .expect("serialize intent reconciliation input"),
                    StrategyCallerProvenance {
                        request_origin: StrategyRequestOrigin::Test,
                        actor_identity: None,
                        correlation_id: None,
                    },
                ),
            )
            .expect("canonical request");
        let snapshot: SnapshotHandle = runtime.visibility_authority().snapshot();
        let execution = runtime
            .commit_strategies()
            .execute(&request, &snapshot)
            .expect("strategy executes against committed basis");
        let commit = {
            let mut authority = runtime.commit_strategies_authority();
            let lowered = authority
                .lower_execution(&request, &execution, TransactionOptions::default())
                .expect("lowered strategy plan");
            let validated = authority
                .validate_lowered_plan(lowered)
                .expect("validated strategy plan");
            authority
                .execute_validated_commit(validated)
                .expect("validated strategy commit executed")
        };

        let current = runtime
            .visibility_reads()
            .read_version(runtime.current_version_id());
        assert_eq!(
            read_entity_name(current.get_entity(entity).expect("committed entity")),
            Some("after")
        );
        assert_eq!(
            commit
                .publication
                .strategy_artifacts
                .as_ref()
                .expect("strategy artifacts")
                .merge_descriptor()
                .semantic_name()
                .as_str(),
            IntentReconciliationStrategy::DEFAULT_SEMANTIC_NAME
        );

        let replay = runtime
            .replay_authority()
            .replay_commit(RelationalReplayRequest {
                commit_id: commit.commit.commit_id,
                branch_id: commit.publication.envelope.branch_context.clone(),
                execution_mode: ReplayExecutionMode::SerialDeterministic,
                verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
            });

        assert!(
            replay.failure.is_none(),
            "unexpected replay failure: {replay:?}"
        );
        assert!(replay
            .compared_surfaces
            .contains(&ReplayObservableSurface::Strategy));
        assert!(replay
            .mismatches
            .iter()
            .all(|mismatch| mismatch.surface != ReplayObservableSurface::Strategy));
    }

    #[test]
    fn merge_planning_classifies_different_strategy_families_as_strategy_intent_conflict() {
        let main_descriptor = strategy_descriptor_named(
            CommitStrategyId(41),
            "strategy.intent.reconcile",
            "strategy.intent",
            "reconcile.desired.state",
        );
        let feature_descriptor = strategy_descriptor_named(
            CommitStrategyId(42),
            "strategy.replica.converge",
            "strategy.replica",
            "replica.desired.state",
        );
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(crate::tests::support::test_schema_registry())
            .commit_strategy(
                CommitStrategyRegistration::new(main_descriptor.clone())
                    .expect("main strategy registration"),
            )
            .commit_strategy(
                CommitStrategyRegistration::new(feature_descriptor.clone())
                    .expect("feature strategy registration"),
            )
            .build();
        let entity = crate::tests::support::create_entity(&mut runtime, "shared");
        let feature_branch =
            crate::tests::support::create_branch_from_main(&mut runtime, "feature");

        {
            let request = CanonicalStrategyCommitRequest::new(
                main_descriptor.id(),
                main_descriptor.digest(),
                CanonicalStrategyInputArtifact::new(
                    StrategyInputSchemaName::new("intent.reconcile.input.v1"),
                    StrategyInputSchemaVersion(1),
                    StrategyRequestCanonicalization::JsonStableObjectOrderV1,
                    br#"{"name":"main-strategy"}"#.to_vec().into(),
                    CanonicalStrategyInputDigest([11; 32]),
                    PersistentArtifactName::new("strategy.intent.reconcile.input"),
                ),
                StrategyCallerProvenance {
                    request_origin: StrategyRequestOrigin::Test,
                    actor_identity: None,
                    correlation_id: None,
                },
            );
            let execution = update_execution_draft(&request, entity, "main-strategy");
            let mut authority = runtime.commit_strategies_authority();
            let lowered = authority
                .lower_execution(&request, &execution, TransactionOptions::default())
                .expect("lowered main strategy plan");
            let validated = authority
                .validate_lowered_plan(lowered)
                .expect("validated main strategy plan");
            authority
                .execute_validated_commit(validated)
                .expect("main strategy commit");
        }

        {
            let request = CanonicalStrategyCommitRequest::new(
                feature_descriptor.id(),
                feature_descriptor.digest(),
                CanonicalStrategyInputArtifact::new(
                    StrategyInputSchemaName::new("intent.reconcile.input.v1"),
                    StrategyInputSchemaVersion(1),
                    StrategyRequestCanonicalization::JsonStableObjectOrderV1,
                    br#"{"name":"feature-strategy"}"#.to_vec().into(),
                    CanonicalStrategyInputDigest([12; 32]),
                    PersistentArtifactName::new("strategy.replica.converge.input"),
                ),
                StrategyCallerProvenance {
                    request_origin: StrategyRequestOrigin::Test,
                    actor_identity: None,
                    correlation_id: None,
                },
            );
            let execution = update_execution_draft(&request, entity, "feature-strategy");
            let mut authority = runtime.commit_strategies_authority();
            let lowered = authority
                .lower_execution(
                    &request,
                    &execution,
                    TransactionOptions {
                        target_branch: Some(feature_branch.clone()),
                        ..TransactionOptions::default()
                    },
                )
                .expect("lowered feature strategy plan");
            let validated = authority
                .validate_lowered_plan(lowered)
                .expect("validated feature strategy plan");
            authority
                .execute_validated_commit(validated)
                .expect("feature strategy commit");
        }

        let planning = runtime
            .merge_access()
            .inspect_planning_scope(MergePlanningRequest::new(
                BranchId("main".to_string()),
                feature_branch,
                MergeIntent::ReconcileIntoTarget,
            ))
            .expect("merge planning artifact");

        let classification = planning
            .conflict_classification
            .classifications
            .iter()
            .find(|classification| {
                classification.record == crate::facade::transactions::RecordRef::Entity(entity)
            })
            .expect("classified shared entity");

        assert_eq!(
            classification.class,
            crate::merge::data::MergeConflictClass::StrategyIntentConflict
        );
        let strategy_evidence = classification
            .strategy_evidence
            .as_ref()
            .expect("strategy conflict evidence");
        assert_eq!(
            strategy_evidence.class,
            crate::merge::data::StrategyConflictClass::DifferentStrategyOverlappingIntent
        );
        assert_eq!(strategy_evidence.source_descriptors.len(), 1);
        assert_eq!(strategy_evidence.target_descriptors.len(), 1);
        assert_eq!(
            planning
                .conflict_classification
                .strategy_intent_conflict_count,
            1
        );
    }

    #[test]
    fn merge_planning_with_real_strategies_preserves_strategy_specific_manual_boundary() {
        let intent_descriptor = IntentReconciliationStrategy::descriptor(CommitStrategyId(71));
        let replica_descriptor = ReplicaConvergenceStrategy::descriptor(CommitStrategyId(72));
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(crate::tests::support::test_schema_registry())
            .commit_strategy(
                CommitStrategyRegistration::new(intent_descriptor.clone())
                    .expect("intent strategy registration"),
            )
            .commit_strategy_executor(IntentReconciliationStrategy::execution_registration(
                &intent_descriptor,
            ))
            .commit_strategy(
                CommitStrategyRegistration::new(replica_descriptor.clone())
                    .expect("replica strategy registration"),
            )
            .commit_strategy_executor(ReplicaConvergenceStrategy::execution_registration(
                &replica_descriptor,
            ))
            .build();
        let entity = crate::tests::support::create_entity(&mut runtime, "shared");
        let feature_branch =
            crate::tests::support::create_branch_from_main(&mut runtime, "feature-real");

        {
            let request = runtime
                .commit_strategies()
                .canonicalize_request(
                    &crate::commit_strategies::data::RawStrategyCommitRequest::new(
                        crate::commit_strategies::data::CommitStrategySemanticName::new(
                            IntentReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
                        ),
                        serde_json::to_vec(&IntentReconciliationInput {
                            entity_id: entity,
                            desired_payload: json!({"name":"main-intent"}),
                        })
                        .expect("serialize intent input"),
                        StrategyCallerProvenance {
                            request_origin: StrategyRequestOrigin::Test,
                            actor_identity: None,
                            correlation_id: None,
                        },
                    ),
                )
                .expect("intent canonical request");
            let snapshot = runtime.visibility_authority().snapshot();
            let execution = runtime
                .commit_strategies()
                .execute(&request, &snapshot)
                .expect("intent execution");
            let mut authority = runtime.commit_strategies_authority();
            let lowered = authority
                .lower_execution(&request, &execution, TransactionOptions::default())
                .expect("lowered intent plan");
            let validated = authority
                .validate_lowered_plan(lowered)
                .expect("validated intent plan");
            authority
                .execute_validated_commit(validated)
                .expect("intent strategy commit");
        }

        {
            let request = runtime
                .commit_strategies()
                .canonicalize_request(
                    &crate::commit_strategies::data::RawStrategyCommitRequest::new(
                        crate::commit_strategies::data::CommitStrategySemanticName::new(
                            ReplicaConvergenceStrategy::DEFAULT_SEMANTIC_NAME,
                        ),
                        serde_json::to_vec(&ReplicaConvergenceInput {
                            entity_id: entity,
                            desired_replicas: 7,
                        })
                        .expect("serialize replica input"),
                        StrategyCallerProvenance {
                            request_origin: StrategyRequestOrigin::Test,
                            actor_identity: None,
                            correlation_id: None,
                        },
                    ),
                )
                .expect("replica canonical request");
            let snapshot = runtime.visibility_authority().snapshot();
            let execution = runtime
                .commit_strategies()
                .execute(&request, &snapshot)
                .expect("replica execution");
            let mut authority = runtime.commit_strategies_authority();
            let lowered = authority
                .lower_execution(
                    &request,
                    &execution,
                    TransactionOptions {
                        target_branch: Some(feature_branch.clone()),
                        ..TransactionOptions::default()
                    },
                )
                .expect("lowered replica plan");
            let validated = authority
                .validate_lowered_plan(lowered)
                .expect("validated replica plan");
            authority
                .execute_validated_commit(validated)
                .expect("replica strategy commit");
        }

        let lowered = runtime
            .merge_access()
            .inspect_planning_scope(MergePlanningRequest::new(
                BranchId("main".to_string()),
                feature_branch,
                MergeIntent::ManualReview,
            ))
            .expect("merge planning scope")
            .lowered_plan
            .expect("lowered merge plan");

        let record = lowered
            .lowered_records
            .iter()
            .find(|record| record.record == crate::transactions::data::RecordRef::Entity(entity))
            .expect("entity conflict record");

        assert_eq!(
            record.classification,
            crate::merge::data::MergeConflictClass::StrategyIntentConflict
        );
        assert_eq!(
            record.policy_proof_boundary.decision_boundary,
            crate::merge::data::MergePolicyDecisionBoundary::RequiresManualResolution {
                class: crate::merge::data::MergeManualResolutionClass::StrategyIntentConflict,
            }
        );
        assert_eq!(
            record.blocked_reason,
            Some(
                crate::merge::data::LoweredMergeBlockedReason::StrategyIntentConflictRequiresManualResolution
            )
        );
    }
}
