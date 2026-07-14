use super::authority_execution_recording::{
    blocked_before_authority_record, blocked_before_candidate_record, canonical_noop_record,
    rejected_receipt_record, request_dispatch_failure_record, successful_authority_record,
    validated_receipt_failure_record, WritebackAuthorityExecutionContext,
};
use super::authority_failure_mapping::{
    map_writeback_failure_class, panic_content_message, validate_writeback_receipt_contract,
};
use super::*;

impl RuntimeBridge {
    /// Executes the full admitted writeback workflow from one bridge-owned request.
    ///
    /// Ordinary hosts should prefer this contract over manually choreographing
    /// policy admission, writeback admission, effect lowering, idempotence
    /// classification, and authority execution across separate calls.
    pub fn execute_admitted_writeback(
        &self,
        request: BridgeAdmittedWritebackExecutionRequest,
    ) -> Result<BridgeAdmittedWritebackExecution, BridgeAdmittedWritebackExecutionError> {
        let policy_contract = self
            .admit_policy_declaration(request.policy_declaration().clone())
            .map_err(BridgeAdmittedWritebackExecutionError::policy_admission)?;
        let lowered_policy = self.lower_admitted_policy(&policy_contract);
        let contract = self
            .admit_writeback_declaration(request.writeback_declaration().clone(), &lowered_policy)
            .map_err(BridgeAdmittedWritebackExecutionError::writeback)?;
        let effect = self.lower_writeback_effect(
            &contract,
            request.causality(),
            request.effect_identity().clone(),
            request.effect_intent().clone(),
        );
        let idempotence = self.classify_writeback_idempotence(
            &effect,
            &lowered_policy,
            request.authoritative_state_basis(),
            request.idempotence_identity().clone(),
            request.idempotence_class(),
        );
        let (outcome, authority_receipt) = self
            .execute_writeback_authority(&contract, &effect, &idempotence)
            .map_err(BridgeAdmittedWritebackExecutionError::writeback)?;
        let replay_bundle =
            self.replay_writeback_bundle(&contract, &effect, &idempotence, &outcome);
        let execution_receipt = BridgeAdmittedWritebackExecutionReceipt::new(
            &request,
            &contract,
            &effect,
            &idempotence,
            &outcome,
            &authority_receipt,
            &replay_bundle,
        );
        self.diagnostics
            .annotate_last_writeback_execution_record(execution_receipt.digest().to_owned());

        Ok(BridgeAdmittedWritebackExecution::new(
            outcome,
            execution_receipt,
        ))
    }

    /// Executes writeback authority without supplying upstream feedback context.
    pub fn execute_writeback_authority(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
    ) -> Result<(BridgeWritebackAuthorityOutcome, TruthWritebackReceipt), BridgeWritebackError>
    {
        let (_, outcome, receipt) = self.execute_writeback_authority_with_feedback_context(
            contract,
            effect,
            idempotence,
            None,
        )?;
        let receipt = receipt
            .expect("authority receipt must exist when feedback loop prevention allowed execution");
        Ok((outcome, receipt))
    }

