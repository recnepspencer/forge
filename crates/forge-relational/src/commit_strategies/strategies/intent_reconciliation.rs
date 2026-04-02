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
use crate::payloads::data::canonicalize_json;
use crate::schema::data::{AspectBinding, AspectComparator};
use crate::transactions::data::{
    EntityMutationIntent, MutationIntent, UpdateEntityFieldsIntent, WorkerIntentBatch,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntentReconciliationInput {
    pub entity_id: EntityId,
    pub desired_payload: serde_json::Value,
}

impl IntentReconciliationInput {
    fn desired_payload_object(
        &self,
    ) -> Result<serde_json::Map<String, Value>, StrategyExecutorFailure> {
        match canonicalize_json(&self.desired_payload) {
            Value::Object(map) => Ok(map),
            _ => Err(StrategyExecutorFailure::new(
                StrategyExecutorFailureClass::InvalidInput,
                "intent reconciliation requires desired_payload to be a JSON object",
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentReconciliationAction {
    NoChange,
    UpdateEntity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntentReconciliationOutput {
    pub entity_id: EntityId,
    pub action: IntentReconciliationAction,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IntentReconciliationStrategy;

impl IntentReconciliationStrategy {
    pub const DEFAULT_SEMANTIC_NAME: &'static str = "strategy.intent.reconcile";
    pub const DEFAULT_FAMILY_NAME: &'static str = "strategy.intent";
    pub const DEFAULT_INPUT_SCHEMA_NAME: &'static str = "intent.reconcile.input.v1";
    pub const DEFAULT_OUTPUT_SCHEMA_NAME: &'static str = "intent.reconcile.output.v1";
    pub const DEFAULT_INTENT_NAME: &'static str = "reconcile.desired.state";
    pub const DEFAULT_ARTIFACT_NAME: &'static str = "strategy.intent.reconcile";

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
    ) -> Result<IntentReconciliationInput, StrategyExecutorFailure> {
        serde_json::from_slice(request.canonical_input().canonical_bytes()).map_err(|error| {
            StrategyExecutorFailure::new(
                StrategyExecutorFailureClass::InvalidInput,
                format!("intent reconciliation input could not be decoded: {error}"),
            )
        })
    }

    fn output_artifact(
        output: &IntentReconciliationOutput,
    ) -> Result<CanonicalStrategyOutputArtifact, StrategyExecutorFailure> {
        let bytes = serde_json::to_vec(output).map_err(|error| {
            StrategyExecutorFailure::new(
                StrategyExecutorFailureClass::DomainRejection,
                format!("intent reconciliation output could not be serialized: {error}"),
            )
        })?;
        Ok(CanonicalStrategyOutputArtifact::new(
            StrategyOutputSchemaName::new(Self::DEFAULT_OUTPUT_SCHEMA_NAME),
            bytes,
            PersistentArtifactName::new(Self::DEFAULT_ARTIFACT_NAME),
        ))
    }

    fn reconcile_fields(
        observation: &StrategyObservationContext<'_>,
        existing: &crate::storage::data::EntityReadRecord,
        desired_payload: &serde_json::Map<String, Value>,
    ) -> Result<std::collections::BTreeMap<String, Value>, StrategyExecutorFailure> {
        let registration = observation
            .schema_registry()
            .entity_registration(existing.kind.kind_id)
            .map_err(|error| {
                StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::DomainRejection,
                    format!(
                        "entity kind registration missing during intent reconciliation: {error:?}"
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
                    crate::symbols::data::InternedString::Raw(raw) => Some(raw.as_str()),
                    _ => None,
                },
                _ => None,
            })
            .collect::<std::collections::BTreeSet<_>>();
        for key in desired_payload.keys() {
            if !declared_scalar_fields.contains(key.as_str()) {
                return Err(StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::DomainRejection,
                    format!(
                        "field '{}' is not a declared scalar entity aspect on kind {}",
                        key, existing.kind.kind_name
                    ),
                ));
            }
        }
        Ok(desired_payload
            .iter()
            .map(|(key, value)| (key.clone(), canonicalize_json(value)))
            .collect())
    }
}

impl CommitStrategyExecutor for IntentReconciliationStrategy {
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
                        "intent reconciliation target entity {:?} is not visible in the committed basis",
                        input.entity_id
                    ),
                )
            })?;
        let desired_fields = Self::reconcile_fields(observation, &existing, &desired_payload)?;
        let unchanged = desired_fields.iter().all(|(key, desired_value)| {
            existing.payload.as_json().and_then(|value| value.get(key)) == Some(desired_value)
        });

        let (action, mutation_program) = if unchanged {
            (
                IntentReconciliationAction::NoChange,
                StrategyMutationProgram::new(Vec::<WorkerIntentBatch>::new()),
            )
        } else {
            let batch = WorkerIntentBatch::new("intent-reconciliation-update").push(
                MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                    UpdateEntityFieldsIntent {
                        entity_id: input.entity_id,
                        fields: desired_fields,
                    },
                )),
            );
            (
                IntentReconciliationAction::UpdateEntity,
                StrategyMutationProgram::new(vec![batch]),
            )
        };

        let output = Self::output_artifact(&IntentReconciliationOutput {
            entity_id: input.entity_id,
            action,
        })?;
        Ok(StrategyExecutionResult::new(output, mutation_program))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        IntentReconciliationAction, IntentReconciliationInput, IntentReconciliationOutput,
        IntentReconciliationStrategy,
    };
    use crate::commit_strategies::data::{
        RawStrategyCommitRequest, StrategyCallerProvenance, StrategyRequestOrigin,
    };
    use crate::logic::builder::RelationalRuntimeBuilder;
    use crate::tests::support::{
        create_entity, entity_payload_aspect, lifecycle_aspect, update_entity, AspectSchemaFixture,
    };
    use serde_json::Value;

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
    fn intent_reconciliation_strategy_emits_update_when_payload_differs() {
        let descriptor = IntentReconciliationStrategy::descriptor(
            crate::commit_strategies::data::CommitStrategyId(501),
        );
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(strategy_registry())
            .commit_strategy(
                crate::commit_strategies::data::CommitStrategyRegistration::new(descriptor.clone())
                    .expect("strategy registration"),
            )
            .commit_strategy_executor(IntentReconciliationStrategy::execution_registration(
                &descriptor,
            ))
            .build();
        let entity = create_entity(&mut runtime, "before");
        let request = runtime
            .commit_strategies()
            .canonicalize_request(&RawStrategyCommitRequest::new(
                crate::commit_strategies::data::CommitStrategySemanticName::new(
                    IntentReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
                ),
                serde_json::to_vec(&IntentReconciliationInput {
                    entity_id: entity,
                    desired_payload: serde_json::json!({"name":"after"}),
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
        let output: IntentReconciliationOutput =
            serde_json::from_slice(execution.output().canonical_bytes()).expect("output decode");

        assert_eq!(output.action, IntentReconciliationAction::UpdateEntity);
        assert_eq!(execution.mutation_program().total_intent_count(), 1);
    }

    #[test]
    fn intent_reconciliation_strategy_emits_noop_when_payload_matches() {
        let descriptor = IntentReconciliationStrategy::descriptor(
            crate::commit_strategies::data::CommitStrategyId(502),
        );
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(strategy_registry())
            .commit_strategy(
                crate::commit_strategies::data::CommitStrategyRegistration::new(descriptor.clone())
                    .expect("strategy registration"),
            )
            .commit_strategy_executor(IntentReconciliationStrategy::execution_registration(
                &descriptor,
            ))
            .build();
        let entity = create_entity(&mut runtime, "before");
        update_entity(&mut runtime, entity, "stable");
        let request = runtime
            .commit_strategies()
            .canonicalize_request(&RawStrategyCommitRequest::new(
                crate::commit_strategies::data::CommitStrategySemanticName::new(
                    IntentReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
                ),
                serde_json::to_vec(&IntentReconciliationInput {
                    entity_id: entity,
                    desired_payload: serde_json::json!({"name":"stable"}),
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
        let output: IntentReconciliationOutput =
            serde_json::from_slice(execution.output().canonical_bytes()).expect("output decode");

        assert_eq!(output.action, IntentReconciliationAction::NoChange);
        assert_eq!(execution.mutation_program().total_intent_count(), 0);
    }

    #[test]
    fn intent_reconciliation_strategy_preserves_untouched_declared_fields() {
        let descriptor = IntentReconciliationStrategy::descriptor(
            crate::commit_strategies::data::CommitStrategyId(503),
        );
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(strategy_registry())
            .commit_strategy(
                crate::commit_strategies::data::CommitStrategyRegistration::new(descriptor.clone())
                    .expect("strategy registration"),
            )
            .commit_strategy_executor(IntentReconciliationStrategy::execution_registration(
                &descriptor,
            ))
            .build();
        let entity = create_entity(&mut runtime, "before");
        let request = runtime
            .commit_strategies()
            .canonicalize_request(&RawStrategyCommitRequest::new(
                crate::commit_strategies::data::CommitStrategySemanticName::new(
                    IntentReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
                ),
                serde_json::to_vec(&IntentReconciliationInput {
                    entity_id: entity,
                    desired_payload: serde_json::json!({"replicas":3}),
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
        let intent = &execution.mutation_program().worker_batches()[0].intents[0];
        let updated_payload = match intent {
            crate::transactions::data::MutationIntent::Entity(
                crate::transactions::data::EntityMutationIntent::UpdateFields(intent),
            ) => serde_json::Value::Object(
                intent
                    .fields
                    .iter()
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect(),
            ),
            other => panic!("expected update entity fields intent, got {other:?}"),
        };

        assert_eq!(
            updated_payload.get("replicas"),
            Some(&Value::Number(serde_json::Number::from(3_u64)))
        );
        assert_eq!(updated_payload.as_object().expect("object").len(), 1);
    }
}
