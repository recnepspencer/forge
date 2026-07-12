use crate::runtime::WorthUiAdmittedTransientInteraction;

use super::super::framework_turn::UiAllocationFrameIngressMailbox;
use super::{UiAllocationFrameGatewayOutcome, UiAllocationFrameSourceFact};
use std::cell::RefCell;
use std::rc::Rc;

/// Submission-only capability for runtime-admitted transient interaction state.
///
/// ```compile_fail
/// use worth_ui_runtime::facade::runtime_handoff::WorthUiInteractionSubmission;
///
/// fn interaction_cannot_submit_resize(mut interaction: WorthUiInteractionSubmission) {
///     interaction.submit_admitted_durable_resize(todo!());
/// }
/// ```
pub struct WorthUiInteractionSubmission {
    mailbox: Rc<RefCell<UiAllocationFrameIngressMailbox>>,
}

impl WorthUiInteractionSubmission {
    pub(in crate::runtime::allocation_frame_dispatch) fn new(
        mailbox: Rc<RefCell<UiAllocationFrameIngressMailbox>>,
    ) -> Self {
        Self { mailbox }
    }

    pub fn submit_admitted_transient_interaction(
        &mut self,
        admitted: WorthUiAdmittedTransientInteraction,
    ) -> UiAllocationFrameGatewayOutcome {
        let fact = UiAllocationFrameSourceFact::Interaction(admitted);
        self.mailbox.borrow_mut().submit_interaction(
            admitted.source_identity(),
            admitted.source_generation(),
            admitted.source_order(),
            admitted.source_order(),
            fact,
        )
    }
}
