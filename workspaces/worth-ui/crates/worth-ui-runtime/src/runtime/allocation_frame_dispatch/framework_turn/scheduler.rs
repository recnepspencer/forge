use std::cell::RefCell;
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

pub(in crate::runtime) struct UiPreparedFrameReplacementCommit {
    mailbox: Rc<RefCell<UiAllocationFrameIngressMailbox>>,
    expected_state: UiAllocationFrameDispatcherState,
    assignment: UiAllocationFrameEpochAssignment,
    transition: Option<UiAllocationFrameReplacementTransition>,
    successor: UiAllocationFrameIngressMailbox,
}

impl UiPreparedFrameReplacementCommit {
    pub(in crate::runtime) fn commit_once(self) {
        debug_assert!(self.transition.is_none());
        let mut mailbox = self.mailbox.borrow_mut();
        assert_eq!(
            mailbox.dispatcher.state(),
            self.expected_state,
            "reserved frame-scheduler predecessor changed before total publication"
        );
        *mailbox = self.successor;
    }

    pub(in crate::runtime) fn assignment(&self) -> UiAllocationFrameEpochAssignment {
        self.assignment
    }

    pub(in crate::runtime) fn successor_state(&self) -> UiAllocationFrameDispatcherState {
        self.successor.dispatcher.state()
    }

    pub(in crate::runtime) fn take_transition_for_receipt(
        &mut self,
    ) -> UiAllocationFrameReplacementTransition {
        self.transition
            .take()
            .expect("prepared frame replacement owns one unbound transition")
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
    ) -> Result<UiPreparedFrameReplacementCommit, UiAllocationFrameDispatchDenial> {
        let mailbox = self.mailbox.borrow();
        let expected_state = mailbox.dispatcher.state();
        let (assignment, transition, successor_dispatcher) =
            mailbox.dispatcher.prepare_replacement_successor()?;
        Ok(UiPreparedFrameReplacementCommit {
            mailbox: Rc::clone(&self.mailbox),
            expected_state,
            assignment,
            transition: Some(transition),
            successor: UiAllocationFrameIngressMailbox {
                dispatcher: successor_dispatcher,
                gateways: UiAllocationFrameGatewayState::launch(),
            },
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
