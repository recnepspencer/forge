use crate::runtime::{
    ForgeQueryGraphCompositionAdmissionTrace, ForgeQueryGraphCompositionAdmissionTraceStage,
    ForgeQueryGraphCompositionDomainInvariantSummary,
};
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
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
        Self::build(
            violation.invariant_family().to_string(),
            violation.message().to_string(),
            context.graph_composition_domain_invariant_summary(),
            violation.violation_digest().to_string(),
        )
    }

    pub(crate) fn from_contributed(
        invariant_family: impl Into<String>,
        message: impl Into<String>,
        domain_invariant_summary: ForgeQueryGraphCompositionDomainInvariantSummary,
    ) -> Self {
        let invariant_family = invariant_family.into();
        let message = message.into();
        let violation = ForgeQueryGraphCompositionInvariantPackViolation::new(
            invariant_family.clone(),
            message.clone(),
        );
        Self::build(
            invariant_family,
            message,
            domain_invariant_summary,
            violation.violation_digest().to_string(),
        )
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

impl ForgeQueryGraphCompositionDomainInvariantDenial {
    fn build(
        invariant_family: String,
        message: String,
        domain_invariant_summary: ForgeQueryGraphCompositionDomainInvariantSummary,
        violation_digest: String,
    ) -> Self {
        use ForgeQueryGraphCompositionAdmissionTraceStage as Stage;

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
        let denial_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::GraphCompositionDomainInvariantDenial,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("hook_family"),
            DOMAIN_INVARIANT_PACK_HOOK_FAMILY,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("invariant_family"),
            invariant_family.as_str(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("summary_digest"),
            domain_invariant_summary.summary_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("admission_trace_digest"),
            admission_trace.admission_trace_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("violation_digest"),
            violation_digest.as_str(),
        )
        .seal()
        .as_str()
        .to_string();
        Self {
            hook_family: DOMAIN_INVARIANT_PACK_HOOK_FAMILY.to_string(),
            invariant_family,
            message,
            domain_invariant_summary,
            admission_trace,
            denial_digest,
        }
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
