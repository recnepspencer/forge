use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ForgeQueryIntentAdmissionDenial {
    stage: &'static str,
    message: String,
}

impl ForgeQueryIntentAdmissionDenial {
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
    declaration: &ForgeQueryIntentDeclaration,
) -> Result<(), ForgeQueryIntentAdmissionDenial> {
    if declaration.source_lane() != ForgeQueryIntentSourceLane::UserAuthored {
        return Err(ForgeQueryIntentAdmissionDenial::new(
            "source-lane-admission",
            format!(
                "Batch 7A admits only user-authored authoritative intents; `{}` source lane requires a later explicit policy boundary",
                declaration.source_lane().as_str()
            ),
        ));
    }

    if declaration.target_lane() != ForgeQueryAuthorityLane::AuthoritativeTruth {
        return Err(ForgeQueryIntentAdmissionDenial::new(
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
    declaration: &ForgeQueryIntentDeclaration,
) -> Result<(), ForgeQueryIntentAdmissionDenial> {
    if declaration.source_lane() != ForgeQueryIntentSourceLane::EffectTriggered {
        return Err(ForgeQueryIntentAdmissionDenial::new(
            "source-lane-admission",
            format!(
                "effect pending intent execution requires effect-triggered source lane, got `{}`",
                declaration.source_lane().as_str()
            ),
        ));
    }

    if declaration.target_lane() != ForgeQueryAuthorityLane::AuthoritativeTruth {
        return Err(ForgeQueryIntentAdmissionDenial::new(
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
    declaration: &ForgeQueryIntentDeclaration,
    effect_policy: ForgeQueryEffectPolicy,
) -> Result<ForgeQueryEffectAdmission, ForgeQueryIntentAdmissionDenial> {
    if effect_policy != ForgeQueryEffectPolicy::SandboxedWriteIntent {
        return Err(ForgeQueryIntentAdmissionDenial::new(
            "preview-effect-policy-admission",
            format!(
                "preview-local write-intent staging requires sandboxed-write-intent policy, got `{}`",
                effect_policy.as_str()
            ),
        ));
    }

    if declaration.source_lane() != ForgeQueryIntentSourceLane::PreviewLocal {
        return Err(ForgeQueryIntentAdmissionDenial::new(
            "source-lane-admission",
            format!(
                "preview intent execution requires preview-local source lane, got `{}`",
                declaration.source_lane().as_str()
            ),
        ));
    }

    if declaration.target_lane() != ForgeQueryAuthorityLane::PreviewTruth {
        return Err(ForgeQueryIntentAdmissionDenial::new(
            "authority-admission",
            format!(
                "preview intent execution admits only preview truth targets, got `{}`",
                declaration.target_lane()
            ),
        ));
    }

    effect_policy
        .admit(
            ForgeQueryEffectAction::WriteIntent,
            ForgeQueryAuthorityLane::PreviewTruth,
        )
        .map_err(|denial| {
            ForgeQueryIntentAdmissionDenial::new(
                "preview-effect-policy-admission",
                denial.to_string(),
            )
        })
}

pub(in crate::runtime) fn admit_branch_intent_declaration(
    declaration: &ForgeQueryIntentDeclaration,
    effect_policy: ForgeQueryEffectPolicy,
) -> Result<ForgeQueryEffectAdmission, ForgeQueryIntentAdmissionDenial> {
    if effect_policy != ForgeQueryEffectPolicy::SandboxedWriteIntent {
        return Err(ForgeQueryIntentAdmissionDenial::new(
            "branch-effect-policy-admission",
            format!(
                "branch-local write-intent staging requires sandboxed-write-intent policy, got `{}`",
                effect_policy.as_str()
            ),
        ));
    }

    if declaration.source_lane() != ForgeQueryIntentSourceLane::BranchLocal {
        return Err(ForgeQueryIntentAdmissionDenial::new(
            "source-lane-admission",
            format!(
                "branch intent execution requires branch-local source lane, got `{}`",
                declaration.source_lane().as_str()
            ),
        ));
    }

    if declaration.target_lane() != ForgeQueryAuthorityLane::BranchLocalTruth {
        return Err(ForgeQueryIntentAdmissionDenial::new(
            "authority-admission",
            format!(
                "branch intent execution admits only branch-local truth targets, got `{}`",
                declaration.target_lane()
            ),
        ));
    }

    effect_policy
        .admit(
            ForgeQueryEffectAction::WriteIntent,
            ForgeQueryAuthorityLane::BranchLocalTruth,
        )
        .map_err(|denial| {
            ForgeQueryIntentAdmissionDenial::new(
                "branch-effect-policy-admission",
                denial.to_string(),
            )
        })
}

pub(crate) fn admit_authoritative_intent_execution(
    declaration: &ForgeQueryIntentDeclaration,
    execution: &ForgeQueryIntentExecution,
) -> Result<(), ForgeQueryIntentAdmissionDenial> {
    if execution.strategy_identity() != declaration.strategy_name() {
        return Err(ForgeQueryIntentAdmissionDenial::new(
            "strategy-admission",
            format!(
                "intent authority returned strategy `{}` for declared strategy `{}`",
                execution.strategy_identity(),
                declaration.strategy_name()
            ),
        ));
    }

    if execution.strategy_version() != declaration.strategy_version() {
        return Err(ForgeQueryIntentAdmissionDenial::new(
            "strategy-version-admission",
            format!(
                "intent authority returned strategy version `{}` for declared version `{}`",
                execution.strategy_version(),
                declaration.strategy_version()
            ),
        ));
    }

    if execution.strategy_descriptor_digest().is_empty() {
        return Err(ForgeQueryIntentAdmissionDenial::new(
            "strategy-descriptor-admission",
            "intent authority returned an empty strategy descriptor digest",
        ));
    }

    if execution.canonical_input_digest() != declaration.input_digest() {
        return Err(ForgeQueryIntentAdmissionDenial::new(
            "input-digest-admission",
            "intent authority returned a canonical input digest that does not match the declaration",
        ));
    }

    if execution.outcome_digest().is_empty() {
        return Err(ForgeQueryIntentAdmissionDenial::new(
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
        return Err(ForgeQueryIntentAdmissionDenial::new(
            "mutation-receipt-admission",
            "intent authority returned an execution without snapshot identity",
        ));
    }

    match execution.execution_kind() {
        ForgeQueryIntentExecutionKind::Mutating => {
            if execution.mutation_receipt().commit_identity.is_empty() {
                return Err(ForgeQueryIntentAdmissionDenial::new(
                    "mutation-receipt-admission",
                    "mutating intent execution must publish commit identity",
                ));
            }
            if execution.mutation_receipt().deltas.is_empty() {
                return Err(ForgeQueryIntentAdmissionDenial::new(
                    "mutation-receipt-admission",
                    "mutating intent execution must publish at least one mutation delta; use idempotent-noop execution for no-op commits",
                ));
            }
        }
        ForgeQueryIntentExecutionKind::IdempotentNoop => {
            if execution.mutation_receipt().commit_identity.is_empty() {
                return Err(ForgeQueryIntentAdmissionDenial::new(
                    "mutation-receipt-admission",
                    "idempotent no-op intent execution must publish commit identity",
                ));
            }
            if !execution.mutation_receipt().deltas.is_empty() {
                return Err(ForgeQueryIntentAdmissionDenial::new(
                    "idempotence-admission",
                    "idempotent no-op intent execution cannot publish mutation deltas",
                ));
            }
        }
        ForgeQueryIntentExecutionKind::InvariantViolation => {
            if !execution.mutation_receipt().commit_identity.is_empty()
                || !execution.mutation_receipt().deltas.is_empty()
            {
                return Err(ForgeQueryIntentAdmissionDenial::new(
                    "invariant-admission",
                    "invariant-violation intent execution cannot publish commit identity or mutation deltas",
                ));
            }
            if execution.invariant_evidence().is_empty() {
                return Err(ForgeQueryIntentAdmissionDenial::new(
                    "invariant-admission",
                    "invariant-violation intent execution must carry invariant evidence",
                ));
            }
            return Err(ForgeQueryIntentAdmissionDenial::new(
                "invariant-admission",
                "intent authority rejected the commit because relational invariants failed",
            ));
        }
    }

    Ok(())
}
