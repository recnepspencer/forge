//! Scripted test implementation of the production provider fault port.

use std::sync::atomic::{AtomicU8, Ordering};

use super::super::provider::fault_port::{
    WorthQueryPrimaryGraphFault, WorthQueryPrimaryGraphFaultPort,
};

#[derive(Default)]
pub(in crate::domain_computation::primary_graph) struct PrimaryGraphFaultController {
    scheduled: AtomicU8,
}

impl PrimaryGraphFaultController {
    pub(in crate::domain_computation::primary_graph) fn schedule(
        &self,
        fault: WorthQueryPrimaryGraphFault,
    ) {
        self.scheduled.fetch_or(mask(fault), Ordering::AcqRel);
    }

    pub(in crate::domain_computation::primary_graph) fn lose_next_commit_response(&self) {
        self.schedule(WorthQueryPrimaryGraphFault::LostCommitResponse);
    }

    pub(in crate::domain_computation::primary_graph) fn reject_next_session_prepare(&self) {
        self.schedule(WorthQueryPrimaryGraphFault::RejectedSessionPreparation);
    }

    pub(in crate::domain_computation::primary_graph) fn reject_next_commit_before_transaction(
        &self,
    ) {
        self.schedule(WorthQueryPrimaryGraphFault::RejectedCommitBeforeTransaction);
    }

    pub(in crate::domain_computation::primary_graph) fn fail_next_index_publication(&self) {
        self.schedule(WorthQueryPrimaryGraphFault::FailedIndexPublication);
    }

    pub(in crate::domain_computation::primary_graph) fn skip_next_invariant_owner_execution(&self) {
        self.schedule(WorthQueryPrimaryGraphFault::SkippedInvariantOwnerExecution);
    }

    pub(in crate::domain_computation::primary_graph) fn violate_next_relational_invariant(&self) {
        self.schedule(WorthQueryPrimaryGraphFault::RelationalInvariantViolation);
    }

    pub(in crate::domain_computation::primary_graph) fn add_next_undeclared_application_touch(
        &self,
    ) {
        self.schedule(WorthQueryPrimaryGraphFault::UndeclaredApplicationTouch);
    }
}

impl WorthQueryPrimaryGraphFaultPort for PrimaryGraphFaultController {
    fn take(&self, fault: WorthQueryPrimaryGraphFault) -> bool {
        let mask = mask(fault);
        let mut scheduled = self.scheduled.load(Ordering::Acquire);
        loop {
            if scheduled & mask == 0 {
                return false;
            }
            match self.scheduled.compare_exchange_weak(
                scheduled,
                scheduled & !mask,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return true,
                Err(observed) => scheduled = observed,
            }
        }
    }
}

const fn mask(fault: WorthQueryPrimaryGraphFault) -> u8 {
    match fault {
        WorthQueryPrimaryGraphFault::LostCommitResponse => 1 << 0,
        WorthQueryPrimaryGraphFault::RejectedSessionPreparation => 1 << 1,
        WorthQueryPrimaryGraphFault::RejectedCommitBeforeTransaction => 1 << 2,
        WorthQueryPrimaryGraphFault::FailedIndexPublication => 1 << 3,
        WorthQueryPrimaryGraphFault::SkippedInvariantOwnerExecution => 1 << 4,
        WorthQueryPrimaryGraphFault::RelationalInvariantViolation => 1 << 5,
        WorthQueryPrimaryGraphFault::UndeclaredApplicationTouch => 1 << 6,
    }
}
