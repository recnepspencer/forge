use crate::offline_verifier::ReopenedRuntimeBoundaryTranscript;
use crate::{
    CheckpointBaseAdmission, RecoveryMemoryEnvelope, RecoveryRedoPlan, RedoApplicationCursor,
    RedoExecutionReceipt, RedoPlanningDenial, WalTailRedoSource,
};
use crate::{
    FreshRuntimeRecoveryExecution, ReopenedRecoveryArtifactAdmissionDenial,
    ReopenedRuntimeRecoverySession, RuntimeRecoveryReportDenial,
};

use super::{
    source_discovery::RecoveryWorkBudgetEvidence, RecoveryCounterSnapshot, RecoveryStoreFootprint,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedRecoveryWorkBounds {
    checkpoint_interval_frames: usize,
    wal_tail_frame_limit: usize,
    max_scanned_segments: usize,
    max_page_redos: usize,
}

impl AdmittedRecoveryWorkBounds {
    pub(crate) const fn new(
        checkpoint_interval_frames: usize,
        wal_tail_frame_limit: usize,
        max_scanned_segments: usize,
        max_page_redos: usize,
    ) -> Self {
        Self {
            checkpoint_interval_frames,
            wal_tail_frame_limit,
            max_scanned_segments,
            max_page_redos,
        }
    }

    pub const fn checkpoint_interval_frames(self) -> usize {
        self.checkpoint_interval_frames
    }

    pub const fn wal_tail_frame_limit(self) -> usize {
        self.wal_tail_frame_limit
    }

    pub const fn max_scanned_segments(self) -> usize {
        self.max_scanned_segments
    }

    pub const fn max_page_redos(self) -> usize {
        self.max_page_redos
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedRecoveryPlan {
    checkpoint: CheckpointBaseAdmission,
    tail: WalTailRedoSource,
    redo_plan: RecoveryRedoPlan,
    evidence: RecoveryWorkBudgetEvidence,
    memory_envelope: RecoveryMemoryEnvelope,
    store_footprint: RecoveryStoreFootprint,
    work_bounds: AdmittedRecoveryWorkBounds,
}

impl BoundedRecoveryPlan {
    pub(crate) const fn new(
        checkpoint: CheckpointBaseAdmission,
        tail: WalTailRedoSource,
        redo_plan: RecoveryRedoPlan,
        evidence: RecoveryWorkBudgetEvidence,
        memory_envelope: RecoveryMemoryEnvelope,
        store_footprint: RecoveryStoreFootprint,
        work_bounds: AdmittedRecoveryWorkBounds,
    ) -> Self {
        Self {
            checkpoint,
            tail,
            redo_plan,
            evidence,
            memory_envelope,
            store_footprint,
            work_bounds,
        }
    }

    pub fn execute(
        &self,
        cursor: &RedoApplicationCursor,
    ) -> Result<BoundedRecoveryReceipt, RedoPlanningDenial> {
        let execution = self.redo_plan.execute(cursor)?;
        let counters = RecoveryCounterSnapshot::from_execution(
            &execution,
            self.evidence,
            self.memory_envelope,
            self.store_footprint,
        );
        Ok(BoundedRecoveryReceipt {
            execution,
            counters,
            work_bounds: self.work_bounds,
        })
    }

    pub(crate) fn execute_reopened_runtime_recovery(
        &self,
        session: &ReopenedRuntimeRecoverySession,
    ) -> Result<(BoundedRecoveryReceipt, FreshRuntimeRecoveryExecution), ReopenedRecoveryDenial>
    {
        let admission = session.admission();
        let receipt = self
            .execute(admission.replay_cursor())
            .map_err(|denial| ReopenedRecoveryDenial::Redo(Box::new(denial)))?;
        let transcript =
            ReopenedRuntimeBoundaryTranscript::from_reopened_runtime_execution(session, &receipt)
                .map_err(ReopenedRecoveryDenial::Runtime)?;
        let execution = FreshRuntimeRecoveryExecution::from_store_recovery_execution(
            admission,
            &transcript,
            &receipt,
        )
        .map_err(ReopenedRecoveryDenial::Runtime)?;
        Ok((receipt, execution))
    }

    pub const fn checkpoint(&self) -> &CheckpointBaseAdmission {
        &self.checkpoint
    }

    pub const fn tail(&self) -> &WalTailRedoSource {
        &self.tail
    }

    pub const fn redo_plan(&self) -> &RecoveryRedoPlan {
        &self.redo_plan
    }

    pub const fn work_bounds(&self) -> AdmittedRecoveryWorkBounds {
        self.work_bounds
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedRecoveryReceipt {
    execution: RedoExecutionReceipt,
    counters: RecoveryCounterSnapshot,
    work_bounds: AdmittedRecoveryWorkBounds,
}

impl BoundedRecoveryReceipt {
    pub const fn execution(&self) -> &RedoExecutionReceipt {
        &self.execution
    }

    pub const fn counters(&self) -> RecoveryCounterSnapshot {
        self.counters
    }

    pub const fn work_bounds(&self) -> AdmittedRecoveryWorkBounds {
        self.work_bounds
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReopenedRecoveryDenial {
    Admission(ReopenedRecoveryArtifactAdmissionDenial),
    Redo(Box<RedoPlanningDenial>),
    Runtime(RuntimeRecoveryReportDenial),
}
