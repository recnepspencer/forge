use super::{
    QueueExecutionPlanBinding, QueueExecutionReadyPlan, QueueExecutionReplayIdentity,
    QueueGroupingDenial,
};

#[derive(Debug, Eq, PartialEq)]
pub struct QueueGroupedReadyPlans {
    first: QueueExecutionReadyPlan,
    second: QueueExecutionReadyPlan,
    replay_identities: [QueueExecutionReplayIdentity; 2],
    grouped_writes: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct QueueGroupingRejected {
    first: QueueExecutionReadyPlan,
    second: QueueExecutionReadyPlan,
    denial: QueueGroupingDenial,
}

#[derive(Debug, Eq, PartialEq)]
pub enum QueueGroupingOutcome {
    Grouped(QueueGroupedReadyPlans),
    Denied(QueueGroupingRejected),
}

pub fn group_ready_queue_pair(
    first: QueueExecutionReadyPlan,
    second: QueueExecutionReadyPlan,
) -> QueueGroupingOutcome {
    if first.backend_profile() != second.backend_profile()
        || first.backend_evidence_class() != second.backend_evidence_class()
    {
        return QueueGroupingOutcome::Denied(QueueGroupingRejected {
            first,
            second,
            denial: QueueGroupingDenial::BackendCapabilityMismatch,
        });
    }
    if first.work().secure_io() != second.work().secure_io() {
        return QueueGroupingOutcome::Denied(QueueGroupingRejected {
            first,
            second,
            denial: QueueGroupingDenial::SecureIoReceiptMismatch,
        });
    }
    match first
        .grouping_basis()
        .compatible_with(second.grouping_basis())
    {
        Ok(()) => QueueGroupingOutcome::Grouped(QueueGroupedReadyPlans {
            replay_identities: [first.replay_identity(), second.replay_identity()],
            first,
            second,
            grouped_writes: 2,
        }),
        Err(denial) => QueueGroupingOutcome::Denied(QueueGroupingRejected {
            first,
            second,
            denial,
        }),
    }
}

impl QueueGroupedReadyPlans {
    pub const fn replay_identities(&self) -> [QueueExecutionReplayIdentity; 2] {
        self.replay_identities
    }

    pub const fn grouped_writes(&self) -> u32 {
        self.grouped_writes
    }

    pub const fn first(&self) -> &QueueExecutionReadyPlan {
        &self.first
    }

    pub const fn second(&self) -> &QueueExecutionReadyPlan {
        &self.second
    }

    pub const fn backend_completion_binding(&self) -> QueueExecutionPlanBinding {
        QueueExecutionPlanBinding::grouped(
            self.replay_identities[0],
            self.replay_identities[1],
            self.first.backend_profile(),
            self.first.backend_evidence_class(),
        )
    }

    pub(crate) fn into_execution_pair(
        self,
    ) -> (QueueExecutionReadyPlan, QueueExecutionReadyPlan, u32) {
        (self.first, self.second, self.grouped_writes)
    }
}

impl QueueGroupingRejected {
    pub const fn denial(&self) -> QueueGroupingDenial {
        self.denial
    }

    pub const fn first(&self) -> &QueueExecutionReadyPlan {
        &self.first
    }

    pub const fn second(&self) -> &QueueExecutionReadyPlan {
        &self.second
    }
}
