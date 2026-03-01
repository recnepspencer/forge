use forge_core::tracing::{PolicyResolutionSource, PolicyResolutionScopeRef, TraceAdjunctRecord};
use forge_core::DecisionId;

/// Value source metadata chosen by policy precedence resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPolicySource {
    pub source: PolicyResolutionSource,
    pub source_scope: Option<PolicyResolutionScopeRef>,
    pub default_used: bool,
}

/// Resolved policy decision for an ambiguous query.
#[derive(Debug, Clone)]
pub struct ResolvedPolicyDecision {
    pub accept_potential_value: bool,
    pub source: ResolvedPolicySource,
    pub decision_id: DecisionId,
    pub adjunct: TraceAdjunctRecord,
}
