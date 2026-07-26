use super::UiFrameworkTransitionPlanningCounters;

#[derive(Debug)]
pub(in crate::runtime::allocation_frame_dispatch::framework_turn) struct UiPlannedFrameworkTransition
{
    active_generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    predecessor_frame_epoch: crate::runtime::WorthUiRuntimeFrameEpoch,
    authority: UiFrameworkTransitionAuthorityPlan,
    counters: UiFrameworkTransitionPlanningCounters,
    family: UiFrameworkTransitionFamilyPlan,
}

#[derive(Debug)]
pub(in crate::runtime::allocation_frame_dispatch::framework_turn) enum UiFrameworkTransitionAuthorityPlan
{
    AdmittedFrame {
        frame_epoch_assignment:
            crate::runtime::allocation_frame_dispatch::UiAllocationFrameEpochAssignment,
        source_order_transition:
            Box<crate::runtime::stream_policy::UiAllocationSourceOrderTransition>,
    },
}

#[derive(Debug)]
pub(in crate::runtime::allocation_frame_dispatch::framework_turn) enum UiFrameworkTransitionFamilyPlan
{
    NoIngress,
    Ordinary(UiOrdinaryAllocationExecutionPlan),
    Viewport(UiViewportResizeExecutionPlan),
    ViewportDenied(crate::runtime::UiViewportResizeDenial),
    ResizePreview(crate::runtime::UiResizePreviewOutcome),
    AllocationDenied(UiDeniedAllocationExecutionPlan),
    DurableResize(UiDurableResizeExecutionPlan),
    DragResize(UiDragResizeExecutionPlan),
}

#[derive(Debug)]
pub(in crate::runtime::allocation_frame_dispatch::framework_turn) struct UiOrdinaryAllocationExecutionPlan
{
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) plan:
        crate::runtime::UiNarrowedAllocationFramePlan,
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) selection:
        crate::graph::UiAdmittedReplanNeighborhoodSet,
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) transaction:
        super::super::allocation_transaction::UiPendingAllocationTransaction,
}

#[derive(Debug)]
pub(in crate::runtime::allocation_frame_dispatch::framework_turn) struct UiViewportResizeExecutionPlan
{
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) transaction:
        super::super::allocation_transaction::UiPendingAllocationTransaction,
}

#[derive(Debug)]
pub(in crate::runtime::allocation_frame_dispatch::framework_turn) struct UiDeniedAllocationExecutionPlan
{
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) plan:
        crate::runtime::UiNarrowedAllocationFramePlan,
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) selection:
        crate::graph::UiAdmittedReplanNeighborhoodSet,
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) denial:
        crate::runtime::UiAllocationReplanTransactionCommitDenial,
}

#[derive(Debug)]
pub(in crate::runtime::allocation_frame_dispatch::framework_turn) struct UiDurableResizeExecutionPlan
{
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) plan:
        crate::runtime::UiNarrowedAllocationFramePlan,
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) selection:
        crate::graph::UiAdmittedReplanNeighborhoodSet,
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) transaction:
        super::super::allocation_transaction::UiPendingAllocationTransaction,
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) extent:
        crate::runtime::UiResizeLogicalExtent,
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) previous_extent:
        Option<crate::runtime::UiResizeLogicalExtent>,
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) requested_mutation: bool,
}

#[derive(Debug)]
pub(in crate::runtime::allocation_frame_dispatch::framework_turn) struct UiDragResizeExecutionPlan {
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) preview:
        crate::runtime::UiResizePreviewOutcome,
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) selection:
        crate::graph::UiAdmittedReplanNeighborhoodSet,
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) transaction:
        super::super::allocation_transaction::UiPendingAllocationTransaction,
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) identity_digest: u64,
    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) extent:
        crate::runtime::UiResizeLogicalExtent,
}

impl UiPlannedFrameworkTransition {
    pub(super) fn admitted_frame(
        active_generation: crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
        predecessor_frame_epoch: crate::runtime::WorthUiRuntimeFrameEpoch,
        frame_epoch_assignment: crate::runtime::allocation_frame_dispatch::UiAllocationFrameEpochAssignment,
        source_order_transition: Box<
            crate::runtime::stream_policy::UiAllocationSourceOrderTransition,
        >,
        counters: UiFrameworkTransitionPlanningCounters,
        family: UiFrameworkTransitionFamilyPlan,
    ) -> Self {
        Self {
            active_generation,
            predecessor_frame_epoch,
            authority: UiFrameworkTransitionAuthorityPlan::AdmittedFrame {
                frame_epoch_assignment,
                source_order_transition,
            },
            counters,
            family,
        }
    }

    pub(in crate::runtime::allocation_frame_dispatch::framework_turn) fn into_parts(
        self,
    ) -> (
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
        crate::runtime::WorthUiRuntimeFrameEpoch,
        UiFrameworkTransitionAuthorityPlan,
        UiFrameworkTransitionPlanningCounters,
        UiFrameworkTransitionFamilyPlan,
    ) {
        (
            self.active_generation,
            self.predecessor_frame_epoch,
            self.authority,
            self.counters,
            self.family,
        )
    }
}
