use crate::facade::{
    BridgeWritebackFeedbackContext, BridgeWritebackLoopDisposition,
    BridgeWritebackLoopPreventionReport,
};

pub(in crate::harness::adapter::adapter_impl) struct FamilyExtensionLoopIsolation {
    feedback_context: BridgeWritebackFeedbackContext,
    loop_prevention: BridgeWritebackLoopPreventionReport,
}

impl FamilyExtensionLoopIsolation {
    pub(in crate::harness::adapter::adapter_impl::writeback_certification::family_extension) fn from_loop_prevention(
        feedback_context: &BridgeWritebackFeedbackContext,
        loop_prevention: &BridgeWritebackLoopPreventionReport,
    ) -> Self {
        Self {
            feedback_context: feedback_context.clone(),
            loop_prevention: loop_prevention.clone(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn feedback_context(
        &self,
    ) -> &BridgeWritebackFeedbackContext {
        &self.feedback_context
    }

    pub(in crate::harness::adapter::adapter_impl) fn loop_prevention(
        &self,
    ) -> &BridgeWritebackLoopPreventionReport {
        &self.loop_prevention
    }

    pub(in crate::harness::adapter::adapter_impl) fn incoming_feedback_provenance_digest(
        &self,
    ) -> &str {
        self.feedback_context.provenance_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn incoming_feedback_causality_digest(
        &self,
    ) -> &str {
        self.feedback_context.causality_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn disposition(
        &self,
    ) -> BridgeWritebackLoopDisposition {
        self.loop_prevention.disposition()
    }

    pub(in crate::harness::adapter::adapter_impl) fn digest(&self) -> &str {
        self.loop_prevention.digest()
    }
}
