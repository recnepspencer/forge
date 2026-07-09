use crate::{
    bulk::{BudgetAdmittedChunkPlan, BulkChunkExecutionOutcome},
    failure::{StoreError, StoreErrorKind},
    wal::DurableMutationId,
};
use worth_relational::facade::replay::CanonicalCommitEnvelope;

#[derive(Debug, Clone)]
pub struct BulkCanonicalChunkExecutionRequest {
    admitted_chunk: BudgetAdmittedChunkPlan,
    canonical_envelope: CanonicalCommitEnvelope,
}

impl BulkCanonicalChunkExecutionRequest {
    pub fn admit(
        admitted_chunk: BudgetAdmittedChunkPlan,
        canonical_envelope: CanonicalCommitEnvelope,
    ) -> Result<Self, StoreError> {
        if canonical_envelope.branch_context != *admitted_chunk.target_branch_scope() {
            return Err(StoreError::new(
                StoreErrorKind::ConcurrentBulkBoundaryViolation,
                format!(
                    "bulk chunk for branch `{}` cannot lower through canonical envelope on branch `{}`",
                    admitted_chunk.target_branch_scope().0,
                    canonical_envelope.branch_context.0
                ),
            ));
        }
        Ok(Self {
            admitted_chunk,
            canonical_envelope,
        })
    }

    pub fn admitted_chunk(&self) -> &BudgetAdmittedChunkPlan {
        &self.admitted_chunk
    }

    pub fn canonical_envelope(&self) -> &CanonicalCommitEnvelope {
        &self.canonical_envelope
    }

    pub(crate) fn runtime_session_id(&self) -> String {
        format!(
            "bulk:{}:{}",
            self.admitted_chunk.program_id(),
            self.admitted_chunk.plan_id()
        )
    }

    pub(crate) fn operation_name(&self) -> String {
        let kind = match self.admitted_chunk.kind() {
            crate::bulk::BulkPlanKind::Ingest => "ingest",
            crate::bulk::BulkPlanKind::Transform => "transform",
        };
        format!(
            "bulk-{kind}-chunk-{}",
            self.admitted_chunk.chunk().ordinal().value()
        )
    }

    pub fn into_parts(self) -> (BudgetAdmittedChunkPlan, CanonicalCommitEnvelope) {
        (self.admitted_chunk, self.canonical_envelope)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurablyExecutedBulkChunk {
    durable_mutation_id: DurableMutationId,
    execution_outcome: BulkChunkExecutionOutcome,
}

impl DurablyExecutedBulkChunk {
    pub fn new(
        durable_mutation_id: DurableMutationId,
        execution_outcome: BulkChunkExecutionOutcome,
    ) -> Self {
        Self {
            durable_mutation_id,
            execution_outcome,
        }
    }

    pub fn durable_mutation_id(&self) -> DurableMutationId {
        self.durable_mutation_id
    }

    pub fn execution_outcome(&self) -> &BulkChunkExecutionOutcome {
        &self.execution_outcome
    }
}
