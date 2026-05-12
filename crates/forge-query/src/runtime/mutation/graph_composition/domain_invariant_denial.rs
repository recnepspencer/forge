use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryGraphCompositionAdmissionTrace, ForgeQueryGraphCompositionAdmissionTraceStage,
    ForgeQueryGraphCompositionDomainInvariantSummary,
};

use super::hooks::{
    ForgeQueryGraphCompositionInvariantPackContext,
    ForgeQueryGraphCompositionInvariantPackViolation,
};

const DOMAIN_INVARIANT_PACK_HOOK_FAMILY: &str = "domain_invariant_pack_hook";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionDomainInvariantDenial {
    hook_family: String,
    invariant_family: String,
    message: String,
    domain_invariant_summary: ForgeQueryGraphCompositionDomainInvariantSummary,
    admission_trace: ForgeQueryGraphCompositionAdmissionTrace,
    denial_digest: String,
}

impl ForgeQueryGraphCompositionDomainInvariantDenial {
    pub(crate) fn from_violation(
        violation: ForgeQueryGraphCompositionInvariantPackViolation,
        context: &ForgeQueryGraphCompositionInvariantPackContext<'_>,
    ) -> Self {
        use ForgeQueryGraphCompositionAdmissionTraceStage as Stage;

        let domain_invariant_summary = context.graph_composition_domain_invariant_summary();
        let admission_trace = ForgeQueryGraphCompositionAdmissionTrace::new(
            vec![
                Stage::ProgramParsed,
                Stage::SymbolsValidated,
                Stage::LoweringValidated,
                Stage::DomainInvariantEvaluated,
                Stage::DeniedBeforeExecution,
            ],
            Stage::DomainInvariantEvaluated,
        );
        let denial_digest = hash_parts(&[
            "forge_query_graph_composition_domain_invariant_denial_v1".to_string(),
            format!("hook:{DOMAIN_INVARIANT_PACK_HOOK_FAMILY}"),
            format!("invariant:{}", violation.invariant_family()),
            format!("message:{}", violation.message()),
            format!("summary:{}", domain_invariant_summary.summary_digest()),
            format!("trace:{}", admission_trace.admission_trace_digest()),
            format!("violation:{}", violation.violation_digest()),
        ]);
        Self {
            hook_family: DOMAIN_INVARIANT_PACK_HOOK_FAMILY.to_string(),
            invariant_family: violation.invariant_family().to_string(),
            message: violation.message().to_string(),
            domain_invariant_summary,
            admission_trace,
            denial_digest,
        }
    }

    pub fn hook_family(&self) -> &str {
        &self.hook_family
    }

    pub fn invariant_family(&self) -> &str {
        &self.invariant_family
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn domain_invariant_summary(&self) -> &ForgeQueryGraphCompositionDomainInvariantSummary {
        &self.domain_invariant_summary
    }

    pub fn admission_trace(&self) -> &ForgeQueryGraphCompositionAdmissionTrace {
        &self.admission_trace
    }

    pub fn failure_stage(&self) -> ForgeQueryGraphCompositionAdmissionTraceStage {
        self.admission_trace.failure_stage()
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}

impl std::fmt::Display for ForgeQueryGraphCompositionDomainInvariantDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "graph composition denied by {} for invariant `{}`: {}",
            self.hook_family, self.invariant_family, self.message
        )
    }
}
