use super::{UiAllocationFrameGatewayOutcome, UiAllocationFrameSourceFact};
use crate::host::UiAdmittedHostMeasurement;
use std::cell::RefCell;
use std::rc::Rc;

use super::super::framework_turn::UiAllocationFrameIngressMailbox;

/// Submission-only capability for already-current host measurement evidence.
///
/// ```compile_fail
/// use worth_ui_runtime::facade::runtime_handoff::WorthUiHostMeasurementSubmission;
///
/// fn host_cannot_submit_query(mut host: WorthUiHostMeasurementSubmission) {
///     host.submit_query_projection_settlement(todo!());
/// }
/// ```
///
/// ```compile_fail
/// use worth_ui_runtime::facade::runtime_handoff::WorthUiHostMeasurementSubmission;
///
/// fn host_cannot_pump(mut host: WorthUiHostMeasurementSubmission) {
///     host.pump_allocation_frame();
/// }
/// ```
pub struct WorthUiHostMeasurementSubmission {
    mailbox: Rc<RefCell<UiAllocationFrameIngressMailbox>>,
}

impl WorthUiHostMeasurementSubmission {
    pub(in crate::runtime::allocation_frame_dispatch) fn new(
        mailbox: Rc<RefCell<UiAllocationFrameIngressMailbox>>,
    ) -> Self {
        Self { mailbox }
    }

    pub fn submit_admitted_host_measurement(
        &mut self,
        admitted: UiAdmittedHostMeasurement,
    ) -> UiAllocationFrameGatewayOutcome {
        let identity = admitted.source_identity();
        let generation = admitted.source_generation();
        let order = admitted.source_order();
        let fact = UiAllocationFrameSourceFact::HostMeasurement(admitted);
        self.mailbox
            .borrow_mut()
            .submit_host(identity, generation, order, order, fact)
    }
}
