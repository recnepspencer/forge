use crate::basis_lifecycle::BasisOperationLane;
use crate::runtime::WorthQueryWorkspace;

use super::cleanup_pending::WorthQueryTransitionedOperationalProjection;
use super::lifecycle_close::{
    close_operational_projection, WorthQueryProjectionCloseCoreOutcome,
    WorthQueryProjectionCloseCoreStopKind,
};
use super::terminal_states::{
    WorthQueryCancelledProjectionPhase, WorthQueryDisposedProjectionPhase,
};
use super::workflow_terminal_states::{
    WorthQueryCancelledWorkflowProjection, WorthQueryDisposedWorkflowProjection,
};
use super::workflow_transition_states::{
    WorthQueryReboundWorkflowProjection, WorthQueryReplacedWorkflowProjection,
};
use super::{
    WorthQueryProjectionLifecycleCloseCause, WorthQueryProjectionLifecycleTransitionCounters,
    WorthQueryProjectionPriorTransitionEvidence,
};

#[must_use = "failed workflow cancellation retains the exact transitioned projection"]
pub enum WorthQueryTransitionedWorkflowProjectionCancellationOutcome<Previous, D, O, F, L>
where
    L: BasisOperationLane,
{
    Cancelled(WorthQueryCancelledWorkflowProjection<D, O, F, L>),
    Stopped(WorthQueryTransitionedWorkflowProjectionCloseStop<Previous>),
}

#[must_use = "failed workflow disposal retains the exact transitioned projection"]
pub enum WorthQueryTransitionedWorkflowProjectionDisposalOutcome<Previous, D, O, F, L>
where
    L: BasisOperationLane,
{
    Disposed(WorthQueryDisposedWorkflowProjection<D, O, F, L>),
    Stopped(WorthQueryTransitionedWorkflowProjectionCloseStop<Previous>),
}

pub struct WorthQueryTransitionedWorkflowProjectionCloseStop<Previous> {
    projection: Previous,
    detail: String,
    counters: WorthQueryProjectionLifecycleTransitionCounters,
}

impl<Previous> WorthQueryTransitionedWorkflowProjectionCloseStop<Previous> {
    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn counters(&self) -> WorthQueryProjectionLifecycleTransitionCounters {
        self.counters
    }

    pub fn into_projection(self) -> Previous {
        self.projection
    }
}

macro_rules! transitioned_terminal_methods {
    ($state:ident, $phase:ty, $evidence:ident) => {
        impl<D, O, F, L: BasisOperationLane> $state<D, O, F, L> {
            pub fn cancel(
                self,
                workspace: &mut WorthQueryWorkspace,
            ) -> WorthQueryTransitionedWorkflowProjectionCancellationOutcome<Self, D, O, F, L>
            {
                let $state {
                    transitioned,
                    witness,
                } = self;
                let (owner, predecessor_close, work, cleanup_work) = transitioned.into_parts();
                match close_operational_projection::<
                    _,
                    L,
                    $phase,
                    WorthQueryCancelledProjectionPhase,
                >(
                    owner,
                    WorthQueryProjectionLifecycleCloseCause::Cancellation,
                    workspace,
                ) {
                    WorthQueryProjectionCloseCoreOutcome::Closed(closed) => {
                        WorthQueryTransitionedWorkflowProjectionCancellationOutcome::Cancelled(
                            WorthQueryCancelledWorkflowProjection {
                                closed,
                                prior_transition: Some(
                                    WorthQueryProjectionPriorTransitionEvidence::$evidence {
                                        witness,
                                        predecessor_close,
                                        work,
                                        cleanup_work,
                                    },
                                ),
                            },
                        )
                    }
                    WorthQueryProjectionCloseCoreOutcome::Stopped(stop) => {
                        let (owner, kind, counters) = stop.into_parts();
                        let projection = $state {
                            transitioned: WorthQueryTransitionedOperationalProjection::from_parts(
                                owner,
                                predecessor_close,
                                work,
                                cleanup_work,
                            ),
                            witness,
                        };
                        WorthQueryTransitionedWorkflowProjectionCancellationOutcome::Stopped(
                            WorthQueryTransitionedWorkflowProjectionCloseStop {
                                projection,
                                detail: close_detail(kind),
                                counters,
                            },
                        )
                    }
                }
            }

            pub fn dispose(
                self,
                workspace: &mut WorthQueryWorkspace,
            ) -> WorthQueryTransitionedWorkflowProjectionDisposalOutcome<Self, D, O, F, L>
            {
                let $state {
                    transitioned,
                    witness,
                } = self;
                let (owner, predecessor_close, work, cleanup_work) = transitioned.into_parts();
                match close_operational_projection::<
                    _,
                    L,
                    $phase,
                    WorthQueryDisposedProjectionPhase,
                >(
                    owner,
                    WorthQueryProjectionLifecycleCloseCause::Disposal,
                    workspace,
                ) {
                    WorthQueryProjectionCloseCoreOutcome::Closed(closed) => {
                        WorthQueryTransitionedWorkflowProjectionDisposalOutcome::Disposed(
                            WorthQueryDisposedWorkflowProjection {
                                closed,
                                prior_transition: Some(
                                    WorthQueryProjectionPriorTransitionEvidence::$evidence {
                                        witness,
                                        predecessor_close,
                                        work,
                                        cleanup_work,
                                    },
                                ),
                            },
                        )
                    }
                    WorthQueryProjectionCloseCoreOutcome::Stopped(stop) => {
                        let (owner, kind, counters) = stop.into_parts();
                        let projection = $state {
                            transitioned: WorthQueryTransitionedOperationalProjection::from_parts(
                                owner,
                                predecessor_close,
                                work,
                                cleanup_work,
                            ),
                            witness,
                        };
                        WorthQueryTransitionedWorkflowProjectionDisposalOutcome::Stopped(
                            WorthQueryTransitionedWorkflowProjectionCloseStop {
                                projection,
                                detail: close_detail(kind),
                                counters,
                            },
                        )
                    }
                }
            }
        }
    };
}

transitioned_terminal_methods!(
    WorthQueryReplacedWorkflowProjection,
    super::transition_states::WorthQueryReplacedProjectionPhase,
    Replacement
);
transitioned_terminal_methods!(
    WorthQueryReboundWorkflowProjection,
    super::transition_states::WorthQueryReboundProjectionPhase,
    Rebind
);

fn close_detail(kind: WorthQueryProjectionCloseCoreStopKind) -> String {
    match kind {
        WorthQueryProjectionCloseCoreStopKind::Runtime(detail) => detail,
    }
}
