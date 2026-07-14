use super::super::{UiAllocationFrameIngressDescriptor, UiAllocationFrameSubmissionOutcome};
use crate::evidence::UiAllocationSourceGatewayEvidence;

#[derive(Debug, PartialEq)]
pub struct UiAllocationFrameGatewayOutcome {
    submission: Option<UiAllocationFrameSubmissionOutcome>,
    evidence: Option<UiAllocationSourceGatewayEvidence>,
    source_admission_denial: Option<super::super::UiAllocationFrameSourceAdmissionDenial>,
    counters: super::super::UiAllocationFrameDispatcherCounters,
    retry_source_fact: Option<super::UiAllocationFrameSourceFact>,
}

impl UiAllocationFrameGatewayOutcome {
    pub(in crate::runtime::allocation_frame_dispatch) fn attempted(
        submission: UiAllocationFrameSubmissionOutcome,
        ingress: UiAllocationFrameIngressDescriptor,
        retry_source_fact: Option<super::UiAllocationFrameSourceFact>,
    ) -> Self {
        let evidence = UiAllocationSourceGatewayEvidence::new(ingress, submission.counters());
        let counters = submission.counters();
        Self {
            submission: Some(submission),
            evidence: Some(evidence),
            source_admission_denial: None,
            counters,
            retry_source_fact,
        }
    }

    pub fn submission(&self) -> Option<&UiAllocationFrameSubmissionOutcome> {
        self.submission.as_ref()
    }

    pub fn counters(&self) -> super::super::UiAllocationFrameDispatcherCounters {
        self.counters
    }

    pub fn evidence(&self) -> Option<&UiAllocationSourceGatewayEvidence> {
        self.evidence.as_ref()
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn source_admission_denied(
        denial: super::super::UiAllocationFrameSourceAdmissionDenial,
        source_fact: super::UiAllocationFrameSourceFact,
        counters: super::super::UiAllocationFrameDispatcherCounters,
    ) -> Self {
        Self {
            submission: None,
            evidence: None,
            source_admission_denial: Some(denial),
            counters,
            retry_source_fact: Some(source_fact),
        }
    }

    pub fn source_admission_denial(
        &self,
    ) -> Option<super::super::UiAllocationFrameSourceAdmissionDenial> {
        self.source_admission_denial
    }

    pub fn retry_source_fact(&self) -> Option<&super::UiAllocationFrameSourceFact> {
        self.retry_source_fact.as_ref()
    }

    pub fn into_retry_source_fact(self) -> Option<super::UiAllocationFrameSourceFact> {
        self.retry_source_fact
    }
}
