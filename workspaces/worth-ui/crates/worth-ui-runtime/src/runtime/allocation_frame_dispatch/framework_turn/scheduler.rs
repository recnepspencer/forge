use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

use crate::runtime::WorthUiRuntimeFrameEpoch;

use super::super::{
    UiAllocationFrameDispatchDenial, UiAllocationFrameDispatcher,
    UiAllocationFrameDispatcherCounters, UiAllocationFrameDispatcherState,
    UiAllocationFrameEpochAssignment, UiAllocationFrameGatewayOutcome,
    UiAllocationFrameGatewayState, UiAllocationFrameQueueDisposition,
    UiAllocationFrameReplacementTransition, UiAllocationFrameSourceFact,
    UiAllocationFrameSourceLane, UiAllocationFrameTransitionOutcome,
};
use super::{UiAllocationFrameDispatchAuthority, UiAllocationFrameTurnOutcome};

#[derive(Debug)]
pub(in crate::runtime::allocation_frame_dispatch) struct UiAllocationFrameIngressMailbox {
    dispatcher: UiAllocationFrameDispatcher,
    gateways: UiAllocationFrameGatewayState,
}

#[derive(Clone, Debug)]
pub(crate) struct UiAllocationFrameFrameworkScheduler {
    mailbox: Rc<RefCell<UiAllocationFrameIngressMailbox>>,
}

pub(in crate::runtime) struct UiPreparedFrameReplacementCommit<'scheduler> {
    mailbox: RefMut<'scheduler, UiAllocationFrameIngressMailbox>,
    assignment: UiAllocationFrameEpochAssignment,
}

impl UiPreparedFrameReplacementCommit<'_> {
    pub(in crate::runtime) fn commit_once(mut self) -> UiAllocationFrameReplacementTransition {
        let transition = self.mailbox.dispatcher.pause_for_replacement();
        self.mailbox
            .dispatcher
            .install_replacement_successor(&transition);
        self.mailbox.gateways = UiAllocationFrameGatewayState::launch();
        transition
    }

    pub(in crate::runtime) fn assignment(&self) -> UiAllocationFrameEpochAssignment {
        self.assignment
    }
}

impl UiAllocationFrameFrameworkScheduler {
    pub(crate) fn launch(epoch: WorthUiRuntimeFrameEpoch) -> Self {
        let dispatcher = UiAllocationFrameDispatcher::launch(epoch);
        let gateways = UiAllocationFrameGatewayState::launch();
        Self {
            mailbox: Rc::new(RefCell::new(UiAllocationFrameIngressMailbox {
                dispatcher,
                gateways,
            })),
        }
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn mailbox(
        &self,
    ) -> Rc<RefCell<UiAllocationFrameIngressMailbox>> {
        Rc::clone(&self.mailbox)
    }

    pub(super) fn run_turn(&self) -> UiAllocationFrameTurnOutcome {
        let mut mailbox = self.mailbox.borrow_mut();
        let transition = mailbox
            .dispatcher
            .dispatch(UiAllocationFrameDispatchAuthority::issue());
        turn_outcome(transition, mailbox.dispatcher.counters())
    }

    pub(crate) fn state(&self) -> UiAllocationFrameDispatcherState {
        self.mailbox.borrow().dispatcher.state()
    }

    pub(crate) fn counters(&self) -> UiAllocationFrameDispatcherCounters {
        self.mailbox.borrow().dispatcher.counters()
    }

    pub(in crate::runtime) fn prepare_replacement_commit(
        &self,
    ) -> Result<UiPreparedFrameReplacementCommit<'_>, UiAllocationFrameDispatchDenial> {
        let mailbox = self.mailbox.borrow_mut();
        let assignment = mailbox
            .dispatcher
            .prepare_replacement_assignment()
            .ok_or(UiAllocationFrameDispatchDenial::EpochExhausted)?;
        Ok(UiPreparedFrameReplacementCommit {
            mailbox,
            assignment,
        })
    }

    pub(crate) fn shutdown(&self) -> UiAllocationFrameQueueDisposition {
        self.mailbox.borrow_mut().dispatcher.shutdown()
    }
}

