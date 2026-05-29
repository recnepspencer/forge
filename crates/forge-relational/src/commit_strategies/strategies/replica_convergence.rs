use forge_foundational::facade::{
    AspectFieldLocator, AspectValue, CanonicalFieldPath, FieldKey, LocatorAuthority,
};
use serde::{Deserialize, Serialize};

use crate::commit_strategies::data::{
    decode_entity_id, encode_entity_id, encode_u64, CanonicalStrategyCommitRequest,
    CanonicalStrategyOutputArtifact, CommitStrategyDescriptor, CommitStrategyExecutionRegistration,
    CommitStrategyExecutor, CommitStrategyFamilyName, CommitStrategyId, CommitStrategyRegistration,
    CommitStrategyRegistrationError, CommitStrategySemanticName, CommitStrategyVersion,
    NativeCodecError, NativeCodecReader, PersistentArtifactName, RawStrategyCommitRequest,
    StrategyCallerProvenance, StrategyExecutionResult, StrategyExecutorFailure,
    StrategyExecutorFailureClass, StrategyInputSchemaName, StrategyInputSchemaVersion,
    StrategyIntentName, StrategyMutationProgram, StrategyObservationContext,
    StrategyOutputSchemaName, StrategyPacketContract, StrategyReadContract, StrategyReadCostClass,
    StrategyReadLocalityClass, StrategyReadScopeClass, StrategyRequestCanonicalization,
    StrategyTraversalBasis,
};
use crate::identity::data::EntityId;
use crate::storage::data::{
    authoritative_aspect_value_field_comparison_key,
    entity_authoritative_aspect_field_comparison_key,
};
use crate::transactions::data::AspectFieldPatch;
use crate::transactions::data::{
    EntityMutationIntent, MutationIntent, UpdateEntityFieldsIntent, WorkerIntentBatch,
};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplicaConvergenceInput {
    pub entity_id: EntityId,
    pub desired_replicas: u64,
}

impl ReplicaConvergenceInput {
    pub fn into_native_canonical_request(
        self,
        caller_provenance: StrategyCallerProvenance,
    ) -> Result<RawStrategyCommitRequest, NativeCodecError> {
        let mut bytes = Vec::new();
        encode_entity_id(&mut bytes, self.entity_id);
        encode_u64(&mut bytes, self.desired_replicas);
        Ok(RawStrategyCommitRequest::from_canonical_bytes(
            CommitStrategySemanticName::new(ReplicaConvergenceStrategy::DEFAULT_SEMANTIC_NAME),
            bytes,
            caller_provenance,
        ))
    }

