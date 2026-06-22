use crate::runtime::ForgeQueryGraphObligationAttachmentEvidence;

use super::ForgeQueryBatchWriteReceipt;

impl ForgeQueryBatchWriteReceipt {
    pub fn graph_obligation_evidence(&self) -> Option<ForgeQueryGraphObligationAttachmentEvidence> {
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
            crate::runtime::ForgeQueryAuthoritativeMutationObligationDispatch,
        >,
    ) -> Self {
        self.obligation_dispatch = obligation_dispatch;
        self
    }
}
