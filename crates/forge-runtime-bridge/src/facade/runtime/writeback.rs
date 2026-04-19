use super::*;

fn map_writeback_failure_class(
    failure_class: BridgeWritebackFailureClass,
) -> BridgeWritebackErrorKind {
    match failure_class {
        BridgeWritebackFailureClass::WritebackNotRequested => {
            BridgeWritebackErrorKind::WritebackNotRequested
        }
        BridgeWritebackFailureClass::PolicyRejected => BridgeWritebackErrorKind::PolicyRejected,
        BridgeWritebackFailureClass::StrategyUnavailable => {
            BridgeWritebackErrorKind::StrategyUnavailable
        }
        BridgeWritebackFailureClass::FamilyBindingMismatch => {
            BridgeWritebackErrorKind::FamilyBindingMismatch
        }
        BridgeWritebackFailureClass::StrategyDescriptorMismatch => {
            BridgeWritebackErrorKind::StrategyDescriptorMismatch
        }
        BridgeWritebackFailureClass::IdempotenceBasisMismatch => {
            BridgeWritebackErrorKind::IdempotenceBasisMismatch
        }
        BridgeWritebackFailureClass::StaleTruthBasis => BridgeWritebackErrorKind::StaleTruthBasis,
        BridgeWritebackFailureClass::InvariantRejected => {
            BridgeWritebackErrorKind::InvariantRejected
        }
        BridgeWritebackFailureClass::MergeAuthorityRejected => {
            BridgeWritebackErrorKind::MergeAuthorityRejected
        }
        BridgeWritebackFailureClass::StrategyFailed => BridgeWritebackErrorKind::StrategyFailed,
        BridgeWritebackFailureClass::StrategyPanicked => BridgeWritebackErrorKind::StrategyPanicked,
        BridgeWritebackFailureClass::ReplayMismatch => BridgeWritebackErrorKind::ReplayMismatch,
        BridgeWritebackFailureClass::AuthorityBypassRejected => {
            BridgeWritebackErrorKind::AuthorityBypassRejected
        }
        BridgeWritebackFailureClass::PreviewWritebackRejected => {
            BridgeWritebackErrorKind::PreviewWritebackRejected
        }
    }
}

fn panic_payload_message(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_string()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "non-string panic payload".to_string()
    }
}

fn map_writeback_error_kind_to_failure_class(
    error_kind: BridgeWritebackErrorKind,
) -> BridgeWritebackFailureClass {
    match error_kind {
        BridgeWritebackErrorKind::WritebackNotRequested => {
            BridgeWritebackFailureClass::WritebackNotRequested
        }
        BridgeWritebackErrorKind::PolicyRejected => BridgeWritebackFailureClass::PolicyRejected,
        BridgeWritebackErrorKind::StrategyUnavailable => {
            BridgeWritebackFailureClass::StrategyUnavailable
        }
        BridgeWritebackErrorKind::FamilyBindingMismatch => {
            BridgeWritebackFailureClass::FamilyBindingMismatch
        }
        BridgeWritebackErrorKind::StrategyDescriptorMismatch => {
            BridgeWritebackFailureClass::StrategyDescriptorMismatch
        }
        BridgeWritebackErrorKind::IdempotenceBasisMismatch => {
            BridgeWritebackFailureClass::IdempotenceBasisMismatch
        }
        BridgeWritebackErrorKind::StaleTruthBasis => BridgeWritebackFailureClass::StaleTruthBasis,
        BridgeWritebackErrorKind::InvariantRejected => {
            BridgeWritebackFailureClass::InvariantRejected
        }
        BridgeWritebackErrorKind::MergeAuthorityRejected => {
            BridgeWritebackFailureClass::MergeAuthorityRejected
        }
        BridgeWritebackErrorKind::StrategyFailed => BridgeWritebackFailureClass::StrategyFailed,
        BridgeWritebackErrorKind::StrategyPanicked => BridgeWritebackFailureClass::StrategyPanicked,
        BridgeWritebackErrorKind::ReplayMismatch => BridgeWritebackFailureClass::ReplayMismatch,
        BridgeWritebackErrorKind::AuthorityBypassRejected => {
            BridgeWritebackFailureClass::AuthorityBypassRejected
        }
        BridgeWritebackErrorKind::PreviewWritebackRejected => {
            BridgeWritebackFailureClass::PreviewWritebackRejected
        }
    }
}