impl UiAllocationFrameIngressMailbox {
    pub(in crate::runtime::allocation_frame_dispatch) fn submit_host(
        &mut self,
        source_identity: u64,
        generation: u64,
        ingress_identity: u64,
        order: u64,
        fact: UiAllocationFrameSourceFact,
    ) -> UiAllocationFrameGatewayOutcome {
        let attempt = super::super::gateway::submit_admitted_source_fact(
            &mut self.dispatcher,
            &mut self.gateways,
            super::super::gateway::UiAllocationFrameSourceSubmission {
                lane: UiAllocationFrameSourceLane::HostMeasurement,
                source_identity: source_identity.into(),
                source_generation: generation,
                ingress_identity,
                source_order: order,
                fact,
            },
        );
        self.finish_submission(attempt)
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn submit_query(
        &mut self,
        source_identity: super::super::UiAllocationFrameSourceIdentity,
        generation: u64,
        ingress_identity: u64,
        order: u64,
        fact: UiAllocationFrameSourceFact,
    ) -> UiAllocationFrameGatewayOutcome {
        let attempt = super::super::gateway::submit_admitted_source_fact(
            &mut self.dispatcher,
            &mut self.gateways,
            super::super::gateway::UiAllocationFrameSourceSubmission {
                lane: UiAllocationFrameSourceLane::QueryProjection,
                source_identity,
                source_generation: generation,
                ingress_identity,
                source_order: order,
                fact,
            },
        );
        self.finish_submission(attempt)
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn submit_interaction(
        &mut self,
        source_identity: u64,
        generation: u64,
        ingress_identity: u64,
        order: u64,
        fact: UiAllocationFrameSourceFact,
    ) -> UiAllocationFrameGatewayOutcome {
        let attempt = super::super::gateway::submit_admitted_source_fact(
            &mut self.dispatcher,
            &mut self.gateways,
            super::super::gateway::UiAllocationFrameSourceSubmission {
                lane: UiAllocationFrameSourceLane::Interaction,
                source_identity: source_identity.into(),
                source_generation: generation,
                ingress_identity,
                source_order: order,
                fact,
            },
        );
        self.finish_submission(attempt)
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn submit_durable_resize(
        &mut self,
        source_identity: u64,
        generation: u64,
        ingress_identity: u64,
        order: u64,
        fact: UiAllocationFrameSourceFact,
    ) -> UiAllocationFrameGatewayOutcome {
        let attempt = super::super::gateway::submit_admitted_source_fact(
            &mut self.dispatcher,
            &mut self.gateways,
            super::super::gateway::UiAllocationFrameSourceSubmission {
                lane: UiAllocationFrameSourceLane::DurableState,
                source_identity: source_identity.into(),
                source_generation: generation,
                ingress_identity,
                source_order: order,
                fact,
            },
        );
        self.finish_submission(attempt)
    }

    fn finish_submission(
        &mut self,
        attempt: super::super::gateway::UiAllocationFrameAdmissionAttempt,
    ) -> UiAllocationFrameGatewayOutcome {
        let attempt = attempt.into_parts();
        let super::super::gateway::UiAllocationFrameAdmissionAttempt::Submitted {
            transition,
            descriptor,
        } = attempt
        else {
            let super::super::gateway::UiAllocationFrameAdmissionAttempt::SourceAdmissionDenied {
                denial,
                source_fact,
            } = attempt
            else {
                unreachable!()
            };
            return UiAllocationFrameGatewayOutcome::source_admission_denied(
                denial,
                *source_fact,
                self.dispatcher.counters(),
            );
        };
        let (outcome, _, rejected_ingress) = transition.into_parts();
        UiAllocationFrameGatewayOutcome::attempted(
            outcome,
            descriptor,
            rejected_ingress.map(|ingress| ingress.into_source_fact()),
        )
    }
}

fn turn_outcome(
    outcome: UiAllocationFrameTransitionOutcome,
    counters: UiAllocationFrameDispatcherCounters,
) -> UiAllocationFrameTurnOutcome {
    match outcome.into_dispatched_frame() {
        Ok(sealed_frame) => {
            let frame_epoch_assignment = sealed_frame.frame_epoch_assignment();
            UiAllocationFrameTurnOutcome::SealedFrameReady {
                sealed_frame: Box::new(sealed_frame),
                frame_epoch_assignment,
            }
        }
        Err(UiAllocationFrameDispatchDenial::EmptyFrame) => {
            UiAllocationFrameTurnOutcome::NoAdmittedIngress { counters }
        }
        Err(denial) => UiAllocationFrameTurnOutcome::Denied { denial, counters },
    }
}
