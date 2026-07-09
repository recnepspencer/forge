use crate::facade::{BridgeWritebackLoopDisposition, BridgeWritebackLoopPreventionReport};

pub(in crate::harness::adapter::adapter_impl) struct AuthorityDenialLoopPreventionEvidence {
    report: BridgeWritebackLoopPreventionReport,
}

impl AuthorityDenialLoopPreventionEvidence {
    pub(super) fn from_loop_prevention(
        loop_prevention: &BridgeWritebackLoopPreventionReport,
    ) -> Self {
        Self {
            report: loop_prevention.clone(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn report(
        &self,
    ) -> &BridgeWritebackLoopPreventionReport {
        &self.report
    }

    pub(in crate::harness::adapter::adapter_impl) fn digest(&self) -> &str {
        self.report.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn disposition(
        &self,
    ) -> BridgeWritebackLoopDisposition {
        self.report.disposition()
    }

    pub(in crate::harness::adapter::adapter_impl) fn current_feedback_provenance_digest(
        &self,
    ) -> &str {
        self.report.current_feedback_provenance_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn current_causality_digest(&self) -> &str {
        self.report.current_causality_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn incoming_feedback_provenance_digest(
        &self,
    ) -> Option<&str> {
        self.report.incoming_feedback_provenance_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn incoming_feedback_causality_digest(
        &self,
    ) -> Option<&str> {
        self.report.incoming_feedback_causality_digest()
    }
}
