use super::WorthQueryAftermathKind;
use crate::domain_installation::WorthQueryAftermathPostcondition;
use crate::domain_installation::WorthQueryWorkflowEffectEvidence;
use crate::memory_workspace::WorthQueryEntityIdentity;

pub struct WorthQueryAftermathOriginalEvidence {
    trace_identity: String,
    kind: WorthQueryAftermathKind,
    postcondition: WorthQueryAftermathPostcondition,
    effects: Vec<WorthQueryWorkflowEffectEvidence>,
    lineage_report_identity: Option<String>,
}

impl WorthQueryAftermathOriginalEvidence {
    pub(crate) fn new(
        trace_identity: String,
        kind: WorthQueryAftermathKind,
        postcondition: WorthQueryAftermathPostcondition,
        effects: Vec<WorthQueryWorkflowEffectEvidence>,
        lineage_report_identity: Option<String>,
    ) -> Self {
        Self {
            trace_identity,
            kind,
            postcondition,
            effects,
            lineage_report_identity,
        }
    }

    pub fn trace_identity(&self) -> &str {
        &self.trace_identity
    }

    pub const fn kind(&self) -> WorthQueryAftermathKind {
        self.kind
    }

    pub fn postcondition(&self) -> &WorthQueryAftermathPostcondition {
        &self.postcondition
    }

    pub fn effect_target(&self, ordinal: usize) -> Option<&WorthQueryEntityIdentity> {
        self.effects
            .get(ordinal)
            .and_then(WorthQueryWorkflowEffectEvidence::mutation_receipt)
            .and_then(crate::runtime::WorthQueryWriteReceipt::target_entity_identity)
    }

    /// Returns the exact Query-owned effect evidence from the original
    /// execution. Domain postcondition evaluators use this evidence to prove
    /// restoration; they must not reconstruct prior truth from labels.
    pub fn effect(&self, ordinal: usize) -> Option<&WorthQueryWorkflowEffectEvidence> {
        self.effects.get(ordinal)
    }

    pub fn effect_count(&self) -> usize {
        self.effects.len()
    }

    pub fn lineage_report_identity(&self) -> Option<&str> {
        self.lineage_report_identity.as_deref()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAftermathExecutionDenialKind {
    DomainPlanUnavailable,
    CandidateExecutionFailed,
    ExactInverseScopeMismatch,
    PostconditionNotEstablished,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAftermathFailureRecoveryPosture {
    DomainRecoveryRequired {
        attempted_kind: WorthQueryAftermathKind,
    },
}

#[derive(Debug)]
pub struct WorthQueryAftermathExecutionDenial {
    kind: WorthQueryAftermathExecutionDenialKind,
    candidate_trace_identity: Option<String>,
    partial_effects: Vec<WorthQueryWorkflowEffectEvidence>,
    recovery_posture: Option<WorthQueryAftermathFailureRecoveryPosture>,
    candidate_execution_stop:
        Option<Box<crate::domain_installation::WorthQueryWorkflowReexecutionStop>>,
}

impl WorthQueryAftermathExecutionDenial {
    pub(crate) fn before_execution(kind: WorthQueryAftermathExecutionDenialKind) -> Self {
        Self {
            kind,
            candidate_trace_identity: None,
            partial_effects: Vec::new(),
            recovery_posture: None,
            candidate_execution_stop: None,
        }
    }

    pub(crate) fn after_execution(
        kind: WorthQueryAftermathExecutionDenialKind,
        trace_identity: &str,
        effects: &[WorthQueryWorkflowEffectEvidence],
        attempted_kind: WorthQueryAftermathKind,
    ) -> Self {
        Self {
            kind,
            candidate_trace_identity: Some(trace_identity.to_owned()),
            partial_effects: effects.to_vec(),
            recovery_posture: Some(
                WorthQueryAftermathFailureRecoveryPosture::DomainRecoveryRequired {
                    attempted_kind,
                },
            ),
            candidate_execution_stop: None,
        }
    }

    pub(crate) fn from_candidate_stop(
        stop: crate::domain_installation::WorthQueryWorkflowReexecutionStop,
        attempted_kind: WorthQueryAftermathKind,
    ) -> Self {
        Self {
            kind: WorthQueryAftermathExecutionDenialKind::CandidateExecutionFailed,
            candidate_trace_identity: None,
            partial_effects: stop.executed_effects().to_vec(),
            recovery_posture: Some(
                WorthQueryAftermathFailureRecoveryPosture::DomainRecoveryRequired {
                    attempted_kind,
                },
            ),
            candidate_execution_stop: Some(Box::new(stop)),
        }
    }

    pub const fn kind(&self) -> WorthQueryAftermathExecutionDenialKind {
        self.kind
    }

    pub fn candidate_trace_identity(&self) -> Option<&str> {
        self.candidate_trace_identity.as_deref()
    }

    pub fn partial_effects(&self) -> &[WorthQueryWorkflowEffectEvidence] {
        &self.partial_effects
    }

    pub const fn recovery_posture(&self) -> Option<WorthQueryAftermathFailureRecoveryPosture> {
        self.recovery_posture
    }

    pub fn candidate_execution_stop(
        &self,
    ) -> Option<&crate::domain_installation::WorthQueryWorkflowReexecutionStop> {
        self.candidate_execution_stop.as_deref()
    }
}

pub(crate) fn exact_inverse_effect_scope_matches_original(
    original: &WorthQueryAftermathOriginalEvidence,
    candidate: &[WorthQueryWorkflowEffectEvidence],
) -> (bool, usize) {
    if original.effects.len() != candidate.len() || original.effects.is_empty() {
        return (false, 0);
    }
    let mut checks = 0;
    for (original, candidate) in original.effects.iter().zip(candidate) {
        checks += 1;
        let Some(original) = original.mutation_receipt() else {
            return (false, checks);
        };
        let Some(candidate) = candidate.mutation_receipt() else {
            return (false, checks);
        };
        if !same_current_target_entity(original, candidate)
            || original.target_collection_identity() != candidate.target_collection_identity()
        {
            return (false, checks);
        }
    }
    (true, checks)
}

fn same_current_target_entity(
    original: &crate::runtime::WorthQueryWriteReceipt,
    candidate: &crate::runtime::WorthQueryWriteReceipt,
) -> bool {
    match (
        original.target_entity_identity(),
        candidate.target_entity_identity(),
    ) {
        (Some(original), Some(candidate)) => original.is_same_current_identity_as(candidate),
        (None, None) => true,
        _ => false,
    }
}
