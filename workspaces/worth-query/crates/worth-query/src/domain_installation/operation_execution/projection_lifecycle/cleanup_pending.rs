use crate::basis_lifecycle::BasisOperationLane;
use crate::runtime::WorthQueryWorkspace;
use worth_proof::PhaseMarker;

use super::lifecycle_close::{
    close_operational_projection, WorthQueryProjectionCloseCoreOutcome,
    WorthQueryProjectionCloseCoreStopKind,
};
use super::operational_owner::WorthQueryOperationalProjection;
use super::states::WorthQueryLiveProjectionPhase;
use super::{
    WorthQueryProjectionLifecycleCloseCause, WorthQueryProjectionLifecycleCloseReceipt,
    WorthQueryProjectionTransitionWork,
};

pub(super) struct WorthQueryCleanupClosedProjectionPhase;
impl PhaseMarker for WorthQueryCleanupClosedProjectionPhase {}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryProjectionCleanupWork {
    predecessor_close_attempts: usize,
    predecessor_close_completions: usize,
    rollback_close_attempts: usize,
    rollback_close_completions: usize,
}

impl WorthQueryProjectionCleanupWork {
    fn retain_predecessor(
        &mut self,
        counters: super::WorthQueryProjectionLifecycleTransitionCounters,
    ) {
        self.predecessor_close_attempts += counters.close_attempts;
        self.predecessor_close_completions += counters.close_completions;
    }

    fn retain_rollback(
        &mut self,
        counters: super::WorthQueryProjectionLifecycleTransitionCounters,
    ) {
        self.rollback_close_attempts += counters.close_attempts;
        self.rollback_close_completions += counters.close_completions;
    }

    pub fn predecessor_close_attempts(self) -> usize {
        self.predecessor_close_attempts
    }

    pub fn predecessor_close_completions(self) -> usize {
        self.predecessor_close_completions
    }

    pub fn rollback_close_attempts(self) -> usize {
        self.rollback_close_attempts
    }

    pub fn rollback_close_completions(self) -> usize {
        self.rollback_close_completions
    }
}

pub(super) struct WorthQueryTransitionedOperationalProjection<
    S,
    L: BasisOperationLane,
    P: PhaseMarker,
> {
    successor: WorthQueryOperationalProjection<S, L, P>,
    predecessor_close: WorthQueryProjectionLifecycleCloseReceipt,
    work: WorthQueryProjectionTransitionWork,
    cleanup_work: WorthQueryProjectionCleanupWork,
}

impl<S, L: BasisOperationLane, P: PhaseMarker>
    WorthQueryTransitionedOperationalProjection<S, L, P>
{
    pub(super) fn from_parts(
        successor: WorthQueryOperationalProjection<S, L, P>,
        predecessor_close: WorthQueryProjectionLifecycleCloseReceipt,
        work: WorthQueryProjectionTransitionWork,
        cleanup_work: WorthQueryProjectionCleanupWork,
    ) -> Self {
        Self {
            successor,
            predecessor_close,
            work,
            cleanup_work,
        }
    }

    pub(super) fn successor(&self) -> &WorthQueryOperationalProjection<S, L, P> {
        &self.successor
    }

    pub(super) fn predecessor_close(&self) -> &WorthQueryProjectionLifecycleCloseReceipt {
        &self.predecessor_close
    }

    pub(super) fn work(&self) -> WorthQueryProjectionTransitionWork {
        self.work
    }

    pub(super) fn cleanup_work(&self) -> WorthQueryProjectionCleanupWork {
        self.cleanup_work
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        WorthQueryOperationalProjection<S, L, P>,
        WorthQueryProjectionLifecycleCloseReceipt,
        WorthQueryProjectionTransitionWork,
        WorthQueryProjectionCleanupWork,
    ) {
        (
            self.successor,
            self.predecessor_close,
            self.work,
            self.cleanup_work,
        )
    }
}

pub(super) struct WorthQueryCleanupPendingCore<Old, New, L: BasisOperationLane, P: PhaseMarker> {
    predecessor: WorthQueryOperationalProjection<Old, L, WorthQueryLiveProjectionPhase>,
    successor: WorthQueryOperationalProjection<New, L, P>,
    predecessor_close_cause: WorthQueryProjectionLifecycleCloseCause,
    rollback_close_cause: WorthQueryProjectionLifecycleCloseCause,
    work: WorthQueryProjectionTransitionWork,
    cleanup_work: WorthQueryProjectionCleanupWork,
}

pub(super) enum WorthQueryCleanupRetryCoreOutcome<Old, New, L, P>
where
    L: BasisOperationLane,
    P: PhaseMarker,
{
    Completed(WorthQueryTransitionedOperationalProjection<New, L, P>),
    Pending {
        pending: WorthQueryCleanupPendingCore<Old, New, L, P>,
        detail: String,
    },
}

