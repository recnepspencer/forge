use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::InvariantCostClass;
use crate::validation::data::InvariantGroupSet;
use crate::validation::engine::{
    HarnessAuditMode, InvariantEngine, InvariantExecutionDisposition, InvariantExecutionMetadata,
    InvariantExecutionRequest, InvariantExecutionResult, InvariantObservation,
    InvariantObservationKind, InvariantRequestProfile,
};

impl RelationalRuntime {
    pub fn invariant_access(&self) -> InvariantAccess<'_> {
        InvariantAccess::new(self)
    }
}

pub struct InvariantAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> InvariantAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn harness_audit(&self, mode: HarnessAuditMode) -> InvariantExecutionResult {
        mode.request_profile().map_or_else(
            || {
                InvariantExecutionResult::skipped(self.execution_metadata(
                    InvariantRequestProfile::HarnessAudit,
                    InvariantObservationKind::Committed,
                    self.runtime.current_version_id(),
                    None,
                    None,
                    InvariantGroupSet::empty(),
                    InvariantCostClass::Global,
                    InvariantExecutionDisposition::SkippedByMayBreakMask,
                ))
            },
            |profile| self.execute_for_runtime(profile),
        )
    }

    pub fn mutation_sensitive_state(&self) -> InvariantExecutionResult {
        self.execute_for_runtime(InvariantRequestProfile::MutationSensitive)
    }

    pub fn snapshot_publication_state(&self) -> InvariantExecutionResult {
        self.execute_for_runtime(InvariantRequestProfile::SnapshotPublication)
    }

    pub fn certification_state(&self) -> InvariantExecutionResult {
        self.execute_for_runtime(InvariantRequestProfile::CertificationBoundary)
    }

    pub(crate) fn mutation_sensitive_for_state(
        &self,
        state: crate::storage::overlay::OverlayStateView<
            'runtime,
            crate::logic::runtime::WorkingState,
        >,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&MergedCommitPlan>,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            InvariantRequestProfile::MutationSensitive,
            InvariantObservation::speculative(state),
            version_id,
            merged_plan,
        )
    }

    pub(crate) fn commit_boundary(
        &self,
        merged_plan: &MergedCommitPlan,
    ) -> InvariantExecutionResult {
        self.execute_for_runtime_plan(InvariantRequestProfile::CommitBoundary, merged_plan)
    }

    pub(crate) fn snapshot_publication_for_state(
        &self,
        state: crate::storage::overlay::OverlayStateView<
            'runtime,
            crate::logic::runtime::WorkingState,
        >,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&MergedCommitPlan>,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            InvariantRequestProfile::SnapshotPublication,
            InvariantObservation::speculative(state),
            version_id,
            merged_plan,
        )
    }

    fn execute_for_runtime(&self, profile: InvariantRequestProfile) -> InvariantExecutionResult {
        self.execute_for_state(
            profile,
            InvariantObservation::committed(self.runtime.storage_access().current_state()),
            self.runtime.current_version_id(),
            None,
        )
    }

    fn execute_for_runtime_plan(
        &self,
        profile: InvariantRequestProfile,
        merged_plan: &'runtime MergedCommitPlan,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            profile,
            InvariantObservation::committed(self.runtime.storage_access().current_state()),
            self.runtime.current_version_id(),
            Some(merged_plan),
        )
    }

    fn execute_for_state(
        &self,
        profile: InvariantRequestProfile,
        observation: InvariantObservation<'runtime>,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
    ) -> InvariantExecutionResult {
        let plan_contract =
            merged_plan.map(crate::validation::data::InvariantPlanContract::from_merged_plan);
        let consumed_groups = profile.consumed_groups();
        let observation_kind = observation.kind();
        if plan_contract
            .is_some_and(|contract| !contract.intersects_consumed_groups(consumed_groups))
        {
            return InvariantExecutionResult::skipped(self.execution_metadata(
                profile,
                observation_kind,
                version_id,
                merged_plan,
                plan_contract,
                InvariantGroupSet::empty(),
                InvariantCostClass::Global,
                InvariantExecutionDisposition::SkippedByPlanContract,
            ));
        }

        let request = InvariantExecutionRequest::from_profile_with_contract(
            profile,
            self.runtime,
            observation,
            version_id,
            merged_plan,
            plan_contract,
        );
        if let Some(preparation_violation) = request.preparation_violation().cloned() {
            return InvariantExecutionResult::executed(
                self.execution_metadata(
                    profile,
                    observation_kind,
                    version_id,
                    merged_plan,
                    plan_contract,
                    request.applicable_groups(),
                    request.max_cost(),
                    InvariantExecutionDisposition::Executed,
                ),
                vec![crate::validation::data::InvariantCheckResult {
                    execution_point: profile.execution_point(),
                    failure_effect: crate::validation::data::InvariantFailureEffect::BlockCommit,
                    rule: crate::validation::data::InvariantReportedRule::Native(
                        crate::validation::data::InvariantRule::RelationIntegrityScopeBudget(
                            self.runtime
                                .config
                                .execution
                                .relation_integrity_scope_budget
                                .max_planned_edges,
                        ),
                    ),
                    groups: crate::validation::data::InvariantGroupSet::of(
                        crate::validation::data::InvariantGroup::RelationIntegrity,
                    )
                    .union(crate::validation::data::InvariantGroupSet::of(
                        crate::validation::data::InvariantGroup::PublicationCoherence,
                    )),
                    witness: preparation_violation.witness_key(),
                    cost: crate::validation::data::InvariantCostClass::Touched,
                    custom_provenance: None,
                    verdict: crate::validation::data::InvariantVerdict::Violation(
                        preparation_violation,
                    ),
                }],
            );
        }
        if !request.should_execute_anything() {
            return InvariantExecutionResult::skipped(self.execution_metadata(
                profile,
                observation_kind,
                version_id,
                merged_plan,
                plan_contract,
                request.applicable_groups(),
                request.max_cost(),
                InvariantExecutionDisposition::SkippedByMayBreakMask,
            ));
        }
        InvariantEngine::new(self.runtime).execute(request)
    }

    fn execution_metadata(
        &self,
        profile: InvariantRequestProfile,
        observation_kind: InvariantObservationKind,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
        plan_contract: Option<crate::validation::data::InvariantPlanContract>,
        applicable_groups: InvariantGroupSet,
        max_cost: InvariantCostClass,
        disposition: InvariantExecutionDisposition,
    ) -> InvariantExecutionMetadata {
        InvariantExecutionMetadata::new(
            profile.execution_point(),
            observation_kind,
            version_id,
            self.runtime.current_version_id(),
            profile.consumed_groups(),
            applicable_groups,
            max_cost,
            disposition,
            plan_contract,
            merged_plan.is_some(),
            self.runtime.config.execution.execution_model,
            None,
            Vec::new(),
            None,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::InvariantAccess;
    use crate::authority::commit::preparation::planning::strategy::PreparationStrategySelection;
    use crate::capabilities::SchemaSource;
    use crate::config::data::{
        CascadeDeletePolicy, CrossContextPolicy, RelationIntegrityScopeBudget,
    };
    use crate::facade::identity::PartitionId;
    use crate::facade::runtime::{
        InvariantCatalog, InvariantRegistration, InvariantRule, RelationalExecutionModel,
    };
    use crate::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
    use crate::facade::schema::{
        EntityKindRegistration, KindAspectDeclarations, RelationKindRegistration,
        RelationalSchemaRegistry, SchemaId, SchemaVersionId,
    };
    use crate::identity::data::KindId;
    use crate::payloads::data::RecordPayload;
    use crate::schema::data::{
        CardinalityContractDeclaration, EndpointKindContractDeclaration,
        RelationIntegrityDeclarations, RelationPayloadClass, SymmetryContractDeclaration,
        SymmetryMode,
    };
    use crate::symbols::data::InternedString;
    use crate::transactions::data::{
        BulkRelationCreateIntent, CreateIntent, DeleteRelationIntent, EntitySpec, MergedCommitPlan,
        MutationIntent, RelationMutationIntent, TransactionId,
    };
    use crate::validation::data::{InvariantFailureEffect, InvariantVerdict};
    use crate::validation::engine::InvariantPlanScopeClass;
    use serde_json::json;

    fn runtime_with_invariants(
        invariant_catalog: InvariantCatalog,
        execution_model: RelationalExecutionModel,
    ) -> RelationalRuntime {
        RelationalRuntimeApi::builder()
            .schema_registry(RelationalSchemaRegistry::new())
            .invariant_catalog(invariant_catalog)
            .execution_model(execution_model)
            .build()
    }

    fn relation_integrity_runtime() -> RelationalRuntime {
        let registry = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: KindId(1),
                kind_name: "test.entity".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::default(),
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: KindId(2),
                    kind_name: "test.relation".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    payload_class: RelationPayloadClass::PayloadBearingRelation,
                    cross_context_policy: CrossContextPolicy::AllowExplicit,
                    cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                    aspect_declarations: KindAspectDeclarations::default(),
                    relation_integrity: RelationIntegrityDeclarations::new(
                        vec![EndpointKindContractDeclaration {
                            contract_id: "no_self".into(),
                            allowed_source_kinds: vec![KindId(1)],
                            allowed_target_kinds: vec![KindId(1)],
                            self_edges_allowed: false,
                            cross_context_policy: CrossContextPolicy::AllowExplicit,
                        }],
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                    ),
                })
            })
            .unwrap();
        RelationalRuntimeApi::builder()
            .schema_registry(registry)
            .build()
    }

    fn relation_symmetry_runtime(mode: SymmetryMode) -> RelationalRuntime {
        let registry = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: KindId(1),
                kind_name: "test.entity".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::default(),
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: KindId(2),
                    kind_name: "test.relation".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    payload_class: RelationPayloadClass::PayloadBearingRelation,
                    cross_context_policy: CrossContextPolicy::AllowExplicit,
                    cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                    aspect_declarations: KindAspectDeclarations::default(),
                    relation_integrity: RelationIntegrityDeclarations::new(
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        vec![SymmetryContractDeclaration {
                            contract_id: "paired_twin".into(),
                            mode,
                        }],
                        Vec::new(),
                    ),
                })
            })
            .unwrap();
        RelationalRuntimeApi::builder()
            .schema_registry(registry)
            .build()
    }

    fn relation_cardinality_runtime() -> RelationalRuntime {
        let registry = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: KindId(1),
                kind_name: "test.entity".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::default(),
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: KindId(2),
                    kind_name: "test.relation".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    payload_class: RelationPayloadClass::PayloadBearingRelation,
                    cross_context_policy: CrossContextPolicy::AllowExplicit,
                    cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                    aspect_declarations: KindAspectDeclarations::default(),
                    relation_integrity: RelationIntegrityDeclarations::new(
                        Vec::new(),
                        vec![CardinalityContractDeclaration {
                            contract_id: "source_max_one".into(),
                            source_max: Some(1),
                            source_min: None,
                            target_max: None,
                            target_min: None,
                            pair_max: None,
                            pair_min: None,
                            pair_min_semantics:
                                crate::schema::data::PairMinimumSemantics::ObservedDirectedPairs,
                            minimum_enforcement:
                                crate::schema::data::MinimumCardinalityEnforcement::CertificationBoundary,
                        }],
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                    ),
                })
            })
            .unwrap();
        RelationalRuntimeApi::builder()
            .schema_registry(registry)
            .build()
    }

    fn relation_integrity_runtime_with_scope_budget(
        relation_integrity_scope_budget: RelationIntegrityScopeBudget,
    ) -> RelationalRuntime {
        let registry = relation_integrity_runtime().schema_registry().clone();
        RelationalRuntimeApi::builder()
            .schema_registry(registry)
            .relation_integrity_scope_budget(relation_integrity_scope_budget)
            .build()
    }

    fn create_entity(
        runtime: &mut RelationalRuntime,
        name: &str,
    ) -> crate::identity::data::EntityId {
        let mut txn =
            runtime.begin_transaction(crate::facade::transactions::TransactionOptions::default());
        txn.push_batch(
            crate::facade::transactions::WorkerIntentBatch::new(name).push(MutationIntent::Create(
                CreateIntent::Entity(EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: InternedString::Raw(name.to_string()),
                    payload: RecordPayload::StructuredJson(json!({"name": name})),
                }),
            )),
        );
        let outcome = txn.commit().unwrap();
        match outcome.changed_records[0] {
            crate::facade::transactions::RecordRef::Entity(entity_id) => entity_id,
            _ => panic!("expected entity"),
        }
    }

    #[test]
    fn commit_boundary_short_circuits_when_plan_contract_cannot_touch_profile_groups() {
        let runtime = runtime_with_invariants(
            InvariantCatalog {
                registrations: vec![InvariantRegistration::commit_boundary_blocking(
                    InvariantRule::UniqueEntityPayloadField("name".to_string()),
                )],
                ..InvariantCatalog::default()
            },
            RelationalExecutionModel::SerialAuthority,
        );
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(1),
            merged_intents: vec![MutationIntent::Relation(RelationMutationIntent::Delete(
                DeleteRelationIntent {
                    relation_id: crate::identity::data::RelationId::new(PartitionId::main(), 0, 1),
                },
            ))],
        };

        let results = InvariantAccess::new(&runtime).commit_boundary(&plan);

        assert!(results.results().is_empty());
    }

    #[test]
    fn staged_parallel_commit_boundary_matches_serial_reference_results() {
        let invariant_catalog = InvariantCatalog {
            registrations: vec![
                InvariantRegistration::commit_boundary_blocking(
                    InvariantRule::UniqueEntityPayloadField("name".to_string()),
                ),
                InvariantRegistration::commit_boundary_blocking(InvariantRule::MaxMergedIntents(0)),
            ],
            ..InvariantCatalog::default()
        };
        let serial_runtime = runtime_with_invariants(
            invariant_catalog.clone(),
            RelationalExecutionModel::SerialAuthority,
        );
        let staged_runtime = runtime_with_invariants(
            invariant_catalog,
            RelationalExecutionModel::StagedParallelPreparation,
        );
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(2),
            merged_intents: vec![MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: InternedString::Raw("dup".to_string()),
                payload: RecordPayload::StructuredJson(json!({"name":"dup"})),
            }))],
        };

        let serial = InvariantAccess::new(&serial_runtime).commit_boundary(&plan);
        let staged = InvariantAccess::new(&staged_runtime).commit_boundary(&plan);

        assert_eq!(serial.results(), staged.results());
        assert_eq!(
            serial.summary().result_count(),
            staged.summary().result_count()
        );
        assert_eq!(
            staged
                .metadata()
                .preparation_strategy()
                .map(|strategy| strategy.selected_mode),
            Some(PreparationStrategySelection::StagedParallel)
        );
        assert!(staged.results().iter().any(|result| {
            result.failure_effect == InvariantFailureEffect::BlockCommit
                && matches!(result.verdict, InvariantVerdict::Violation(_))
        }));
    }

    #[test]
    fn commit_boundary_metadata_exposes_proof_boundary_summary_for_packet_backed_execution() {
        let mut runtime = relation_integrity_runtime();
        let source = {
            let mut txn = runtime
                .begin_transaction(crate::facade::transactions::TransactionOptions::default());
            txn.push_batch(
                crate::facade::transactions::WorkerIntentBatch::new("source").push(
                    MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                        partition_id: PartitionId::main(),
                        kind_id: KindId(1),
                        client_key: InternedString::Raw("source".to_string()),
                        payload: RecordPayload::StructuredJson(json!({"name":"source"})),
                    })),
                ),
            );
            let outcome = txn.commit().unwrap();
            match outcome.changed_records[0] {
                crate::facade::transactions::RecordRef::Entity(entity_id) => entity_id,
                _ => panic!("expected entity"),
            }
        };
        let target = {
            let mut txn = runtime
                .begin_transaction(crate::facade::transactions::TransactionOptions::default());
            txn.push_batch(
                crate::facade::transactions::WorkerIntentBatch::new("target").push(
                    MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                        partition_id: PartitionId::main(),
                        kind_id: KindId(1),
                        client_key: InternedString::Raw("target".to_string()),
                        payload: RecordPayload::StructuredJson(json!({"name":"target"})),
                    })),
                ),
            );
            let outcome = txn.commit().unwrap();
            match outcome.changed_records[0] {
                crate::facade::transactions::RecordRef::Entity(entity_id) => entity_id,
                _ => panic!("expected entity"),
            }
        };
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(3),
            merged_intents: vec![MutationIntent::Create(CreateIntent::Relation(
                crate::transactions::data::RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_key: InternedString::Raw("planned".to_string()),
                    source,
                    target,
                    payload: Some(RecordPayload::StructuredJson(json!({"label":"planned"}))),
                },
            ))],
        };

        let result = InvariantAccess::new(&runtime).commit_boundary(&plan);
        let summary = result
            .metadata()
            .proof_boundary()
            .expect("proof boundary summary");

        assert_eq!(
            summary.scope_class(),
            InvariantPlanScopeClass::PartitionScope
        );
        assert!(summary.widened_causes().is_empty());
        assert_eq!(summary.packet_count(), 1);
        assert_eq!(summary.touched_partition_count(), 1);
    }

    #[test]
    fn commit_boundary_symmetry_failure_fields_localize_missing_twin_endpoints() {
        let mut runtime = relation_symmetry_runtime(SymmetryMode::PairedTwinRequired);
        let source = {
            let mut txn = runtime
                .begin_transaction(crate::facade::transactions::TransactionOptions::default());
            txn.push_batch(
                crate::facade::transactions::WorkerIntentBatch::new("source").push(
                    MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                        partition_id: PartitionId::main(),
                        kind_id: KindId(1),
                        client_key: InternedString::Raw("source".to_string()),
                        payload: RecordPayload::StructuredJson(json!({"name":"source"})),
                    })),
                ),
            );
            let outcome = txn.commit().unwrap();
            match outcome.changed_records[0] {
                crate::facade::transactions::RecordRef::Entity(entity_id) => entity_id,
                _ => panic!("expected entity"),
            }
        };
        let target = {
            let mut txn = runtime
                .begin_transaction(crate::facade::transactions::TransactionOptions::default());
            txn.push_batch(
                crate::facade::transactions::WorkerIntentBatch::new("target").push(
                    MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                        partition_id: PartitionId::main(),
                        kind_id: KindId(1),
                        client_key: InternedString::Raw("target".to_string()),
                        payload: RecordPayload::StructuredJson(json!({"name":"target"})),
                    })),
                ),
            );
            let outcome = txn.commit().unwrap();
            match outcome.changed_records[0] {
                crate::facade::transactions::RecordRef::Entity(entity_id) => entity_id,
                _ => panic!("expected entity"),
            }
        };
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(4),
            merged_intents: vec![MutationIntent::Create(CreateIntent::Relation(
                crate::transactions::data::RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_key: InternedString::Raw("missing-twin".to_string()),
                    source,
                    target,
                    payload: Some(RecordPayload::StructuredJson(
                        json!({"label":"missing-twin"}),
                    )),
                },
            ))],
        };

        let result = InvariantAccess::new(&runtime).commit_boundary(&plan);
        let failure = result
            .summary()
            .blocking_failure()
            .expect("blocking symmetry failure");
        let fields = failure.fields();

        assert_eq!(
            failure.violation().code,
            crate::diagnostics::data::DiagnosticCode::RelationSymmetryViolation
        );
        assert_eq!(fields["contract_id"], json!("paired_twin"));
        assert_eq!(fields["relation_kind_id"], json!(2));
        assert_eq!(fields["source"], json!(source));
        assert_eq!(fields["target"], json!(target));
        assert_eq!(fields["mode"], json!("paired"));
    }

    #[test]
    fn commit_boundary_cardinality_failure_fields_localize_nonmanifold_like_overflow() {
        let mut runtime = relation_cardinality_runtime();
        let source = {
            let mut txn = runtime
                .begin_transaction(crate::facade::transactions::TransactionOptions::default());
            txn.push_batch(
                crate::facade::transactions::WorkerIntentBatch::new("source").push(
                    MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                        partition_id: PartitionId::main(),
                        kind_id: KindId(1),
                        client_key: InternedString::Raw("source".to_string()),
                        payload: RecordPayload::StructuredJson(json!({"name":"source"})),
                    })),
                ),
            );
            let outcome = txn.commit().unwrap();
            match outcome.changed_records[0] {
                crate::facade::transactions::RecordRef::Entity(entity_id) => entity_id,
                _ => panic!("expected entity"),
            }
        };
        let target_a = {
            let mut txn = runtime
                .begin_transaction(crate::facade::transactions::TransactionOptions::default());
            txn.push_batch(
                crate::facade::transactions::WorkerIntentBatch::new("target-a").push(
                    MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                        partition_id: PartitionId::main(),
                        kind_id: KindId(1),
                        client_key: InternedString::Raw("target-a".to_string()),
                        payload: RecordPayload::StructuredJson(json!({"name":"target-a"})),
                    })),
                ),
            );
            let outcome = txn.commit().unwrap();
            match outcome.changed_records[0] {
                crate::facade::transactions::RecordRef::Entity(entity_id) => entity_id,
                _ => panic!("expected entity"),
            }
        };
        let target_b = {
            let mut txn = runtime
                .begin_transaction(crate::facade::transactions::TransactionOptions::default());
            txn.push_batch(
                crate::facade::transactions::WorkerIntentBatch::new("target-b").push(
                    MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                        partition_id: PartitionId::main(),
                        kind_id: KindId(1),
                        client_key: InternedString::Raw("target-b".to_string()),
                        payload: RecordPayload::StructuredJson(json!({"name":"target-b"})),
                    })),
                ),
            );
            let outcome = txn.commit().unwrap();
            match outcome.changed_records[0] {
                crate::facade::transactions::RecordRef::Entity(entity_id) => entity_id,
                _ => panic!("expected entity"),
            }
        };
        let _accepted = {
            let mut txn = runtime
                .begin_transaction(crate::facade::transactions::TransactionOptions::default());
            txn.push_batch(
                crate::facade::transactions::WorkerIntentBatch::new("accepted").push(
                    MutationIntent::Create(CreateIntent::Relation(
                        crate::transactions::data::RelationSpec {
                            partition_id: PartitionId::main(),
                            kind_id: KindId(2),
                            client_key: InternedString::Raw("accepted".to_string()),
                            source,
                            target: target_a,
                            payload: Some(RecordPayload::StructuredJson(
                                json!({"label":"accepted"}),
                            )),
                        },
                    )),
                ),
            );
            txn.commit().unwrap()
        };
        let overflow_plan = MergedCommitPlan {
            transaction_id: TransactionId(5),
            merged_intents: vec![MutationIntent::Create(CreateIntent::Relation(
                crate::transactions::data::RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_key: InternedString::Raw("overflow".to_string()),
                    source,
                    target: target_b,
                    payload: Some(RecordPayload::StructuredJson(json!({"label":"overflow"}))),
                },
            ))],
        };

        let result = InvariantAccess::new(&runtime).commit_boundary(&overflow_plan);
        let failure = result
            .summary()
            .blocking_failure()
            .expect("blocking cardinality failure");
        let fields = failure.fields();

        assert_eq!(
            failure.violation().code,
            crate::diagnostics::data::DiagnosticCode::RelationCardinalityViolation
        );
        assert_eq!(fields["contract_id"], json!("source_max_one"));
        assert_eq!(fields["relation_kind_id"], json!(2));
        assert_eq!(fields["entity_id"], json!(source));
        assert_eq!(fields["boundary"], json!("source"));
        assert_eq!(fields["count"], json!(2));
        assert_eq!(fields["limit"], json!(1));
    }

    #[test]
    fn commit_boundary_reports_relation_integrity_scope_budget_violation_as_blocking_failure() {
        let mut runtime =
            relation_integrity_runtime_with_scope_budget(RelationIntegrityScopeBudget {
                max_relation_kinds: 8,
                max_touched_entities: 16,
                max_deleted_entities: 8,
                max_scanned_relations: 16,
                max_planned_edges: 1,
            });
        let source_a = create_entity(&mut runtime, "source-a");
        let target_a = create_entity(&mut runtime, "target-a");
        let source_b = create_entity(&mut runtime, "source-b");
        let target_b = create_entity(&mut runtime, "target-b");
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(6),
            merged_intents: vec![MutationIntent::Create(CreateIntent::BulkRelations(
                BulkRelationCreateIntent {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_keys: vec![
                        InternedString::Raw("edge-a".to_string()),
                        InternedString::Raw("edge-b".to_string()),
                    ],
                    endpoints: vec![(source_a, target_a), (source_b, target_b)],
                    payloads: vec![
                        Some(RecordPayload::StructuredJson(json!({"label":"edge-a"}))),
                        Some(RecordPayload::StructuredJson(json!({"label":"edge-b"}))),
                    ],
                },
            ))],
        };

        let result = InvariantAccess::new(&runtime).commit_boundary(&plan);
        let failure = result
            .summary()
            .blocking_failure()
            .expect("blocking scope budget failure");
        let fields = failure.fields();

        assert_eq!(
            failure.code(),
            crate::diagnostics::data::DiagnosticCode::PreparationFailure
        );
        assert_eq!(fields["limit_name"], json!("max_planned_edges"));
        assert_eq!(fields["limit"], json!(1));
        assert_eq!(fields["observed"], json!(2));
        assert_eq!(fields["planned_edge_count"], json!(2));
    }
}
