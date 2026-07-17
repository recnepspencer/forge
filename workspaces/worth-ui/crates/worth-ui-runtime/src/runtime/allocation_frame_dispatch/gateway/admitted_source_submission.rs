use super::super::{
    UiAllocationFrameDispatcher, UiAllocationFrameGatewayState, UiAllocationFrameSourceLane,
    UiAllocationFrameSubmissionTransition,
};
use super::UiAllocationFrameSourceFact;

pub(in crate::runtime::allocation_frame_dispatch) struct UiAllocationFrameSourceSubmission {
    pub lane: UiAllocationFrameSourceLane,
    pub source_identity: super::super::UiAllocationFrameSourceIdentity,
    pub source_generation: u64,
    pub ingress_identity: u64,
    pub source_order: u64,
    pub fact: UiAllocationFrameSourceFact,
}

pub(in crate::runtime::allocation_frame_dispatch) enum UiAllocationFrameAdmissionAttempt {
    Submitted {
        transition: Box<UiAllocationFrameSubmissionTransition>,
        descriptor: super::super::UiAllocationFrameIngressDescriptor,
    },
    SourceAdmissionDenied {
        denial: super::super::UiAllocationFrameSourceAdmissionDenial,
        source_fact: Box<UiAllocationFrameSourceFact>,
    },
}

impl UiAllocationFrameAdmissionAttempt {
    pub(in crate::runtime::allocation_frame_dispatch) fn into_parts(self) -> Self {
        self
    }
}

pub(in crate::runtime::allocation_frame_dispatch) fn submit_admitted_source_fact(
    dispatcher: &mut UiAllocationFrameDispatcher,
    gateways: &mut UiAllocationFrameGatewayState,
    submission: UiAllocationFrameSourceSubmission,
) -> UiAllocationFrameAdmissionAttempt {
    let ingress = match gateways.admit(dispatcher, submission) {
        Ok(ingress) => ingress,
        Err((denial, source_fact)) => {
            return UiAllocationFrameAdmissionAttempt::SourceAdmissionDenied {
                denial,
                source_fact,
            }
        }
    };
    let descriptor = ingress.descriptor();
    UiAllocationFrameAdmissionAttempt::Submitted {
        transition: Box::new(dispatcher.submit(ingress)),
        descriptor,
    }
}
