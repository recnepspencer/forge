use crate::identity::hash_parts;
use crate::workflow::QueryWorkflowDeclaration;

use super::counters::EffectLifecycleCounters;
use super::normalized::NormalizedEffectIntent;
use super::taxonomy::DeniedEffectEligibilityKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectEligibilityDecisionTrace {
    normalized_digest: String,
    outcome: &'static str,
    message: &'static str,
    trace_digest: String,
}

impl EffectEligibilityDecisionTrace {
    pub(crate) fn new(
        normalized: &NormalizedEffectIntent,
        outcome: &'static str,
        message: &'static str,
    ) -> Self {
        let trace_digest = hash_parts(&[
            format!("normalized:{}", normalized.normalized_digest()),
            format!("outcome:{outcome}"),
            format!("message:{message}"),
        ]);
        Self {
            normalized_digest: normalized.normalized_digest().to_string(),
            outcome,
            message,
            trace_digest,
        }
    }

    pub fn normalized_digest(&self) -> &str {
        &self.normalized_digest
    }

    pub fn outcome(&self) -> &'static str {
        self.outcome
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn trace_digest(&self) -> &str {
        &self.trace_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EffectEligibility {
    normalized: NormalizedEffectIntent,
    workflow_declaration: QueryWorkflowDeclaration,
    decision_trace: EffectEligibilityDecisionTrace,
    counters: EffectLifecycleCounters,
}

impl EffectEligibility {
    pub(crate) fn new(
        normalized: NormalizedEffectIntent,
        workflow_declaration: QueryWorkflowDeclaration,
        support_row_count: usize,
    ) -> Self {
        Self {
            decision_trace: EffectEligibilityDecisionTrace::new(
                &normalized,
                "admitted",
                "effect is admitted",
            ),
            normalized,
            workflow_declaration,
            counters: EffectLifecycleCounters::admitted(support_row_count),
        }
    }

    pub fn normalized(&self) -> &NormalizedEffectIntent {
        &self.normalized
    }

    pub fn decision_trace(&self) -> &EffectEligibilityDecisionTrace {
        &self.decision_trace
    }

    pub fn workflow_declaration(&self) -> &QueryWorkflowDeclaration {
        &self.workflow_declaration
    }

    pub fn counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeniedEffectEligibility {
    denial_kind: DeniedEffectEligibilityKind,
    decision_trace: EffectEligibilityDecisionTrace,
    counters: EffectLifecycleCounters,
}

impl DeniedEffectEligibility {
    pub(crate) fn new(
        denial_kind: DeniedEffectEligibilityKind,
        normalized: &NormalizedEffectIntent,
        message: &'static str,
        support_row_count: usize,
    ) -> Self {
        Self {
            denial_kind,
            decision_trace: EffectEligibilityDecisionTrace::new(normalized, "denied", message),
            counters: EffectLifecycleCounters::denied(support_row_count),
        }
    }

    pub fn denial_kind(&self) -> DeniedEffectEligibilityKind {
        self.denial_kind
    }

    pub fn decision_trace(&self) -> &EffectEligibilityDecisionTrace {
        &self.decision_trace
    }

    pub fn counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RebindRequiredEffectEligibility {
    denial_kind: DeniedEffectEligibilityKind,
    decision_trace: EffectEligibilityDecisionTrace,
    counters: EffectLifecycleCounters,
}

impl RebindRequiredEffectEligibility {
    pub(crate) fn new(
        normalized: &NormalizedEffectIntent,
        message: &'static str,
        support_row_count: usize,
    ) -> Self {
        Self {
            denial_kind: DeniedEffectEligibilityKind::PreviewRebindRequired,
            decision_trace: EffectEligibilityDecisionTrace::new(
                normalized,
                "rebind_required",
                message,
            ),
            counters: EffectLifecycleCounters::rebind_required(support_row_count),
        }
    }

    pub fn denial_kind(&self) -> DeniedEffectEligibilityKind {
        self.denial_kind
    }

    pub fn decision_trace(&self) -> &EffectEligibilityDecisionTrace {
        &self.decision_trace
    }

    pub fn counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredEffectEligibility {
    denial_kind: DeniedEffectEligibilityKind,
    decision_trace: EffectEligibilityDecisionTrace,
    counters: EffectLifecycleCounters,
}

impl DeferredEffectEligibility {
    pub(crate) fn new(
        normalized: &NormalizedEffectIntent,
        message: &'static str,
        support_row_count: usize,
    ) -> Self {
        Self {
            denial_kind: DeniedEffectEligibilityKind::DeferredToLaterMilestone,
            decision_trace: EffectEligibilityDecisionTrace::new(normalized, "deferred", message),
            counters: EffectLifecycleCounters::deferred(support_row_count),
        }
    }

    pub fn denial_kind(&self) -> DeniedEffectEligibilityKind {
        self.denial_kind
    }

    pub fn decision_trace(&self) -> &EffectEligibilityDecisionTrace {
        &self.decision_trace
    }

    pub fn counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EffectEligibilityOutcome {
    Admitted(EffectEligibility),
    Denied(DeniedEffectEligibility),
    RebindRequired(RebindRequiredEffectEligibility),
    Deferred(DeferredEffectEligibility),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedEffectIntent {
    normalized: NormalizedEffectIntent,
    workflow_declaration: QueryWorkflowDeclaration,
    admitted_digest: String,
}

impl AdmittedEffectIntent {
    pub(crate) fn new(eligibility: EffectEligibility) -> Self {
        let admitted_digest = eligibility.normalized.admitted_digest();
        Self {
            normalized: eligibility.normalized,
            workflow_declaration: eligibility.workflow_declaration,
            admitted_digest,
        }
    }

    pub fn normalized(&self) -> &NormalizedEffectIntent {
        &self.normalized
    }

    pub fn admitted_digest(&self) -> &str {
        &self.admitted_digest
    }

    pub fn workflow_declaration(&self) -> &QueryWorkflowDeclaration {
        &self.workflow_declaration
    }
}
