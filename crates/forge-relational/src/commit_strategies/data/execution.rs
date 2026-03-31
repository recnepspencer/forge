use std::fmt;
use std::sync::Arc;
use std::{cell::RefCell, collections::BTreeSet};

use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
use crate::snapshots::data::SnapshotHandle;
use crate::storage::data::{EntityReadRecord, RelationReadRecord};
use crate::transactions::data::WorkerIntentBatch;
use crate::visibility::materialization::read_records::{
    EntityRecordProjection, RelationRecordProjection, VisibilityProjectionView,
};

use super::{
    CanonicalStrategyCommitRequest, CanonicalStrategyInputDigest, CommitStrategyDescriptor,
    CommitStrategyDescriptorDigest, CommitStrategyId, PersistentArtifactName,
    StrategyOutputSchemaName, StrategyReadContract,
};

pub trait CommitStrategyExecutor: Send + Sync + 'static {
    fn execute(
        &self,
        request: &CanonicalStrategyCommitRequest,
        observation: &StrategyObservationContext<'_>,
    ) -> Result<StrategyExecutionResult, StrategyExecutorFailure>;
}

#[derive(Clone)]
pub struct CommitStrategyExecutionRegistration {
    strategy_id: CommitStrategyId,
    descriptor_digest: CommitStrategyDescriptorDigest,
    executor: Arc<dyn CommitStrategyExecutor>,
}

impl fmt::Debug for CommitStrategyExecutionRegistration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommitStrategyExecutionRegistration")
            .field("strategy_id", &self.strategy_id)
            .field("descriptor_digest", &self.descriptor_digest)
            .finish()
    }
}

impl CommitStrategyExecutionRegistration {
    pub fn new<E>(descriptor: &CommitStrategyDescriptor, executor: E) -> Self
    where
        E: CommitStrategyExecutor,
    {
        Self {
            strategy_id: descriptor.id(),
            descriptor_digest: descriptor.digest(),
            executor: Arc::new(executor),
        }
    }

    pub fn strategy_id(&self) -> CommitStrategyId {
        self.strategy_id
    }

    pub fn descriptor_digest(&self) -> CommitStrategyDescriptorDigest {
        self.descriptor_digest
    }

