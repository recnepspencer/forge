use crate::{
    AdmittedRecoverySource, CheckpointBaseAdmission, RecoveryMemoryAllocation, RecoveryRedoPlan,
    WalTailRedoSource,
};

use super::{
    AdmittedRecoveryWorkBounds, BoundedRecoveryPlan, BoundedRecoverySourceAdmission,
    BoundedRecoverySourcePrecedenceGraph, CheckpointIntervalContract, RecoveryBudgetDenial,
    RecoveryBudgetDenialKind, RecoveryStoreFootprint, WalTailReplayBudget,
};

#[derive(Debug)]
pub struct RecoveryBudget<'runtime> {
    checkpoint_interval: CheckpointIntervalContract,
    wal_tail: WalTailReplayBudget,
    memory_allocation: RecoveryMemoryAllocation<'runtime>,
    max_memory_envelope_bytes: Option<u64>,
    max_allocation_bytes: Option<u64>,
    max_checkpoint_discovery_candidates: Option<usize>,
    store_footprint: RecoveryStoreFootprint,
}

impl<'runtime> RecoveryBudget<'runtime> {
    pub fn new(
        checkpoint_interval: CheckpointIntervalContract,
        wal_tail: WalTailReplayBudget,
        memory_allocation: RecoveryMemoryAllocation<'runtime>,
    ) -> Self {
        Self {
            checkpoint_interval,
            wal_tail,
            memory_allocation,
            max_memory_envelope_bytes: None,
            max_allocation_bytes: None,
            max_checkpoint_discovery_candidates: None,
            store_footprint: RecoveryStoreFootprint::empty(),
        }
    }

    pub fn with_max_memory_envelope_bytes(mut self, max_bytes: u64) -> Self {
        self.max_memory_envelope_bytes = Some(max_bytes);
        self
    }

    pub fn with_max_allocation_bytes(mut self, max_bytes: u64) -> Self {
        self.max_allocation_bytes = Some(max_bytes);
        self
    }

    pub fn with_checkpoint_discovery_candidates(mut self, max_candidates: usize) -> Self {
        self.max_checkpoint_discovery_candidates = Some(max_candidates);
        self
    }

    pub fn with_store_footprint(mut self, footprint: RecoveryStoreFootprint) -> Self {
        self.store_footprint = footprint;
        self
    }

