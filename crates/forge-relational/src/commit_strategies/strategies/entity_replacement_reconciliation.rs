use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::commit_strategies::data::{
    CanonicalStrategyCommitRequest, CanonicalStrategyOutputArtifact, CommitStrategyDescriptor,
    CommitStrategyExecutionRegistration, CommitStrategyExecutor, CommitStrategyFamilyName,
    CommitStrategyId, CommitStrategyRegistration, CommitStrategyRegistrationError,
    CommitStrategySemanticName, CommitStrategyVersion, PersistentArtifactName,
    StrategyExecutionResult, StrategyExecutorFailure, StrategyExecutorFailureClass,
    StrategyInputSchemaName, StrategyInputSchemaVersion, StrategyIntentName,
    StrategyMutationProgram, StrategyObservationContext, StrategyOutputSchemaName,
    StrategyPacketContract, StrategyReadContract, StrategyReadCostClass, StrategyReadLocalityClass,
    StrategyReadScopeClass, StrategyRequestCanonicalization, StrategyTraversalBasis,
};
use crate::identity::data::EntityId;
use crate::payloads::data::{canonicalize_json, RecordPayload};
use crate::schema::data::{AspectBinding, AspectComparator};
use crate::symbols::data::InternedString;
use crate::transactions::data::{
    EntityMutationIntent, EntitySpec, MutationIntent, ReplaceEntityIntent, WorkerIntentBatch,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityReplacementReconciliationInput {
    pub entity_id: EntityId,
    pub replacement_client_key: String,
    pub desired_payload: serde_json::Value,
}

impl EntityReplacementReconciliationInput {
    fn desired_payload_object(
        &self,
    ) -> Result<serde_json::Map<String, Value>, StrategyExecutorFailure> {
        match canonicalize_json(&self.desired_payload) {
            Value::Object(map) => Ok(map),
            _ => Err(StrategyExecutorFailure::new(
                StrategyExecutorFailureClass::InvalidInput,
                "entity replacement reconciliation requires desired_payload to be a JSON object",
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityReplacementReconciliationAction {
    NoChange,
    ReplaceEntity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityReplacementReconciliationOutput {
    pub entity_id: EntityId,
    pub action: EntityReplacementReconciliationAction,
    pub replacement_client_key: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct EntityReplacementReconciliationStrategy;

impl EntityReplacementReconciliationStrategy {
    pub const DEFAULT_SEMANTIC_NAME: &'static str = "strategy.replace.entity.reconcile";
    pub const DEFAULT_FAMILY_NAME: &'static str = "strategy.replace";
    pub const DEFAULT_INPUT_SCHEMA_NAME: &'static str = "entity.replace.reconcile.input.v1";
    pub const DEFAULT_OUTPUT_SCHEMA_NAME: &'static str = "entity.replace.reconcile.output.v1";
    pub const DEFAULT_INTENT_NAME: &'static str = "entity.replace.reconcile";
    pub const DEFAULT_ARTIFACT_NAME: &'static str = "strategy.replace.entity.reconcile";

    pub fn descriptor(strategy_id: CommitStrategyId) -> CommitStrategyDescriptor {
        CommitStrategyDescriptor::new(
            strategy_id,
            CommitStrategySemanticName::new(Self::DEFAULT_SEMANTIC_NAME),
            CommitStrategyFamilyName::new(Self::DEFAULT_FAMILY_NAME),
            CommitStrategyVersion::new(1, 0),
            StrategyIntentName::new(Self::DEFAULT_INTENT_NAME),
            StrategyInputSchemaName::new(Self::DEFAULT_INPUT_SCHEMA_NAME),
            StrategyInputSchemaVersion(1),
            StrategyOutputSchemaName::new(Self::DEFAULT_OUTPUT_SCHEMA_NAME),
            StrategyRequestCanonicalization::JsonStableObjectOrderV1,
            StrategyReadContract {
                scope_class: StrategyReadScopeClass::ExplicitTargetsOnly,
                locality_class: StrategyReadLocalityClass::SinglePartition,
                traversal_basis: StrategyTraversalBasis::NoTraversal,
                packet_contract: StrategyPacketContract::ProjectionOnly,
                cost_class: StrategyReadCostClass::ORequestedSurface,
            },
            PersistentArtifactName::new(Self::DEFAULT_ARTIFACT_NAME),
        )
    }

    pub fn registration(
        strategy_id: CommitStrategyId,
    ) -> Result<CommitStrategyRegistration, CommitStrategyRegistrationError> {
        CommitStrategyRegistration::new(Self::descriptor(strategy_id))
    }

    pub fn execution_registration(
        descriptor: &CommitStrategyDescriptor,
    ) -> CommitStrategyExecutionRegistration {
        CommitStrategyExecutionRegistration::new(descriptor, Self)
    }

    fn parse_input(
        request: &CanonicalStrategyCommitRequest,
    ) -> Result<EntityReplacementReconciliationInput, StrategyExecutorFailure> {
        serde_json::from_slice(request.canonical_input().canonical_bytes()).map_err(|error| {
            StrategyExecutorFailure::new(
                StrategyExecutorFailureClass::InvalidInput,
                format!("entity replacement reconciliation input could not be decoded: {error}"),
            )
        })
    }

    fn output_artifact(
        output: &EntityReplacementReconciliationOutput,
    ) -> Result<CanonicalStrategyOutputArtifact, StrategyExecutorFailure> {
        let bytes = serde_json::to_vec(output).map_err(|error| {
            StrategyExecutorFailure::new(
                StrategyExecutorFailureClass::DomainRejection,
                format!(
                    "entity replacement reconciliation output could not be serialized: {error}"
                ),
            )
        })?;
        Ok(CanonicalStrategyOutputArtifact::new(
            StrategyOutputSchemaName::new(Self::DEFAULT_OUTPUT_SCHEMA_NAME),
            bytes,
            PersistentArtifactName::new(Self::DEFAULT_ARTIFACT_NAME),
        ))
    }

    fn reconciled_payload(
        observation: &StrategyObservationContext<'_>,
        existing: &crate::storage::data::EntityReadRecord,
        desired_payload: &serde_json::Map<String, Value>,
    ) -> Result<serde_json::Map<String, Value>, StrategyExecutorFailure> {
        let mut reconciled = existing
            .payload
            .as_json()
            .and_then(|value| value.as_object().cloned())
            .ok_or_else(|| {
                StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::DomainRejection,
                    "entity replacement reconciliation requires the existing payload to be a JSON object",
                )
            })?;
        let registration = observation
            .schema_registry()
            .entity_registration(existing.kind.kind_id)
            .map_err(|error| {
                StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::DomainRejection,
                    format!(
                        "entity kind registration missing during replacement reconciliation: {error:?}"
                    ),
                )
            })?;
        let declared_scalar_fields = registration
            .aspect_declarations
            .aspects
            .iter()
            .filter_map(|aspect| match (&aspect.binding, aspect.comparator) {
                (
                    AspectBinding::EntityPayloadField { field },
                    AspectComparator::JsonScalarEquality,
                ) => match field {
                    InternedString::Raw(raw) => Some(raw.as_str()),
                    _ => None,
                },
                _ => None,
            })
            .collect::<std::collections::BTreeSet<_>>();
        for (key, value) in desired_payload {
            if !declared_scalar_fields.contains(key.as_str()) {
                return Err(StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::DomainRejection,
                    format!(
                        "field '{}' is not a declared scalar entity aspect on kind {}",
                        key, existing.kind.kind_name
                    ),
                ));
            }
            reconciled.insert(key.clone(), canonicalize_json(value));
        }
        Ok(reconciled)
    }
}

impl CommitStrategyExecutor for EntityReplacementReconciliationStrategy {
    fn execute(
        &self,
        request: &CanonicalStrategyCommitRequest,
        observation: &StrategyObservationContext<'_>,
    ) -> Result<StrategyExecutionResult, StrategyExecutorFailure> {
        let input = Self::parse_input(request)?;
        let desired_payload = input.desired_payload_object()?;
        let existing = observation
            .visibility()
            .entity_record(input.entity_id)?
            .ok_or_else(|| {
                StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::DomainRejection,
                    format!(
                        "entity replacement reconciliation target entity {:?} is not visible in the committed basis",
                        input.entity_id
                    ),
                )
            })?;
        let reconciled_payload =
            Self::reconciled_payload(observation, &existing, &desired_payload)?;
        // The committed read surface does not currently expose the live client key,
        // so replacement reconciliation must conservatively honor explicit replacement
        // requests even when the payload is otherwise unchanged.
        let unchanged = existing
            .payload
            .as_json()
            .and_then(|value| value.as_object())
            == Some(&reconciled_payload)
            && input.replacement_client_key.is_empty();

        let (action, mutation_program) = if unchanged {
            (
                EntityReplacementReconciliationAction::NoChange,
                StrategyMutationProgram::new(Vec::<WorkerIntentBatch>::new()),
            )
        } else {
            let batch = WorkerIntentBatch::new("entity-replacement-reconciliation").push(
                MutationIntent::Entity(EntityMutationIntent::Replace(ReplaceEntityIntent {
                    entity_id: input.entity_id,
                    replacement: EntitySpec {
                        partition_id: input.entity_id.partition_id,
                        kind_id: existing.kind.kind_id,
                        client_key: InternedString::Raw(input.replacement_client_key.clone()),
                        payload: RecordPayload::StructuredJson(Value::Object(reconciled_payload)),
                    },
                })),
            );
            (
                EntityReplacementReconciliationAction::ReplaceEntity,
                StrategyMutationProgram::new(vec![batch]),
            )
        };

        let output = Self::output_artifact(&EntityReplacementReconciliationOutput {
            entity_id: input.entity_id,
            action,
            replacement_client_key: input.replacement_client_key,
        })?;
        Ok(StrategyExecutionResult::new(output, mutation_program))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        EntityReplacementReconciliationAction, EntityReplacementReconciliationInput,
        EntityReplacementReconciliationOutput, EntityReplacementReconciliationStrategy,
    };
    use crate::commit_strategies::data::{
        RawStrategyCommitRequest, StrategyCallerProvenance, StrategyExecutorFailureClass,
        StrategyRequestOrigin,
    };
    use crate::logic::builder::RelationalRuntimeBuilder;
    use crate::tests::support::{
        create_entity, entity_payload_aspect, lifecycle_aspect, AspectSchemaFixture,
    };
    use serde_json::{json, Value};

    fn strategy_registry() -> crate::schema::data::RelationalSchemaRegistry {
        AspectSchemaFixture {
            entity_aspects: vec![
                entity_payload_aspect("name", "name"),
                entity_payload_aspect("replicas", "replicas"),
                lifecycle_aspect(),
            ],
            ..AspectSchemaFixture::default()
        }
        .build_registry()
    }

    #[test]
    fn entity_replacement_reconciliation_strategy_replaces_with_normalized_client_key_and_preserved_fields(
    ) {
        let descriptor = EntityReplacementReconciliationStrategy::descriptor(
            crate::commit_strategies::data::CommitStrategyId(711),
        );
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(strategy_registry())
            .commit_strategy(
                crate::commit_strategies::data::CommitStrategyRegistration::new(descriptor.clone())
                    .expect("strategy registration"),
            )
            .commit_strategy_executor(
                EntityReplacementReconciliationStrategy::execution_registration(&descriptor),
            )
            .build();
        let entity = create_entity(&mut runtime, "before");
        let request = runtime
            .commit_strategies()
            .canonicalize_request(&RawStrategyCommitRequest::new(
                crate::commit_strategies::data::CommitStrategySemanticName::new(
                    EntityReplacementReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
                ),
                serde_json::to_vec(&EntityReplacementReconciliationInput {
                    entity_id: entity,
                    replacement_client_key: "service-replacement".to_string(),
                    desired_payload: json!({"replicas": 3}),
                })
                .expect("serialize input"),
                StrategyCallerProvenance {
                    request_origin: StrategyRequestOrigin::Test,
                    actor_identity: None,
                    correlation_id: None,
                },
            ))
            .expect("canonical request");
        let snapshot = runtime.visibility_authority().snapshot();
        let execution = runtime
            .commit_strategies()
            .execute(&request, &snapshot)
            .expect("strategy execution");
        let output: EntityReplacementReconciliationOutput =
            serde_json::from_slice(execution.output().canonical_bytes()).expect("output decode");
        let replacement = match &execution.mutation_program().worker_batches()[0].intents[0] {
            crate::transactions::data::MutationIntent::Entity(
                crate::transactions::data::EntityMutationIntent::Replace(intent),
            ) => intent,
            other => panic!("expected replace entity intent, got {other:?}"),
        };
        let replacement_payload = replacement
            .replacement
            .payload
            .as_json()
            .expect("replacement payload should be structured json");

        assert_eq!(
            output.action,
            EntityReplacementReconciliationAction::ReplaceEntity
        );
        assert_eq!(
            replacement.replacement.client_key,
            crate::symbols::data::InternedString::Raw("service-replacement".to_string())
        );
        assert_eq!(
            replacement_payload.get("name"),
            Some(&Value::String("before".to_string()))
        );
        assert_eq!(
            replacement_payload.get("replicas"),
            Some(&Value::Number(serde_json::Number::from(3_u64)))
        );
    }

    #[test]
    fn entity_replacement_reconciliation_strategy_rejects_undeclared_fields() {
        let descriptor = EntityReplacementReconciliationStrategy::descriptor(
            crate::commit_strategies::data::CommitStrategyId(712),
        );
        let registry = AspectSchemaFixture {
            entity_aspects: vec![entity_payload_aspect("name", "name"), lifecycle_aspect()],
            ..AspectSchemaFixture::default()
        }
        .build_registry();
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(registry)
            .commit_strategy(
                crate::commit_strategies::data::CommitStrategyRegistration::new(descriptor.clone())
                    .expect("strategy registration"),
            )
            .commit_strategy_executor(
                EntityReplacementReconciliationStrategy::execution_registration(&descriptor),
            )
            .build();
        let entity = create_entity(&mut runtime, "before");
        let request = runtime
            .commit_strategies()
            .canonicalize_request(&RawStrategyCommitRequest::new(
                crate::commit_strategies::data::CommitStrategySemanticName::new(
                    EntityReplacementReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
                ),
                serde_json::to_vec(&EntityReplacementReconciliationInput {
                    entity_id: entity,
                    replacement_client_key: "service-replacement".to_string(),
                    desired_payload: json!({"replicas": 3}),
                })
                .expect("serialize input"),
                StrategyCallerProvenance {
                    request_origin: StrategyRequestOrigin::Test,
                    actor_identity: None,
                    correlation_id: None,
                },
            ))
            .expect("canonical request");
        let snapshot = runtime.visibility_authority().snapshot();
        let error = runtime
            .commit_strategies()
            .execute(&request, &snapshot)
            .expect_err("undeclared replacement fields should fail");

        match error {
            crate::commit_strategies::StrategyExecutionError::ExecutorFailed {
                failure, ..
            } => {
                assert_eq!(failure.class, StrategyExecutorFailureClass::DomainRejection);
                assert!(failure
                    .detail
                    .contains("not a declared scalar entity aspect"));
            }
            other => panic!("expected executor failure, got {other:?}"),
        }
    }

    #[test]
    fn entity_replacement_reconciliation_strategy_replaces_when_only_client_key_changes() {
        let descriptor = EntityReplacementReconciliationStrategy::descriptor(
            crate::facade::commit_strategies::CommitStrategyId(77),
        );
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(strategy_registry())
            .commit_strategy(
                EntityReplacementReconciliationStrategy::registration(descriptor.id())
                    .expect("registration"),
            )
            .commit_strategy_executor(
                EntityReplacementReconciliationStrategy::execution_registration(&descriptor),
            )
            .build();
        let entity = create_entity(&mut runtime, "service");
        let request = runtime
            .commit_strategies()
            .canonicalize_request(&RawStrategyCommitRequest::new(
                crate::facade::commit_strategies::CommitStrategySemanticName::new(
                    EntityReplacementReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
                ),
                serde_json::to_vec(&EntityReplacementReconciliationInput {
                    entity_id: entity,
                    replacement_client_key: "service-v2".to_string(),
                    desired_payload: json!({"name":"service"}),
                })
                .expect("serialize input"),
                StrategyCallerProvenance {
                    request_origin: StrategyRequestOrigin::Test,
                    actor_identity: None,
                    correlation_id: None,
                },
            ))
            .expect("canonical request");
        let snapshot = runtime.visibility_authority().snapshot();
        let execution = runtime
            .commit_strategies()
            .execute(&request, &snapshot)
            .expect("strategy execution");

        let output = serde_json::from_slice::<EntityReplacementReconciliationOutput>(
            execution.output().canonical_bytes(),
        )
        .expect("decode output");
        assert_eq!(
            output.action,
            EntityReplacementReconciliationAction::ReplaceEntity
        );
    }
}
