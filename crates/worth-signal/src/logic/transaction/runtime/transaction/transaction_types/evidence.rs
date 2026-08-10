use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::data::temporal::{
    IntervalWakeRegeneration, LoweredTemporalEligibility, ReadyTemporalWake, RetiredTemporalWake,
    RuntimeClockBasis, ScheduledTemporalWake, TemporalExecutionSummary,
    TemporalPreviousValueReference, TemporalWakeReschedule, TemporalWakeReuse,
};
use crate::logic::planner::ExecutionReport;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalEligibilityFact {
    pub node: NodeId,
    pub eligibility: LoweredTemporalEligibility,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TemporalTransactionEvidence {
    pub clock_basis: RuntimeClockBasis,
    pub eligibility_facts: Vec<TemporalEligibilityFact>,
    pub scheduled_wakes: Vec<ScheduledTemporalWake>,
    pub ready_wakes: Vec<ReadyTemporalWake>,
    pub retired_wakes: Vec<RetiredTemporalWake>,
    pub rescheduled_wakes: Vec<TemporalWakeReschedule>,
    pub reused_wakes: Vec<TemporalWakeReuse>,
    pub interval_regenerations: Vec<IntervalWakeRegeneration>,
    pub previous_value_references: Vec<TemporalPreviousValueReference>,
}

impl TemporalTransactionEvidence {
    pub fn has_temporal_facts(&self) -> bool {
        !self.eligibility_facts.is_empty()
            || !self.scheduled_wakes.is_empty()
            || !self.ready_wakes.is_empty()
            || !self.retired_wakes.is_empty()
            || !self.rescheduled_wakes.is_empty()
            || !self.reused_wakes.is_empty()
            || !self.interval_regenerations.is_empty()
            || !self.previous_value_references.is_empty()
    }
}

#[derive(Debug, Clone, Default)]
pub(in crate::logic::transaction::runtime) struct TransactionTemporalScratch {
    pub summary: TemporalExecutionSummary,
    pub eligibility_facts: Vec<TemporalEligibilityFact>,
    pub scheduled_wakes: Vec<ScheduledTemporalWake>,
    pub ready_wakes: Vec<ReadyTemporalWake>,
    pub retired_wakes: Vec<RetiredTemporalWake>,
    pub rescheduled_wakes: Vec<TemporalWakeReschedule>,
    pub reused_wakes: Vec<TemporalWakeReuse>,
    pub interval_regenerations: Vec<IntervalWakeRegeneration>,
    pub previous_value_references: Vec<TemporalPreviousValueReference>,
}

impl TransactionTemporalScratch {
    pub fn absorb_report(&mut self, report: &ExecutionReport) {
        self.summary.absorb(report.temporal_summary);
        for stage in &report.stages {
            for task in &stage.task_records {
                if let Some(eligibility) = task.temporal_eligibility.clone() {
                    self.eligibility_facts.push(TemporalEligibilityFact {
                        node: task.node,
                        eligibility,
                    });
                }
            }
        }
    }

    pub fn record_scheduled_wake(&mut self, wake: ScheduledTemporalWake) {
        self.scheduled_wakes.push(wake);
    }

    pub fn record_ready_wake(&mut self, wake: ReadyTemporalWake) {
        self.ready_wakes.push(wake);
    }

    pub fn record_retired_wake(&mut self, wake: RetiredTemporalWake) {
        self.retired_wakes.push(wake);
    }

    pub fn record_rescheduled_wake(&mut self, reschedule: TemporalWakeReschedule) {
        self.rescheduled_wakes.push(reschedule);
    }

    pub fn record_reused_wake(&mut self, reuse: TemporalWakeReuse) {
        self.reused_wakes.push(reuse);
    }

    pub fn record_interval_regeneration(&mut self, regeneration: IntervalWakeRegeneration) {
        self.interval_regenerations.push(regeneration);
    }

    pub fn record_previous_value_reference(&mut self, reference: TemporalPreviousValueReference) {
        self.previous_value_references.push(reference);
    }

    pub fn boundary_evidence(&self, clock_basis: RuntimeClockBasis) -> TemporalTransactionEvidence {
        TemporalTransactionEvidence {
            clock_basis,
            eligibility_facts: self.eligibility_facts.clone(),
            scheduled_wakes: self.scheduled_wakes.clone(),
            ready_wakes: self.ready_wakes.clone(),
            retired_wakes: self.retired_wakes.clone(),
            rescheduled_wakes: self.rescheduled_wakes.clone(),
            reused_wakes: self.reused_wakes.clone(),
            interval_regenerations: self.interval_regenerations.clone(),
            previous_value_references: self.previous_value_references.clone(),
        }
    }
}