fn writeback_causality_match_count(loop_prevention: &BridgeWritebackLoopPreventionReport) -> usize {
    usize::from(
        loop_prevention
            .incoming_feedback_causality_digest()
            .is_some(),
    )
}

fn writeback_execution_counters(
    loop_prevention: &BridgeWritebackLoopPreventionReport,
    outcome: Option<&BridgeWritebackAuthorityOutcome>,
    error_kind: Option<BridgeWritebackErrorKind>,
    candidate_present: bool,
    mapper_record_present: bool,
    request_present: bool,
    receipt_present: bool,
    replay_bundle_present: bool,
) -> BridgeWritebackCounters {
    debug_assert!(
        !receipt_present || request_present,
        "receipts should only exist for emitted requests"
    );
    debug_assert!(
        !replay_bundle_present || outcome.is_some(),
        "replay bundles should only exist for lowered outcomes"
    );
    let authority_boundary_observed = request_present || receipt_present;
    let noop_count = usize::from(
        loop_prevention.disposition() == BridgeWritebackLoopDisposition::CanonicalNoop
            || outcome.is_some_and(|value| {
                value.outcome_class() == BridgeWritebackOutcomeClass::CanonicalNoop
            }),
    );
    let commit_count = usize::from(outcome.is_some_and(|value| {
        value.outcome_class() == BridgeWritebackOutcomeClass::AuthoritativeCommit
    }));
    let strategy_rejection_count = usize::from(matches!(
        error_kind,
        Some(
            BridgeWritebackErrorKind::StrategyUnavailable
                | BridgeWritebackErrorKind::FamilyBindingMismatch
                | BridgeWritebackErrorKind::StrategyDescriptorMismatch
        )
    ));
    let validation_rejection_count = usize::from(matches!(
        error_kind,
        Some(
            BridgeWritebackErrorKind::WritebackNotRequested
                | BridgeWritebackErrorKind::PolicyRejected
                | BridgeWritebackErrorKind::IdempotenceBasisMismatch
                | BridgeWritebackErrorKind::InvariantRejected
                | BridgeWritebackErrorKind::PreviewWritebackRejected
        )
    ));

    BridgeWritebackCounters::new(
        1,
        1,
        usize::from(candidate_present),
        1 + usize::from(mapper_record_present),
        usize::from(authority_boundary_observed),
        1,
        1,
        strategy_rejection_count,
        1,
        writeback_causality_match_count(loop_prevention),
        1,
        usize::from(
            loop_prevention.disposition() == BridgeWritebackLoopDisposition::RejectAsUnsafeFeedback,
        ),
        noop_count,
        commit_count,
        usize::from(error_kind.is_some()),
        usize::from(matches!(
            error_kind,
            Some(BridgeWritebackErrorKind::AuthorityBypassRejected)
        )),
        validation_rejection_count,
        0,
        0,
    )
}

fn writeback_replay_validation_counters(mismatch: bool) -> BridgeWritebackCounters {
    BridgeWritebackCounters::new(
        1,
        1,
        0,
        1,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        usize::from(mismatch),
        0,
        0,
        1,
        usize::from(mismatch),
    )
}

fn writeback_failure_digest(
    error: &BridgeWritebackError,
    contract: &AdmittedBridgeWritebackContract,
    effect: &BridgeDerivedWritebackEffect,
    idempotence: &BridgeWritebackIdempotenceBasis,
) -> std::sync::Arc<str> {
    use sha2::{Digest, Sha256};

    let canonical_basis = format!(
        "bridge-writeback-execution-failure|kind:{:?}|contract={}|effect={}|idempotence={}|message={}",
        error.kind(),
        contract.digest(),
        effect.digest(),
        idempotence.digest(),
        error
    );
    let digest = Sha256::digest(canonical_basis.as_bytes());
    std::sync::Arc::from(format!(
        "bridge-writeback-execution-failure:sha256:{digest:x}"
    ))
}

