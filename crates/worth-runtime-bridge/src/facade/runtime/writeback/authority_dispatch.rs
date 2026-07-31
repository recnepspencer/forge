use super::authority_execution_recording::{
    blocked_before_authority_record, rejected_receipt_record, request_dispatch_failure_record,
    validated_receipt_failure_record, WritebackAuthorityAttempt,
};
use super::authority_failure_mapping::{
    map_writeback_failure_class, panic_content_message, validate_writeback_receipt_contract,
};
use super::*;

impl RuntimeBridge {
    pub(super) fn dispatch_writeback_authority(
        &self,
        attempt: &WritebackAuthorityAttempt<'_>,
    ) -> Result<TruthWritebackReceipt, BridgeWritebackError> {
        let authority = self.writeback_authority.as_ref().ok_or_else(|| {
            let error = BridgeWritebackError::new(
                BridgeWritebackErrorKind::AuthorityDenied,
                "runtime has no truth writeback authority bound",
            );
            self.diagnostics
                .record_writeback_execution(blocked_before_authority_record(
                    attempt.execution(),
                    attempt.mapper_record(),
                    attempt.candidate(),
                    &error,
                ));
            error
        })?;
        let receipt = self.execute_writeback_authority_request(authority.as_ref(), attempt)?;
        self.admit_writeback_authority_receipt(attempt, receipt)
    }

    fn execute_writeback_authority_request(
        &self,
        authority: &dyn TruthWritebackAuthority,
        attempt: &WritebackAuthorityAttempt<'_>,
    ) -> Result<TruthWritebackReceipt, BridgeWritebackError> {
        let result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            authority.execute_writeback(attempt.request().clone())
        })) {
            Ok(result) => result.map_err(|transport_error| {
                BridgeWritebackError::new(
                    BridgeWritebackErrorKind::StrategyFailed,
                    format!("truth writeback authority failed: {transport_error}"),
                )
            }),
            Err(panic_content) => Err(BridgeWritebackError::new(
                BridgeWritebackErrorKind::StrategyPanicked,
                format!(
                    "truth writeback authority panicked: {}",
                    panic_content_message(panic_content)
                ),
            )),
        };
        result.inspect_err(|error| {
            self.record_writeback_dispatch_failure(attempt, error);
        })
    }

    fn admit_writeback_authority_receipt(
        &self,
        attempt: &WritebackAuthorityAttempt<'_>,
        receipt: TruthWritebackReceipt,
    ) -> Result<TruthWritebackReceipt, BridgeWritebackError> {
        if let Err(error) = validate_writeback_receipt_contract(attempt.request(), &receipt) {
            self.diagnostics
                .record_writeback_execution(validated_receipt_failure_record(
                    attempt, &receipt, &error,
                ));
            return Err(error);
        }
        if receipt.outcome_class() != BridgeWritebackOutcomeClass::Rejected {
            return Ok(receipt);
        }
        let failure_class = receipt
            .failure_class()
            .expect("rejected receipts must carry a failure class after validation");
        let error = BridgeWritebackError::new(
            map_writeback_failure_class(failure_class),
            format!(
                "truth writeback authority rejected request `{}` with failure `{failure_class:?}`",
                receipt.request_digest()
            ),
        );
        self.diagnostics
            .record_writeback_execution(rejected_receipt_record(
                attempt,
                &receipt,
                failure_class,
                &error,
            ));
        Err(error)
    }

    fn record_writeback_dispatch_failure(
        &self,
        attempt: &WritebackAuthorityAttempt<'_>,
        error: &BridgeWritebackError,
    ) {
        self.diagnostics
            .record_writeback_execution(request_dispatch_failure_record(attempt, error));
    }
}
