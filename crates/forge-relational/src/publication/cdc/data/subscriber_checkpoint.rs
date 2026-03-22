use crate::publication::patch::data::PatchStreamPosition;
use crate::replay::data::ReplaySchemaVersion;
use crate::schema::data::{DescriptorSemanticsVersion, SchemaVersionId};
use serde::{Deserialize, Serialize};

use super::{
    NormalizedContinuationProof, SubscriberContinuationAssessment, SubscriberContinuationSummary,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SubscriberCheckpointBasis {
    position: PatchStreamPosition,
    replay_schema_version: ReplaySchemaVersion,
    schema_version: SchemaVersionId,
}

impl SubscriberCheckpointBasis {
    pub(crate) fn new(
        position: PatchStreamPosition,
        replay_schema_version: ReplaySchemaVersion,
        schema_version: SchemaVersionId,
    ) -> Self {
        Self {
            position,
            replay_schema_version,
            schema_version,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriberCheckpoint {
    position: PatchStreamPosition,
    replay_schema_version: ReplaySchemaVersion,
    schema_version: SchemaVersionId,
    subscriber_contract_id: String,
    normalized_continuation_proof: NormalizedContinuationProof,
    continuation_summary: SubscriberContinuationSummary,
    descriptor_semantics_version: DescriptorSemanticsVersion,
}

impl SubscriberCheckpoint {
    pub(crate) fn from_assessment(
        position: PatchStreamPosition,
        replay_schema_version: ReplaySchemaVersion,
        schema_version: SchemaVersionId,
        subscriber_contract_id: String,
        continuation_assessment: &SubscriberContinuationAssessment,
        descriptor_semantics_version: DescriptorSemanticsVersion,
    ) -> Self {
        Self {
            position,
            replay_schema_version,
            schema_version,
            subscriber_contract_id,
            normalized_continuation_proof: continuation_assessment
                .normalized_continuation_proof
                .clone(),
            continuation_summary: continuation_assessment.continuation_summary.clone(),
            descriptor_semantics_version,
        }
    }

    pub(crate) fn from_basis_with_assessment(
        basis: SubscriberCheckpointBasis,
        subscriber_contract_id: String,
        continuation_assessment: &SubscriberContinuationAssessment,
        descriptor_semantics_version: DescriptorSemanticsVersion,
    ) -> Self {
        Self::from_assessment(
            basis.position,
            basis.replay_schema_version,
            basis.schema_version,
            subscriber_contract_id,
            continuation_assessment,
            descriptor_semantics_version,
        )
    }

    pub fn position(&self) -> PatchStreamPosition {
        self.position
    }

    pub fn replay_schema_version(&self) -> &ReplaySchemaVersion {
        &self.replay_schema_version
    }

    pub fn schema_version(&self) -> SchemaVersionId {
        self.schema_version
    }

    pub fn subscriber_contract_id(&self) -> &str {
        &self.subscriber_contract_id
    }

    pub fn normalized_continuation_proof(&self) -> &NormalizedContinuationProof {
        &self.normalized_continuation_proof
    }

    pub fn continuation_summary(&self) -> &SubscriberContinuationSummary {
        &self.continuation_summary
    }

    pub fn descriptor_semantics_version(&self) -> DescriptorSemanticsVersion {
        self.descriptor_semantics_version
    }

    #[cfg(test)]
    pub(crate) fn with_incoherent_continuation_for_test(
        mut self,
        subscriber_contract_id: String,
        normalized_continuation_proof: NormalizedContinuationProof,
        continuation_summary: SubscriberContinuationSummary,
        descriptor_semantics_version: DescriptorSemanticsVersion,
    ) -> Self {
        self.subscriber_contract_id = subscriber_contract_id;
        self.normalized_continuation_proof = normalized_continuation_proof;
        self.continuation_summary = continuation_summary;
        self.descriptor_semantics_version = descriptor_semantics_version;
        self
    }
}