    fn decode(bytes: &[u8]) -> Result<Self, NativeCodecError> {
        let mut reader = NativeCodecReader::new(bytes);
        let entity_id = decode_entity_id(&mut reader)?;
        let desired_replicas = reader.read_u64()?;
        reader.finish()?;
        Ok(Self {
            entity_id,
            desired_replicas,
        })
    }
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

impl ReplicaConvergenceOutput {
    pub fn decode(bytes: &[u8]) -> Result<Self, NativeCodecError> {
        let mut reader = NativeCodecReader::new(bytes);
        let entity_id = decode_entity_id(&mut reader)?;
        let action = match reader.read_u8()? {
            0 => ReplicaConvergenceAction::NoChange,
            1 => ReplicaConvergenceAction::UpdateReplicas,
            tag => {
                return Err(NativeCodecError::new(format!(
                    "unknown replica convergence action tag {tag}"
                )))
            }
        };
        let desired_replicas = reader.read_u64()?;
        reader.finish()?;
        Ok(Self {
            entity_id,
            action,
            desired_replicas,
        })
    }
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
            StrategyRequestCanonicalization::NativeCanonicalBytesV1,
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
        ReplicaConvergenceInput::decode(request.canonical_input().canonical_bytes()).map_err(
            |error| {
                StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::InvalidInput,
                    format!(
                        "replica convergence input could not be decoded: {}",
                        error.detail()
                    ),
                )
            },
        )
    }

    fn output_artifact(output: &ReplicaConvergenceOutput) -> CanonicalStrategyOutputArtifact {
        let mut bytes = Vec::new();
        encode_entity_id(&mut bytes, output.entity_id);
        bytes.push(match output.action {
            ReplicaConvergenceAction::NoChange => 0,
            ReplicaConvergenceAction::UpdateReplicas => 1,
        });
        encode_u64(&mut bytes, output.desired_replicas);
        CanonicalStrategyOutputArtifact::new(
            StrategyOutputSchemaName::new(Self::DEFAULT_OUTPUT_SCHEMA_NAME),
            bytes,
            PersistentArtifactName::new(Self::DEFAULT_ARTIFACT_NAME),
        )
    }

    fn require_lowered_entity_scalar_field(
        observation: &StrategyObservationContext<'_>,
        existing: &crate::storage::data::EntityReadRecord,
        field: &FieldKey,
    ) -> Result<forge_foundational::facade::AspectKey, StrategyExecutorFailure> {
        let lowered_plan =
            observation
                .entity_aspect_plan(existing.kind.kind_id)
                .ok_or_else(|| {
                    StrategyExecutorFailure::new(
                        StrategyExecutorFailureClass::DomainRejection,
                        format!(
                            "lowered foundational entity aspect plan is missing for kind {} during replica convergence",
                            existing.kind.kind_name
                        ),
                    )
                })?;
        lowered_plan
            .entity_scalar_field_aspect_key(field)
            .ok_or_else(|| {
                StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::DomainRejection,
                    format!(
                        "field '{}' is not a lowered foundational scalar entity aspect on kind {}",
                        field.as_str(),
                        existing.kind.kind_name
                    ),
                )
            })
    }

    fn reconcile_fields(
        observation: &StrategyObservationContext<'_>,
        existing: &crate::storage::data::EntityReadRecord,
        desired_replicas: u64,
    ) -> Result<AspectFieldPatch, StrategyExecutorFailure> {
        let replicas = FieldKey::new("replicas").expect("static field key must be valid");
        let aspect_key =
            Self::require_lowered_entity_scalar_field(observation, existing, &replicas)?;
        Ok(AspectFieldPatch::single(
            aspect_key,
            replicas,
            AspectValue::UInt64(desired_replicas),
        ))
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
        let replicas = FieldKey::new("replicas").expect("static field key must be valid");
        let replicas_aspect_key =
            Self::require_lowered_entity_scalar_field(observation, &existing, &replicas)?;
        let desired_replicas_value = desired_fields
            .get_single_field(&replicas_aspect_key, &replicas)
            .cloned()
            .expect("replicas field");
        let desired_replicas_comparison_key =
            authoritative_aspect_value_field_comparison_key(&desired_replicas_value);

        let replicas_field_locator = AspectFieldLocator::new(
            LocatorAuthority::Planned,
            replicas_aspect_key,
            CanonicalFieldPath::single(replicas.clone()),
        );

        let (action, mutation_program) =
            if entity_authoritative_aspect_field_comparison_key(&existing, &replicas_field_locator)
                == Some(desired_replicas_comparison_key)
            {
                (
                    ReplicaConvergenceAction::NoChange,
                    StrategyMutationProgram::new(Vec::<WorkerIntentBatch>::new()),
                )
            } else {
                let batch = WorkerIntentBatch::new("replica-convergence-update").push(
                    MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                        UpdateEntityFieldsIntent {
                            entity_id: input.entity_id,
                            fields: desired_fields,
                        },
                    )),
                );
                (
                    ReplicaConvergenceAction::UpdateReplicas,
                    StrategyMutationProgram::new(vec![batch]),
                )
            };

        let output = Self::output_artifact(&ReplicaConvergenceOutput {
            entity_id: input.entity_id,
            action,
            desired_replicas: input.desired_replicas,
        });
        Ok(StrategyExecutionResult::new(output, mutation_program))
    }
}
