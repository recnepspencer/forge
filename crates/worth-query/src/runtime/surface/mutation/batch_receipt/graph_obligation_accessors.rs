use crate::runtime::WorthQueryGraphObligationAttachmentEvidence;

use super::WorthQueryBatchWriteReceipt;

impl WorthQueryBatchWriteReceipt {
    pub fn graph_obligation_evidence(&self) -> Option<WorthQueryGraphObligationAttachmentEvidence> {
        self.obligation_dispatch
            .as_ref()
            .map(|dispatch| dispatch.attachment_evidence())
    }

    pub fn graph_obligation_envelope_digest(&self) -> Option<&str> {
        self.obligation_dispatch
            .as_ref()
            .and_then(|dispatch| dispatch.envelope_digest())
    }

    pub(in crate::runtime) fn with_obligation_dispatch(
        mut self,
        obligation_dispatch: Option<
            crate::runtime::WorthQueryAuthoritativeMutationObligationDispatch,
        >,
    ) -> Self {
        self.obligation_dispatch = obligation_dispatch;
        self
    }
}
