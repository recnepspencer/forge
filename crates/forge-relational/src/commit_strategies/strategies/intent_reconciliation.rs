use serde::{Deserialize, Serialize};

use forge_foundational::facade::FieldKey;

use crate::commit_strategies::data::{
    decode_aspect_field_patch, decode_entity_id, encode_aspect_field_patch, encode_entity_id,
    CanonicalStrategyCommitRequest, CanonicalStrategyOutputArtifact, CommitStrategyDescriptor,
    CommitStrategyExecutionRegistration, CommitStrategyExecutor, CommitStrategyFamilyName,
    CommitStrategyId, CommitStrategyRegistration, CommitStrategyRegistrationError,
    CommitStrategySemanticName, CommitStrategyVersion, NativeCodecError, NativeCodecReader,
    NativeStrategyCommitRequest, PersistentArtifactName, StrategyCallerProvenance,
    StrategyExecutionResult, StrategyExecutorFailure, StrategyExecutorFailureClass,
    StrategyInputSchemaName, StrategyInputSchemaVersion, StrategyIntentName,
    StrategyMutationProgram, StrategyObservationContext, StrategyOutputSchemaName,
    StrategyPacketContract, StrategyReadContract, StrategyReadCostClass, StrategyReadLocalityClass,
    StrategyReadScopeClass, StrategyRequestCanonicalization, StrategyTraversalBasis,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntentReconciliationInput {
    pub entity_id: EntityId,
    pub desired_fields: AspectFieldPatch,
}

impl IntentReconciliationInput {
    pub fn into_native_canonical_request(
        self,
        caller_provenance: StrategyCallerProvenance,
    ) -> Result<NativeStrategyCommitRequest, NativeCodecError> {
        let mut bytes = Vec::new();
        encode_entity_id(&mut bytes, self.entity_id);
        encode_aspect_field_patch(&mut bytes, &self.desired_fields)?;
        Ok(NativeStrategyCommitRequest::from_native_canonical_bytes(
            CommitStrategySemanticName::new(IntentReconciliationStrategy::DEFAULT_SEMANTIC_NAME),
            bytes,
            caller_provenance,
        ))
    }

    fn decode(bytes: &[u8]) -> Result<Self, NativeCodecError> {
        let mut reader = NativeCodecReader::new(bytes);
        let entity_id = decode_entity_id(&mut reader)?;
        let desired_fields = decode_aspect_field_patch(&mut reader)?;
        reader.finish()?;
        Ok(Self {
            entity_id,
            desired_fields,
        })
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

impl IntentReconciliationOutput {
    pub fn decode(bytes: &[u8]) -> Result<Self, NativeCodecError> {
        let mut reader = NativeCodecReader::new(bytes);
        let entity_id = decode_entity_id(&mut reader)?;
        let action = match reader.read_u8()? {
            0 => IntentReconciliationAction::NoChange,
            1 => IntentReconciliationAction::UpdateEntity,
            tag => {
                return Err(NativeCodecError::new(format!(
                    "unknown intent reconciliation action tag {tag}"
                )))
            }
        };
        reader.finish()?;
        Ok(Self { entity_id, action })
    }
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
    ) -> Result<IntentReconciliationInput, StrategyExecutorFailure> {
        IntentReconciliationInput::decode(request.canonical_input().canonical_bytes()).map_err(
            |error| {
                StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::InvalidInput,
                    format!(
                        "intent reconciliation input could not be decoded: {}",
                        error.detail()
                    ),
                )
            },
        )
    }

    fn output_artifact(output: &IntentReconciliationOutput) -> CanonicalStrategyOutputArtifact {
        let mut bytes = Vec::new();
        encode_entity_id(&mut bytes, output.entity_id);
        bytes.push(match output.action {
            IntentReconciliationAction::NoChange => 0,
            IntentReconciliationAction::UpdateEntity => 1,
        });
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
                            "lowered foundational entity aspect plan is missing for kind {} during intent reconciliation",
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
        desired_fields: &AspectFieldPatch,
    ) -> Result<AspectFieldPatch, StrategyExecutorFailure> {
        for target in desired_fields.targets() {
            let [field] = target.field_path().fields() else {
                return Err(StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::DomainRejection,
                    format!(
                        "intent reconciliation target field path '{}' is not a single foundational field path",
                        crate::transactions::data::aspect_field_patch_target_label(target)
                    ),
                ));
            };
            let aspect_key =
                Self::require_lowered_entity_scalar_field(observation, existing, field)?;
            if &aspect_key != target.aspect_key() {
                return Err(StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::DomainRejection,
                    format!(
                        "intent reconciliation target '{}' does not match lowered scalar aspect {:?}",
                        crate::transactions::data::aspect_field_patch_target_label(target),
                        aspect_key
                    ),
                ));
            }
        }
        Ok(desired_fields.clone())
    }

    fn authoritative_field_matches(
        existing: &crate::storage::data::EntityReadRecord,
        target: &crate::transactions::data::AspectFieldPatchTarget,
        desired_value: &forge_foundational::facade::AspectValue,
    ) -> bool {
        let desired_comparison_key = authoritative_aspect_value_field_comparison_key(desired_value);
        entity_authoritative_aspect_field_comparison_key(existing, target.locator())
            == Some(desired_comparison_key)
    }
}

impl CommitStrategyExecutor for IntentReconciliationStrategy {
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
                        "intent reconciliation target entity {:?} is not visible in the committed basis",
                        input.entity_id
                    ),
                )
            })?;
        let desired_fields = Self::reconcile_fields(observation, &existing, &input.desired_fields)?;
        let unchanged = desired_fields.iter().all(|(target, desired_value)| {
            let [_field] = target.field_path().fields() else {
                return false;
            };
            Self::authoritative_field_matches(&existing, target, desired_value)
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
        });
        Ok(StrategyExecutionResult::new(output, mutation_program))
    }
}

#[cfg(test)]
mod tests;