    pub(crate) fn executor(&self) -> Arc<dyn CommitStrategyExecutor> {
        Arc::clone(&self.executor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrategyExecutorFailureClass {
    InvalidInput,
    ReadContractViolation,
    ProjectionContractViolation,
    DomainRejection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyExecutorFailure {
    pub class: StrategyExecutorFailureClass,
    pub detail: Arc<str>,
}

impl StrategyExecutorFailure {
    pub fn new(class: StrategyExecutorFailureClass, detail: impl Into<Arc<str>>) -> Self {
        Self {
            class,
            detail: detail.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CanonicalStrategyOutputDigest(pub [u8; 32]);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CanonicalStrategyOutputArtifact {
    schema_name: StrategyOutputSchemaName,
    canonical_bytes: Arc<[u8]>,
    digest: CanonicalStrategyOutputDigest,
    artifact_name: PersistentArtifactName,
}

impl CanonicalStrategyOutputArtifact {
    pub fn new(
        schema_name: StrategyOutputSchemaName,
        canonical_bytes: impl Into<Arc<[u8]>>,
        artifact_name: PersistentArtifactName,
    ) -> Self {
        let canonical_bytes = canonical_bytes.into();
        Self {
            schema_name,
            digest: compute_output_digest(&canonical_bytes),
            canonical_bytes,
            artifact_name,
        }
    }

    pub fn schema_name(&self) -> &StrategyOutputSchemaName {
        &self.schema_name
    }

    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }

    pub fn digest(&self) -> CanonicalStrategyOutputDigest {
        self.digest
    }

    pub fn artifact_name(&self) -> &PersistentArtifactName {
        &self.artifact_name
    }
}

impl<'de> Deserialize<'de> for CanonicalStrategyOutputArtifact {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawCanonicalStrategyOutputArtifact {
            schema_name: StrategyOutputSchemaName,
            canonical_bytes: Arc<[u8]>,
            digest: CanonicalStrategyOutputDigest,
            artifact_name: PersistentArtifactName,
        }

        let raw = RawCanonicalStrategyOutputArtifact::deserialize(deserializer)?;
        let expected_digest = compute_output_digest(&raw.canonical_bytes);
        if raw.digest != expected_digest {
            return Err(D::Error::custom(
                "strategy output digest does not match canonical output bytes",
            ));
        }
        Ok(Self {
            schema_name: raw.schema_name,
            canonical_bytes: raw.canonical_bytes,
            digest: raw.digest,
            artifact_name: raw.artifact_name,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StrategyExecutionSummary {
    pub entity_record_reads: usize,
    pub relation_record_reads: usize,
    pub projected_partition_reads: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyRequestBinding {
    strategy_id: CommitStrategyId,
    descriptor_digest: CommitStrategyDescriptorDigest,
    input_digest: CanonicalStrategyInputDigest,
}

impl StrategyRequestBinding {
    fn from_request(request: &CanonicalStrategyCommitRequest) -> Self {
        Self {
            strategy_id: request.strategy_id(),
            descriptor_digest: request.descriptor_digest(),
            input_digest: request.canonical_input().digest(),
        }
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StrategyMutationProgramDigest(pub [u8; 32]);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StrategyMutationProgram {
    worker_batches: Arc<[WorkerIntentBatch]>,
    digest: StrategyMutationProgramDigest,
    total_intent_count: usize,
}

impl StrategyMutationProgram {
    pub fn new(worker_batches: impl Into<Arc<[WorkerIntentBatch]>>) -> Self {
        let worker_batches = worker_batches.into();
        let digest = compute_mutation_program_digest(&worker_batches);
        let total_intent_count = worker_batches.iter().map(|batch| batch.intents.len()).sum();
        Self {
            worker_batches,
            digest,
            total_intent_count,
        }
    }

    pub fn worker_batches(&self) -> &[WorkerIntentBatch] {
        &self.worker_batches
    }

    pub fn digest(&self) -> StrategyMutationProgramDigest {
        self.digest
    }

    pub fn total_intent_count(&self) -> usize {
        self.total_intent_count
    }
}

impl<'de> Deserialize<'de> for StrategyMutationProgram {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawStrategyMutationProgram {
            worker_batches: Arc<[WorkerIntentBatch]>,
            digest: StrategyMutationProgramDigest,
            total_intent_count: usize,
        }

        let raw = RawStrategyMutationProgram::deserialize(deserializer)?;
        let expected_digest = compute_mutation_program_digest(&raw.worker_batches);
        if raw.digest != expected_digest {
            return Err(D::Error::custom(
                "strategy mutation program digest does not match canonical worker batches",
            ));
        }
        let expected_total_intent_count: usize = raw
            .worker_batches
            .iter()
            .map(|batch| batch.intents.len())
            .sum();
        if raw.total_intent_count != expected_total_intent_count {
            return Err(D::Error::custom(
                "strategy mutation program intent count does not match canonical worker batches",
            ));
        }
        Ok(Self {
            worker_batches: raw.worker_batches,
            digest: raw.digest,
            total_intent_count: raw.total_intent_count,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyExecutionResult {
    output: CanonicalStrategyOutputArtifact,
    mutation_program: StrategyMutationProgram,
}

impl StrategyExecutionResult {
    pub fn new(
        output: CanonicalStrategyOutputArtifact,
        mutation_program: StrategyMutationProgram,
    ) -> Self {
        Self {
            output,
            mutation_program,
        }
    }

    pub fn output(&self) -> &CanonicalStrategyOutputArtifact {
        &self.output
    }

    pub fn mutation_program(&self) -> &StrategyMutationProgram {
        &self.mutation_program
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyExecutionDraft {
    request_binding: StrategyRequestBinding,
    output: CanonicalStrategyOutputArtifact,
    mutation_program: StrategyMutationProgram,
    summary: StrategyExecutionSummary,
}

impl StrategyExecutionDraft {
    pub(crate) fn from_measured_result(
        request: &CanonicalStrategyCommitRequest,
        result: StrategyExecutionResult,
        summary: StrategyExecutionSummary,
    ) -> Self {
        Self {
            request_binding: StrategyRequestBinding::from_request(request),
            output: result.output,
            mutation_program: result.mutation_program,
            summary,
        }
    }

    pub fn request_binding(&self) -> &StrategyRequestBinding {
        &self.request_binding
    }

    pub fn output(&self) -> &CanonicalStrategyOutputArtifact {
        &self.output
    }

    pub fn mutation_program(&self) -> &StrategyMutationProgram {
        &self.mutation_program
    }

    pub fn summary(&self) -> StrategyExecutionSummary {
        self.summary
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StrategyVisibilityReadView<'observation, 'runtime> {
    projection: VisibilityProjectionView<'runtime>,
    read_contract: &'runtime StrategyReadContract,
    metrics: &'observation RefCell<StrategyObservationMetrics>,
}

impl<'observation, 'runtime> StrategyVisibilityReadView<'observation, 'runtime> {
    const fn new(
        projection: VisibilityProjectionView<'runtime>,
        read_contract: &'runtime StrategyReadContract,
        metrics: &'observation RefCell<StrategyObservationMetrics>,
    ) -> Self {
        Self {
            projection,
            read_contract,
            metrics,
        }
    }

    pub fn version_id(&self) -> VersionId {
        self.projection.version_id()
    }

    pub fn entities<T: EntityRecordProjection>(&self) -> Result<Vec<T>, StrategyExecutorFailure> {
        self.admit_kind_scan(None, "cross-partition entity kind scan")?;
        let values = self.safe_projection("cross-partition entity kind scan", || {
            self.projection.entities::<T>()
        })?;
        self.record_entity_reads(None, values.len());
        Ok(values)
    }

    pub fn entities_in<T: EntityRecordProjection>(
        &self,
        partition_id: PartitionId,
    ) -> Result<Vec<T>, StrategyExecutorFailure> {
        self.admit_kind_scan(Some(partition_id), "partition-scoped entity kind scan")?;
        let values = self.safe_projection("partition-scoped entity kind scan", || {
            self.projection.entities_in::<T>(partition_id)
        })?;
        self.record_entity_reads(Some(partition_id), values.len());
        Ok(values)
    }

    pub fn entity<T: EntityRecordProjection>(
        &self,
        entity_id: EntityId,
    ) -> Result<Option<T>, StrategyExecutorFailure> {
        self.admit_target_lookup(entity_id.partition_id, "entity lookup")?;
        let value =
            self.safe_projection("entity lookup", || self.projection.entity::<T>(entity_id))?;
        self.record_entity_reads(Some(entity_id.partition_id), usize::from(value.is_some()));
        Ok(value)
    }

    pub fn relations<T: RelationRecordProjection>(
        &self,
    ) -> Result<Vec<T>, StrategyExecutorFailure> {
        self.admit_kind_scan(None, "cross-partition relation kind scan")?;
        let values = self.safe_projection("cross-partition relation kind scan", || {
            self.projection.relations::<T>()
        })?;
        self.record_relation_reads(None, values.len());
        Ok(values)
    }

    pub fn relations_in<T: RelationRecordProjection>(
        &self,
        partition_id: PartitionId,
    ) -> Result<Vec<T>, StrategyExecutorFailure> {
        self.admit_kind_scan(Some(partition_id), "partition-scoped relation kind scan")?;
        let values = self.safe_projection("partition-scoped relation kind scan", || {
            self.projection.relations_in::<T>(partition_id)
        })?;
        self.record_relation_reads(Some(partition_id), values.len());
        Ok(values)
    }

    pub fn relation<T: RelationRecordProjection>(
        &self,
        relation_id: RelationId,
    ) -> Result<Option<T>, StrategyExecutorFailure> {
        self.admit_target_lookup(relation_id.partition_id, "relation lookup")?;
        let value = self.safe_projection("relation lookup", || {
            self.projection.relation::<T>(relation_id)
        })?;
        self.record_relation_reads(Some(relation_id.partition_id), usize::from(value.is_some()));
        Ok(value)
    }

    pub fn entity_records(
        &self,
        kind_id: KindId,
    ) -> Result<Vec<EntityReadRecord>, StrategyExecutorFailure> {
        self.admit_kind_scan(None, "cross-partition entity record scan")?;
        let values = self.safe_projection("cross-partition entity record scan", || {
            self.projection.entity_records(kind_id)
        })?;
        self.record_entity_reads(None, values.len());
        Ok(values)
    }

    pub fn entity_records_in(
        &self,
        partition_id: PartitionId,
        kind_id: KindId,
    ) -> Result<Vec<EntityReadRecord>, StrategyExecutorFailure> {
        self.admit_kind_scan(Some(partition_id), "partition-scoped entity record scan")?;
        let values = self.safe_projection("partition-scoped entity record scan", || {
            self.projection.entity_records_in(partition_id, kind_id)
        })?;
        self.record_entity_reads(Some(partition_id), values.len());
        Ok(values)
    }

    pub fn entity_record(
        &self,
        entity_id: EntityId,
    ) -> Result<Option<EntityReadRecord>, StrategyExecutorFailure> {
        self.admit_target_lookup(entity_id.partition_id, "entity record lookup")?;
        let value = self.safe_projection("entity record lookup", || {
            self.projection.entity_record(entity_id)
        })?;
        self.record_entity_reads(Some(entity_id.partition_id), usize::from(value.is_some()));
        Ok(value)
    }

    pub fn relation_records(
        &self,
        kind_id: KindId,
    ) -> Result<Vec<RelationReadRecord>, StrategyExecutorFailure> {
        self.admit_kind_scan(None, "cross-partition relation record scan")?;
        let values = self.safe_projection("cross-partition relation record scan", || {
            self.projection.relation_records(kind_id)
        })?;
        self.record_relation_reads(None, values.len());
        Ok(values)
    }

    pub fn relation_records_in(
        &self,
        partition_id: PartitionId,
        kind_id: KindId,
    ) -> Result<Vec<RelationReadRecord>, StrategyExecutorFailure> {
        self.admit_kind_scan(Some(partition_id), "partition-scoped relation record scan")?;
        let values = self.safe_projection("partition-scoped relation record scan", || {
            self.projection.relation_records_in(partition_id, kind_id)
        })?;
        self.record_relation_reads(Some(partition_id), values.len());
        Ok(values)
    }

    pub fn relation_record(
        &self,
        relation_id: RelationId,
    ) -> Result<Option<RelationReadRecord>, StrategyExecutorFailure> {
        self.admit_target_lookup(relation_id.partition_id, "relation record lookup")?;
        let value = self.safe_projection("relation record lookup", || {
            self.projection.relation_record(relation_id)
        })?;
        self.record_relation_reads(Some(relation_id.partition_id), usize::from(value.is_some()));
        Ok(value)
    }

    fn admit_target_lookup(
        &self,
        partition_id: PartitionId,
        operation: &'static str,
    ) -> Result<(), StrategyExecutorFailure> {
        self.reject_if_packet_plan_only(operation)?;
        self.enforce_locality(partition_id, operation)
    }

    fn admit_kind_scan(
        &self,
        partition_id: Option<PartitionId>,
        operation: &'static str,
    ) -> Result<(), StrategyExecutorFailure> {
        self.reject_if_packet_plan_only(operation)?;
        match self.read_contract.scope_class {
            super::StrategyReadScopeClass::ExplicitTargetsOnly => {
                return Err(StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::ReadContractViolation,
                    format!("{operation} is forbidden by ExplicitTargetsOnly strategy read scope"),
                ));
            }
            super::StrategyReadScopeClass::PartitionBoundedScan if partition_id.is_none() => {
                return Err(StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::ReadContractViolation,
                    format!("{operation} is forbidden by PartitionBoundedScan strategy read scope"),
                ));
            }
            super::StrategyReadScopeClass::BoundedNeighborhood => {
                return Err(StrategyExecutorFailure::new(
                    StrategyExecutorFailureClass::ReadContractViolation,
                    format!(
                        "{operation} is forbidden because bounded-neighborhood traversal is not exposed yet"
                    ),
                ));
            }
            super::StrategyReadScopeClass::KindBoundedScan
            | super::StrategyReadScopeClass::PartitionBoundedScan => {}
        }
        if let Some(partition_id) = partition_id {
            self.enforce_locality(partition_id, operation)?;
        } else if matches!(
            self.read_contract.locality_class,
            super::StrategyReadLocalityClass::SinglePartition
        ) {
            return Err(StrategyExecutorFailure::new(
                StrategyExecutorFailureClass::ReadContractViolation,
                format!("{operation} is forbidden by SinglePartition strategy locality"),
            ));
        }
        Ok(())
    }

    fn reject_if_packet_plan_only(
        &self,
        operation: &'static str,
    ) -> Result<(), StrategyExecutorFailure> {
        if matches!(
            self.read_contract.packet_contract,
            super::StrategyPacketContract::PlannedPacketOnly
        ) {
            return Err(StrategyExecutorFailure::new(
                StrategyExecutorFailureClass::ReadContractViolation,
                format!(
                    "{operation} requires planned packet reads, but packet-planned strategy execution is not implemented yet"
                ),
            ));
        }
        Ok(())
    }

    fn enforce_locality(
        &self,
        partition_id: PartitionId,
        operation: &'static str,
    ) -> Result<(), StrategyExecutorFailure> {
        let mut metrics = self.metrics.borrow_mut();
        if matches!(
            self.read_contract.locality_class,
            super::StrategyReadLocalityClass::SinglePartition
        ) {
            match metrics.first_partition {
                Some(first_partition) if first_partition != partition_id => {
                    return Err(StrategyExecutorFailure::new(
                        StrategyExecutorFailureClass::ReadContractViolation,
                        format!(
                            "{operation} crossed from partition {} into partition {} under SinglePartition locality",
                            first_partition.0, partition_id.0
                        ),
                    ));
                }
                None => metrics.first_partition = Some(partition_id),
                Some(_) => {}
            }
        }
        Ok(())
    }

    fn record_entity_reads(&self, partition_id: Option<PartitionId>, count: usize) {
        if count == 0 {
            return;
        }
        let mut metrics = self.metrics.borrow_mut();
        metrics.entity_record_reads += count;
        if let Some(partition_id) = partition_id {
            metrics.partitions_touched.insert(partition_id);
        }
    }

    fn record_relation_reads(&self, partition_id: Option<PartitionId>, count: usize) {
        if count == 0 {
            return;
        }
        let mut metrics = self.metrics.borrow_mut();
        metrics.relation_record_reads += count;
        if let Some(partition_id) = partition_id {
            metrics.partitions_touched.insert(partition_id);
        }
    }

    fn projection_failure(
        &self,
        operation: &'static str,
        payload: Box<dyn std::any::Any + Send>,
    ) -> StrategyExecutorFailure {
        let detail = if let Some(message) = payload.downcast_ref::<&'static str>() {
            (*message).to_string()
        } else if let Some(message) = payload.downcast_ref::<String>() {
            message.clone()
        } else {
            format!("{operation} violated the projection contract")
        };
        StrategyExecutorFailure::new(
            StrategyExecutorFailureClass::ProjectionContractViolation,
            detail,
        )
    }

    fn safe_projection<T>(
        &self,
        operation: &'static str,
        run: impl FnOnce() -> T,
    ) -> Result<T, StrategyExecutorFailure> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(run))
            .map_err(|payload| self.projection_failure(operation, payload))
    }
}

#[derive(Debug)]
pub struct StrategyObservationContext<'runtime> {
    snapshot: &'runtime SnapshotHandle,
    read_contract: &'runtime StrategyReadContract,
    metrics: RefCell<StrategyObservationMetrics>,
    projection: VisibilityProjectionView<'runtime>,
}

impl<'runtime> StrategyObservationContext<'runtime> {
    pub(crate) fn new(
        snapshot: &'runtime SnapshotHandle,
        read_contract: &'runtime StrategyReadContract,
        visibility: VisibilityProjectionView<'runtime>,
    ) -> Self {
        Self {
            snapshot,
            read_contract,
            metrics: RefCell::new(StrategyObservationMetrics::default()),
            projection: visibility,
        }
    }

    pub fn snapshot(&self) -> &SnapshotHandle {
        self.snapshot
    }

    pub fn version_id(&self) -> VersionId {
        self.projection.version_id()
    }

    pub fn read_contract(&self) -> &StrategyReadContract {
        self.read_contract
    }

    pub fn visibility(&self) -> StrategyVisibilityReadView<'_, 'runtime> {
        StrategyVisibilityReadView::new(self.projection, self.read_contract, &self.metrics)
    }

    pub(crate) fn measured_summary(&self) -> StrategyExecutionSummary {
        self.metrics.borrow().summary()
    }
}

#[derive(Debug, Default)]
struct StrategyObservationMetrics {
    entity_record_reads: usize,
    relation_record_reads: usize,
    partitions_touched: BTreeSet<PartitionId>,
    first_partition: Option<PartitionId>,
}

impl StrategyObservationMetrics {
    fn summary(&self) -> StrategyExecutionSummary {
        StrategyExecutionSummary {
            entity_record_reads: self.entity_record_reads,
            relation_record_reads: self.relation_record_reads,
            projected_partition_reads: self.partitions_touched.len(),
        }
    }
}

fn compute_output_digest(canonical_bytes: &[u8]) -> CanonicalStrategyOutputDigest {
    CanonicalStrategyOutputDigest(Sha256::digest(canonical_bytes).into())
}

fn compute_mutation_program_digest(
    worker_batches: &[WorkerIntentBatch],
) -> StrategyMutationProgramDigest {
    let bytes =
        serde_json::to_vec(worker_batches).expect("strategy mutation program digest serialization");
    StrategyMutationProgramDigest(Sha256::digest(bytes).into())
}

#[cfg(test)]
mod tests {
    use super::{
        CanonicalStrategyOutputArtifact, PersistentArtifactName, StrategyMutationProgram,
        StrategyOutputSchemaName,
    };
    use crate::facade::transactions::{CreateIntent, MutationIntent, WorkerIntentBatch};
    use crate::identity::data::{KindId, PartitionId};
    use crate::payloads::data::RecordPayload;
    use crate::symbols::data::InternedString;
    use crate::transactions::data::EntitySpec;
    use serde_json::json;

    fn artifact() -> CanonicalStrategyOutputArtifact {
        CanonicalStrategyOutputArtifact::new(
            StrategyOutputSchemaName::new("intent.reconcile.output.v1"),
            br#"{"status":"ok"}"#.to_vec(),
            PersistentArtifactName::new("strategy.intent.reconcile"),
        )
    }

    fn mutation_program() -> StrategyMutationProgram {
        StrategyMutationProgram::new(vec![WorkerIntentBatch::new("reconcile").push(
            MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId(1),
                kind_id: KindId(1),
                client_key: InternedString::from("deployment-a"),
                payload: RecordPayload::from(json!({"replicas": 3})),
            })),
        )])
    }

    #[test]
    fn output_artifact_roundtrip_preserves_verified_digest() {
        let artifact = artifact();
        let bytes = serde_json::to_vec(&artifact).expect("serialize output artifact");
        let roundtripped: CanonicalStrategyOutputArtifact =
            serde_json::from_slice(&bytes).expect("deserialize output artifact");

        assert_eq!(roundtripped.digest(), artifact.digest());
        assert_eq!(roundtripped.canonical_bytes(), br#"{"status":"ok"}"#);
    }

    #[test]
    fn output_artifact_deserialization_rejects_forged_digest() {
        let mut value = serde_json::to_value(artifact()).expect("artifact value");
        value["canonical_bytes"] =
            serde_json::json!([123, 34, 98, 97, 100, 34, 58, 116, 114, 117, 101, 125]);

        let error = serde_json::from_value::<CanonicalStrategyOutputArtifact>(value).unwrap_err();
        assert!(error
            .to_string()
            .contains("strategy output digest does not match canonical output bytes"));
    }

    #[test]
    fn mutation_program_roundtrip_preserves_verified_digest_and_intent_count() {
        let program = mutation_program();
        let bytes = serde_json::to_vec(&program).expect("serialize mutation program");
        let roundtripped: StrategyMutationProgram =
            serde_json::from_slice(&bytes).expect("deserialize mutation program");

        assert_eq!(roundtripped.digest(), program.digest());
        assert_eq!(roundtripped.total_intent_count(), 1);
    }

    #[test]
    fn mutation_program_deserialization_rejects_forged_digest() {
        let mut value = serde_json::to_value(mutation_program()).expect("program value");
        value["digest"] = serde_json::Value::Array(
            std::iter::repeat(serde_json::Value::from(0))
                .take(32)
                .collect(),
        );

        let error = serde_json::from_value::<StrategyMutationProgram>(value).unwrap_err();
        assert!(error
            .to_string()
            .contains("strategy mutation program digest does not match canonical worker batches"));
    }
}
