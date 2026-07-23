use super::WorthUiFrameworkTurnExecution;

/// A preview-bearing completion is consumed by exactly one terminal transition.
///
/// ```compile_fail
/// use worth_ui_runtime::facade::host_observation::{
///     UiHostPreviewDiscardReason, WorthUiPreviewPaintHost,
/// };
/// use worth_ui_runtime::facade::runtime_handoff::WorthUiFrameworkTurnCompletion;
/// fn resolve_twice(
///     completion: WorthUiFrameworkTurnCompletion<'_>,
///     host: &mut impl WorthUiPreviewPaintHost,
/// ) {
///     let _first = completion.resolve_preview_paint(host);
///     let _second = completion.discard_preview_paint(UiHostPreviewDiscardReason::Superseded);
/// }
/// ```
#[derive(Debug)]
#[must_use = "framework turn completion must be executed or explicitly resolved"]
pub enum WorthUiFrameworkTurnCompletion<'runtime> {
    ReadyToExecute {
        execution: WorthUiFrameworkTurnExecution<'runtime>,
    },
    AllocationInvalidationsNarrowed {
        plan: crate::runtime::UiNarrowedAllocationFramePlan,
        selection: crate::graph::UiAdmittedReplanNeighborhoodSet,
        transaction: crate::runtime::UiAllocationReplanTransactionOutcome,
        planning_counters: super::UiFrameworkTransitionPlanningCounters,
    },
    ViewportResizeResolved {
        outcome: crate::runtime::UiViewportResizeOutcome,
        planning_counters: super::UiFrameworkTransitionPlanningCounters,
    },
    ViewportResizeDenied {
        denial: crate::runtime::UiViewportResizeDenial,
        planning_counters: super::UiFrameworkTransitionPlanningCounters,
    },
    ResizePreviewPublished {
        pending: WorthUiPendingPreviewPaint<'runtime>,
        planning_counters: super::UiFrameworkTransitionPlanningCounters,
    },
    DurableResizeCommitted {
        outcome: crate::runtime::UiDurableResizeCommitOutcome,
        selection: crate::graph::UiAdmittedReplanNeighborhoodSet,
        planning_counters: super::UiFrameworkTransitionPlanningCounters,
    },
    DragResizePreviewPending {
        preview: WorthUiPendingPreviewPaint<'runtime>,
        durable: WorthUiPendingDurableResize<'runtime>,
        planning_counters: super::UiFrameworkTransitionPlanningCounters,
    },
    AllocationReplanSelectionDenied {
        denial: crate::graph::UiReplanLocalityDenial,
    },
    AllocationFrameResolutionDenied {
        rejection: crate::runtime::UiAllocationFrameRejection,
    },
    AllocationInvalidationNarrowingDenied {
        rejection: crate::runtime::UiAllocationInvalidationNarrowingRejection,
    },
    FrameworkTransitionPlanningDenied {
        denial: super::UiFrameworkTransitionPlanningDenial,
    },
    FrameworkTransitionExecutionDenied {
        denial: super::UiFrameworkTransitionExecutionDenial,
    },
    Denied {
        denial: crate::runtime::UiAllocationFrameDispatchDenial,
        counters: crate::runtime::UiAllocationFrameDispatcherCounters,
    },
}

#[derive(Debug)]
pub enum WorthUiPreviewPaintFollowOn {
    PreviewOnly,
    DurableResizeCommitted {
        outcome: Box<crate::runtime::UiDurableResizeCommitOutcome>,
        selection: Box<crate::graph::UiAdmittedReplanNeighborhoodSet>,
    },
    DurableResizeDenied {
        report: Box<crate::runtime::UiDurableResizeCommitDenialReport>,
        selection: Box<crate::graph::UiAdmittedReplanNeighborhoodSet>,
    },
    DurableResizeSuppressedByPreviewIsolation {
        violation: crate::runtime::UiPreviewPaintIsolationViolation,
        selection: Box<crate::graph::UiAdmittedReplanNeighborhoodSet>,
    },
}

#[derive(Debug)]
pub struct WorthUiResolvedPreviewPaintCompletion {
    disposition: crate::host::UiHostPreviewPaintDisposition,
    isolation: crate::runtime::UiPreviewPaintIsolationOutcome,
    follow_on: WorthUiPreviewPaintFollowOn,
}

#[derive(Debug)]
#[must_use = "pending preview paint must be consumed or explicitly discarded"]
pub struct WorthUiPendingPreviewPaint<'runtime> {
    paint: crate::host::UiHostPreviewPaintInput,
    isolation: crate::runtime::allocation_receipt::UiPreviewPaintIsolationPort<'runtime>,
}