fn validate_writeback_receipt_contract(
    request: &TruthWritebackRequest,
    receipt: &TruthWritebackReceipt,
) -> Result<(), BridgeWritebackError> {
    if receipt.request_digest() != request.digest() {
        return Err(BridgeWritebackError::new(
            BridgeWritebackErrorKind::InvariantRejected,
            format!(
                "truth writeback authority returned receipt `{}` for request `{}`",
                receipt.request_digest(),
                request.digest()
            ),
        ));
    }

    match (receipt.outcome_class(), receipt.failure_class()) {
        (BridgeWritebackOutcomeClass::Rejected, None) => Err(BridgeWritebackError::new(
            BridgeWritebackErrorKind::InvariantRejected,
            format!(
                "truth writeback authority returned rejected receipt `{}` without a failure class",
                receipt.digest()
            ),
        )),
        (BridgeWritebackOutcomeClass::CanonicalNoop, Some(_))
        | (BridgeWritebackOutcomeClass::AuthoritativeCommit, Some(_)) => {
            Err(BridgeWritebackError::new(
                BridgeWritebackErrorKind::InvariantRejected,
                format!(
                    "truth writeback authority returned non-rejected receipt `{}` with a failure class",
                    receipt.digest()
                ),
            ))
        }
        _ => Ok(()),
    }
}

impl RuntimeBridge {
    /// Specialist validation entrypoint for writeback declarations.
    ///
    /// Everyday bridge flows should reach writeback through higher-level
    /// promotion or authority workflows, not by assembling declarations by hand.
    pub fn validate_writeback_declaration(
        &self,
        declaration: BridgeWritebackDeclaration,
    ) -> Result<ValidatedBridgeWritebackDeclaration, BridgeWritebackError> {
        ValidatedBridgeWritebackDeclaration::new(declaration)
    }

    /// Admits one writeback declaration against a lowered runtime policy.
    pub fn admit_writeback_declaration(
        &self,
        declaration: BridgeWritebackDeclaration,
        lowered_policy: &LoweredBridgeExecutionPolicy,
    ) -> Result<AdmittedBridgeWritebackContract, BridgeWritebackError> {
        let validated = self.validate_writeback_declaration(declaration)?;
        let authority_inputs = BridgeWritebackAuthorityInputs::new(
            self.policy.allow_replay_artifacts(),
            self.policy.diagnostics_tier(),
        );
        let contract =
            AdmittedBridgeWritebackContract::new(validated, authority_inputs, lowered_policy)?;
        self.diagnostics
            .record_writeback_admission(BridgeWritebackFamilyAdmissionRecord::new(&contract));
        Ok(contract)
    }

    /// Lowers a writeback effect from contract, causality, and effect identity inputs.
    pub fn lower_writeback_effect(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        causality: &BridgeWritebackCausalityBasis,
        effect_identity: BridgeWritebackEffectIdentity,
        effect_digest: impl Into<std::sync::Arc<str>>,
    ) -> BridgeDerivedWritebackEffect {
        let mapped_input = self.map_writeback_family_input(
            contract,
            causality,
            effect_digest,
            "bridge-mapper-evidence:none",
        );
        BridgeDerivedWritebackEffect::new(effect_identity, &mapped_input)
    }

    /// Produces and records the mapper envelope for a writeback family input.
    pub fn lower_writeback_mapper_envelope(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        causality: &BridgeWritebackCausalityBasis,
        domain_payload_digest: impl Into<std::sync::Arc<str>>,
        domain_evidence_digest: impl Into<std::sync::Arc<str>>,
    ) -> BridgeWritebackMapperEnvelope {
        let envelope = BridgeWritebackMapperEnvelope::new(
            contract,
            causality,
            domain_payload_digest,
            domain_evidence_digest,
        );
        self.diagnostics
            .record_writeback_mapper_envelope(envelope.clone());
        envelope
    }

    /// Maps bridge-native writeback family inputs from mapper-envelope evidence.
    pub fn map_writeback_family_input(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        causality: &BridgeWritebackCausalityBasis,
        domain_payload_digest: impl Into<std::sync::Arc<str>>,
        domain_evidence_digest: impl Into<std::sync::Arc<str>>,
    ) -> BridgeMappedWritebackFamilyInput {
        let envelope = self.lower_writeback_mapper_envelope(
            contract,
            causality,
            domain_payload_digest,
            domain_evidence_digest,
        );
        let mapped_input = BridgeMappedWritebackFamilyInput::from_mapper_envelope(&envelope);
        self.diagnostics
            .record_writeback_mapped_family_input(mapped_input.clone());
        mapped_input
    }

