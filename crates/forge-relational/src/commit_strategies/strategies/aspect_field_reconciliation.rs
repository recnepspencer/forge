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
use crate::payloads::data::canonicalize_json;
use crate::schema::data::{AspectBinding, AspectComparator};
use crate::transactions::data::{
    EntityMutationIntent, MutationIntent, UpdateEntityFieldsIntent, WorkerIntentBatch,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectFieldReconciliationInput {
    pub entity_id: crate::identity::data::EntityId,
    pub field_name: String,
    pub desired_value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectFieldReconciliationOutput {
    pub entity_id: crate::identity::data::EntityId,
    pub field_name: String,
    pub updated: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AspectFieldReconciliationStrategy;

impl AspectFieldReconciliationStrategy {
    pub const DEFAULT_SEMANTIC_NAME: &'static str = "strategy.aspect.field.reconcile";
    pub const DEFAULT_FAMILY_NAME: &'static str = "strategy.aspect";
    pub const DEFAULT_INPUT_SCHEMA_NAME: &'static str = "aspect.field.reconcile.input.v1";
    pub const DEFAULT_OUTPUT_SCHEMA_NAME: &'static str = "aspect.field.reconcile.output.v1";
    pub const DEFAULT_INTENT_NAME: &'static str = "aspect.scalar.field.reconcile";
    pub const DEFAULT_ARTIFACT_NAME: &'static str = "strategy.aspect.field.reconcile";

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
    ) -> Result<AspectFieldReconciliationInput, StrategyExecutorFailure> {
        serde_json::from_slice(request.canonical_input().canonical_bytes()).map_err(|error| {
            StrategyExecutorFailure::new(
                StrategyExecutorFailureClass::InvalidInput,
                format!("aspect field reconciliation input could not be decoded: {error}"),
            )
        })
    }

    fn output_artifact(
        output: &AspectFieldReconciliationOutput,
    ) -> Result<CanonicalStrategyOutputArtifact, StrategyExecutorFailure> {
        let bytes = serde_json::to_vec(output).map_err(|error| {
            StrategyExecutorFailure::new(
                StrategyExecutorFailureClass::DomainRejection,
                format!("aspect field reconciliation output could not be serialized: {error}"),
            )
        })?;
        Ok(CanonicalStrategyOutputArtifact::new(
            StrategyOutputSchemaName::new(Self::DEFAULT_OUTPUT_SCHEMA_NAME),
            bytes,
            PersistentArtifactName::new(Self::DEFAULT_ARTIFACT_NAME),
        ))
    }
}

impl CommitStrategyExecutor for AspectFieldReconciliationStrategy {
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
                        "aspect field reconciliation target entity {:?} is not visible in the committed basis",
                        input.entity_id
                    ),
                )
            })?;
        let registration = observation
            .schema_registry()
            .entity_registration(existing.kind.kind_id)
            .map_err(|error| {
                StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::DomainRejection,
                    format!(
                        "entity kind registration missing during aspect reconciliation: {error:?}"
                    ),
                )
            })?;
        let field_aspect_declared = registration
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
                        crate::symbols::data::InternedString::Raw(raw) if raw == &input.field_name
                    )
                )
            });
        if !field_aspect_declared {
            return Err(StrategyExecutorFailure::new(
                StrategyExecutorFailureClass::DomainRejection,
                format!(
                    "field '{}' is not a declared scalar entity aspect on kind {}",
                    input.field_name, existing.kind.kind_name
                ),
            ));
        }
        let desired_value = canonicalize_json(&input.desired_value);
        let updated = existing
            .payload
            .as_json()
            .and_then(|value| value.get(&input.field_name))
            != Some(&desired_value);
        let mutation_program = if updated {
            let mut fields = std::collections::BTreeMap::new();
            fields.insert(input.field_name.clone(), desired_value);
            let batch = WorkerIntentBatch::new("aspect-field-reconciliation-update").push(
                MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                    UpdateEntityFieldsIntent {
                        entity_id: input.entity_id,
                        fields,
                    },
                )),
            );
            StrategyMutationProgram::new(vec![batch])
        } else {
            StrategyMutationProgram::new(Vec::<WorkerIntentBatch>::new())
        };

        let output = Self::output_artifact(&AspectFieldReconciliationOutput {
            entity_id: input.entity_id,
            field_name: input.field_name,
            updated,
        })?;
        Ok(StrategyExecutionResult::new(output, mutation_program))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AspectFieldReconciliationInput, AspectFieldReconciliationOutput,
        AspectFieldReconciliationStrategy,
    };
    use crate::commit_strategies::data::{
        RawStrategyCommitRequest, StrategyCallerProvenance, StrategyExecutorFailureClass,
        StrategyRequestOrigin,
    };
    use crate::config::data::CascadeDeletePolicy;
    use crate::logic::builder::RelationalRuntimeBuilder;
    use crate::tests::support::{
        create_entity, entity_payload_aspect, lifecycle_aspect, AspectSchemaFixture,
    };
    use serde_json::{json, Value};

    #[test]
    fn aspect_field_reconciliation_strategy_updates_only_declared_field_aspect() {
        let descriptor = AspectFieldReconciliationStrategy::descriptor(
            crate::commit_strategies::data::CommitStrategyId(701),
        );
        let registry = AspectSchemaFixture {
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            entity_aspects: vec![
                entity_payload_aspect("name", "name"),
                entity_payload_aspect("replicas", "replicas"),
                lifecycle_aspect(),
            ],
            ..AspectSchemaFixture::default()
        }
        .build_registry();
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(registry)
            .commit_strategy(
                crate::commit_strategies::data::CommitStrategyRegistration::new(descriptor.clone())
                    .expect("strategy registration"),
            )
            .commit_strategy_executor(AspectFieldReconciliationStrategy::execution_registration(
                &descriptor,
            ))
            .build();
        let entity = create_entity(&mut runtime, "before");
        crate::tests::support::update_entity(&mut runtime, entity, "before");
        let request = runtime
            .commit_strategies()
            .canonicalize_request(&RawStrategyCommitRequest::new(
                crate::commit_strategies::data::CommitStrategySemanticName::new(
                    AspectFieldReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
                ),
                serde_json::to_vec(&AspectFieldReconciliationInput {
                    entity_id: entity,
                    field_name: "replicas".to_string(),
                    desired_value: json!(5),
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
        let output: AspectFieldReconciliationOutput =
            serde_json::from_slice(execution.output().canonical_bytes()).expect("output decode");
        let updated_payload = match &execution.mutation_program().worker_batches()[0].intents[0] {
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

        assert!(output.updated);
        assert_eq!(
            updated_payload.get("replicas"),
            Some(&Value::Number(serde_json::Number::from(5_u64)))
        );
        assert_eq!(updated_payload.as_object().expect("object").len(), 1);
    }

    #[test]
    fn aspect_field_reconciliation_strategy_rejects_undeclared_field() {
        let descriptor = AspectFieldReconciliationStrategy::descriptor(
            crate::commit_strategies::data::CommitStrategyId(702),
        );
        let registry = AspectSchemaFixture {
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
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
            .commit_strategy_executor(AspectFieldReconciliationStrategy::execution_registration(
                &descriptor,
            ))
            .build();
        let entity = create_entity(&mut runtime, "before");
        let request = runtime
            .commit_strategies()
            .canonicalize_request(&RawStrategyCommitRequest::new(
                crate::commit_strategies::data::CommitStrategySemanticName::new(
                    AspectFieldReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
                ),
                serde_json::to_vec(&AspectFieldReconciliationInput {
                    entity_id: entity,
                    field_name: "replicas".to_string(),
                    desired_value: json!(5),
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
            .expect_err("undeclared aspect field should be rejected");

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
}