impl<'runtime> WorthUiPendingPreviewPaint<'runtime> {
    pub(super) fn new(
        paint: crate::host::UiHostPreviewPaintInput,
        isolation: crate::runtime::allocation_receipt::UiPreviewPaintIsolationPort<'runtime>,
    ) -> Self {
        Self { paint, isolation }
    }
    fn finish(
        self,
        finish: impl FnOnce(
            crate::host::UiHostPreviewPaintInput,
        ) -> crate::host::UiHostPreviewPaintDisposition,
    ) -> (
        crate::host::UiHostPreviewPaintDisposition,
        crate::runtime::UiPreviewPaintIsolationOutcome,
    ) {
        let frame_epoch = self.paint.frame_epoch();
        let before = self.isolation.capture();
        let disposition = finish(self.paint);
        let after = self.isolation.capture();
        (disposition, self.isolation.seal(frame_epoch, before, after))
    }
}

#[derive(Debug)]
#[must_use = "pending durable resize must follow preview disposition"]
pub struct WorthUiPendingDurableResize<'runtime> {
    commit_port: super::UiPendingDurableResizeCommitPort<'runtime>,
    selection: crate::graph::UiAdmittedReplanNeighborhoodSet,
    identity: u64,
    extent: crate::runtime::UiResizeLogicalExtent,
}

impl<'runtime> WorthUiPendingDurableResize<'runtime> {
    pub(super) fn new(
        commit_port: super::UiPendingDurableResizeCommitPort<'runtime>,
        selection: crate::graph::UiAdmittedReplanNeighborhoodSet,
        identity: u64,
        extent: crate::runtime::UiResizeLogicalExtent,
    ) -> Self {
        Self {
            commit_port,
            selection,
            identity,
            extent,
        }
    }

    fn commit(self) -> WorthUiPreviewPaintFollowOn {
        let frame_epoch = self.selection.frame_epoch();
        let (transaction, durable_state, mutated) = self.commit_port.commit();
        match transaction {
            crate::runtime::UiAllocationReplanTransactionOutcome::Committed(committed) => {
                WorthUiPreviewPaintFollowOn::DurableResizeCommitted {
                    outcome: Box::new(crate::runtime::UiDurableResizeCommitOutcome::new(
                        self.extent,
                        committed,
                        durable_state.expect("committed drag owns activated semantic state"),
                        mutated,
                        false,
                    )),
                    selection: Box::new(self.selection),
                }
            }
            crate::runtime::UiAllocationReplanTransactionOutcome::Replayed(committed) => {
                WorthUiPreviewPaintFollowOn::DurableResizeCommitted {
                    outcome: Box::new(crate::runtime::UiDurableResizeCommitOutcome::new(
                        self.extent,
                        committed,
                        durable_state.expect("replayed drag owns activated semantic state"),
                        false,
                        true,
                    )),
                    selection: Box::new(self.selection),
                }
            }
            crate::runtime::UiAllocationReplanTransactionOutcome::Denied(denial) => {
                WorthUiPreviewPaintFollowOn::DurableResizeDenied {
                    report: Box::new(crate::runtime::UiDurableResizeCommitDenialReport::new(
                        denial,
                        self.identity,
                        self.extent,
                        frame_epoch,
                    )),
                    selection: Box::new(self.selection),
                }
            }
        }
    }
}

impl<'runtime> WorthUiFrameworkTurnCompletion<'runtime> {
    pub fn planning_counters(&self) -> Option<super::UiFrameworkTransitionPlanningCounters> {
        match self {
            Self::ReadyToExecute { execution } => Some(execution.planning_counters()),
            Self::AllocationInvalidationsNarrowed {
                planning_counters, ..
            }
            | Self::ViewportResizeResolved {
                planning_counters, ..
            }
            | Self::ViewportResizeDenied {
                planning_counters, ..
            }
            | Self::ResizePreviewPublished {
                planning_counters, ..
            }
            | Self::DurableResizeCommitted {
                planning_counters, ..
            }
            | Self::DragResizePreviewPending {
                planning_counters, ..
            } => Some(*planning_counters),
            _ => None,
        }
    }