    pub fn source_precedence_graph(
        self,
        profile: impl Into<String>,
    ) -> BoundedRecoverySourcePrecedenceGraph<'runtime> {
        BoundedRecoverySourcePrecedenceGraph::new(self, profile)
    }

    pub(crate) const fn checkpoint_interval_frame_limit(&self) -> usize {
        self.checkpoint_interval.max_tail_frame_count()
    }

    pub(crate) const fn wal_tail_frame_limit(&self) -> usize {
        self.wal_tail.max_frame_count()
    }

    pub fn admit_recovery(
        self,
        source_admission: BoundedRecoverySourceAdmission,
        redo_plan: RecoveryRedoPlan,
    ) -> Result<BoundedRecoveryPlan<'runtime>, RecoveryBudgetDenial> {
        let (source, evidence) = source_admission.into_parts();
        self.require_source_trace_matches_plan(&source, &redo_plan)?;
        let checkpoint = admitted_checkpoint_base(&source)?;
        let tail = admitted_wal_tail(&source)?;
        self.checkpoint_interval.admit_tail(&checkpoint, &tail)?;
        self.require_declared_tail_matches_plan(&tail, &redo_plan)?;
        self.wal_tail
            .admit_replay_work(&redo_plan, evidence.scanned_segments())?;
        self.require_discovery_budget(&redo_plan)?;
        self.require_memory_budget()?;
        let work_bounds = AdmittedRecoveryWorkBounds::new(
            self.checkpoint_interval.max_tail_frame_count(),
            self.wal_tail.max_frame_count(),
            self.wal_tail.max_scanned_segments(),
            self.wal_tail.max_page_redos(),
        );
        Ok(BoundedRecoveryPlan::new(
            checkpoint,
            tail,
            redo_plan,
            evidence,
            self.memory_allocation,
            self.store_footprint,
            work_bounds,
        ))
    }

    pub(crate) fn require_source_candidate_count(
        &self,
        discovered: usize,
    ) -> Result<(), RecoveryBudgetDenial> {
        let Some(max) = self.max_checkpoint_discovery_candidates else {
            return Ok(());
        };
        if discovered > max {
            return Err(RecoveryBudgetDenial::new(
                RecoveryBudgetDenialKind::CheckpointDiscoveryBudgetExceeded { discovered, max },
            ));
        }
        Ok(())
    }

    fn require_declared_tail_matches_plan(
        &self,
        tail: &WalTailRedoSource,
        plan: &RecoveryRedoPlan,
    ) -> Result<(), RecoveryBudgetDenial> {
        let declared_tail_range = tail.lsn_range();
        let planned_source_range = plan.valid_prefix().source_range();
        if declared_tail_range != planned_source_range {
            return Err(RecoveryBudgetDenial::new(
                RecoveryBudgetDenialKind::WalTailSourceMismatch {
                    declared_tail_range,
                    planned_source_range,
                },
            ));
        }
        Ok(())
    }

    fn require_source_trace_matches_plan(
        &self,
        source: &AdmittedRecoverySource,
        plan: &RecoveryRedoPlan,
    ) -> Result<(), RecoveryBudgetDenial> {
        let admitted_trace = source.trace();
        let planned_trace = plan.source_trace();
        if admitted_trace != planned_trace {
            return Err(RecoveryBudgetDenial::new(
                RecoveryBudgetDenialKind::RecoverySourceAdmissionMismatch {
                    admitted_kind: admitted_trace.kind(),
                    planned_kind: planned_trace.kind(),
                    admitted_candidates: admitted_trace.candidate_count(),
                    planned_candidates: planned_trace.candidate_count(),
                },
            ));
        }
        Ok(())
    }

    fn require_discovery_budget(
        &self,
        plan: &RecoveryRedoPlan,
    ) -> Result<(), RecoveryBudgetDenial> {
        self.require_source_candidate_count(plan.source_trace().candidate_count())
    }

    fn require_memory_budget(&self) -> Result<(), RecoveryBudgetDenial> {
        let memory = self.memory_allocation.counters();
        if let Some(max_bytes) = self.max_memory_envelope_bytes {
            let admitted_bytes = memory.resident_bytes_admitted();
            if admitted_bytes > max_bytes {
                return Err(RecoveryBudgetDenial::new(
                    RecoveryBudgetDenialKind::MemoryEnvelopeBudgetExceeded {
                        admitted_bytes,
                        max_bytes,
                    },
                ));
            }
        }
        if let Some(max_bytes) = self.max_allocation_bytes {
            let allocated_bytes = memory.allocation_bytes_allocated();
            if allocated_bytes > max_bytes {
                return Err(RecoveryBudgetDenial::new(
                    RecoveryBudgetDenialKind::AllocationBudgetExceeded {
                        allocated_bytes,
                        max_bytes,
                    },
                ));
            }
        }
        Ok(())
    }
}

fn admitted_checkpoint_base(
    source: &AdmittedRecoverySource,
) -> Result<CheckpointBaseAdmission, RecoveryBudgetDenial> {
    source.selected_checkpoint().cloned().ok_or_else(|| {
        RecoveryBudgetDenial::new(
            RecoveryBudgetDenialKind::MissingCheckpointBaseForBoundedRecovery {
                source_kind: source.trace().kind(),
            },
        )
    })
}

fn admitted_wal_tail(
    source: &AdmittedRecoverySource,
) -> Result<WalTailRedoSource, RecoveryBudgetDenial> {
    source.selected_wal_tail().cloned().ok_or_else(|| {
        RecoveryBudgetDenial::new(RecoveryBudgetDenialKind::MissingWalTailForBoundedRecovery {
            source_kind: source.trace().kind(),
        })
    })
}
