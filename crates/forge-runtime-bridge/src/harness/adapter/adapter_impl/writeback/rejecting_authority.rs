use super::*;

#[derive(Debug, Clone)]
pub(in crate::harness::adapter::adapter_impl::writeback) struct RejectingTruthWritebackAuthority {
    failure_class: crate::facade::BridgeWritebackFailureClass,
    last_rejected_attempt: Arc<RwLock<Option<RejectedTruthWritebackAttempt>>>,
}

#[derive(Debug, Clone)]
struct RejectedTruthWritebackAttempt {
    request: crate::adapter::TruthWritebackRequest,
    receipt: crate::adapter::TruthWritebackReceipt,
}

impl RejectedTruthWritebackAttempt {
    fn from_request_and_receipt(
        request: &crate::adapter::TruthWritebackRequest,
        receipt: &crate::adapter::TruthWritebackReceipt,
    ) -> Self {
        Self {
            request: request.clone(),
            receipt: receipt.clone(),
        }
    }
}

impl RejectingTruthWritebackAuthority {
    pub(in crate::harness::adapter::adapter_impl::writeback) fn new(
        failure_class: crate::facade::BridgeWritebackFailureClass,
    ) -> Self {
        Self {
            failure_class,
            last_rejected_attempt: Arc::new(RwLock::new(None)),
        }
    }

    pub(in crate::harness::adapter::adapter_impl::writeback) fn last_request(
        &self,
    ) -> Option<crate::adapter::TruthWritebackRequest> {
        self.last_rejected_attempt
            .read()
            .expect("rejecting writeback authority lock poisoned")
            .as_ref()
            .map(|attempt| attempt.request.clone())
    }

    pub(in crate::harness::adapter::adapter_impl::writeback) fn last_receipt(
        &self,
    ) -> Option<crate::adapter::TruthWritebackReceipt> {
        self.last_rejected_attempt
            .read()
            .expect("rejecting writeback authority lock poisoned")
            .as_ref()
            .map(|attempt| attempt.receipt.clone())
    }
}

impl crate::adapter::TruthWritebackAuthority for RejectingTruthWritebackAuthority {
    fn execute_writeback(
        &self,
        request: crate::adapter::TruthWritebackRequest,
    ) -> Result<crate::adapter::TruthWritebackReceipt, crate::adapter::TruthWritebackAuthorityError>
    {
        let receipt = crate::adapter::TruthWritebackReceipt::new_with_failure_class(
            crate::facade::BridgeWritebackOutcomeClass::Rejected,
            Some(self.failure_class),
            &request,
        );
        let rejected_attempt =
            RejectedTruthWritebackAttempt::from_request_and_receipt(&request, &receipt);
        *self
            .last_rejected_attempt
            .write()
            .expect("rejecting writeback authority lock poisoned") = Some(rejected_attempt);
        Ok(receipt)
    }
}