    /// Derives feedback provenance for a lowered writeback effect.
    pub fn derive_writeback_feedback_provenance(
        &self,
        effect: &BridgeDerivedWritebackEffect,
    ) -> BridgeWritebackFeedbackProvenance {
        BridgeWritebackFeedbackProvenance::new(effect)
    }

    /// Computes idempotence basis data for a writeback effect under one policy.
    pub fn classify_writeback_idempotence(
        &self,
        effect: &BridgeDerivedWritebackEffect,
        lowered_policy: &LoweredBridgeExecutionPolicy,
        authoritative_state_digest: impl Into<std::sync::Arc<str>>,
        idempotence_identity: BridgeWritebackIdempotenceIdentity,
        idempotence_class: BridgeWritebackIdempotenceClass,
    ) -> BridgeWritebackIdempotenceBasis {
        BridgeWritebackIdempotenceBasis::new(
            idempotence_identity,
            effect,
            lowered_policy.digest(),
            authoritative_state_digest,
            idempotence_class,
        )
    }

    /// Classifies whether incoming feedback would create a writeback loop.
    pub fn classify_writeback_loop_prevention(
        &self,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        incoming_feedback_provenance_digest: Option<impl Into<std::sync::Arc<str>>>,
        incoming_feedback_causality_digest: Option<impl Into<std::sync::Arc<str>>>,
    ) -> BridgeWritebackLoopPreventionReport {
        BridgeWritebackLoopPreventionReport::classify(
            effect,
            idempotence,
            incoming_feedback_provenance_digest,
            incoming_feedback_causality_digest,
        )
    }

    /// Classifies strategy compatibility for a lowered writeback candidate.
    pub fn classify_writeback_strategy_compatibility(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
    ) -> BridgeWritebackStrategyCompatibilityReport {
        BridgeWritebackStrategyCompatibilityReport::classify(contract, effect, idempotence)
    }

    /// Validates a fully assembled writeback candidate before authority execution.
    pub fn validate_writeback_candidate(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        loop_prevention: &BridgeWritebackLoopPreventionReport,
        strategy_compatibility: &BridgeWritebackStrategyCompatibilityReport,
    ) -> Result<BridgeValidatedWritebackCandidate, BridgeWritebackError> {
        BridgeValidatedWritebackCandidate::new(
            contract,
            effect,
            idempotence,
            loop_prevention,
            strategy_compatibility,
        )
    }

    /// Produces the replay bundle for an executed writeback outcome.
    pub fn replay_writeback_bundle(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        outcome: &BridgeWritebackAuthorityOutcome,
    ) -> BridgeWritebackReplayBundle {
        BridgeWritebackReplayBundle::from_canonical_records(contract, effect, idempotence, outcome)
    }

