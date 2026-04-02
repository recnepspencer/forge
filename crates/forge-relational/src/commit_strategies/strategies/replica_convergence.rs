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
use crate::schema::data::{AspectBinding, AspectComparator};
use crate::transactions::data::{
    EntityMutationIntent, MutationIntent, UpdateEntityFieldsIntent, WorkerIntentBatch,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplicaConvergenceInput {
    pub entity_id: EntityId,
    pub desired_replicas: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplicaConvergenceAction {
    NoChange,
    UpdateReplicas,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplicaConvergenceOutput {
    pub entity_id: EntityId,
    pub action: ReplicaConvergenceAction,
    pub desired_replicas: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ReplicaConvergenceStrategy;

impl ReplicaConvergenceStrategy {
    pub const DEFAULT_SEMANTIC_NAME: &'static str = "strategy.replica.converge";
    pub const DEFAULT_FAMILY_NAME: &'static str = "strategy.replica";
    pub const DEFAULT_INPUT_SCHEMA_NAME: &'static str = "replica.converge.input.v1";
    pub const DEFAULT_OUTPUT_SCHEMA_NAME: &'static str = "replica.converge.output.v1";
    pub const DEFAULT_INTENT_NAME: &'static str = "replica.desired.state";
    pub const DEFAULT_ARTIFACT_NAME: &'static str = "strategy.replica.converge";

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
    ) -> Result<ReplicaConvergenceInput, StrategyExecutorFailure> {
        serde_json::from_slice(request.canonical_input().canonical_bytes()).map_err(|error| {
            StrategyExecutorFailure::new(
                StrategyExecutorFailureClass::InvalidInput,
                format!("replica convergence input could not be decoded: {error}"),
            )
        })
    }

    fn output_artifact(
        output: &ReplicaConvergenceOutput,
    ) -> Result<CanonicalStrategyOutputArtifact, StrategyExecutorFailure> {
        let bytes = serde_json::to_vec(output).map_err(|error| {
            StrategyExecutorFailure::new(
                StrategyExecutorFailureClass::DomainRejection,
                format!("replica convergence output could not be serialized: {error}"),
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
        desired_replicas: u64,
    ) -> Result<std::collections::BTreeMap<String, Value>, StrategyExecutorFailure> {
        let registration = observation
            .schema_registry()
            .entity_registration(existing.kind.kind_id)
            .map_err(|error| {
                StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::DomainRejection,
                    format!(
                        "entity kind registration missing during replica convergence: {error:?}"
                    ),
                )
            })?;
        let replicas_declared = registration
            .aspect_declarations
            .aspects
            .iter()
            .any(|aspect| {
                matches!(
                    (&aspect.binding, aspect.comparator),
                    (
                        AspectBinding::EntityPayloadField { field },
                        AspectComparator::JsonScalarEquality,
                    ) if matches!(
                        field,
                        crate::symbols::data::InternedString::Raw(raw) if raw == "replicas"
                    )
                )
            });
        if !replicas_declared {
            return Err(StrategyExecutorFailure::new(
                StrategyExecutorFailureClass::DomainRejection,
                format!(
                    "field 'replicas' is not a declared scalar entity aspect on kind {}",
                    existing.kind.kind_name
                ),
            ));
        }
        let mut fields = std::collections::BTreeMap::new();
        fields.insert(
            "replicas".to_string(),
            Value::Number(serde_json::Number::from(desired_replicas)),
        );
        Ok(fields)
    }
}

impl CommitStrategyExecutor for ReplicaConvergenceStrategy {
    fn execute(
        &self,
        request: &CanonicalStrategyCommitRequest,
        observation: &StrategyObservationContext<'_>,
    ) -> Result<StrategyExecutionResult, StrategyExecutorFailure> {
        let input = Self::parse_input(request)?;
        let existing = observation
            .visibility()
            .entity_record(input.entity_id)?
            .ok_or_else(|| {
                StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::DomainRejection,
                    format!(
                        "replica convergence target entity {:?} is not visible in the committed basis",
                        input.entity_id
                    ),
                )
            })?;
        let desired_fields =
            Self::reconcile_fields(observation, &existing, input.desired_replicas)?;
        let desired_replicas_value = desired_fields
            .get("replicas")
            .cloned()
            .expect("replicas field");
        let existing_replicas = existing
            .payload
            .as_json()
            .and_then(|value| value.get("replicas"))
            .cloned();

        let (action, mutation_program) = if existing_replicas.as_ref()
            == Some(&desired_replicas_value)
        {
            (
                ReplicaConvergenceAction::NoChange,
                StrategyMutationProgram::new(Vec::<WorkerIntentBatch>::new()),
            )
        } else {
            let batch =
                WorkerIntentBatch::new("replica-convergence-update").push(MutationIntent::Entity(
                    EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                        entity_id: input.entity_id,
                        fields: desired_fields,
                    }),
                ));
            (
                ReplicaConvergenceAction::UpdateReplicas,
                StrategyMutationProgram::new(vec![batch]),
            )
        };

        let output = Self::output_artifact(&ReplicaConvergenceOutput {
            entity_id: input.entity_id,
            action,
            desired_replicas: input.desired_replicas,
        })?;
        Ok(StrategyExecutionResult::new(output, mutation_program))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ReplicaConvergenceAction, ReplicaConvergenceInput, ReplicaConvergenceOutput,
        ReplicaConvergenceStrategy,
    };
    use crate::commit_strategies::data::{
        RawStrategyCommitRequest, StrategyCallerProvenance, StrategyRequestOrigin,
    };
    use crate::logic::builder::RelationalRuntimeBuilder;
    use crate::tests::support::{entity_payload_aspect, lifecycle_aspect, AspectSchemaFixture};
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
    fn replica_convergence_strategy_updates_replicas_and_preserves_other_fields() {
        let descriptor = ReplicaConvergenceStrategy::descriptor(
            crate::commit_strategies::data::CommitStrategyId(601),
        );
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(strategy_registry())
            .commit_strategy(
                crate::commit_strategies::data::CommitStrategyRegistration::new(descriptor.clone())
                    .expect("strategy registration"),
            )
            .commit_strategy_executor(ReplicaConvergenceStrategy::execution_registration(
                &descriptor,
            ))
            .build();
        let entity = crate::tests::support::create_entity(&mut runtime, "before");
        let request = runtime
            .commit_strategies()
            .canonicalize_request(&RawStrategyCommitRequest::new(
                crate::commit_strategies::data::CommitStrategySemanticName::new(
                    ReplicaConvergenceStrategy::DEFAULT_SEMANTIC_NAME,
                ),
                serde_json::to_vec(&ReplicaConvergenceInput {
                    entity_id: entity,
                    desired_replicas: 5,
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
        let output: ReplicaConvergenceOutput =
            serde_json::from_slice(execution.output().canonical_bytes()).expect("output decode");
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

        assert_eq!(output.action, ReplicaConvergenceAction::UpdateReplicas);
        assert_eq!(
            updated_payload.get("replicas"),
            Some(&Value::Number(serde_json::Number::from(5_u64)))
        );
        assert_eq!(updated_payload.as_object().expect("object").len(), 1);
    }
}
