use worth_store_physical_backend::{
    BackendQueueExecutionBudgetBinding, BackendQueueExecutionPlanBinding,
    BackendQueueExecutionReplayBinding, BackendTargetProfile, CapabilityEvidenceClass,
};

use crate::{BackgroundResourceBudget, IoSchedulerBackendCapabilityRequirement};

use super::{
    QueueGroupingBasis, QueueRecoveryOrdering, QueueWorkClass, QueueWorkDeclaration,
    QueueWritebackPolicy, S6QueueDurabilityClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueExecutionReplayIdentity {
    work_class: QueueWorkClass,
    backend_requirement: IoSchedulerBackendCapabilityRequirement,
    durability_class: S6QueueDurabilityClass,
    grouping_basis: QueueGroupingBasis,
    requested_budget: BackgroundResourceBudget,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueExecutionPlanBinding {
    replay_identity: QueueExecutionReplayIdentity,
    backend_profile: BackendTargetProfile,
    backend_evidence_class: CapabilityEvidenceClass,
    grouped_replay_identity: Option<QueueExecutionReplayIdentity>,
}

impl QueueExecutionReplayIdentity {
    pub(crate) const fn new(
        work: QueueWorkDeclaration,
        grouping_basis: QueueGroupingBasis,
    ) -> Self {
        Self {
            work_class: work.class(),
            backend_requirement: work.backend_requirement(),
            durability_class: work.durability_class(),
            grouping_basis,
            requested_budget: work.requested_budget(),
        }
    }

    pub const fn work_class(self) -> QueueWorkClass {
        self.work_class
    }

    pub const fn backend_requirement(self) -> IoSchedulerBackendCapabilityRequirement {
        self.backend_requirement
    }

    pub const fn durability_class(self) -> S6QueueDurabilityClass {
        self.durability_class
    }

    pub const fn grouping_basis(self) -> QueueGroupingBasis {
        self.grouping_basis
    }

    pub const fn requested_budget(self) -> BackgroundResourceBudget {
        self.requested_budget
    }

    pub const fn backend_completion_binding(
        self,
        backend_profile: BackendTargetProfile,
        backend_evidence_class: CapabilityEvidenceClass,
    ) -> QueueExecutionPlanBinding {
        QueueExecutionPlanBinding {
            replay_identity: self,
            backend_profile,
            backend_evidence_class,
            grouped_replay_identity: None,
        }
    }
}

impl QueueExecutionPlanBinding {
    pub(crate) const fn grouped(
        first: QueueExecutionReplayIdentity,
        second: QueueExecutionReplayIdentity,
        backend_profile: BackendTargetProfile,
        backend_evidence_class: CapabilityEvidenceClass,
    ) -> Self {
        Self {
            replay_identity: first,
            backend_profile,
            backend_evidence_class,
            grouped_replay_identity: Some(second),
        }
    }

    pub const fn replay_identity(self) -> QueueExecutionReplayIdentity {
        self.replay_identity
    }

    pub const fn backend_profile(self) -> BackendTargetProfile {
        self.backend_profile
    }

    pub const fn backend_evidence_class(self) -> CapabilityEvidenceClass {
        self.backend_evidence_class
    }

    pub const fn grouped_replay_identity(self) -> Option<QueueExecutionReplayIdentity> {
        self.grouped_replay_identity
    }

    pub fn backend_execution_binding(self) -> BackendQueueExecutionPlanBinding {
        BackendQueueExecutionPlanBinding::from_store_replay_binding(
            backend_replay_binding(self.replay_identity),
            self.grouped_replay_identity.map(backend_replay_binding),
            self.backend_profile,
            self.backend_evidence_class,
            self.grouped_replay_identity.map_or(0, |_| 2),
        )
    }
}

fn backend_replay_binding(
    identity: QueueExecutionReplayIdentity,
) -> BackendQueueExecutionReplayBinding {
    let grouping = identity.grouping_basis;
    BackendQueueExecutionReplayBinding::from_store_queue_replay(
        tag_work_class(identity.work_class),
        tag_backend_requirement(identity.backend_requirement),
        tag_durability(identity.durability_class),
        grouping.security_scope_identity(),
        grouping.tenant_scope(),
        grouping.key_scope(),
        grouping.authenticity_requirement(),
        grouping.flush_epoch(),
        tag_recovery_ordering(grouping.recovery_ordering()),
        tag_writeback_policy(grouping.writeback_policy()),
        backend_budget_binding(identity.requested_budget),
    )
}

fn backend_budget_binding(budget: BackgroundResourceBudget) -> BackendQueueExecutionBudgetBinding {
    BackendQueueExecutionBudgetBinding::new(
        budget.queue_slots(),
        budget.bandwidth_tokens(),
        budget.flush_permits(),
        budget.sync_debt(),
        budget.read_ahead_window(),
        budget.write_back_window(),
        budget.dirty_page_budget(),
        budget.worker_permits(),
        budget.cache_residency_hints(),
        budget.reclaim_permits(),
    )
}

const fn tag_work_class(work_class: QueueWorkClass) -> u8 {
    match work_class {
        QueueWorkClass::Foreground(lane) => 10 + tag_foreground_lane(lane),
        QueueWorkClass::Background(class) => 40 + tag_background_class(class),
    }
}

const fn tag_foreground_lane(lane: crate::foreground_reservation::ForegroundIoLaneKind) -> u8 {
    match lane {
        crate::foreground_reservation::ForegroundIoLaneKind::PointRead => 1,
        crate::foreground_reservation::ForegroundIoLaneKind::RangeRead => 2,
        crate::foreground_reservation::ForegroundIoLaneKind::CommitCriticalWalWrite => 3,
        crate::foreground_reservation::ForegroundIoLaneKind::OrdinaryPageWrite => 4,
        crate::foreground_reservation::ForegroundIoLaneKind::InteractiveRead => 5,
        crate::foreground_reservation::ForegroundIoLaneKind::InternalForegroundRead => 6,
    }
}

const fn tag_background_class(class: crate::BackgroundIoPressureClass) -> u8 {
    match class {
        crate::BackgroundIoPressureClass::CompactionRewrite => 1,
        crate::BackgroundIoPressureClass::CheckpointFlush => 2,
        crate::BackgroundIoPressureClass::ScrubScan => 3,
        crate::BackgroundIoPressureClass::ReplicationPrepRead => 4,
        crate::BackgroundIoPressureClass::IngestPressure => 5,
        crate::BackgroundIoPressureClass::MigrationPressure => 6,
        crate::BackgroundIoPressureClass::BackupPrepRead => 7,
        crate::BackgroundIoPressureClass::RepairScan => 8,
        crate::BackgroundIoPressureClass::VerificationPressure => 9,
    }
}

const fn tag_backend_requirement(requirement: IoSchedulerBackendCapabilityRequirement) -> u8 {
    match requirement {
        IoSchedulerBackendCapabilityRequirement::BufferedFile => 1,
        IoSchedulerBackendCapabilityRequirement::DirectIo => 2,
        IoSchedulerBackendCapabilityRequirement::Mmap => 3,
        IoSchedulerBackendCapabilityRequirement::AsyncIo => 4,
        IoSchedulerBackendCapabilityRequirement::Fsync => 5,
        IoSchedulerBackendCapabilityRequirement::DirectorySync => 6,
        IoSchedulerBackendCapabilityRequirement::DurableRename => 7,
        IoSchedulerBackendCapabilityRequirement::SecureFrameIo => 8,
    }
}

const fn tag_durability(durability: S6QueueDurabilityClass) -> u8 {
    match durability {
        S6QueueDurabilityClass::ReadOnly => 1,
        S6QueueDurabilityClass::BufferedWrite => 2,
        S6QueueDurabilityClass::WalCommit => 3,
        S6QueueDurabilityClass::PlatformDurable => 4,
    }
}

const fn tag_recovery_ordering(ordering: QueueRecoveryOrdering) -> u8 {
    match ordering {
        QueueRecoveryOrdering::NotRecoveryCritical => 1,
        QueueRecoveryOrdering::WalBeforeData => 2,
        QueueRecoveryOrdering::RecoveryReadOnly => 3,
    }
}

const fn tag_writeback_policy(policy: QueueWritebackPolicy) -> u8 {
    match policy {
        QueueWritebackPolicy::None => 1,
        QueueWritebackPolicy::Immediate => 2,
        QueueWritebackPolicy::DeferredWithinFlushEpoch => 3,
    }
}