    pub fn into_execution(self) -> Result<WorthUiFrameworkTurnExecution<'runtime>, Box<Self>> {
        match self {
            Self::ReadyToExecute { execution } => Ok(execution),
            other => Err(Box::new(other)),
        }
    }
    pub fn narrowed_plan(&self) -> Option<&crate::runtime::UiNarrowedAllocationFramePlan> {
        match self {
            Self::AllocationInvalidationsNarrowed { plan, .. } => Some(plan),
            _ => None,
        }
    }
    pub fn replan_selection(&self) -> Option<&crate::graph::UiAdmittedReplanNeighborhoodSet> {
        match self {
            Self::AllocationInvalidationsNarrowed { selection, .. }
            | Self::DurableResizeCommitted { selection, .. }
            | Self::DragResizePreviewPending {
                durable: WorthUiPendingDurableResize { selection, .. },
                ..
            } => Some(selection),
            _ => None,
        }
    }
    pub fn replan_transaction(
        &self,
    ) -> Option<&crate::runtime::UiAllocationReplanTransactionOutcome> {
        match self {
            Self::AllocationInvalidationsNarrowed { transaction, .. } => Some(transaction),
            _ => None,
        }
    }
    pub fn denied_replan_inspection(
        &self,
    ) -> Option<worth_ui_inspection::UiAllocationInspectionDeniedAttempt> {
        let Self::AllocationInvalidationsNarrowed {
            plan,
            selection,
            transaction: crate::runtime::UiAllocationReplanTransactionOutcome::Denied(denial),
            ..
        } = self
        else {
            return None;
        };
        Some(crate::evidence::project_denied_replan_inspection(
            plan, selection, denial,
        ))
    }
    pub fn viewport_resize_outcome(&self) -> Option<&crate::runtime::UiViewportResizeOutcome> {
        match self {
            Self::ViewportResizeResolved { outcome, .. } => Some(outcome),
            _ => None,
        }
    }
    pub fn durable_resize_outcome(&self) -> Option<&crate::runtime::UiDurableResizeCommitOutcome> {
        match self {
            Self::DurableResizeCommitted { outcome, .. } => Some(outcome),
            _ => None,
        }
    }
    pub fn resolve_preview_paint(
        self,
        host: &mut impl crate::host::WorthUiPreviewPaintHost,
    ) -> Result<WorthUiResolvedPreviewPaintCompletion, Box<Self>> {
        self.finish_preview(|input| input.consume(host))
    }
    pub fn discard_preview_paint(
        self,
        reason: crate::host::UiHostPreviewDiscardReason,
    ) -> Result<WorthUiResolvedPreviewPaintCompletion, Box<Self>> {
        self.finish_preview(|input| input.discard(reason))
    }
    fn finish_preview(
        self,
        finish: impl FnOnce(
            crate::host::UiHostPreviewPaintInput,
        ) -> crate::host::UiHostPreviewPaintDisposition,
    ) -> Result<WorthUiResolvedPreviewPaintCompletion, Box<Self>> {
        match self {
            Self::ResizePreviewPublished { pending, .. } => {
                let (disposition, isolation) = pending.finish(finish);
                Ok(WorthUiResolvedPreviewPaintCompletion {
                    disposition,
                    isolation,
                    follow_on: WorthUiPreviewPaintFollowOn::PreviewOnly,
                })
            }
            Self::DragResizePreviewPending {
                preview, durable, ..
            } => {
                let (disposition, isolation) = preview.finish(finish);
                let follow_on = match isolation {
                    crate::runtime::UiPreviewPaintIsolationOutcome::Verified(_) => durable.commit(),
                    crate::runtime::UiPreviewPaintIsolationOutcome::Violated(violation) => {
                        WorthUiPreviewPaintFollowOn::DurableResizeSuppressedByPreviewIsolation {
                            violation,
                            selection: Box::new(durable.selection),
                        }
                    }
                };
                Ok(WorthUiResolvedPreviewPaintCompletion {
                    disposition,
                    isolation,
                    follow_on,
                })
            }
            other => Err(Box::new(other)),
        }
    }
}

impl WorthUiResolvedPreviewPaintCompletion {
    pub fn disposition(&self) -> crate::host::UiHostPreviewPaintDisposition {
        self.disposition
    }
    pub fn follow_on(&self) -> &WorthUiPreviewPaintFollowOn {
        &self.follow_on
    }
    pub fn isolation(&self) -> crate::runtime::UiPreviewPaintIsolationOutcome {
        self.isolation
    }
}
impl WorthUiPreviewPaintFollowOn {
    pub fn durable_resize_outcome(&self) -> Option<&crate::runtime::UiDurableResizeCommitOutcome> {
        match self {
            Self::PreviewOnly => None,
            Self::DurableResizeCommitted { outcome, .. } => Some(outcome.as_ref()),
            Self::DurableResizeDenied { .. } => None,
            Self::DurableResizeSuppressedByPreviewIsolation { .. } => None,
        }
    }
    pub fn durable_resize_denial(
        &self,
    ) -> Option<&crate::runtime::UiDurableResizeCommitDenialReport> {
        match self {
            Self::DurableResizeDenied { report, .. } => Some(report.as_ref()),
            _ => None,
        }
    }
    pub fn replan_selection(&self) -> Option<&crate::graph::UiAdmittedReplanNeighborhoodSet> {
        match self {
            Self::PreviewOnly => None,
            Self::DurableResizeCommitted { selection, .. } => Some(selection.as_ref()),
            Self::DurableResizeDenied { selection, .. } => Some(selection.as_ref()),
            Self::DurableResizeSuppressedByPreviewIsolation { selection, .. } => {
                Some(selection.as_ref())
            }
        }
    }
}
