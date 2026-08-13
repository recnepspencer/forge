use worth_store_io_scheduler::BackgroundPacingOutcome;

/// Exact Store-owned reason cleanup work did not reach executable admission.
pub enum PhysicalRecoveryCleanupAdmissionDenialKind {
    Request,
    SubmissionDenied(crate::physical_runtime::PhysicalWorkSubmissionDenial),
    SubmissionDeferred(crate::physical_runtime::PhysicalWorkSubmissionDeferred),
    SubmissionStale(crate::physical_runtime::PhysicalWorkSubmissionStale),
    SubmissionFailed(crate::physical_runtime::PhysicalWorkSubmissionFailure),
    PreEffect(crate::physical_runtime::PhysicalWorkPreEffectDenial),
    SchedulerForegroundCapacity(
        worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundAdmissionDenial,
    ),
    SchedulerBackgroundCapacity(worth_store_io_scheduler::BackgroundPacingDenial),
    BackgroundPacing(BackgroundPacingOutcome),
    Scheduler(crate::physical_runtime::PhysicalSchedulerDenial),
}

/// Stage-honest progress retained when cleanup admission is denied or deferred.
pub struct PhysicalRecoveryCleanupAdmissionDenial {
    kind: PhysicalRecoveryCleanupAdmissionDenialKind,
    submitted: bool,
    cancelled: bool,
}

impl PhysicalRecoveryCleanupAdmissionDenial {
    pub(super) const fn before_submission(
        kind: PhysicalRecoveryCleanupAdmissionDenialKind,
    ) -> Self {
        Self {
            kind,
            submitted: false,
            cancelled: false,
        }
    }

    pub(super) const fn after_submission(
        kind: PhysicalRecoveryCleanupAdmissionDenialKind,
        cancelled: bool,
    ) -> Self {
        Self {
            kind,
            submitted: true,
            cancelled,
        }
    }

    pub const fn kind(&self) -> &PhysicalRecoveryCleanupAdmissionDenialKind {
        &self.kind
    }

    pub const fn submission_recorded(&self) -> bool {
        self.submitted
    }

    pub const fn cancellation_recorded(&self) -> bool {
        self.cancelled
    }

    pub const fn scheduler_deferred(&self) -> bool {
        matches!(
            self.kind,
            PhysicalRecoveryCleanupAdmissionDenialKind::SubmissionDeferred(_)
                | PhysicalRecoveryCleanupAdmissionDenialKind::SchedulerForegroundCapacity(_)
                | PhysicalRecoveryCleanupAdmissionDenialKind::SchedulerBackgroundCapacity(_)
                | PhysicalRecoveryCleanupAdmissionDenialKind::BackgroundPacing(_)
        )
    }
}
