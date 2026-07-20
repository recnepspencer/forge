use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryIntentAdmissionDenial {
    stage: &'static str,
    message: String,
}

impl WorthQueryIntentAdmissionDenial {
    pub(crate) fn new(stage: &'static str, message: impl Into<String>) -> Self {
        Self {
            stage,
            message: message.into(),
        }
    }

    pub(crate) fn stage(&self) -> &'static str {
        self.stage
    }

    pub(crate) fn message(&self) -> &str {
        &self.message
    }
}

pub(crate) fn admit_authoritative_intent_declaration(
    declaration: &WorthQueryIntentDeclaration,
) -> Result<(), WorthQueryIntentAdmissionDenial> {
    if declaration.source_lane() != WorthQueryIntentSourceLane::UserAuthored {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "source-lane-admission",
            format!(
                "Batch 7A admits only user-authored authoritative intents; `{}` source lane requires a later explicit policy boundary",
                declaration.source_lane().as_str()
            ),
        ));
    }

    if declaration.target_lane() != WorthQueryAuthorityLane::AuthoritativeTruth {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "authority-admission",
            format!(
                "Batch 7A admits only authoritative truth targets, got `{}`",
                declaration.target_lane()
            ),
        ));
    }

    Ok(())
}

pub(crate) fn admit_effect_triggered_intent_declaration(
    declaration: &WorthQueryIntentDeclaration,
) -> Result<(), WorthQueryIntentAdmissionDenial> {
    if declaration.source_lane() != WorthQueryIntentSourceLane::EffectTriggered {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "source-lane-admission",
            format!(
                "effect pending intent execution requires effect-triggered source lane, got `{}`",
                declaration.source_lane().as_str()
            ),
        ));
    }

    if declaration.target_lane() != WorthQueryAuthorityLane::AuthoritativeTruth {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "authority-admission",
            format!(
                "effect pending intent execution admits only authoritative truth targets, got `{}`",
                declaration.target_lane()
            ),
        ));
    }

    Ok(())
}

pub(in crate::runtime) fn admit_preview_intent_declaration(
    declaration: &WorthQueryIntentDeclaration,
    effect_policy: WorthQueryEffectPolicy,
) -> Result<WorthQueryEffectAdmission, WorthQueryIntentAdmissionDenial> {
    if effect_policy != WorthQueryEffectPolicy::SandboxedWriteIntent {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "preview-effect-policy-admission",
            format!(
                "preview-local write-intent staging requires sandboxed-write-intent policy, got `{}`",
                effect_policy.as_str()
            ),
        ));
    }

    if declaration.source_lane() != WorthQueryIntentSourceLane::PreviewLocal {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "source-lane-admission",
            format!(
                "preview intent execution requires preview-local source lane, got `{}`",
                declaration.source_lane().as_str()
            ),
        ));
    }

    if declaration.target_lane() != WorthQueryAuthorityLane::PreviewTruth {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "authority-admission",
            format!(
                "preview intent execution admits only preview truth targets, got `{}`",
                declaration.target_lane()
            ),
        ));
    }

    effect_policy
        .admit(
            WorthQueryEffectAction::WriteIntent,
            WorthQueryAuthorityLane::PreviewTruth,
        )
        .map_err(|denial| {
            WorthQueryIntentAdmissionDenial::new(
                "preview-effect-policy-admission",
                denial.to_string(),
            )
        })
}

pub(in crate::runtime) fn admit_branch_intent_declaration(
    declaration: &WorthQueryIntentDeclaration,
    effect_policy: WorthQueryEffectPolicy,
) -> Result<WorthQueryEffectAdmission, WorthQueryIntentAdmissionDenial> {
    if effect_policy != WorthQueryEffectPolicy::SandboxedWriteIntent {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "branch-effect-policy-admission",
            format!(
                "branch-local write-intent staging requires sandboxed-write-intent policy, got `{}`",
                effect_policy.as_str()
            ),
        ));
    }

    if declaration.source_lane() != WorthQueryIntentSourceLane::BranchLocal {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "source-lane-admission",
            format!(
                "branch intent execution requires branch-local source lane, got `{}`",
                declaration.source_lane().as_str()
            ),
        ));
    }

    if declaration.target_lane() != WorthQueryAuthorityLane::BranchLocalTruth {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "authority-admission",
            format!(
                "branch intent execution admits only branch-local truth targets, got `{}`",
                declaration.target_lane()
            ),
        ));
    }

    effect_policy
        .admit(
            WorthQueryEffectAction::WriteIntent,
            WorthQueryAuthorityLane::BranchLocalTruth,
        )
        .map_err(|denial| {
            WorthQueryIntentAdmissionDenial::new(
                "branch-effect-policy-admission",
                denial.to_string(),
            )
        })
}

