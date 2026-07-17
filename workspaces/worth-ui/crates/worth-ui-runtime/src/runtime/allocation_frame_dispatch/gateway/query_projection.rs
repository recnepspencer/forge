use worth_ui_query_binding::WorthUiQueryMeasurementFactSettlement;

use super::super::framework_turn::UiAllocationFrameIngressMailbox;
use super::{
    UiAllocationFrameGatewayOutcome, UiAllocationFrameQuerySettlementPosture,
    UiAllocationFrameQueryWarningPosture, UiAllocationFrameSourceFact,
};
use std::cell::RefCell;
use std::rc::Rc;
use worth_ui_query_binding::WorthUiQueryProjectionWarningKind;

/// Submission-only capability for an admitted Query projection settlement.
///
/// ```compile_fail
/// use worth_ui_runtime::facade::runtime_handoff::WorthUiQueryProjectionSubmission;
///
/// fn query_cannot_submit_interaction(mut query: WorthUiQueryProjectionSubmission) {
///     query.submit_admitted_transient_interaction(todo!());
/// }
/// ```
pub struct WorthUiQueryProjectionSubmission {
    mailbox: Rc<RefCell<UiAllocationFrameIngressMailbox>>,
}

impl WorthUiQueryProjectionSubmission {
    pub(in crate::runtime::allocation_frame_dispatch) fn new(
        mailbox: Rc<RefCell<UiAllocationFrameIngressMailbox>>,
    ) -> Self {
        Self { mailbox }
    }

    pub fn submit_query_projection_settlement(
        &mut self,
        settlement: WorthUiQueryMeasurementFactSettlement,
    ) -> UiAllocationFrameGatewayOutcome {
        let source_identity = super::super::UiAllocationFrameSourceIdentity::from_query(
            settlement.allocation_source_identity().clone(),
        );
        let ingress_identity = settlement.allocation_ingress_identity();
        let source_generation = settlement.allocation_source_generation().as_u64();
        let source_order = settlement.allocation_source_order().as_u64();
        let posture = if settlement.is_partial() {
            UiAllocationFrameQuerySettlementPosture::Partial
        } else {
            UiAllocationFrameQuerySettlementPosture::Settled
        };
        let warnings = query_warning_posture(&settlement);
        let fact = UiAllocationFrameSourceFact::QueryProjection {
            source: Box::new(settlement),
            posture,
            warnings,
        };
        self.mailbox.borrow_mut().submit_query(
            source_identity,
            source_generation,
            ingress_identity,
            source_order,
            fact,
        )
    }
}

fn query_warning_posture(
    settlement: &WorthUiQueryMeasurementFactSettlement,
) -> UiAllocationFrameQueryWarningPosture {
    let mut row_bound = false;
    let mut preview = false;
    for warning in settlement.warning_kinds() {
        match warning {
            WorthUiQueryProjectionWarningKind::QueryContextRowBound => row_bound = true,
            WorthUiQueryProjectionWarningKind::PreviewDerivedContext => preview = true,
        }
    }
    match (row_bound, preview) {
        (false, false) => UiAllocationFrameQueryWarningPosture::None,
        (true, false) => UiAllocationFrameQueryWarningPosture::QueryContextRowBound,
        (false, true) => UiAllocationFrameQueryWarningPosture::PreviewDerivedContext,
        (true, true) => {
            UiAllocationFrameQueryWarningPosture::QueryContextRowBoundAndPreviewDerivedContext
        }
    }
}
