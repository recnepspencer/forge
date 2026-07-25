use super::WorthUiFrameworkTurnExecution;

mod follow_on;

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
        pending: WorthUiPendingMountedPreviewProjection<'runtime>,
        planning_counters: super::UiFrameworkTransitionPlanningCounters,
    },
    DurableResizeCommitted {
        outcome: crate::runtime::UiDurableResizeCommitOutcome,
        selection: crate::graph::UiAdmittedReplanNeighborhoodSet,
        planning_counters: super::UiFrameworkTransitionPlanningCounters,
    },
    DragResizePreviewPending {
        preview: WorthUiPendingMountedPreviewProjection<'runtime>,
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
pub enum WorthUiMountedPreviewFollowOn {
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

pub(crate) enum UiPendingMountedPreviewTransition<'runtime> {
    PreviewOnly {
        preview: WorthUiPendingMountedPreviewProjection<'runtime>,
    },
    DragResize {
        preview: WorthUiPendingMountedPreviewProjection<'runtime>,
        durable: Box<WorthUiPendingDurableResize<'runtime>>,
    },
}

pub(crate) struct UiResolvedMountedPreviewTransition {
    pub(crate) isolation: crate::runtime::UiPreviewPaintIsolationOutcome,
    pub(crate) follow_on: WorthUiMountedPreviewFollowOn,
}

#[derive(Debug)]
#[must_use = "pending mounted preview projection must be presented or superseded"]
pub struct WorthUiPendingMountedPreviewProjection<'runtime> {
    preview: crate::runtime::UiResizePreviewOutcome,
    isolation: crate::runtime::allocation_receipt::UiPreviewPaintIsolationPort<'runtime>,
}

impl<'runtime> WorthUiPendingMountedPreviewProjection<'runtime> {
    pub(super) fn new(
        preview: crate::runtime::UiResizePreviewOutcome,
        isolation: crate::runtime::allocation_receipt::UiPreviewPaintIsolationPort<'runtime>,
    ) -> Self {
        Self { preview, isolation }
    }

    pub fn target(&self) -> crate::graph::UiGraphNodeIdentity {
        self.preview.preview_candidate().target()
    }

    pub fn frame_epoch(&self) -> crate::runtime::UiAllocationFrameEpoch {
        self.preview.preview_candidate().frame_epoch()
    }

    pub fn extent(&self) -> crate::runtime::UiResizeLogicalExtent {
        self.preview.preview_candidate().extent()
    }

    pub fn candidate_count(&self) -> u16 {
        u16::try_from(
            self.preview
                .preview_candidate()
                .allocation_candidates()
                .len(),
        )
        .expect("preview candidate count is bounded by admitted allocation breadth")
    }

    pub fn all_candidates_admitted(&self) -> bool {
        self.preview
            .preview_candidate()
            .allocation_candidates()
            .iter()
            .all(|candidate| candidate.candidate_is_admitted())
    }

    pub fn stream_counters(&self) -> crate::runtime::UiDragResizeCounters {
        self.preview.counters()
    }

    pub(crate) fn capture_isolation_basis(
        &self,
    ) -> crate::runtime::allocation_receipt::UiAllocationTruthRevision {
        self.isolation.capture()
    }

    pub(crate) fn seal_isolation(
        self,
        before: crate::runtime::allocation_receipt::UiAllocationTruthRevision,
    ) -> crate::runtime::UiPreviewPaintIsolationOutcome {
        let frame_epoch = self.frame_epoch();
        let after = self.isolation.capture();
        self.isolation.seal(frame_epoch, before, after)
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

    fn commit(self) -> WorthUiMountedPreviewFollowOn {
        let frame_epoch = self.selection.frame_epoch();
        let (transaction, durable_state, mutated) = self.commit_port.commit();
        match transaction {
            crate::runtime::UiAllocationReplanTransactionOutcome::Committed(committed) => {
                WorthUiMountedPreviewFollowOn::DurableResizeCommitted {
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
                WorthUiMountedPreviewFollowOn::DurableResizeCommitted {
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
                WorthUiMountedPreviewFollowOn::DurableResizeDenied {
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
            | Self::DurableResizeCommitted { selection, .. } => Some(selection),
            Self::DragResizePreviewPending { durable, .. } => Some(&durable.selection),
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
    pub(crate) fn into_pending_mounted_preview(
        self,
    ) -> Result<
        (
            UiPendingMountedPreviewTransition<'runtime>,
            super::UiFrameworkTransitionPlanningCounters,
        ),
        Box<Self>,
    > {
        match self {
            Self::ResizePreviewPublished {
                pending,
                planning_counters,
            } => Ok((
                UiPendingMountedPreviewTransition::PreviewOnly { preview: pending },
                planning_counters,
            )),
            Self::DragResizePreviewPending {
                preview,
                durable,
                planning_counters,
            } => Ok((
                UiPendingMountedPreviewTransition::DragResize {
                    preview,
                    durable: Box::new(durable),
                },
                planning_counters,
            )),
            other => Err(Box::new(other)),
        }
    }
}

impl UiPendingMountedPreviewTransition<'_> {
    pub(crate) fn preview(&self) -> &WorthUiPendingMountedPreviewProjection<'_> {
        match self {
            Self::PreviewOnly { preview } | Self::DragResize { preview, .. } => preview,
        }
    }

    pub(crate) fn finish(
        self,
        before: crate::runtime::allocation_receipt::UiAllocationTruthRevision,
    ) -> UiResolvedMountedPreviewTransition {
        let (preview, durable) = match self {
            Self::PreviewOnly { preview } => (preview, None),
            Self::DragResize { preview, durable } => (preview, Some(*durable)),
        };
        let isolation = preview.seal_isolation(before);
        let follow_on = match (isolation, durable) {
            (crate::runtime::UiPreviewPaintIsolationOutcome::Verified(_), Some(durable)) => {
                durable.commit()
            }
            (crate::runtime::UiPreviewPaintIsolationOutcome::Verified(_), None) => {
                WorthUiMountedPreviewFollowOn::PreviewOnly
            }
            (
                crate::runtime::UiPreviewPaintIsolationOutcome::Violated(violation),
                Some(durable),
            ) => WorthUiMountedPreviewFollowOn::DurableResizeSuppressedByPreviewIsolation {
                violation,
                selection: Box::new(durable.selection),
            },
            (crate::runtime::UiPreviewPaintIsolationOutcome::Violated(_), None) => {
                WorthUiMountedPreviewFollowOn::PreviewOnly
            }
        };
        UiResolvedMountedPreviewTransition {
            isolation,
            follow_on,
        }
    }
}
