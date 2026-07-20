use super::scope::WorthQueryEvidenceScope;

pub(crate) fn graph_application_evidence_scope_as_str(
    scope: WorthQueryEvidenceScope,
) -> &'static str {
    match scope {
        WorthQueryEvidenceScope::GraphCompositionDomainInvariantDenial => {
            "graph-composition-domain-invariant-denial"
        }
        WorthQueryEvidenceScope::GraphCompositionInvariantViolation => {
            "graph-composition-invariant-violation"
        }
        WorthQueryEvidenceScope::GraphTouchDescriptor => "graph-touch-descriptor",
        WorthQueryEvidenceScope::GraphTouchDescriptorRow => "graph-touch-descriptor-row",
        WorthQueryEvidenceScope::GraphObligationRuleIdentity => "graph-obligation-rule-identity",
        WorthQueryEvidenceScope::GraphObligationDispatchContext => {
            "graph-obligation-dispatch-context"
        }
        WorthQueryEvidenceScope::GraphObligationDispatchPlan => "graph-obligation-dispatch-plan",
        WorthQueryEvidenceScope::GraphObligationDispatchEnvelope => {
            "graph-obligation-dispatch-envelope"
        }
        WorthQueryEvidenceScope::GraphObligationExecutionBudget => {
            "graph-obligation-execution-budget"
        }
        WorthQueryEvidenceScope::GraphObligationExecutorContract => {
            "graph-obligation-executor-contract"
        }
        WorthQueryEvidenceScope::GraphObligationExecutionInput => {
            "graph-obligation-execution-input"
        }
        WorthQueryEvidenceScope::GraphObligationExecutionContext => {
            "graph-obligation-execution-context"
        }
        WorthQueryEvidenceScope::GraphObligationStateLoadPlan => "graph-obligation-state-load-plan",
        WorthQueryEvidenceScope::GraphObligationStateLoadCounters => {
            "graph-obligation-state-load-counters"
        }
        WorthQueryEvidenceScope::GraphObligationExecutionResultRow => {
            "graph-obligation-execution-result-row"
        }
        WorthQueryEvidenceScope::GraphObligationExecutionResultEnvelope => {
            "graph-obligation-execution-result-envelope"
        }
        WorthQueryEvidenceScope::GraphObligationReduction => "graph-obligation-reduction",
        WorthQueryEvidenceScope::GraphObligationDenialProjection => {
            "graph-obligation-denial-projection"
        }
        WorthQueryEvidenceScope::GraphObligationDenialProjectionRow => {
            "graph-obligation-denial-projection-row"
        }
        WorthQueryEvidenceScope::GraphObligationAttachmentEvidence => {
            "graph-obligation-attachment-evidence"
        }
        WorthQueryEvidenceScope::GraphObligationDenialAttachmentProjection => {
            "graph-obligation-denial-attachment-projection"
        }
        WorthQueryEvidenceScope::GraphObligationDenialAttachmentProjectionRow => {
            "graph-obligation-denial-attachment-projection-row"
        }
        WorthQueryEvidenceScope::GraphObligationMaterializedDispatch => {
            "graph-obligation-materialized-dispatch"
        }
        WorthQueryEvidenceScope::GraphObligationSupportMatrixRow => {
            "graph-obligation-support-matrix-row"
        }
        WorthQueryEvidenceScope::GraphObligationSupportMatrix => "graph-obligation-support-matrix",
        WorthQueryEvidenceScope::GraphObligationTouchSelector => "graph-obligation-touch-selector",
        WorthQueryEvidenceScope::GraphObligationOperatingWorldSelector => {
            "graph-obligation-operating-world-selector"
        }
        WorthQueryEvidenceScope::GraphObligationOperatingWorldDescriptor => {
            "graph-obligation-operating-world-descriptor"
        }
        WorthQueryEvidenceScope::GraphObligationSupportPosture => {
            "graph-obligation-support-posture"
        }
        WorthQueryEvidenceScope::GraphObligationRegistration => "graph-obligation-registration",
        WorthQueryEvidenceScope::GraphObligationRegistrationCatalog => {
            "graph-obligation-registration-catalog"
        }
        WorthQueryEvidenceScope::GraphObligationIndex => "graph-obligation-index",
        WorthQueryEvidenceScope::GraphObligationIndexEntry => "graph-obligation-index-entry",
        WorthQueryEvidenceScope::GraphObligationIndexComplexityContract => {
            "graph-obligation-index-complexity-contract"
        }
        WorthQueryEvidenceScope::GraphObligationIndexBuildCounters => {
            "graph-obligation-index-build-counters"
        }
        WorthQueryEvidenceScope::GraphObligationSelection => "graph-obligation-selection",
        WorthQueryEvidenceScope::GraphObligationSelectionCounters => {
            "graph-obligation-selection-counters"
        }
        WorthQueryEvidenceScope::GraphObligationIndexSupportRow => {
            "graph-obligation-index-support-row"
        }
        WorthQueryEvidenceScope::ReadDomainInvariantDenial => "read-domain-invariant-denial",
        WorthQueryEvidenceScope::ReadInvariantViolation => "read-invariant-violation",
        WorthQueryEvidenceScope::ApplicationSupportSectionPosture => {
            "application-support-section-posture"
        }
        WorthQueryEvidenceScope::ApplicationSupportReport => "application-support-report",
        _ => unreachable!("graph/application scope helper called with unrelated scope"),
    }
}
