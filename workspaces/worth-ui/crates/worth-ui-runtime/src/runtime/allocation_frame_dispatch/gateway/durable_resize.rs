use crate::runtime::WorthUiAdmittedDurableResizeSourceFact;

use super::super::framework_turn::UiAllocationFrameIngressMailbox;
use super::{UiAllocationFrameGatewayOutcome, UiAllocationFrameSourceFact};
use std::cell::RefCell;
use std::rc::Rc;

/// Submission-only capability for an admitted durable-resize reconciliation input.
///
/// ```compile_fail
/// use worth_ui_runtime::facade::runtime_handoff::WorthUiDurableResizeSubmission;
///
/// fn resize_cannot_submit_host(mut resize: WorthUiDurableResizeSubmission) {
///     resize.submit_current_host_measurement(todo!());
/// }
/// ```
pub struct WorthUiDurableResizeSubmission {
    mailbox: Rc<RefCell<UiAllocationFrameIngressMailbox>>,
}

impl WorthUiDurableResizeSubmission {
    pub(in crate::runtime::allocation_frame_dispatch) fn new(
        mailbox: Rc<RefCell<UiAllocationFrameIngressMailbox>>,
    ) -> Self {
        Self { mailbox }
    }

    pub fn submit_admitted_durable_resize(
        &mut self,
        admitted: WorthUiAdmittedDurableResizeSourceFact,
    ) -> UiAllocationFrameGatewayOutcome {
        let identity = admitted.source_identity();
        let generation = admitted.source_generation();
        let order = admitted.source_order();
        let fact = UiAllocationFrameSourceFact::DurableResize(admitted);
        self.mailbox
            .borrow_mut()
            .submit_durable_resize(identity, generation, order, order, fact)
    }
}
