use serde::{Deserialize, Serialize};

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
use crate::payloads::data::RecordPayload;
use crate::transactions::data::{
    EntityMutationIntent, MutationIntent, UpdateEntityIntent, WorkerIntentBatch,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntentReconciliationInput {
    pub entity_id: EntityId,
    pub desired_payload: serde_json::Value,
}

impl IntentReconciliationInput {
    fn desired_record_payload(&self) -> RecordPayload {
        RecordPayload::from(self.desired_payload.clone())
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
}

impl CommitStrategyExecutor for IntentReconciliationStrategy {
    fn execute(
        &self,
        request: &CanonicalStrategyCommitRequest,
        observation: &StrategyObservationContext<'_>,
    ) -> Result<StrategyExecutionResult, StrategyExecutorFailure> {
        let input = Self::parse_input(request)?;
        let desired_payload = input.desired_record_payload();
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

        let (action, mutation_program) = if existing.payload == desired_payload {
            (
                IntentReconciliationAction::NoChange,
                StrategyMutationProgram::new(Vec::<WorkerIntentBatch>::new()),
            )
        } else {
            let batch = WorkerIntentBatch::new("intent-reconciliation-update").push(
                MutationIntent::Entity(EntityMutationIntent::Update(UpdateEntityIntent {
                    entity_id: input.entity_id,
                    payload: desired_payload,
                })),
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
    use crate::tests::support::{create_entity, test_schema_registry, update_entity};

    #[test]
    fn intent_reconciliation_strategy_emits_update_when_payload_differs() {
        let descriptor = IntentReconciliationStrategy::descriptor(
            crate::commit_strategies::data::CommitStrategyId(501),
        );
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(test_schema_registry())
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
            .schema_registry(test_schema_registry())
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
}
