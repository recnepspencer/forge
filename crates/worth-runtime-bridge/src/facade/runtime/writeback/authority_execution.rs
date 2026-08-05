use super::authority_execution_artifacts::BridgeWritebackAuthorityExecutionArtifacts;
use super::authority_execution_recording::{
    successful_authority_record, WritebackAuthorityAttempt, WritebackAuthorityExecutionContext,
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
        self.execute_writeback_authority_artifacts_with_feedback_context(
            contract,
            effect,
            idempotence,
            incoming_feedback_context,
        )
        .map(BridgeWritebackAuthorityExecutionArtifacts::into_public_result)
    }

    pub(super) fn execute_writeback_authority_artifacts_with_feedback_context(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        incoming_feedback_context: Option<&BridgeWritebackFeedbackContext>,
    ) -> Result<BridgeWritebackAuthorityExecutionArtifacts, BridgeWritebackError> {
        if !effect.mutation_subject_effect_intent_match() {
            return Err(BridgeWritebackError::new(
                BridgeWritebackErrorKind::CausalityEffectMismatch,
                "bridge mutation subject does not match the writeback effect intent",
            ));
        }
        let loop_prevention =
            self.classify_writeback_loop_prevention(effect, idempotence, incoming_feedback_context);
        if let Some(terminal) = self.resolve_terminal_writeback_loop_disposition(
            contract,
            effect,
            idempotence,
            loop_prevention.clone(),
        ) {
            return terminal;
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
        let prepared = self.prepare_writeback_authority_candidate(&execution_context)?;
        let feedback_provenance = self.derive_writeback_feedback_provenance(effect);
        let request =
            TruthWritebackRequest::from_evidence(crate::adapter::TruthWritebackRequestEvidence {
                contract,
                candidate: prepared.candidate(),
                effect,
                mapper_witness: prepared.mapper_witness(),
                feedback_provenance: &feedback_provenance,
                loop_prevention: &loop_prevention,
                strategy_coherence: &strategy_coherence,
                idempotence,
            });
        let attempt = WritebackAuthorityAttempt::new(&execution_context, &prepared, &request);
        let receipt = self.dispatch_writeback_authority(&attempt)?;
        Ok(self.complete_writeback_authority_execution(&attempt, receipt))
    }

    fn complete_writeback_authority_execution(
        &self,
        attempt: &WritebackAuthorityAttempt<'_>,
        receipt: TruthWritebackReceipt,
    ) -> BridgeWritebackAuthorityExecutionArtifacts {
        let outcome = match receipt.outcome_class() {
            BridgeWritebackOutcomeClass::CanonicalNoop => {
                BridgeWritebackAuthorityOutcome::canonical_noop(attempt.execution().idempotence())
            }
            BridgeWritebackOutcomeClass::AuthoritativeCommit => {
                BridgeWritebackAuthorityOutcome::authoritative_commit(
                    attempt.execution().idempotence(),
                    &receipt,
                )
            }
            BridgeWritebackOutcomeClass::Rejected => unreachable!(
                "rejected authority receipts are converted into typed bridge errors before outcome lowering"
            ),
        };
        let replay_bundle = self.replay_writeback_bundle(
            attempt.execution().contract(),
            attempt.execution().effect(),
            attempt.execution().idempotence(),
            &outcome,
        );
        let execution_record =
            successful_authority_record(attempt, &outcome, &replay_bundle, &receipt);
        self.diagnostics
            .record_writeback_execution(execution_record.clone());

        BridgeWritebackAuthorityExecutionArtifacts::new(
            attempt.execution().loop_prevention().clone(),
            outcome,
            Some(receipt),
            execution_record,
        )
    }
}
