use super::super::{
    UiAllocationFrameDispatcher, UiAllocationFrameGatewayState, UiAllocationFrameSourceLane,
    UiAllocationFrameSubmissionTransition,
};
use super::UiAllocationFrameSourceFact;

pub(in crate::runtime::allocation_frame_dispatch) enum UiAllocationFrameAdmissionAttempt {
    Submitted {
        transition: UiAllocationFrameSubmissionTransition,
        descriptor: super::super::UiAllocationFrameIngressDescriptor,
    },
    SourceAdmissionDenied {
        denial: super::super::UiAllocationFrameSourceAdmissionDenial,
        source_fact: UiAllocationFrameSourceFact,
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
    lane: UiAllocationFrameSourceLane,
    source_identity: super::super::UiAllocationFrameSourceIdentity,
    source_generation: u64,
    ingress_identity: u64,
    source_order: u64,
    fact: UiAllocationFrameSourceFact,
) -> UiAllocationFrameAdmissionAttempt {
    let ingress = match gateways.admit(
        dispatcher,
        lane,
        source_identity,
        source_generation,
        ingress_identity,
        source_order,
        fact,
    ) {
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
        transition: dispatcher.submit(ingress),
        descriptor,
    }
}
