use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize};

use crate::config::data::RelationalRuntimeConfig;
use crate::history::data::CommitId;
use crate::identity::data::VersionId;
use crate::schema::data::{
    schema_authority_snapshot_digest_bytes, DescriptorCanonicalizationVersion,
    DescriptorSemanticsVersion,
};
use crate::transactions::data::{AspectFieldPatchTarget, CommitValidationSummary};
use std::sync::Arc;

use super::canonical_digest::{
    commit_validation_summary_digest, fallback_intent_scope_digest, lowering_summary_digest,
    preview_validation_cost_digest, runtime_execution_model_digest,
    runtime_invariant_catalog_digest, runtime_planning_contract_digest,
};
use super::native_strategy_intent_scope::{
    native_strategy_intent_scope_digest, native_strategy_intent_scope_targets,
};
use super::{
    CanonicalStrategyCommitRequest, CanonicalStrategyInputArtifact, CanonicalStrategyInputDigest,
    CanonicalStrategyOutputDigest, CommitStrategyDescriptor, CommitStrategyDescriptorDigest,
    CommitStrategyFamilyName, CommitStrategyId, CommitStrategySemanticName, CommitStrategyVersion,
    LoweredStrategyCommitPlan, StrategyCallerProvenance, StrategyInputSchemaName,
    StrategyInputSchemaVersion, StrategyIntentName, StrategyLoweringProvenance,
    StrategyLoweringSummary, StrategyMutationProgramDigest, StrategyOutputSchemaName,
    StrategyPreviewValidationCostSummary, StrategyRequestCanonicalization, StrategyRequestOrigin,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrategyMergeConflictClass {
    IntentReconciliation,
    ReplicaConvergence,
    EntityReplacement,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StrategyIntentScopeDigest([u8; 32]);

impl StrategyIntentScopeDigest {
    pub fn new(digest: [u8; 32]) -> Self {
        Self(digest)
    }

    pub fn bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyMergeSemantics {
    conflict_class: StrategyMergeConflictClass,
    requires_causal_comparison: bool,
    respects_aspect_merge_policies: bool,
}

impl StrategyMergeSemantics {
    pub fn new(
        conflict_class: StrategyMergeConflictClass,
        requires_causal_comparison: bool,
        respects_aspect_merge_policies: bool,
    ) -> Self {
        Self {
            conflict_class,
            requires_causal_comparison,
            respects_aspect_merge_policies,
        }
    }

    pub fn conflict_class(&self) -> StrategyMergeConflictClass {
        self.conflict_class
    }

    pub fn requires_causal_comparison(&self) -> bool {
        self.requires_causal_comparison
    }

    pub fn respects_aspect_merge_policies(&self) -> bool {
        self.respects_aspect_merge_policies
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyMergeDescriptor {
    strategy_id: CommitStrategyId,
    descriptor_digest: CommitStrategyDescriptorDigest,
    semantic_name: CommitStrategySemanticName,
    family_name: CommitStrategyFamilyName,
    version: CommitStrategyVersion,
    intent_name: StrategyIntentName,
    intent_scope_digest: StrategyIntentScopeDigest,
    intent_scope_targets: Arc<[AspectFieldPatchTarget]>,
    merge_semantics: StrategyMergeSemantics,
    lowering_summary_digest: [u8; 32],
}

impl StrategyMergeDescriptor {
    pub fn from_descriptor_and_lowered(
        descriptor: &CommitStrategyDescriptor,
        lowered: &LoweredStrategyCommitPlan,
    ) -> Self {
        Self {
            strategy_id: descriptor.id(),
            descriptor_digest: descriptor.digest(),
            semantic_name: descriptor.semantic_name().clone(),
            family_name: descriptor.family_name().clone(),
            version: descriptor.version(),
            intent_name: descriptor.intent_name().clone(),
            intent_scope_digest: strategy_intent_scope_digest(descriptor, lowered),
            intent_scope_targets: strategy_intent_scope_targets(descriptor, lowered),
            merge_semantics: merge_semantics_for_descriptor(descriptor),
            lowering_summary_digest: lowering_summary_digest(lowered.lowering_summary()),
        }
    }

    pub fn strategy_id(&self) -> CommitStrategyId {
        self.strategy_id
    }

    pub fn descriptor_digest(&self) -> CommitStrategyDescriptorDigest {
        self.descriptor_digest
    }

    pub fn semantic_name(&self) -> &CommitStrategySemanticName {
        &self.semantic_name
    }

    pub fn family_name(&self) -> &CommitStrategyFamilyName {
        &self.family_name
    }

    pub fn version(&self) -> CommitStrategyVersion {
        self.version
    }

    pub fn intent_name(&self) -> &StrategyIntentName {
        &self.intent_name
    }

    pub fn intent_scope_digest(&self) -> StrategyIntentScopeDigest {
        self.intent_scope_digest
    }

    pub fn intent_scope_targets(&self) -> &[AspectFieldPatchTarget] {
        &self.intent_scope_targets
    }

    pub fn merge_semantics(&self) -> StrategyMergeSemantics {
        self.merge_semantics
    }

    pub fn lowering_summary_digest(&self) -> &[u8; 32] {
        &self.lowering_summary_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StrategyReplayDescriptor {
    strategy_id: CommitStrategyId,
    descriptor_digest: CommitStrategyDescriptorDigest,
    input_digest: CanonicalStrategyInputDigest,
    output_digest: CanonicalStrategyOutputDigest,
    mutation_program_digest: StrategyMutationProgramDigest,
    input_schema_name: StrategyInputSchemaName,
    input_schema_version: StrategyInputSchemaVersion,
    input_canonicalization: StrategyRequestCanonicalization,
    output_schema_name: StrategyOutputSchemaName,
    lowering_summary_digest: [u8; 32],
    preview_validation_summary_digest: Option<[u8; 32]>,
    preview_validation_cost_digest: Option<[u8; 32]>,
    validated_against_commit_id: Option<CommitId>,
    validated_against_version_id: Option<VersionId>,
    runtime_determinism_basis: StrategyRuntimeDeterminismBasis,
}

impl StrategyReplayDescriptor {
    pub fn from_lowered(
        lowered: &LoweredStrategyCommitPlan,
        runtime_config: &RelationalRuntimeConfig,
    ) -> Self {
        Self {
            strategy_id: lowered.lowering_provenance().strategy_id(),
            descriptor_digest: lowered.lowering_provenance().descriptor_digest(),
            input_digest: lowered.lowering_provenance().input_digest(),
            output_digest: lowered.lowering_provenance().output_digest(),
            mutation_program_digest: lowered.lowering_provenance().mutation_program_digest(),
            input_schema_name: lowered.request().canonical_input().schema_name().clone(),
            input_schema_version: lowered.request().canonical_input().schema_version(),
            input_canonicalization: lowered.request().canonical_input().canonicalization(),
            output_schema_name: lowered.execution().output().schema_name().clone(),
            lowering_summary_digest: lowering_summary_digest(lowered.lowering_summary()),
            preview_validation_summary_digest: None,
            preview_validation_cost_digest: None,
            validated_against_commit_id: None,
            validated_against_version_id: None,
            runtime_determinism_basis: StrategyRuntimeDeterminismBasis::from_runtime_config(
                runtime_config,
            ),
        }
    }

    fn with_preview_validation(
        mut self,
        preview_validation_summary: &CommitValidationSummary,
        preview_validation_cost: &StrategyPreviewValidationCostSummary,
        validated_against_commit_id: Option<CommitId>,
        validated_against_version_id: VersionId,
    ) -> Self {
        self.preview_validation_summary_digest =
            Some(commit_validation_summary_digest(preview_validation_summary));
        self.preview_validation_cost_digest =
            Some(preview_validation_cost_digest(preview_validation_cost));
        self.validated_against_commit_id = validated_against_commit_id;
        self.validated_against_version_id = Some(validated_against_version_id);
        self
    }

    pub fn strategy_id(&self) -> CommitStrategyId {
        self.strategy_id
    }

    pub fn descriptor_digest(&self) -> CommitStrategyDescriptorDigest {
        self.descriptor_digest
    }

    pub fn input_digest(&self) -> CanonicalStrategyInputDigest {
        self.input_digest
    }

    pub fn output_digest(&self) -> CanonicalStrategyOutputDigest {
        self.output_digest
    }

    pub fn mutation_program_digest(&self) -> StrategyMutationProgramDigest {
        self.mutation_program_digest
    }

    pub fn input_schema_name(&self) -> &StrategyInputSchemaName {
        &self.input_schema_name
    }

    pub fn input_schema_version(&self) -> StrategyInputSchemaVersion {
        self.input_schema_version
    }

    pub fn input_canonicalization(&self) -> StrategyRequestCanonicalization {
        self.input_canonicalization
    }

    pub fn output_schema_name(&self) -> &StrategyOutputSchemaName {
        &self.output_schema_name
    }

    pub fn lowering_summary_digest(&self) -> &[u8; 32] {
        &self.lowering_summary_digest
    }

    pub fn preview_validation_summary_digest(&self) -> Option<&[u8; 32]> {
        self.preview_validation_summary_digest.as_ref()
    }

    pub fn preview_validation_cost_digest(&self) -> Option<&[u8; 32]> {
        self.preview_validation_cost_digest.as_ref()
    }

    pub fn validated_against_commit_id(&self) -> Option<CommitId> {
        self.validated_against_commit_id
    }

    pub fn validated_against_version_id(&self) -> Option<VersionId> {
        self.validated_against_version_id
    }

    pub fn runtime_determinism_basis(&self) -> &StrategyRuntimeDeterminismBasis {
        &self.runtime_determinism_basis
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyRuntimeDeterminismBasis {
    schema_registry_digest: [u8; 32],
    invariant_catalog_digest: [u8; 32],
    planning_contract_digest: [u8; 32],
    execution_model_digest: [u8; 32],
    descriptor_semantics_version: DescriptorSemanticsVersion,
    descriptor_canonicalization_version: DescriptorCanonicalizationVersion,
}

impl StrategyRuntimeDeterminismBasis {
    pub fn from_runtime_config(runtime_config: &RelationalRuntimeConfig) -> Self {
        let schema_authority = runtime_config.schema.registry.authority_snapshot();
        Self {
            schema_registry_digest: schema_authority_snapshot_digest_bytes(&schema_authority),
            invariant_catalog_digest: runtime_invariant_catalog_digest(
                &runtime_config.schema.invariant_catalog,
            ),
            planning_contract_digest: runtime_planning_contract_digest(
                &runtime_config.execution.planning,
            ),
            execution_model_digest: runtime_execution_model_digest(
                runtime_config.execution.execution_model,
            ),
            descriptor_semantics_version: runtime_config
                .schema
                .descriptor_semantics_policy
                .current_write_version(),
            descriptor_canonicalization_version: runtime_config
                .schema
                .descriptor_canonicalization_policy
                .current_write_version(),
        }
    }

    pub fn schema_registry_digest(&self) -> &[u8; 32] {
        &self.schema_registry_digest
    }

    pub fn invariant_catalog_digest(&self) -> &[u8; 32] {
        &self.invariant_catalog_digest
    }

    pub fn planning_contract_digest(&self) -> &[u8; 32] {
        &self.planning_contract_digest
    }

    pub fn execution_model_digest(&self) -> &[u8; 32] {
        &self.execution_model_digest
    }

    pub fn descriptor_semantics_version(&self) -> DescriptorSemanticsVersion {
        self.descriptor_semantics_version
    }

    pub fn descriptor_canonicalization_version(&self) -> DescriptorCanonicalizationVersion {
        self.descriptor_canonicalization_version
    }
}

impl<'de> Deserialize<'de> for StrategyReplayDescriptor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawStrategyReplayDescriptor {
            strategy_id: CommitStrategyId,
            descriptor_digest: CommitStrategyDescriptorDigest,
            input_digest: CanonicalStrategyInputDigest,
            output_digest: CanonicalStrategyOutputDigest,
            mutation_program_digest: StrategyMutationProgramDigest,
            input_schema_name: StrategyInputSchemaName,
            input_schema_version: StrategyInputSchemaVersion,
            input_canonicalization: StrategyRequestCanonicalization,
            output_schema_name: StrategyOutputSchemaName,
            lowering_summary_digest: [u8; 32],
            preview_validation_summary_digest: Option<[u8; 32]>,
            preview_validation_cost_digest: Option<[u8; 32]>,
            validated_against_commit_id: Option<CommitId>,
            validated_against_version_id: Option<VersionId>,
            runtime_determinism_basis: StrategyRuntimeDeterminismBasis,
        }

        let raw = RawStrategyReplayDescriptor::deserialize(deserializer)?;
        Ok(Self {
            strategy_id: raw.strategy_id,
            descriptor_digest: raw.descriptor_digest,
            input_digest: raw.input_digest,
            output_digest: raw.output_digest,
            mutation_program_digest: raw.mutation_program_digest,
            input_schema_name: raw.input_schema_name,
            input_schema_version: raw.input_schema_version,
            input_canonicalization: raw.input_canonicalization,
            output_schema_name: raw.output_schema_name,
            lowering_summary_digest: raw.lowering_summary_digest,
            preview_validation_summary_digest: raw.preview_validation_summary_digest,
            preview_validation_cost_digest: raw.preview_validation_cost_digest,
            validated_against_commit_id: raw.validated_against_commit_id,
            validated_against_version_id: raw.validated_against_version_id,
            runtime_determinism_basis: raw.runtime_determinism_basis,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StrategyCommitArtifactBundle {
    lowering_provenance: StrategyLoweringProvenance,
    lowering_summary: StrategyLoweringSummary,
    canonical_input: CanonicalStrategyInputArtifact,
    merge_descriptor: StrategyMergeDescriptor,
    replay_descriptor: StrategyReplayDescriptor,
    preview_validation_summary: Option<CommitValidationSummary>,
    preview_validation_cost: Option<StrategyPreviewValidationCostSummary>,
    validated_against_commit_id: Option<CommitId>,
    validated_against_version_id: Option<VersionId>,
}

impl StrategyCommitArtifactBundle {
    pub fn from_lowered(
        lowered: &LoweredStrategyCommitPlan,
        descriptor: &CommitStrategyDescriptor,
        runtime_config: &RelationalRuntimeConfig,
    ) -> Self {
        Self {
            lowering_provenance: lowered.lowering_provenance(),
            lowering_summary: lowered.lowering_summary().clone(),
            canonical_input: lowered.request().canonical_input().clone(),
            merge_descriptor: StrategyMergeDescriptor::from_descriptor_and_lowered(
                descriptor, lowered,
            ),
            replay_descriptor: StrategyReplayDescriptor::from_lowered(lowered, runtime_config),
            preview_validation_summary: None,
            preview_validation_cost: None,
            validated_against_commit_id: None,
            validated_against_version_id: None,
        }
    }

    pub fn with_preview_validation(
        mut self,
        preview_validation_summary: CommitValidationSummary,
        preview_validation_cost: StrategyPreviewValidationCostSummary,
        validated_against_commit_id: Option<CommitId>,
        validated_against_version_id: VersionId,
    ) -> Self {
        self.replay_descriptor = self.replay_descriptor.with_preview_validation(
            &preview_validation_summary,
            &preview_validation_cost,
            validated_against_commit_id,
            validated_against_version_id,
        );
        self.preview_validation_summary = Some(preview_validation_summary);
        self.preview_validation_cost = Some(preview_validation_cost);
        self.validated_against_commit_id = validated_against_commit_id;
        self.validated_against_version_id = Some(validated_against_version_id);
        self
    }

    pub fn lowering_provenance(&self) -> StrategyLoweringProvenance {
        self.lowering_provenance
    }

    pub fn lowering_summary(&self) -> &StrategyLoweringSummary {
        &self.lowering_summary
    }

    pub fn canonical_input(&self) -> &CanonicalStrategyInputArtifact {
        &self.canonical_input
    }

    pub fn merge_descriptor(&self) -> &StrategyMergeDescriptor {
        &self.merge_descriptor
    }

    pub fn replay_descriptor(&self) -> &StrategyReplayDescriptor {
        &self.replay_descriptor
    }

    pub fn preview_validation_summary(&self) -> Option<&CommitValidationSummary> {
        self.preview_validation_summary.as_ref()
    }

    pub fn preview_validation_cost(&self) -> Option<StrategyPreviewValidationCostSummary> {
        self.preview_validation_cost
    }

    pub fn validated_against_commit_id(&self) -> Option<CommitId> {
        self.validated_against_commit_id
    }

    pub fn validated_against_version_id(&self) -> Option<VersionId> {
        self.validated_against_version_id
    }

    pub fn replay_request(&self) -> CanonicalStrategyCommitRequest {
        CanonicalStrategyCommitRequest::new(
            self.replay_descriptor.strategy_id(),
            self.replay_descriptor.descriptor_digest(),
            self.canonical_input.clone(),
            StrategyCallerProvenance {
                request_origin: StrategyRequestOrigin::Replay,
                actor_identity: None,
                correlation_id: None,
            },
        )
    }

    fn validate_consistency(&self) -> Result<(), &'static str> {
        if self.lowering_provenance.strategy_id() != self.replay_descriptor.strategy_id() {
            return Err(
                "strategy replay descriptor strategy id does not match lowering provenance",
            );
        }
        if self.lowering_provenance.descriptor_digest()
            != self.replay_descriptor.descriptor_digest()
        {
            return Err(
                "strategy replay descriptor descriptor digest does not match lowering provenance",
            );
        }
        if self.lowering_provenance.input_digest() != self.replay_descriptor.input_digest() {
            return Err(
                "strategy replay descriptor input digest does not match lowering provenance",
            );
        }
        if self.lowering_provenance.output_digest() != self.replay_descriptor.output_digest() {
            return Err(
                "strategy replay descriptor output digest does not match lowering provenance",
            );
        }
        if self.lowering_provenance.mutation_program_digest()
            != self.replay_descriptor.mutation_program_digest()
        {
            return Err(
                "strategy replay descriptor mutation program digest does not match lowering provenance",
            );
        }
        if self.canonical_input.digest() != self.replay_descriptor.input_digest() {
            return Err(
                "strategy canonical input artifact digest does not match strategy replay descriptor",
            );
        }
        if self.canonical_input.schema_name() != self.replay_descriptor.input_schema_name() {
            return Err(
                "strategy canonical input schema name does not match strategy replay descriptor",
            );
        }
        if self.canonical_input.schema_version() != self.replay_descriptor.input_schema_version() {
            return Err(
                "strategy canonical input schema version does not match strategy replay descriptor",
            );
        }
        if self.canonical_input.canonicalization()
            != self.replay_descriptor.input_canonicalization()
        {
            return Err(
                "strategy canonical input canonicalization does not match strategy replay descriptor",
            );
        }
        if lowering_summary_digest(&self.lowering_summary)
            != *self.replay_descriptor.lowering_summary_digest()
        {
            return Err(
                "strategy lowering summary does not match strategy replay descriptor digest",
            );
        }
        if self.merge_descriptor.strategy_id() != self.lowering_provenance.strategy_id() {
            return Err("strategy merge descriptor strategy id does not match lowering provenance");
        }
        if self.merge_descriptor.descriptor_digest() != self.lowering_provenance.descriptor_digest()
        {
            return Err(
                "strategy merge descriptor descriptor digest does not match lowering provenance",
            );
        }
        if self.merge_descriptor.lowering_summary_digest()
            != self.replay_descriptor.lowering_summary_digest()
        {
            return Err(
                "strategy merge descriptor lowering summary digest does not match strategy replay descriptor digest",
            );
        }
        match (
            self.preview_validation_summary.as_ref(),
            self.replay_descriptor.preview_validation_summary_digest(),
        ) {
            (Some(summary), Some(expected_digest))
                if commit_validation_summary_digest(summary) == *expected_digest => {}
            (None, None) => {}
            _ => {
                return Err(
                    "strategy preview validation summary does not match strategy replay descriptor digest",
                )
            }
        }
        match (
            self.preview_validation_cost.as_ref(),
            self.replay_descriptor.preview_validation_cost_digest(),
        ) {
            (Some(summary), Some(expected_digest))
                if preview_validation_cost_digest(summary) == *expected_digest => {}
            (None, None) => {}
            _ => return Err(
                "strategy preview validation cost does not match strategy replay descriptor digest",
            ),
        }
        if self.validated_against_version_id
            != self.replay_descriptor.validated_against_version_id()
        {
            return Err(
                "strategy validated-against version id does not match strategy replay descriptor",
            );
        }
        if self.validated_against_commit_id != self.replay_descriptor.validated_against_commit_id()
        {
            return Err(
                "strategy validated-against commit id does not match strategy replay descriptor",
            );
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for StrategyCommitArtifactBundle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawStrategyCommitArtifactBundle {
            lowering_provenance: StrategyLoweringProvenance,
            lowering_summary: StrategyLoweringSummary,
            canonical_input: CanonicalStrategyInputArtifact,
            merge_descriptor: StrategyMergeDescriptor,
            replay_descriptor: StrategyReplayDescriptor,
            preview_validation_summary: Option<CommitValidationSummary>,
            preview_validation_cost: Option<StrategyPreviewValidationCostSummary>,
            validated_against_commit_id: Option<CommitId>,
            validated_against_version_id: Option<VersionId>,
        }

        let raw = RawStrategyCommitArtifactBundle::deserialize(deserializer)?;
        let bundle = Self {
            lowering_provenance: raw.lowering_provenance,
            lowering_summary: raw.lowering_summary,
            canonical_input: raw.canonical_input,
            merge_descriptor: raw.merge_descriptor,
            replay_descriptor: raw.replay_descriptor,
            preview_validation_summary: raw.preview_validation_summary,
            preview_validation_cost: raw.preview_validation_cost,
            validated_against_commit_id: raw.validated_against_commit_id,
            validated_against_version_id: raw.validated_against_version_id,
        };
        bundle.validate_consistency().map_err(D::Error::custom)?;
        Ok(bundle)
    }
}

fn merge_conflict_class_for_descriptor(
    descriptor: &CommitStrategyDescriptor,
) -> StrategyMergeConflictClass {
    match descriptor.family_name().as_str() {
        "strategy.intent" => StrategyMergeConflictClass::IntentReconciliation,
        "strategy.replica" => StrategyMergeConflictClass::ReplicaConvergence,
        "strategy.replace" => StrategyMergeConflictClass::EntityReplacement,
        _ => StrategyMergeConflictClass::Custom,
    }
}

fn merge_semantics_for_descriptor(descriptor: &CommitStrategyDescriptor) -> StrategyMergeSemantics {
    let requires_causal_comparison =
        !matches!(descriptor.family_name().as_str(), "strategy.aspect");
    StrategyMergeSemantics::new(
        merge_conflict_class_for_descriptor(descriptor),
        requires_causal_comparison,
        true,
    )
}

fn strategy_intent_scope_digest(
    descriptor: &CommitStrategyDescriptor,
    lowered: &LoweredStrategyCommitPlan,
) -> StrategyIntentScopeDigest {
    if let Some(digest) = semantic_intent_scope_digest(descriptor, lowered) {
        return StrategyIntentScopeDigest::new(digest);
    }
    let mut targets = lowered
        .merged_plan()
        .merged_intents
        .iter()
        .filter_map(|intent| intent.existing_record_target())
        .collect::<Vec<_>>();
    targets.sort();
    targets.dedup();
    StrategyIntentScopeDigest::new(fallback_intent_scope_digest(
        lowered.request().strategy_id(),
        lowered.request().canonical_input().schema_name(),
        lowered.request().canonical_input().schema_version(),
        lowered.request().canonical_input().digest(),
        &targets,
    ))
}

fn strategy_intent_scope_targets(
    descriptor: &CommitStrategyDescriptor,
    lowered: &LoweredStrategyCommitPlan,
) -> Arc<[AspectFieldPatchTarget]> {
    semantic_intent_scope_targets(descriptor, lowered)
        .unwrap_or_default()
        .into()
}

fn semantic_intent_scope_digest(
    descriptor: &CommitStrategyDescriptor,
    lowered: &LoweredStrategyCommitPlan,
) -> Option<[u8; 32]> {
    native_strategy_intent_scope_digest(descriptor, lowered)
}

fn semantic_intent_scope_targets(
    descriptor: &CommitStrategyDescriptor,
    lowered: &LoweredStrategyCommitPlan,
) -> Option<Vec<AspectFieldPatchTarget>> {
    native_strategy_intent_scope_targets(descriptor, lowered)
}

#[cfg(test)]
mod tests {
    use super::{
        StrategyCommitArtifactBundle, StrategyIntentScopeDigest, StrategyMergeConflictClass,
        StrategyMergeDescriptor, StrategyMergeSemantics,
    };
    use crate::capabilities::{RuntimeConfigSource, SchemaSource};
    use crate::commit_strategies::data::canonical_digest::native_entity_fields_scope_digest;
    use crate::commit_strategies::data::{
        CanonicalStrategyCommitRequest, CanonicalStrategyInputArtifact,
        CanonicalStrategyInputDigest, CanonicalStrategyOutputArtifact, CommitStrategyDescriptor,
        CommitStrategyFamilyName, CommitStrategyId, CommitStrategySemanticName,
        CommitStrategyVersion, PersistentArtifactName, StrategyCallerProvenance,
        StrategyExecutionDraft, StrategyExecutionResult, StrategyExecutionSummary,
        StrategyInputSchemaName, StrategyInputSchemaVersion, StrategyIntentName,
        StrategyMutationProgram, StrategyOutputSchemaName, StrategyPacketContract,
        StrategyReadContract, StrategyReadCostClass, StrategyReadLocalityClass,
        StrategyReadScopeClass, StrategyRequestCanonicalization, StrategyRequestOrigin,
        StrategyTraversalBasis,
    };
    use crate::facade::transactions::{
        CreateIntent, MutationIntent, TransactionOptions, WorkerIntentBatch,
    };
    use crate::identity::data::{EntityId, KindId, PartitionId};
    use crate::logic::builder::RelationalRuntimeBuilder;
    use crate::symbols::data::ClientKey;
    use crate::transactions::data::{
        AspectFieldPatch, AspectFieldPatchTarget, CommitValidationSummary, EntitySpec,
    };
    use forge_foundational::facade::{AspectKey, AspectValue, FieldKey, InternedString};
    use std::sync::Arc;

    fn descriptor() -> CommitStrategyDescriptor {
        CommitStrategyDescriptor::new(
            CommitStrategyId(41),
            CommitStrategySemanticName::new("strategy.intent.reconcile"),
            CommitStrategyFamilyName::new("strategy.intent"),
            CommitStrategyVersion::new(1, 0),
            StrategyIntentName::new("reconcile.desired.state"),
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
            PersistentArtifactName::new("strategy.intent.reconcile"),
        )
    }

    fn canonical_request() -> CanonicalStrategyCommitRequest {
        let descriptor = descriptor();
        CanonicalStrategyCommitRequest::new(
            CommitStrategyId(41),
            descriptor.digest(),
            CanonicalStrategyInputArtifact::new(
                StrategyInputSchemaName::new("intent.reconcile.input.v1"),
                StrategyInputSchemaVersion(1),
                StrategyRequestCanonicalization::NativeCanonicalBytesV1,
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

    fn execution_draft(request: &CanonicalStrategyCommitRequest) -> StrategyExecutionDraft {
        let batch = WorkerIntentBatch::new("reconcile-deployment").push(MutationIntent::Create(
            CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId(1),
                kind_id: KindId(1),
                client_key: ClientKey::from("deployment-a"),
                fields: AspectFieldPatch::single(
                    AspectKey::new("name").expect("valid name aspect key"),
                    FieldKey::new("name").expect("valid name field key"),
                    AspectValue::String(InternedString::Raw("deployment-a".to_string())),
                ),
            }),
        ));

        StrategyExecutionDraft::from_measured_result(
            request,
            StrategyExecutionResult::new(
                CanonicalStrategyOutputArtifact::new(
                    StrategyOutputSchemaName::new("intent.reconcile.output.v1"),
                    br#"{"status":"planned"}"#.to_vec(),
                    PersistentArtifactName::new("strategy.intent.reconcile.output"),
                ),
                StrategyMutationProgram::new(vec![batch]),
            ),
            StrategyExecutionSummary::default(),
        )
    }

    #[test]
    fn strategy_commit_artifact_bundle_roundtrip_preserves_verified_consistency() {
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(crate::tests::support::test_schema_registry())
            .build();
        let request = canonical_request();
        let execution = execution_draft(&request);
        let lowered = runtime
            .commit_strategies_authority()
            .lower_execution(&request, &execution, TransactionOptions::default())
            .expect("lowered strategy plan");
        let bundle = StrategyCommitArtifactBundle::from_lowered(
            &lowered,
            &descriptor(),
            runtime.runtime_config(),
        );

        let bytes = serde_json::to_vec(&bundle).expect("serialize strategy bundle");
        let roundtripped: StrategyCommitArtifactBundle =
            serde_json::from_slice(&bytes).expect("deserialize strategy bundle");

        assert_eq!(roundtripped, bundle);
        assert_eq!(
            roundtripped.merge_descriptor().semantic_name().as_str(),
            "strategy.intent.reconcile"
        );
        assert_eq!(
            roundtripped
                .replay_request()
                .canonical_input()
                .canonical_bytes(),
            request.canonical_input().canonical_bytes()
        );
        assert_eq!(
            roundtripped
                .replay_descriptor()
                .runtime_determinism_basis()
                .schema_registry_digest(),
            &runtime.schema_registry().authority_digest_bytes()
        );
    }

    #[test]
    fn strategy_commit_artifact_bundle_rejects_drift_between_summary_and_descriptor() {
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(crate::tests::support::test_schema_registry())
            .build();
        let request = canonical_request();
        let execution = execution_draft(&request);
        let lowered = runtime
            .commit_strategies_authority()
            .lower_execution(&request, &execution, TransactionOptions::default())
            .expect("lowered strategy plan");
        let bundle = StrategyCommitArtifactBundle::from_lowered(
            &lowered,
            &descriptor(),
            runtime.runtime_config(),
        );
        let mut value = serde_json::to_value(&bundle).expect("bundle value");
        value["lowering_summary"]["worker_batch_count"] = serde_json::json!(99);

        let error = serde_json::from_value::<StrategyCommitArtifactBundle>(value).unwrap_err();
        assert!(error.to_string().contains(
            "strategy lowering summary does not match strategy replay descriptor digest"
        ));
    }

    #[test]
    fn strategy_commit_artifact_bundle_rejects_preview_validation_cost_drift() {
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(crate::tests::support::test_schema_registry())
            .build();
        let request = canonical_request();
        let execution = execution_draft(&request);
        let lowered = runtime
            .commit_strategies_authority()
            .lower_execution(&request, &execution, TransactionOptions::default())
            .expect("lowered strategy plan");
        let bundle = StrategyCommitArtifactBundle::from_lowered(
            &lowered,
            &descriptor(),
            runtime.runtime_config(),
        )
        .with_preview_validation(
            CommitValidationSummary {
                execution_count: 3,
                ..CommitValidationSummary::default()
            },
            crate::commit_strategies::data::StrategyPreviewValidationCostSummary::new(
                crate::identity::data::VersionId(1),
                1,
                1,
                1,
                0,
                2,
            ),
            None,
            crate::identity::data::VersionId(0),
        );
        let mut value = serde_json::to_value(&bundle).expect("bundle value");
        value["preview_validation_cost"]["post_mutation_preview_pass_count"] = serde_json::json!(3);

        let error = serde_json::from_value::<StrategyCommitArtifactBundle>(value).unwrap_err();
        assert!(error.to_string().contains(
            "strategy preview validation cost does not match strategy replay descriptor digest"
        ));
    }

    #[test]
    fn strategy_intent_scope_targets_preserve_aspect_identity() {
        let field = FieldKey::new("replicas").expect("valid field");
        let desired_target = AspectFieldPatchTarget::single(
            AspectKey::new("deployment.desired").expect("valid desired aspect"),
            field.clone(),
        );
        let observed_target = AspectFieldPatchTarget::single(
            AspectKey::new("deployment.observed").expect("valid observed aspect"),
            field,
        );

        assert_ne!(
            native_entity_fields_scope_digest(
                EntityId::new(PartitionId(1), 7, 0),
                &[desired_target]
            ),
            native_entity_fields_scope_digest(
                EntityId::new(PartitionId(1), 7, 0),
                &[observed_target]
            ),
            "strategy scope digest must not collapse same field path under different aspects"
        );
    }

    #[test]
    fn strategy_merge_descriptor_roundtrips_typed_intent_scope_targets() {
        let field = FieldKey::new("replicas").expect("valid field");
        let target = AspectFieldPatchTarget::single(
            AspectKey::new("deployment.desired").expect("valid aspect"),
            field,
        );
        let descriptor = StrategyMergeDescriptor {
            strategy_id: CommitStrategyId(41),
            descriptor_digest: descriptor().digest(),
            semantic_name: CommitStrategySemanticName::new("strategy.intent.reconcile"),
            family_name: CommitStrategyFamilyName::new("strategy.intent"),
            version: CommitStrategyVersion::new(1, 0),
            intent_name: StrategyIntentName::new("reconcile.desired.state"),
            intent_scope_digest: StrategyIntentScopeDigest::new([5; 32]),
            intent_scope_targets: Arc::from([target.clone()]),
            merge_semantics: StrategyMergeSemantics::new(
                StrategyMergeConflictClass::IntentReconciliation,
                true,
                true,
            ),
            lowering_summary_digest: [9; 32],
        };

        let bytes = serde_json::to_vec(&descriptor).expect("serialize merge descriptor");
        let roundtripped: StrategyMergeDescriptor =
            serde_json::from_slice(&bytes).expect("deserialize merge descriptor");

        assert_eq!(roundtripped.intent_scope_targets(), &[target]);
    }
}
