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
use super::transition_states::{
    WorthQueryReboundDomainProjection, WorthQueryReplacedDomainProjection,
};
use super::{
    WorthQueryCancelledDomainProjection, WorthQueryDisposedDomainProjection,
    WorthQueryProjectionLifecycleCloseCause, WorthQueryProjectionLifecycleTransitionCounters,
    WorthQueryProjectionPriorTransitionEvidence,
};

#[must_use = "failed cancellation retains the exact transitioned projection"]
pub enum WorthQueryTransitionedProjectionCancellationOutcome<Previous, D, O, F, L>
where
    L: BasisOperationLane,
{
    Cancelled(WorthQueryCancelledDomainProjection<D, O, F, L>),
    Stopped(WorthQueryTransitionedProjectionCloseStop<Previous>),
}

#[must_use = "failed disposal retains the exact transitioned projection"]
pub enum WorthQueryTransitionedProjectionDisposalOutcome<Previous, D, O, F, L>
where
    L: BasisOperationLane,
{
    Disposed(WorthQueryDisposedDomainProjection<D, O, F, L>),
    Stopped(WorthQueryTransitionedProjectionCloseStop<Previous>),
}

pub struct WorthQueryTransitionedProjectionCloseStop<Previous> {
    projection: Previous,
    detail: String,
    counters: WorthQueryProjectionLifecycleTransitionCounters,
}

impl<Previous> WorthQueryTransitionedProjectionCloseStop<Previous> {
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
    (
        $state:ident,
        $phase:ty,
        $evidence:ident
    ) => {
        impl<D, O, F, L: BasisOperationLane> $state<D, O, F, L> {
            pub fn cancel(
                self,
                workspace: &mut WorthQueryWorkspace,
            ) -> WorthQueryTransitionedProjectionCancellationOutcome<Self, D, O, F, L> {
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
                        WorthQueryTransitionedProjectionCancellationOutcome::Cancelled(
                            WorthQueryCancelledDomainProjection {
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
                        WorthQueryTransitionedProjectionCancellationOutcome::Stopped(
                            WorthQueryTransitionedProjectionCloseStop {
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
            ) -> WorthQueryTransitionedProjectionDisposalOutcome<Self, D, O, F, L> {
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
                        WorthQueryTransitionedProjectionDisposalOutcome::Disposed(
                            WorthQueryDisposedDomainProjection {
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
                        WorthQueryTransitionedProjectionDisposalOutcome::Stopped(
                            WorthQueryTransitionedProjectionCloseStop {
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
    WorthQueryReplacedDomainProjection,
    super::transition_states::WorthQueryReplacedProjectionPhase,
    Replacement
);
transitioned_terminal_methods!(
    WorthQueryReboundDomainProjection,
    super::transition_states::WorthQueryReboundProjectionPhase,
    Rebind
);

fn close_detail(kind: WorthQueryProjectionCloseCoreStopKind) -> String {
    match kind {
        WorthQueryProjectionCloseCoreStopKind::Runtime(detail) => detail,
    }
}