    /// Executes the full writeback authority workflow with explicit feedback context.
    ///
    /// This is the highest-value specialist entrypoint for tests or hosts that
    /// need bridge-native writeback proof, loop prevention, and receipt capture.
    pub fn execute_writeback_authority_with_feedback_context(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        incoming_feedback_context: Option<&BridgeWritebackFeedbackContext>,
    ) -> Result<
        (
            BridgeWritebackLoopPreventionReport,
            BridgeWritebackAuthorityOutcome,
            Option<TruthWritebackReceipt>,
        ),
        BridgeWritebackError,
    > {
        let loop_prevention =
            self.classify_writeback_loop_prevention(effect, idempotence, incoming_feedback_context);
        match loop_prevention.disposition() {
            BridgeWritebackLoopDisposition::CanonicalNoop => {
                let outcome = BridgeWritebackAuthorityOutcome::canonical_noop(idempotence);
                let strategy_coherence =
                    self.classify_writeback_strategy_coherence(contract, effect, idempotence);
                let replay_bundle =
                    self.replay_writeback_bundle(contract, effect, idempotence, &outcome);
                let execution_context = WritebackAuthorityExecutionContext::new(
                    contract,
                    effect,
                    idempotence,
                    &loop_prevention,
                    &strategy_coherence,
                );
                let execution_record =
                    canonical_noop_record(&execution_context, &outcome, &replay_bundle);
                self.diagnostics
                    .record_writeback_execution(execution_record);
                return Ok((loop_prevention, outcome, None));
            }
            BridgeWritebackLoopDisposition::RejectAsUnsafeFeedback => {
                let error = BridgeWritebackError::new(
                    BridgeWritebackErrorKind::InvariantRejected,
                    format!(
                        "unsafe bridge feedback suppressed before authority execution: {}",
                        loop_prevention.digest()
                    ),
                );
                let strategy_coherence =
                    self.classify_writeback_strategy_coherence(contract, effect, idempotence);
                let execution_context = WritebackAuthorityExecutionContext::new(
                    contract,
                    effect,
                    idempotence,
                    &loop_prevention,
                    &strategy_coherence,
                );
                let execution_record = blocked_before_candidate_record(&execution_context, &error);
                self.diagnostics
                    .record_writeback_execution(execution_record);
                return Err(error);
            }
            BridgeWritebackLoopDisposition::AllowAuthoritativeAttempt => {}
        }

        let strategy_coherence =
            self.classify_writeback_strategy_coherence(contract, effect, idempotence);
        let execution_context = WritebackAuthorityExecutionContext::new(
            contract,
            effect,
            idempotence,
            &loop_prevention,
            &strategy_coherence,
        );
        let candidate = match self.validate_writeback_candidate(
            contract,
            effect,
            idempotence,
            &loop_prevention,
            &strategy_coherence,
        ) {
            Ok(candidate) => candidate,
            Err(error) => {
                let execution_record = blocked_before_candidate_record(&execution_context, &error);
                self.diagnostics
                    .record_writeback_execution(execution_record);
                return Err(error);
            }
        };
        let mapper_witness = BridgeWritebackMapperWitness::issue_from_effect(effect);
        let mapper_record = BridgeWritebackMapperRecord::new(&mapper_witness, &candidate);
        self.diagnostics
            .record_writeback_mapper(mapper_record.clone());

        let authority = match self.writeback_authority.as_ref() {
            Some(authority) => authority,
            None => {
                let error = BridgeWritebackError::new(
                    BridgeWritebackErrorKind::AuthorityDenied,
                    "runtime has no truth writeback authority bound",
                );
                let execution_record = blocked_before_authority_record(
                    &execution_context,
                    &mapper_record,
                    &candidate,
                    &error,
                );
                self.diagnostics
                    .record_writeback_execution(execution_record);
                return Err(error);
            }
        };
        let feedback_provenance = self.derive_writeback_feedback_provenance(effect);
        let request =
            TruthWritebackRequest::from_evidence(crate::adapter::TruthWritebackRequestEvidence {
                contract,
                candidate: &candidate,
                effect,
                mapper_witness: &mapper_witness,
                feedback_provenance: &feedback_provenance,
                loop_prevention: &loop_prevention,
                strategy_coherence: &strategy_coherence,
                idempotence,
            });
        let request_for_validation = request.clone();
        let authority_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            authority.execute_writeback(request)
        })) {
            Ok(result) => result,
            Err(panic_content) => {
                let error = BridgeWritebackError::new(
                    BridgeWritebackErrorKind::StrategyPanicked,
                    format!(
                        "truth writeback authority panicked: {}",
                        panic_content_message(panic_content)
                    ),
                );
                let execution_record = request_dispatch_failure_record(
                    &execution_context,
                    &mapper_record,
                    &candidate,
                    &request_for_validation,
                    &error,
                );
                self.diagnostics
                    .record_writeback_execution(execution_record);
                return Err(error);
            }
        };
        let receipt = match authority_result {
            Ok(receipt) => receipt,
            Err(transport_error) => {
                let error = BridgeWritebackError::new(
                    BridgeWritebackErrorKind::StrategyFailed,
                    format!("truth writeback authority failed: {transport_error}"),
                );
                let execution_record = request_dispatch_failure_record(
                    &execution_context,
                    &mapper_record,
                    &candidate,
                    &request_for_validation,
                    &error,
                );
                self.diagnostics
                    .record_writeback_execution(execution_record);
                return Err(error);
            }
        };
        if let Err(error) = validate_writeback_receipt_contract(&request_for_validation, &receipt) {
            let execution_record = validated_receipt_failure_record(
                &execution_context,
                &mapper_record,
                &candidate,
                &request_for_validation,
                &receipt,
                &error,
            );
            self.diagnostics
                .record_writeback_execution(execution_record);
            return Err(error);
        }
        if receipt.outcome_class() == BridgeWritebackOutcomeClass::Rejected {
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
            let execution_record = rejected_receipt_record(
                &execution_context,
                &mapper_record,
                &candidate,
                &request_for_validation,
                &receipt,
                failure_class,
                &error,
            );
            self.diagnostics
                .record_writeback_execution(execution_record);
            return Err(error);
        }
        let outcome = match receipt.outcome_class() {
            BridgeWritebackOutcomeClass::CanonicalNoop => {
                BridgeWritebackAuthorityOutcome::canonical_noop(idempotence)
            }
            BridgeWritebackOutcomeClass::AuthoritativeCommit => {
                BridgeWritebackAuthorityOutcome::authoritative_commit(idempotence, &receipt)
            }
            BridgeWritebackOutcomeClass::Rejected => unreachable!(
                "rejected authority receipts are converted into typed bridge errors before outcome lowering"
            ),
        };
        let replay_bundle = self.replay_writeback_bundle(contract, effect, idempotence, &outcome);
        let execution_record = successful_authority_record(
            &execution_context,
            &mapper_record,
            &candidate,
            &outcome,
            &replay_bundle,
            &request_for_validation,
            &receipt,
        );
        self.diagnostics
            .record_writeback_execution(execution_record);

        Ok((loop_prevention, outcome, Some(receipt)))
    }
}