pub(super) enum WorthQueryCleanupRollbackCoreOutcome<Old, New, L, P>
where
    L: BasisOperationLane,
    P: PhaseMarker,
{
    Restored {
        predecessor: WorthQueryOperationalProjection<Old, L, WorthQueryLiveProjectionPhase>,
        rollback_close: WorthQueryProjectionLifecycleCloseReceipt,
        work: WorthQueryProjectionCleanupWork,
    },
    Pending {
        pending: WorthQueryCleanupPendingCore<Old, New, L, P>,
        detail: String,
    },
}

pub(super) fn finish_transition<Old, New, L, P>(
    predecessor: WorthQueryOperationalProjection<Old, L, WorthQueryLiveProjectionPhase>,
    successor: WorthQueryOperationalProjection<New, L, P>,
    predecessor_close_cause: WorthQueryProjectionLifecycleCloseCause,
    rollback_close_cause: WorthQueryProjectionLifecycleCloseCause,
    work: WorthQueryProjectionTransitionWork,
    workspace: &mut WorthQueryWorkspace,
) -> WorthQueryCleanupRetryCoreOutcome<Old, New, L, P>
where
    L: BasisOperationLane,
    P: PhaseMarker,
{
    WorthQueryCleanupPendingCore {
        predecessor,
        successor,
        predecessor_close_cause,
        rollback_close_cause,
        work,
        cleanup_work: WorthQueryProjectionCleanupWork::default(),
    }
    .retry(workspace)
}

impl<Old, New, L, P> WorthQueryCleanupPendingCore<Old, New, L, P>
where
    L: BasisOperationLane,
    P: PhaseMarker,
{
    pub(super) fn work(&self) -> WorthQueryProjectionTransitionWork {
        self.work
    }

    pub(super) fn cleanup_work(&self) -> WorthQueryProjectionCleanupWork {
        self.cleanup_work
    }

    pub(super) fn predecessor_resource_name(&self) -> &str {
        self.predecessor.handle().name()
    }

    pub(super) fn successor_resource_name(&self) -> &str {
        self.successor.handle().name()
    }

    pub(super) fn retry(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryCleanupRetryCoreOutcome<Old, New, L, P> {
        match close_operational_projection::<_, L, _, WorthQueryCleanupClosedProjectionPhase>(
            self.predecessor,
            self.predecessor_close_cause,
            workspace,
        ) {
            WorthQueryProjectionCloseCoreOutcome::Closed(closed) => {
                let (_, _, predecessor_close, _) = closed.into_parts();
                let mut cleanup_work = self.cleanup_work;
                cleanup_work.retain_predecessor(predecessor_close.counters());
                WorthQueryCleanupRetryCoreOutcome::Completed(
                    WorthQueryTransitionedOperationalProjection {
                        successor: self.successor,
                        predecessor_close,
                        work: self.work,
                        cleanup_work,
                    },
                )
            }
            WorthQueryProjectionCloseCoreOutcome::Stopped(stop) => {
                let (predecessor, kind, counters) = stop.into_parts();
                let mut cleanup_work = self.cleanup_work;
                cleanup_work.retain_predecessor(counters);
                WorthQueryCleanupRetryCoreOutcome::Pending {
                    pending: WorthQueryCleanupPendingCore {
                        predecessor,
                        successor: self.successor,
                        predecessor_close_cause: self.predecessor_close_cause,
                        rollback_close_cause: self.rollback_close_cause,
                        work: self.work,
                        cleanup_work,
                    },
                    detail: close_detail(kind),
                }
            }
        }
    }

    pub(super) fn rollback(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryCleanupRollbackCoreOutcome<Old, New, L, P> {
        match close_operational_projection::<_, L, _, WorthQueryCleanupClosedProjectionPhase>(
            self.successor,
            self.rollback_close_cause,
            workspace,
        ) {
            WorthQueryProjectionCloseCoreOutcome::Closed(closed) => {
                let (_, _, rollback_close, _) = closed.into_parts();
                let mut work = self.cleanup_work;
                work.retain_rollback(rollback_close.counters());
                WorthQueryCleanupRollbackCoreOutcome::Restored {
                    predecessor: self.predecessor,
                    rollback_close,
                    work,
                }
            }
            WorthQueryProjectionCloseCoreOutcome::Stopped(stop) => {
                let (successor, kind, counters) = stop.into_parts();
                let mut cleanup_work = self.cleanup_work;
                cleanup_work.retain_rollback(counters);
                WorthQueryCleanupRollbackCoreOutcome::Pending {
                    pending: WorthQueryCleanupPendingCore {
                        predecessor: self.predecessor,
                        successor,
                        predecessor_close_cause: self.predecessor_close_cause,
                        rollback_close_cause: self.rollback_close_cause,
                        work: self.work,
                        cleanup_work,
                    },
                    detail: close_detail(kind),
                }
            }
        }
    }
}

fn close_detail(kind: WorthQueryProjectionCloseCoreStopKind) -> String {
    match kind {
        WorthQueryProjectionCloseCoreStopKind::Runtime(detail) => detail,
    }
}
