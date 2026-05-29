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
        AspectFieldReconciliationInput, AspectFieldReconciliationStrategy,
        EntityReplacementReconciliationInput, EntityReplacementReconciliationStrategy,
        IntentReconciliationInput, IntentReconciliationStrategy, ReplicaConvergenceInput,
        ReplicaConvergenceStrategy,
    };
    use crate::durability::data::DurableStoreLayout;
    use crate::facade::durability::DurabilityMode;
    use crate::facade::history::BranchId;
    use crate::facade::merge::{MergeIntent, MergePlanningRequest};
    use crate::facade::replay::{
        RelationalReplayRequest, ReplayExecutionMode, ReplayFailureClass, ReplayMismatchClass,
        ReplayObservableSurface, ReplayVerificationMode,
    };
    use crate::facade::transactions::{
        CreateIntent, EntityMutationIntent, MutationIntent, TransactionOptions,
        UpdateEntityFieldsIntent, WorkerIntentBatch,
    };
    use crate::identity::data::{EntityId, KindId, PartitionId};
    use crate::logic::builder::RelationalRuntimeBuilder;
    use crate::snapshots::data::SnapshotHandle;
    use crate::symbols::data::ClientKey;
    use crate::tests::support::{
        changed_entities, entity_field_aspect, entity_u64_field_aspect, lifecycle_aspect,
        read_entity_name, unique_test_store_path, AspectSchemaFixture,
    };
    use crate::transactions::data::{AspectFieldPatch, AspectFieldPatchTarget};
    use crate::transactions::data::{EntitySpec, TransactionCommitError};
    use forge_foundational::facade::{
        AspectFieldLocator, AspectKey, AspectValue, CanonicalFieldPath, FieldKey, InternedString,
        LocatorAuthority,
    };

    fn strategy_schema_registry() -> crate::schema::data::RelationalSchemaRegistry {
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

    fn strategy_name_and_replicas_patch(name: &str, replicas: u64) -> AspectFieldPatch {
        AspectFieldPatch::from(std::collections::BTreeMap::from([
            (
                AspectFieldPatchTarget::single(
                    AspectKey::new("name").expect("valid name aspect key"),
                    FieldKey::new("name").expect("valid name field key"),
                ),
                AspectValue::String(InternedString::Raw(name.to_string())),
            ),
            (
                AspectFieldPatchTarget::single(
                    AspectKey::new("replicas").expect("valid replicas aspect key"),
                    FieldKey::new("replicas").expect("valid replicas field key"),
                ),
                AspectValue::UInt64(replicas),
            ),
        ]))
    }

    fn strategy_field_locator(aspect_key: AspectKey, field_key: FieldKey) -> AspectFieldLocator {
        AspectFieldLocator::new(
            LocatorAuthority::Planned,
            aspect_key,
            CanonicalFieldPath::single(field_key),
        )
    }

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
            StrategyRequestCanonicalization::NativeCanonicalBytesV1,
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

    #[derive(Clone, Copy)]
    struct DeterministicFailureExecutor;

    impl CommitStrategyExecutor for DeterministicFailureExecutor {
        fn execute(
            &self,
            _request: &CanonicalStrategyCommitRequest,
            _observation: &crate::commit_strategies::data::StrategyObservationContext<'_>,
        ) -> Result<StrategyExecutionResult, StrategyExecutorFailure> {
            Err(StrategyExecutorFailure::new(
                crate::commit_strategies::data::StrategyExecutorFailureClass::DomainRejection,
                "deterministic hostile replay failure",
            ))
        }
    }

    fn persisted_intent_runtime(
        root_path: std::path::PathBuf,
        include_executor: bool,
    ) -> crate::facade::runtime::RelationalRuntime {
        let descriptor = IntentReconciliationStrategy::descriptor(CommitStrategyId(161));
        let mut builder = RelationalRuntimeBuilder::new()
            .schema_registry(strategy_schema_registry())
            .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
            .durable_store_layout(DurableStoreLayout {
                root_path,
                segment_commit_capacity: 2,
            })
            .commit_strategy(
                CommitStrategyRegistration::new(descriptor.clone())
                    .expect("intent strategy registration"),
            );
        if include_executor {
            builder = builder.commit_strategy_executor(
                IntentReconciliationStrategy::execution_registration(&descriptor),
            );
        }
        builder.build()
    }

    fn persisted_intent_runtime_with_failing_executor(
        root_path: std::path::PathBuf,
    ) -> crate::facade::runtime::RelationalRuntime {
        let descriptor = IntentReconciliationStrategy::descriptor(CommitStrategyId(161));
        RelationalRuntimeBuilder::new()
            .schema_registry(strategy_schema_registry())
            .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
            .durable_store_layout(DurableStoreLayout {
                root_path,
                segment_commit_capacity: 2,
            })
            .commit_strategy(
                CommitStrategyRegistration::new(descriptor.clone())
                    .expect("intent strategy registration"),
            )
            .commit_strategy_executor(CommitStrategyExecutionRegistration::new(
                &descriptor,
                DeterministicFailureExecutor,
            ))
            .build()
    }

    fn execute_persisted_intent_strategy_commit(
        runtime: &mut crate::facade::runtime::RelationalRuntime,
        entity: EntityId,
    ) -> crate::facade::transactions::CommitResult {
        let request = runtime
            .commit_strategies()
            .canonicalize_request(
                &IntentReconciliationInput {
                    entity_id: entity,
                    desired_fields: crate::transactions::data::AspectFieldPatch::single(
                        forge_foundational::facade::AspectKey::new("name")
                            .expect("valid test aspect key"),
                        FieldKey::new("name").expect("valid test field key"),
                        forge_foundational::facade::AspectValue::String(
                            forge_foundational::facade::InternedString::Raw("after".to_string()),
                        ),
                    ),
                }
                .into_native_canonical_request(StrategyCallerProvenance {
                    request_origin: StrategyRequestOrigin::Test,
                    actor_identity: None,
                    correlation_id: None,
                })
                .expect("native canonical strategy request"),
            )
            .expect("canonical request");
        let snapshot = runtime.visibility_authority().snapshot();
        let execution = runtime
            .commit_strategies()
            .execute(&request, &snapshot)
            .expect("strategy execution");
        let mut authority = runtime.commit_strategies_authority();
        let lowered = authority
            .lower_execution(&request, &execution, TransactionOptions::default())
            .expect("lowered strategy plan");
        let validated = authority
            .validate_lowered_plan(lowered)
            .expect("validated strategy plan");
        authority
            .execute_validated_commit(validated)
            .expect("validated strategy commit")
    }

    fn canonical_request() -> CanonicalStrategyCommitRequest {
        let descriptor = strategy_descriptor();
        CanonicalStrategyCommitRequest::new(
            CommitStrategyId(41),
            descriptor.digest(),
            CanonicalStrategyInputArtifact::new(
                StrategyInputSchemaName::new("intent.reconcile.input.v1"),
                StrategyInputSchemaVersion(1),
                StrategyRequestCanonicalization::NativeCanonicalBytesV1,
                b"fixture-input".to_vec().into(),
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
                client_key: ClientKey::from("deployment-a"),
                fields: strategy_name_and_replicas_patch("deployment-a", 3),
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
            EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                entity_id,
                fields: crate::transactions::data::AspectFieldPatch::single(
                    AspectKey::new("name").expect("valid name aspect key"),
                    FieldKey::new("name").expect("valid name field key"),
                    AspectValue::String(InternedString::Raw(name.to_string())),
                ),
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
            strategy_artifacts
                .merge_descriptor()
                .merge_semantics()
                .conflict_class(),
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

    #[test]
    fn replay_commit_certifies_strategy_surface_for_strategy_bearing_commit() {
        let descriptor = strategy_descriptor();
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(strategy_schema_registry())
            .commit_strategy(strategy_registration())
            .commit_strategy_executor(CommitStrategyExecutionRegistration::new(
                &descriptor,
                PlanningExecutor,
            ))
            .build();
        let request = runtime
            .commit_strategies()
            .canonicalize_request(
                &crate::commit_strategies::data::RawStrategyCommitRequest::from_canonical_bytes(
                    CommitStrategySemanticName::new("strategy.intent.reconcile"),
                    b"fixture-input".to_vec(),
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
            .schema_registry(strategy_schema_registry())
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
                &IntentReconciliationInput {
                    entity_id: entity,
                    desired_fields: crate::transactions::data::AspectFieldPatch::single(
                        forge_foundational::facade::AspectKey::new("name")
                            .expect("valid test aspect key"),
                        FieldKey::new("name").expect("valid test field key"),
                        forge_foundational::facade::AspectValue::String(
                            forge_foundational::facade::InternedString::Raw("after".to_string()),
                        ),
                    ),
                }
                .into_native_canonical_request(StrategyCallerProvenance {
                    request_origin: StrategyRequestOrigin::Test,
                    actor_identity: None,
                    correlation_id: None,
                })
                .expect("native canonical strategy request"),
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
            .read_truth()
            .read_version(runtime.current_version_id());
        assert_eq!(
            read_entity_name(current.get_entity(entity).expect("committed entity")),
            Some("after".into())
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
    fn entity_replacement_reconciliation_strategy_commits_lineage_sensitive_replace_and_replays() {
        let descriptor = EntityReplacementReconciliationStrategy::descriptor(CommitStrategyId(62));
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(strategy_schema_registry())
            .commit_strategy(
                CommitStrategyRegistration::new(descriptor.clone())
                    .expect("replacement strategy registration"),
            )
            .commit_strategy_executor(
                EntityReplacementReconciliationStrategy::execution_registration(&descriptor),
            )
            .build();
        let entity = crate::tests::support::create_entity(&mut runtime, "before");
        let original_lineage = runtime
            .lineage_access()
            .for_record(entity)
            .expect("original lineage")
            .lineage_id;
        let request = runtime
            .commit_strategies()
            .canonicalize_request(
                &EntityReplacementReconciliationInput {
                    entity_id: entity,
                    replacement_client_key: "service-replacement".to_string(),
                    desired_fields: strategy_name_and_replicas_patch("before", 3),
                }
                .into_native_canonical_request(StrategyCallerProvenance {
                    request_origin: StrategyRequestOrigin::Test,
                    actor_identity: None,
                    correlation_id: None,
                })
                .expect("native canonical strategy request"),
            )
            .expect("canonical replacement request");
        let snapshot: SnapshotHandle = runtime.visibility_authority().snapshot();
        let execution = runtime
            .commit_strategies()
            .execute(&request, &snapshot)
            .expect("replacement strategy executes against committed basis");
        let commit = {
            let mut authority = runtime.commit_strategies_authority();
            let lowered = authority
                .lower_execution(&request, &execution, TransactionOptions::default())
                .expect("lowered replacement strategy plan");
            let validated = authority
                .validate_lowered_plan(lowered)
                .expect("validated replacement strategy plan");
            authority
                .execute_validated_commit(validated)
                .expect("validated replacement strategy commit executed")
        };
        let current = runtime
            .read_truth()
            .read_version(runtime.current_version_id());
        let replacement_record = changed_entities(&commit)
            .into_iter()
            .find_map(|entity_id| current.get_entity(entity_id).cloned())
            .expect("replacement entity visible");
        let replacement_lineage = runtime
            .lineage_access()
            .for_record(replacement_record.entity_id)
            .expect("replacement lineage")
            .lineage_id;
        let strategy_artifacts = commit
            .publication
            .strategy_artifacts
            .as_ref()
            .expect("replacement strategy artifacts");

        assert_ne!(original_lineage, replacement_lineage);
        assert_eq!(read_entity_name(&replacement_record), Some("before".into()));
        let expected_replicas_key =
            crate::storage::data::authoritative_aspect_value_field_comparison_key(
                &AspectValue::UInt64(3),
            );
        let replicas_locator = AspectFieldLocator::new(
            LocatorAuthority::Planned,
            AspectKey::new("replicas").expect("valid replicas aspect"),
            CanonicalFieldPath::single(FieldKey::new("replicas").expect("valid replicas field")),
        );
        assert_eq!(
            crate::storage::data::entity_authoritative_aspect_field_comparison_key(
                &replacement_record,
                &replicas_locator
            ),
            Some(expected_replicas_key)
        );
        assert_eq!(
            strategy_artifacts
                .lowering_summary()
                .normalized_client_key_count(),
            1
        );
        assert_eq!(
            strategy_artifacts
                .lowering_summary()
                .lineage_transition_count(),
            1
        );
        assert!(commit
            .publication
            .envelope
            .lineage_decision_log()
            .iter()
            .any(|decision| decision.kind
                == crate::lineage::data::LineageDecisionKind::ReplaceAccepted));

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
            "unexpected replacement replay failure: {replay:?}"
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
            "strategy.aspect.field.reconcile",
            "strategy.aspect",
            "aspect.scalar.field.reconcile",
        );
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(strategy_schema_registry())
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
            let request = runtime
                .commit_strategies()
                .canonicalize_request(
                    &IntentReconciliationInput {
                        entity_id: entity,
                        desired_fields: AspectFieldPatch::single(
                            forge_foundational::facade::AspectKey::new("name")
                                .expect("valid test aspect key"),
                            FieldKey::new("name").expect("valid test field key"),
                            forge_foundational::facade::AspectValue::String("main-strategy".into()),
                        ),
                    }
                    .into_native_canonical_request(StrategyCallerProvenance {
                        request_origin: StrategyRequestOrigin::Test,
                        actor_identity: None,
                        correlation_id: None,
                    })
                    .expect("raw main strategy request"),
                )
                .expect("main canonical request");
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
            let request = runtime
                .commit_strategies()
                .canonicalize_request(
                    &AspectFieldReconciliationInput {
                        entity_id: entity,
                        field_locator: strategy_field_locator(
                            crate::tests::support::aspect_key("name"),
                            crate::tests::support::field_key("name"),
                        ),
                        desired_value: forge_foundational::facade::AspectValue::String(
                            "feature-strategy".into(),
                        ),
                    }
                    .into_native_canonical_request(StrategyCallerProvenance {
                        request_origin: StrategyRequestOrigin::Test,
                        actor_identity: None,
                        correlation_id: None,
                    })
                    .expect("raw feature strategy request"),
                )
                .expect("feature canonical request");
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
            .merge()
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
            .schema_registry(strategy_schema_registry())
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
                    &IntentReconciliationInput {
                        entity_id: entity,
                        desired_fields: strategy_name_and_replicas_patch("main-intent", 1),
                    }
                    .into_native_canonical_request(StrategyCallerProvenance {
                        request_origin: StrategyRequestOrigin::Test,
                        actor_identity: None,
                        correlation_id: None,
                    })
                    .expect("native canonical strategy request"),
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
                    &ReplicaConvergenceInput {
                        entity_id: entity,
                        desired_replicas: 7,
                    }
                    .into_native_canonical_request(StrategyCallerProvenance {
                        request_origin: StrategyRequestOrigin::Test,
                        actor_identity: None,
                        correlation_id: None,
                    })
                    .expect("native canonical strategy request"),
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
            .merge()
            .inspect_planning_scope(MergePlanningRequest::new(
                BranchId("main".to_string()),
                feature_branch,
                MergeIntent::ReconcileIntoTarget,
            ))
            .expect("merge planning scope");

        let classification_index = lowered
            .conflict_classification
            .classifications
            .iter()
            .position(|classification| {
                classification.record == crate::transactions::data::RecordRef::Entity(entity)
            })
            .expect("entity conflict classification index");
        let policy_record = lowered
            .policy_resolution
            .records
            .iter()
            .find(|record| record.record == crate::transactions::data::RecordRef::Entity(entity))
            .expect("entity policy record");

        assert_eq!(
            lowered.conflict_classification.classifications[classification_index].class,
            crate::merge::data::MergeConflictClass::StrategyIntentConflict
        );
        assert_eq!(
            policy_record.proof_boundary.decision_boundary,
            crate::merge::data::MergePolicyDecisionBoundary::RequiresManualResolution {
                class: crate::merge::data::MergeManualResolutionClass::StrategyIntentConflict,
            }
        );
        assert!(lowered.digest_basis.lowered_plan.blocked_reasons[classification_index].is_some());
    }

    #[test]
    fn merge_planning_distinguishes_disjoint_aspect_intent_from_strategy_intent_conflict() {
        let aspect_descriptor = AspectFieldReconciliationStrategy::descriptor(CommitStrategyId(91));
        let replica_descriptor = ReplicaConvergenceStrategy::descriptor(CommitStrategyId(92));
        let registry = AspectSchemaFixture {
            cascade_delete_policy: crate::config::data::CascadeDeletePolicy::CascadeDeleteRelations,
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
        .build_registry();
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(registry)
            .commit_strategy(
                CommitStrategyRegistration::new(aspect_descriptor.clone())
                    .expect("aspect strategy registration"),
            )
            .commit_strategy_executor(AspectFieldReconciliationStrategy::execution_registration(
                &aspect_descriptor,
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
            crate::tests::support::create_branch_from_main(&mut runtime, "feature-aspects");

        {
            let request = runtime
                .commit_strategies()
                .canonicalize_request(
                    &AspectFieldReconciliationInput {
                        entity_id: entity,
                        field_locator: strategy_field_locator(
                            crate::tests::support::aspect_key("name"),
                            crate::tests::support::field_key("name"),
                        ),
                        desired_value: forge_foundational::facade::AspectValue::String(
                            "main-name".into(),
                        ),
                    }
                    .into_native_canonical_request(StrategyCallerProvenance {
                        request_origin: StrategyRequestOrigin::Test,
                        actor_identity: None,
                        correlation_id: None,
                    })
                    .expect("native canonical strategy request"),
                )
                .expect("aspect canonical request");
            let snapshot = runtime.visibility_authority().snapshot();
            let execution = runtime
                .commit_strategies()
                .execute(&request, &snapshot)
                .expect("aspect execution");
            let mut authority = runtime.commit_strategies_authority();
            let lowered = authority
                .lower_execution(&request, &execution, TransactionOptions::default())
                .expect("lowered aspect plan");
            let validated = authority
                .validate_lowered_plan(lowered)
                .expect("validated aspect plan");
            authority
                .execute_validated_commit(validated)
                .expect("aspect strategy commit");
        }

        {
            let request = runtime
                .commit_strategies()
                .canonicalize_request(
                    &ReplicaConvergenceInput {
                        entity_id: entity,
                        desired_replicas: 7,
                    }
                    .into_native_canonical_request(StrategyCallerProvenance {
                        request_origin: StrategyRequestOrigin::Test,
                        actor_identity: None,
                        correlation_id: None,
                    })
                    .expect("native canonical strategy request"),
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

        let planning = runtime
            .merge()
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
                classification.record == crate::transactions::data::RecordRef::Entity(entity)
            })
            .expect("classified shared entity");

        assert_ne!(
            classification.class,
            crate::merge::data::MergeConflictClass::StrategyIntentConflict
        );
        assert!(
            classification.strategy_evidence.is_none(),
            "disjoint declared aspect intent should not synthesize strategy conflict evidence: {classification:?}"
        );
    }

    #[test]
    fn merge_planning_classifies_same_declared_aspect_field_as_strategy_intent_conflict() {
        let aspect_descriptor = AspectFieldReconciliationStrategy::descriptor(CommitStrategyId(93));
        let registry = AspectSchemaFixture {
            cascade_delete_policy: crate::config::data::CascadeDeletePolicy::CascadeDeleteRelations,
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
        .build_registry();
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(registry)
            .commit_strategy(
                CommitStrategyRegistration::new(aspect_descriptor.clone())
                    .expect("aspect strategy registration"),
            )
            .commit_strategy_executor(AspectFieldReconciliationStrategy::execution_registration(
                &aspect_descriptor,
            ))
            .build();
        let entity = crate::tests::support::create_entity(&mut runtime, "shared");
        let feature_branch =
            crate::tests::support::create_branch_from_main(&mut runtime, "feature-same-aspect");

        for (branch, desired_value) in [
            (None, "main-name"),
            (Some(feature_branch.clone()), "feature-name"),
        ] {
            let request = runtime
                .commit_strategies()
                .canonicalize_request(
                    &AspectFieldReconciliationInput {
                        entity_id: entity,
                        field_locator: strategy_field_locator(
                            crate::tests::support::aspect_key("name"),
                            crate::tests::support::field_key("name"),
                        ),
                        desired_value: forge_foundational::facade::AspectValue::String(
                            desired_value.into(),
                        ),
                    }
                    .into_native_canonical_request(StrategyCallerProvenance {
                        request_origin: StrategyRequestOrigin::Test,
                        actor_identity: None,
                        correlation_id: None,
                    })
                    .expect("native canonical strategy request"),
                )
                .expect("aspect canonical request");
            let snapshot = runtime.visibility_authority().snapshot();
            let execution = runtime
                .commit_strategies()
                .execute(&request, &snapshot)
                .expect("aspect execution");
            let mut authority = runtime.commit_strategies_authority();
            let lowered = authority
                .lower_execution(
                    &request,
                    &execution,
                    TransactionOptions {
                        target_branch: branch,
                        ..TransactionOptions::default()
                    },
                )
                .expect("lowered aspect plan");
            let validated = authority
                .validate_lowered_plan(lowered)
                .expect("validated aspect plan");
            authority
                .execute_validated_commit(validated)
                .expect("aspect strategy commit");
        }

        let planning = runtime
            .merge()
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
                classification.record == crate::transactions::data::RecordRef::Entity(entity)
            })
            .expect("classified shared entity");

        assert_eq!(
            classification.class,
            crate::merge::data::MergeConflictClass::StrategyIntentConflict
        );
        assert_eq!(
            classification
                .strategy_evidence
                .as_ref()
                .expect("strategy conflict evidence")
                .class,
            crate::merge::data::StrategyConflictClass::SameStrategyDivergentOutput
        );
    }

    #[test]
    fn replay_commit_reports_strategy_executor_unavailable_when_recovered_runtime_lacks_executor() {
        let root_path = unique_test_store_path("forge-relational-strategy-replay-missing-executor");
        let mut runtime = persisted_intent_runtime(root_path.clone(), true);
        let entity = crate::tests::support::create_entity(&mut runtime, "before");
        let commit = execute_persisted_intent_strategy_commit(&mut runtime, entity);
        let branch_head_before = runtime
            .history()
            .branch_head(&BranchId("main".to_string()))
            .cloned();
        let mut recovery_plan = runtime.durability().recovery_plan(
            crate::durability::data::RecoveryVerificationMode::AuditRecoveryVerification,
        );
        recovery_plan.commit_strategy_executors = Default::default();

        let mut recovered = persisted_intent_runtime(root_path, false);
        recovered
            .durability_authority()
            .recover(recovery_plan)
            .expect("recovery without strategy executor");
        let branch_head_after_recovery = recovered
            .history()
            .branch_head(&BranchId("main".to_string()))
            .cloned();

        let replay = recovered
            .replay_authority()
            .replay_commit(RelationalReplayRequest {
                commit_id: commit.commit.commit_id,
                branch_id: BranchId("main".to_string()),
                execution_mode: ReplayExecutionMode::SerialDeterministic,
                verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
            });

        assert_eq!(replay.failure, Some(ReplayFailureClass::ObservableMismatch));
        assert!(replay
            .compared_surfaces
            .contains(&ReplayObservableSurface::Strategy));
        assert!(replay.mismatches.iter().any(|mismatch| {
            mismatch.class == ReplayMismatchClass::StrategyExecutorUnavailable
                && mismatch.surface == ReplayObservableSurface::Strategy
        }));
        assert_eq!(
            recovered
                .history()
                .branch_head(&BranchId("main".to_string()))
                .cloned(),
            branch_head_after_recovery
        );
        assert_eq!(branch_head_after_recovery, branch_head_before);
    }

    #[test]
    fn replay_commit_reports_strategy_execution_failure_when_recovered_executor_rejects() {
        let root_path = unique_test_store_path("forge-relational-strategy-replay-failing-executor");
        let mut runtime = persisted_intent_runtime(root_path.clone(), true);
        let entity = crate::tests::support::create_entity(&mut runtime, "before");
        let commit = execute_persisted_intent_strategy_commit(&mut runtime, entity);
        let mut recovery_plan = runtime.durability().recovery_plan(
            crate::durability::data::RecoveryVerificationMode::AuditRecoveryVerification,
        );

        let mut recovered = persisted_intent_runtime_with_failing_executor(root_path);
        recovery_plan.commit_strategy_executors =
            recovered.commit_strategy_executor_registry().clone();
        recovered
            .durability_authority()
            .recover(recovery_plan)
            .expect("recovery with hostile failing executor");
        let branch_head_before_replay = recovered
            .history()
            .branch_head(&BranchId("main".to_string()))
            .cloned();

        let replay = recovered
            .replay_authority()
            .replay_commit(RelationalReplayRequest {
                commit_id: commit.commit.commit_id,
                branch_id: BranchId("main".to_string()),
                execution_mode: ReplayExecutionMode::SerialDeterministic,
                verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
            });

        assert_eq!(replay.failure, Some(ReplayFailureClass::ObservableMismatch));
        assert!(replay
            .compared_surfaces
            .contains(&ReplayObservableSurface::Strategy));
        assert!(replay.mismatches.iter().any(|mismatch| {
            mismatch.class == ReplayMismatchClass::StrategyExecutionFailure
                && mismatch.surface == ReplayObservableSurface::Strategy
        }));
        assert_eq!(
            recovered
                .history()
                .branch_head(&BranchId("main".to_string()))
                .cloned(),
            branch_head_before_replay
        );
    }
}
