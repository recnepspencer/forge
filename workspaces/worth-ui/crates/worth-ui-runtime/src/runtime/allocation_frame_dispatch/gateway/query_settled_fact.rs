use std::cell::RefCell;
use std::rc::Rc;

use super::super::framework_turn::UiAllocationFrameIngressMailbox;
use super::{UiAllocationFrameGatewayOutcome, UiAllocationFrameSourceFact};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiQueryFrameIngressCounters {
    link_resolution_count: usize,
    retained_fact_resolution_count: usize,
    allocation_submission_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryFrameIngressDenial {
    StaleApplicationGeneration,
    PlanRowNotActive,
    PlanBindingMismatch,
    RetainedFact(worth_ui_query_binding::WorthUiQueryViewExecutionEvidenceDenial),
}

#[derive(Debug)]
pub struct WorthUiQueryFrameIngressOutcome {
    gateway: UiAllocationFrameGatewayOutcome,
    counters: WorthUiQueryFrameIngressCounters,
}

impl WorthUiQueryFrameIngressCounters {
    pub fn link_resolution_count(self) -> usize {
        self.link_resolution_count
    }

    pub fn retained_fact_resolution_count(self) -> usize {
        self.retained_fact_resolution_count
    }

    pub fn allocation_submission_count(self) -> usize {
        self.allocation_submission_count
    }

    pub(crate) fn record_link_resolution(&mut self) {
        self.link_resolution_count += 1;
    }

    pub(crate) fn record_retained_fact_resolution(&mut self) {
        self.retained_fact_resolution_count += 1;
    }

    pub(crate) fn record_allocation_submission(&mut self) {
        self.allocation_submission_count += 1;
    }
}

impl WorthUiQueryFrameIngressOutcome {
    pub(crate) fn new(
        gateway: UiAllocationFrameGatewayOutcome,
        counters: WorthUiQueryFrameIngressCounters,
    ) -> Self {
        Self { gateway, counters }
    }

    pub fn gateway(&self) -> &UiAllocationFrameGatewayOutcome {
        &self.gateway
    }

    pub fn counters(&self) -> WorthUiQueryFrameIngressCounters {
        self.counters
    }

    pub fn into_gateway(self) -> UiAllocationFrameGatewayOutcome {
        self.gateway
    }
}

pub(crate) struct WorthUiQuerySettledFactSubmission {
    mailbox: Rc<RefCell<UiAllocationFrameIngressMailbox>>,
}

impl WorthUiQuerySettledFactSubmission {
    pub(in crate::runtime::allocation_frame_dispatch) fn new(
        mailbox: Rc<RefCell<UiAllocationFrameIngressMailbox>>,
    ) -> Self {
        Self { mailbox }
    }

    pub(crate) fn submit(
        &mut self,
        plan_index: u32,
        view_binding_id: crate::capability::ViewBindingId,
        fact: worth_ui_query_binding::WorthUiSettledSnapshotFact,
    ) -> UiAllocationFrameGatewayOutcome {
        let source_generation = fact
            .source_generation()
            .expect("retained settled facts carry a source generation")
            .as_u64();
        let source_order = fact
            .source_order()
            .expect("retained settled facts carry a source order")
            .as_u64();
        self.mailbox.borrow_mut().submit_query(
            u64::from(plan_index).into(),
            source_generation,
            source_order,
            source_order,
            UiAllocationFrameSourceFact::QuerySettledFact {
                view_binding_id,
                fact: Box::new(fact),
            },
        )
    }
}