pub(crate) fn admit_authoritative_intent_execution(
    declaration: &WorthQueryIntentDeclaration,
    execution: &WorthQueryIntentExecution,
) -> Result<(), WorthQueryIntentAdmissionDenial> {
    if execution.strategy_identity() != declaration.strategy_name() {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "strategy-admission",
            format!(
                "intent authority returned strategy `{}` for declared strategy `{}`",
                execution.strategy_identity(),
                declaration.strategy_name()
            ),
        ));
    }

    if execution.strategy_version() != declaration.strategy_version() {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "strategy-version-admission",
            format!(
                "intent authority returned strategy version `{}` for declared version `{}`",
                execution.strategy_version(),
                declaration.strategy_version()
            ),
        ));
    }

    if execution.strategy_descriptor_digest().is_empty() {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "strategy-descriptor-admission",
            "intent authority returned an empty strategy descriptor digest",
        ));
    }

    if execution.canonical_input_digest() != declaration.input_digest() {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "input-digest-admission",
            "intent authority returned a canonical input digest that does not match the declaration",
        ));
    }

    if execution.outcome_digest().is_empty() {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "outcome-digest-admission",
            "intent authority returned an empty intent outcome digest",
        ));
    }

    if execution
        .mutation_receipt()
        .snapshot_identity
        .evidence_identity()
        .as_str()
        .is_empty()
    {
        return Err(WorthQueryIntentAdmissionDenial::new(
            "mutation-receipt-admission",
            "intent authority returned an execution without snapshot identity",
        ));
    }

    match execution.execution_kind() {
        WorthQueryIntentExecutionKind::Mutating => {
            if execution.mutation_receipt().commit_identity.is_empty() {
                return Err(WorthQueryIntentAdmissionDenial::new(
                    "mutation-receipt-admission",
                    "mutating intent execution must publish commit identity",
                ));
            }
            if execution.mutation_receipt().deltas.is_empty() {
                return Err(WorthQueryIntentAdmissionDenial::new(
                    "mutation-receipt-admission",
                    "mutating intent execution must publish at least one mutation delta; use idempotent-noop execution for no-op commits",
                ));
            }
        }
        WorthQueryIntentExecutionKind::IdempotentNoop => {
            if execution.mutation_receipt().commit_identity.is_empty() {
                return Err(WorthQueryIntentAdmissionDenial::new(
                    "mutation-receipt-admission",
                    "idempotent no-op intent execution must publish commit identity",
                ));
            }
            if !execution.mutation_receipt().deltas.is_empty() {
                return Err(WorthQueryIntentAdmissionDenial::new(
                    "idempotence-admission",
                    "idempotent no-op intent execution cannot publish mutation deltas",
                ));
            }
        }
        WorthQueryIntentExecutionKind::InvariantViolation => {
            if !execution.mutation_receipt().commit_identity.is_empty()
                || !execution.mutation_receipt().deltas.is_empty()
            {
                return Err(WorthQueryIntentAdmissionDenial::new(
                    "invariant-admission",
                    "invariant-violation intent execution cannot publish commit identity or mutation deltas",
                ));
            }
            if execution.invariant_evidence().is_empty() {
                return Err(WorthQueryIntentAdmissionDenial::new(
                    "invariant-admission",
                    "invariant-violation intent execution must carry invariant evidence",
                ));
            }
            return Err(WorthQueryIntentAdmissionDenial::new(
                "invariant-admission",
                "intent authority rejected the commit because relational invariants failed",
            ));
        }
    }

    Ok(())
}
