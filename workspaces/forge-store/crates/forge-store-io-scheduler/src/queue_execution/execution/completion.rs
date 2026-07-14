use forge_store_physical_backend::{
    BackendQueueExecutionBackpressure, BackendQueueExecutionCompletion,
    BackendQueueExecutionPosture, BackendQueueSpeculativeScope,
};

use super::{QueueExecutionPlanBinding, QueueExecutionReadyPlan, QueueGroupedReadyPlans};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueBackendCompletionAuthorityDenial {
    BackendPostureMismatch,
    BackendPlanBindingMismatch,
    BackendGroupedWriteMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueBackendCompletionAuthority {
    binding: QueueExecutionPlanBinding,
    posture: BackendQueueExecutionPosture,
    grouped_writes: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueBackendCompletionEvidence {
    binding: QueueExecutionPlanBinding,
    posture: BackendQueueExecutionPosture,
    queue_depth_sample: u32,
    grouped_writes: u32,
    read_ahead_units: u64,
    read_ahead_scope: Option<BackendQueueSpeculativeScope>,
    write_back_units: u64,
    write_back_scope: Option<BackendQueueSpeculativeScope>,
    mechanical_retries: u64,
    partial_read_events: u64,
    short_write_events: u64,
    backpressure: Option<BackendQueueExecutionBackpressure>,
    foreground_wait_events: u64,
}

impl QueueBackendCompletionAuthority {
    pub(crate) fn for_ready_plan(
        plan: &QueueExecutionReadyPlan,
        completion: BackendQueueExecutionCompletion,
    ) -> Result<Self, QueueBackendCompletionAuthorityDenial> {
        let posture = completion.posture();
        if posture.profile() != plan.backend_profile()
            || posture.evidence_class() != plan.backend_evidence_class()
        {
            return Err(QueueBackendCompletionAuthorityDenial::BackendPostureMismatch);
        }
        if completion.binding()
            != plan
                .backend_completion_binding()
                .backend_execution_binding()
        {
            return Err(QueueBackendCompletionAuthorityDenial::BackendPlanBindingMismatch);
        }
        if completion.grouped_writes() != 0 {
            return Err(QueueBackendCompletionAuthorityDenial::BackendGroupedWriteMismatch);
        }
        Ok(Self {
            binding: plan.backend_completion_binding(),
            posture,
            grouped_writes: completion.grouped_writes(),
        })
    }

    pub(crate) fn for_grouped_plans(
        grouped: &QueueGroupedReadyPlans,
        completion: BackendQueueExecutionCompletion,
    ) -> Result<Self, QueueBackendCompletionAuthorityDenial> {
        let posture = completion.posture();
        if posture.profile() != grouped.first().backend_profile()
            || posture.evidence_class() != grouped.first().backend_evidence_class()
        {
            return Err(QueueBackendCompletionAuthorityDenial::BackendPostureMismatch);
        }
        if completion.binding()
            != grouped
                .backend_completion_binding()
                .backend_execution_binding()
        {
            return Err(QueueBackendCompletionAuthorityDenial::BackendPlanBindingMismatch);
        }
        if completion.grouped_writes() != grouped.grouped_writes() {
            return Err(QueueBackendCompletionAuthorityDenial::BackendGroupedWriteMismatch);
        }
        Ok(Self {
            binding: grouped.backend_completion_binding(),
            posture,
            grouped_writes: completion.grouped_writes(),
        })
    }
}

impl QueueBackendCompletionEvidence {
    pub(crate) const fn from_backend_completion(
        authority: QueueBackendCompletionAuthority,
        completion: BackendQueueExecutionCompletion,
    ) -> Self {
        Self {
            binding: authority.binding,
            posture: authority.posture,
            queue_depth_sample: completion.queue_depth_sample(),
            grouped_writes: authority.grouped_writes,
            read_ahead_units: completion.read_ahead_units(),
            read_ahead_scope: completion.read_ahead_scope(),
            write_back_units: completion.write_back_units(),
            write_back_scope: completion.write_back_scope(),
            mechanical_retries: completion.mechanical_retries(),
            partial_read_events: completion.partial_read_events(),
            short_write_events: completion.short_write_events(),
            backpressure: completion.backpressure(),
            foreground_wait_events: completion.foreground_wait_events(),
        }
    }

    pub(crate) const fn binding(self) -> QueueExecutionPlanBinding {
        self.binding
    }
    pub(crate) const fn posture(self) -> BackendQueueExecutionPosture {
        self.posture
    }
    pub(crate) const fn queue_depth_sample(self) -> u32 {
        self.queue_depth_sample
    }
    pub(crate) const fn grouped_writes(self) -> u32 {
        self.grouped_writes
    }
    pub(crate) const fn read_ahead_units(self) -> u64 {
        self.read_ahead_units
    }
    pub(crate) const fn read_ahead_scope(self) -> Option<BackendQueueSpeculativeScope> {
        self.read_ahead_scope
    }
    pub(crate) const fn write_back_units(self) -> u64 {
        self.write_back_units
    }
    pub(crate) const fn write_back_scope(self) -> Option<BackendQueueSpeculativeScope> {
        self.write_back_scope
    }
    pub(crate) const fn mechanical_retries(self) -> u64 {
        self.mechanical_retries
    }
    pub(crate) const fn partial_read_events(self) -> u64 {
        self.partial_read_events
    }
    pub(crate) const fn short_write_events(self) -> u64 {
        self.short_write_events
    }
    pub(crate) const fn backpressure(self) -> Option<BackendQueueExecutionBackpressure> {
        self.backpressure
    }
    pub(crate) const fn foreground_wait_events(self) -> u64 {
        self.foreground_wait_events
    }
}
