use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};
use std::io::{self, Write};

use crate::identity::data::VersionId;
use crate::transactions::data::CommitValidationSummary;

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
    Custom,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyMergeDescriptor {
    strategy_id: CommitStrategyId,
    descriptor_digest: CommitStrategyDescriptorDigest,
    semantic_name: CommitStrategySemanticName,
    family_name: CommitStrategyFamilyName,
    version: CommitStrategyVersion,
    intent_name: StrategyIntentName,
    conflict_class: StrategyMergeConflictClass,
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
            conflict_class: merge_conflict_class_for_descriptor(descriptor),
            lowering_summary_digest: stable_digest(lowered.lowering_summary()),
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

    pub fn conflict_class(&self) -> StrategyMergeConflictClass {
        self.conflict_class
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
    validated_against_version_id: Option<VersionId>,
}

impl StrategyReplayDescriptor {
    pub fn from_lowered(lowered: &LoweredStrategyCommitPlan) -> Self {
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
            lowering_summary_digest: stable_digest(lowered.lowering_summary()),
            preview_validation_summary_digest: None,
            preview_validation_cost_digest: None,
            validated_against_version_id: None,
        }
    }

    fn with_preview_validation(
        mut self,
        preview_validation_summary: &CommitValidationSummary,
        preview_validation_cost: &StrategyPreviewValidationCostSummary,
        validated_against_version_id: VersionId,
    ) -> Self {
        self.preview_validation_summary_digest = Some(stable_digest(preview_validation_summary));
        self.preview_validation_cost_digest = Some(stable_digest(preview_validation_cost));
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

    pub fn validated_against_version_id(&self) -> Option<VersionId> {
        self.validated_against_version_id
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
            validated_against_version_id: Option<VersionId>,
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
            validated_against_version_id: raw.validated_against_version_id,
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
    validated_against_version_id: Option<VersionId>,
}

impl StrategyCommitArtifactBundle {
    pub fn from_lowered(
        lowered: &LoweredStrategyCommitPlan,
        descriptor: &CommitStrategyDescriptor,
    ) -> Self {
        Self {
            lowering_provenance: lowered.lowering_provenance(),
            lowering_summary: lowered.lowering_summary().clone(),
            canonical_input: lowered.request().canonical_input().clone(),
            merge_descriptor: StrategyMergeDescriptor::from_descriptor_and_lowered(
                descriptor, lowered,
            ),
            replay_descriptor: StrategyReplayDescriptor::from_lowered(lowered),
            preview_validation_summary: None,
            preview_validation_cost: None,
            validated_against_version_id: None,
        }
    }

    pub fn with_preview_validation(
        mut self,
        preview_validation_summary: CommitValidationSummary,
        preview_validation_cost: StrategyPreviewValidationCostSummary,
        validated_against_version_id: VersionId,
    ) -> Self {
        self.replay_descriptor = self.replay_descriptor.with_preview_validation(
            &preview_validation_summary,
            &preview_validation_cost,
            validated_against_version_id,
        );
        self.preview_validation_summary = Some(preview_validation_summary);
        self.preview_validation_cost = Some(preview_validation_cost);
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
        if stable_digest(&self.lowering_summary)
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
                if stable_digest(summary) == *expected_digest => {}
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
                if stable_digest(summary) == *expected_digest => {}
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
            validated_against_version_id: raw.validated_against_version_id,
        };
        bundle.validate_consistency().map_err(D::Error::custom)?;
        Ok(bundle)
    }
}

struct DigestWriter<'a> {
    hasher: &'a mut Sha256,
}

impl Write for DigestWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.hasher.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn stable_digest<T: Serialize + ?Sized>(value: &T) -> [u8; 32] {
    let mut hasher = Sha256::new();
    if let Err(error) = serde_json::to_writer(
        DigestWriter {
            hasher: &mut hasher,
        },
        value,
    ) {
        hasher.update(b"stable-digest-serialization-error:");
        hasher.update(std::any::type_name::<T>().as_bytes());
        hasher.update(b":");
        hasher.update(error.to_string().as_bytes());
    }
    let digest = hasher.finalize();
    let mut out = [0_u8; 32];
    out.copy_from_slice(&digest);
    out
}

fn merge_conflict_class_for_descriptor(
    descriptor: &CommitStrategyDescriptor,
) -> StrategyMergeConflictClass {
    match descriptor.family_name().as_str() {
        "strategy.intent" => StrategyMergeConflictClass::IntentReconciliation,
        "strategy.replica" => StrategyMergeConflictClass::ReplicaConvergence,
        _ => StrategyMergeConflictClass::Custom,
    }
}

#[cfg(test)]
mod tests {
    use super::StrategyCommitArtifactBundle;
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
    use crate::identity::data::{KindId, PartitionId};
    use crate::logic::builder::RelationalRuntimeBuilder;
    use crate::payloads::data::RecordPayload;
    use crate::symbols::data::InternedString;
    use crate::transactions::data::{CommitValidationSummary, EntitySpec};
    use serde_json::json;

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
            StrategyRequestCanonicalization::JsonStableObjectOrderV1,
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
                StrategyRequestCanonicalization::JsonStableObjectOrderV1,
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
                client_key: InternedString::from("deployment-a"),
                payload: RecordPayload::from(json!({"replicas": 3})),
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
        let bundle = StrategyCommitArtifactBundle::from_lowered(&lowered, &descriptor());

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
        let bundle = StrategyCommitArtifactBundle::from_lowered(&lowered, &descriptor());
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
        let bundle = StrategyCommitArtifactBundle::from_lowered(&lowered, &descriptor())
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
                crate::identity::data::VersionId(0),
            );
        let mut value = serde_json::to_value(&bundle).expect("bundle value");
        value["preview_validation_cost"]["post_mutation_preview_pass_count"] = serde_json::json!(3);

        let error = serde_json::from_value::<StrategyCommitArtifactBundle>(value).unwrap_err();
        assert!(error.to_string().contains(
            "strategy preview validation cost does not match strategy replay descriptor digest"
        ));
    }
}