    /// Verifies that a replayed writeback bundle still matches the expected semantics.
    pub fn validate_replayed_writeback_bundle(
        &self,
        expected: &BridgeWritebackReplayBundle,
        replayed: &BridgeWritebackReplayBundle,
    ) -> Result<(), BridgeWritebackError> {
        let mismatch = expected.semantic_digest() != replayed.semantic_digest();
        let failure_class = mismatch.then_some(BridgeWritebackFailureClass::ReplayMismatch);
        let counters = writeback_replay_validation_counters(mismatch);
        let replay_record =
            BridgeWritebackReplayRecord::new(expected, replayed, failure_class, counters);
        self.diagnostics.record_writeback_replay(replay_record);
        if expected.semantic_digest() != replayed.semantic_digest() {
            return Err(BridgeWritebackError::new(
                BridgeWritebackErrorKind::ReplayMismatch,
                format!(
                    "writeback replay semantic mismatch: expected `{}`, replayed `{}`",
                    expected.semantic_digest(),
                    replayed.semantic_digest()
                ),
            ));
        }

        Ok(())
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
            None::<std::sync::Arc<str>>,
            None::<std::sync::Arc<str>>,
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
        incoming_feedback_provenance_digest: Option<impl Into<std::sync::Arc<str>>>,
        incoming_feedback_causality_digest: Option<impl Into<std::sync::Arc<str>>>,
    ) -> Result<
        (
            BridgeWritebackLoopPreventionReport,
            BridgeWritebackAuthorityOutcome,
            Option<TruthWritebackReceipt>,
        ),
        BridgeWritebackError,
    > {
        let loop_prevention = self.classify_writeback_loop_prevention(
            effect,
            idempotence,
            incoming_feedback_provenance_digest,
            incoming_feedback_causality_digest,
        );
        match loop_prevention.disposition() {
            BridgeWritebackLoopDisposition::CanonicalNoop => {
                let outcome = BridgeWritebackAuthorityOutcome::canonical_noop(idempotence);
                let strategy_compatibility =
                    self.classify_writeback_strategy_compatibility(contract, effect, idempotence);
                let replay_bundle =
                    self.replay_writeback_bundle(contract, effect, idempotence, &outcome);
                let execution_record = BridgeWritebackExecutionRecord::new(
                    contract,
                    effect,
                    idempotence,
                    &loop_prevention,
                    &strategy_compatibility,
                    None,
                    None,
                    Some(&outcome),
                    Some(&replay_bundle),
                    None,
                    None,
                    None,
                    None::<std::sync::Arc<str>>,
                    writeback_execution_counters(
                        &loop_prevention,
                        Some(&outcome),
                        None,
                        false,
                        false,
                        false,
                        false,
                        true,
                    ),
                );
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
                let strategy_compatibility =
                    self.classify_writeback_strategy_compatibility(contract, effect, idempotence);
                let execution_record = BridgeWritebackExecutionRecord::new(
                    contract,
                    effect,
                    idempotence,
                    &loop_prevention,
                    &strategy_compatibility,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(map_writeback_error_kind_to_failure_class(error.kind())),
                    Some(writeback_failure_digest(
                        &error,
                        contract,
                        effect,
                        idempotence,
                    )),
                    writeback_execution_counters(
                        &loop_prevention,
                        None,
                        Some(error.kind()),
                        false,
                        false,
                        false,
                        false,
                        false,
                    ),
                );
                self.diagnostics
                    .record_writeback_execution(execution_record);
                return Err(error);
            }
            BridgeWritebackLoopDisposition::AllowAuthoritativeAttempt => {}
        }

        let strategy_compatibility =
            self.classify_writeback_strategy_compatibility(contract, effect, idempotence);
        let candidate = match self.validate_writeback_candidate(
            contract,
            effect,
            idempotence,
            &loop_prevention,
            &strategy_compatibility,
        ) {
            Ok(candidate) => candidate,
            Err(error) => {
                let execution_record = BridgeWritebackExecutionRecord::new(
                    contract,
                    effect,
                    idempotence,
                    &loop_prevention,
                    &strategy_compatibility,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(map_writeback_error_kind_to_failure_class(error.kind())),
                    Some(writeback_failure_digest(
                        &error,
                        contract,
                        effect,
                        idempotence,
                    )),
                    writeback_execution_counters(
                        &loop_prevention,
                        None,
                        Some(error.kind()),
                        false,
                        false,
                        false,
                        false,
                        false,
                    ),
                );
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
                    BridgeWritebackErrorKind::AuthorityBypassRejected,
                    "runtime has no truth writeback authority bound",
                );
                let execution_record = BridgeWritebackExecutionRecord::new(
                    contract,
                    effect,
                    idempotence,
                    &loop_prevention,
                    &strategy_compatibility,
                    Some(&mapper_record),
                    Some(&candidate),
                    None,
                    None,
                    None,
                    None,
                    Some(map_writeback_error_kind_to_failure_class(error.kind())),
                    Some(writeback_failure_digest(
                        &error,
                        contract,
                        effect,
                        idempotence,
                    )),
                    writeback_execution_counters(
                        &loop_prevention,
                        None,
                        Some(error.kind()),
                        true,
                        true,
                        false,
                        false,
                        false,
                    ),
                );
                self.diagnostics
                    .record_writeback_execution(execution_record);
                return Err(error);
            }
        };
        let feedback_provenance = self.derive_writeback_feedback_provenance(effect);
        let request = TruthWritebackRequest::new(
            effect.family_kind(),
            contract.digest(),
            candidate.digest(),
            effect.mapped_input_digest(),
            mapper_witness.digest(),
            effect.digest(),
            effect.effect_digest(),
            effect.effect_class(),
            effect.strategy_class(),
            feedback_provenance.digest(),
            loop_prevention.digest(),
            loop_prevention.disposition(),
            strategy_compatibility.digest(),
            idempotence.causality_digest(),
            idempotence.digest(),
            idempotence.idempotence_class(),
            effect.strategy_descriptor_digest(),
        );
        let request_for_validation = request.clone();
        let authority_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            authority.execute_writeback(request)
        })) {
            Ok(result) => result,
            Err(payload) => {
                let error = BridgeWritebackError::new(
                    BridgeWritebackErrorKind::StrategyPanicked,
                    format!(
                        "truth writeback authority panicked: {}",
                        panic_payload_message(payload)
                    ),
                );
                let execution_record = BridgeWritebackExecutionRecord::new(
                    contract,
                    effect,
                    idempotence,
                    &loop_prevention,
                    &strategy_compatibility,
                    Some(&mapper_record),
                    Some(&candidate),
                    None,
                    None,
                    Some(&request_for_validation),
                    None,
                    Some(map_writeback_error_kind_to_failure_class(error.kind())),
                    Some(writeback_failure_digest(
                        &error,
                        contract,
                        effect,
                        idempotence,
                    )),
                    writeback_execution_counters(
                        &loop_prevention,
                        None,
                        Some(error.kind()),
                        true,
                        true,
                        true,
                        false,
                        false,
                    ),
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
                let execution_record = BridgeWritebackExecutionRecord::new(
                    contract,
                    effect,
                    idempotence,
                    &loop_prevention,
                    &strategy_compatibility,
                    Some(&mapper_record),
                    Some(&candidate),
                    None,
                    None,
                    Some(&request_for_validation),
                    None,
                    Some(map_writeback_error_kind_to_failure_class(error.kind())),
                    Some(writeback_failure_digest(
                        &error,
                        contract,
                        effect,
                        idempotence,
                    )),
                    writeback_execution_counters(
                        &loop_prevention,
                        None,
                        Some(error.kind()),
                        true,
                        true,
                        true,
                        false,
                        false,
                    ),
                );
                self.diagnostics
                    .record_writeback_execution(execution_record);
                return Err(error);
            }
        };
        if let Err(error) = validate_writeback_receipt_contract(&request_for_validation, &receipt) {
            let execution_record = BridgeWritebackExecutionRecord::new(
                contract,
                effect,
                idempotence,
                &loop_prevention,
                &strategy_compatibility,
                Some(&mapper_record),
                Some(&candidate),
                None,
                None,
                Some(&request_for_validation),
                Some(&receipt),
                Some(map_writeback_error_kind_to_failure_class(error.kind())),
                Some(writeback_failure_digest(
                    &error,
                    contract,
                    effect,
                    idempotence,
                )),
                writeback_execution_counters(
                    &loop_prevention,
                    None,
                    Some(error.kind()),
                    true,
                    true,
                    true,
                    true,
                    false,
                ),
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
            let execution_record = BridgeWritebackExecutionRecord::new(
                contract,
                effect,
                idempotence,
                &loop_prevention,
                &strategy_compatibility,
                Some(&mapper_record),
                Some(&candidate),
                None,
                None,
                Some(&request_for_validation),
                Some(&receipt),
                Some(failure_class),
                Some(writeback_failure_digest(
                    &error,
                    contract,
                    effect,
                    idempotence,
                )),
                writeback_execution_counters(
                    &loop_prevention,
                    None,
                    Some(error.kind()),
                    true,
                    true,
                    true,
                    true,
                    false,
                ),
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
                BridgeWritebackAuthorityOutcome::authoritative_commit(
                    idempotence,
                    receipt.authoritative_artifact_digest(),
                )
            }
            BridgeWritebackOutcomeClass::Rejected => unreachable!(
                "rejected authority receipts are converted into typed bridge errors before outcome lowering"
            ),
        };
        let replay_bundle = self.replay_writeback_bundle(contract, effect, idempotence, &outcome);
        let execution_record = BridgeWritebackExecutionRecord::new(
            contract,
            effect,
            idempotence,
            &loop_prevention,
            &strategy_compatibility,
            Some(&mapper_record),
            Some(&candidate),
            Some(&outcome),
            Some(&replay_bundle),
            Some(&request_for_validation),
            Some(&receipt),
            None,
            None::<std::sync::Arc<str>>,
            writeback_execution_counters(
                &loop_prevention,
                Some(&outcome),
                None,
                true,
                true,
                true,
                true,
                true,
            ),
        );
        self.diagnostics
            .record_writeback_execution(execution_record);

        Ok((loop_prevention, outcome, Some(receipt)))
    }
}
