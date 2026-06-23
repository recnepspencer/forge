use super::scope::ForgeQueryEvidenceScope;

pub(crate) fn graph_application_evidence_scope_as_str(
    scope: ForgeQueryEvidenceScope,
) -> &'static str {
    match scope {
        ForgeQueryEvidenceScope::GraphCompositionDomainInvariantDenial => {
            "graph-composition-domain-invariant-denial"
        }
        ForgeQueryEvidenceScope::GraphCompositionInvariantViolation => {
            "graph-composition-invariant-violation"
        }
        ForgeQueryEvidenceScope::GraphTouchDescriptor => "graph-touch-descriptor",
        ForgeQueryEvidenceScope::GraphTouchDescriptorRow => "graph-touch-descriptor-row",
        ForgeQueryEvidenceScope::GraphObligationRuleIdentity => "graph-obligation-rule-identity",
        ForgeQueryEvidenceScope::GraphObligationDispatchContext => {
            "graph-obligation-dispatch-context"
        }
        ForgeQueryEvidenceScope::GraphObligationDispatchPlan => "graph-obligation-dispatch-plan",
        ForgeQueryEvidenceScope::GraphObligationDispatchEnvelope => {
            "graph-obligation-dispatch-envelope"
        }
        ForgeQueryEvidenceScope::GraphObligationExecutionBudget => {
            "graph-obligation-execution-budget"
        }
        ForgeQueryEvidenceScope::GraphObligationExecutorContract => {
            "graph-obligation-executor-contract"
        }
        ForgeQueryEvidenceScope::GraphObligationExecutionInput => {
            "graph-obligation-execution-input"
        }
        ForgeQueryEvidenceScope::GraphObligationExecutionContext => {
            "graph-obligation-execution-context"
        }
        ForgeQueryEvidenceScope::GraphObligationStateLoadPlan => "graph-obligation-state-load-plan",
        ForgeQueryEvidenceScope::GraphObligationStateLoadCounters => {
            "graph-obligation-state-load-counters"
        }
        ForgeQueryEvidenceScope::GraphObligationExecutionResultRow => {
            "graph-obligation-execution-result-row"
        }
        ForgeQueryEvidenceScope::GraphObligationExecutionResultEnvelope => {
            "graph-obligation-execution-result-envelope"
        }
        ForgeQueryEvidenceScope::GraphObligationReduction => "graph-obligation-reduction",
        ForgeQueryEvidenceScope::GraphObligationDenialProjection => {
            "graph-obligation-denial-projection"
        }
        ForgeQueryEvidenceScope::GraphObligationDenialProjectionRow => {
            "graph-obligation-denial-projection-row"
        }
        ForgeQueryEvidenceScope::GraphObligationAttachmentEvidence => {
            "graph-obligation-attachment-evidence"
        }
        ForgeQueryEvidenceScope::GraphObligationDenialAttachmentProjection => {
            "graph-obligation-denial-attachment-projection"
        }
        ForgeQueryEvidenceScope::GraphObligationDenialAttachmentProjectionRow => {
            "graph-obligation-denial-attachment-projection-row"
        }
        ForgeQueryEvidenceScope::GraphObligationMaterializedDispatch => {
            "graph-obligation-materialized-dispatch"
        }
        ForgeQueryEvidenceScope::GraphObligationSupportMatrixRow => {
            "graph-obligation-support-matrix-row"
        }
        ForgeQueryEvidenceScope::GraphObligationSupportMatrix => "graph-obligation-support-matrix",
        ForgeQueryEvidenceScope::GraphObligationTouchSelector => "graph-obligation-touch-selector",
        ForgeQueryEvidenceScope::GraphObligationOperatingWorldSelector => {
            "graph-obligation-operating-world-selector"
        }
        ForgeQueryEvidenceScope::GraphObligationOperatingWorldDescriptor => {
            "graph-obligation-operating-world-descriptor"
        }
        ForgeQueryEvidenceScope::GraphObligationSupportPosture => {
            "graph-obligation-support-posture"
        }
        ForgeQueryEvidenceScope::GraphObligationRegistration => "graph-obligation-registration",
        ForgeQueryEvidenceScope::GraphObligationRegistrationCatalog => {
            "graph-obligation-registration-catalog"
        }
        ForgeQueryEvidenceScope::GraphObligationIndex => "graph-obligation-index",
        ForgeQueryEvidenceScope::GraphObligationIndexEntry => "graph-obligation-index-entry",
        ForgeQueryEvidenceScope::GraphObligationIndexComplexityContract => {
            "graph-obligation-index-complexity-contract"
        }
        ForgeQueryEvidenceScope::GraphObligationIndexBuildCounters => {
            "graph-obligation-index-build-counters"
        }
        ForgeQueryEvidenceScope::GraphObligationSelection => "graph-obligation-selection",
        ForgeQueryEvidenceScope::GraphObligationSelectionCounters => {
            "graph-obligation-selection-counters"
        }
        ForgeQueryEvidenceScope::GraphObligationIndexSupportRow => {
            "graph-obligation-index-support-row"
        }
        ForgeQueryEvidenceScope::ReadDomainInvariantDenial => "read-domain-invariant-denial",
        ForgeQueryEvidenceScope::ReadInvariantViolation => "read-invariant-violation",
        ForgeQueryEvidenceScope::ApplicationSupportSectionPosture => {
            "application-support-section-posture"
        }
        ForgeQueryEvidenceScope::ApplicationSupportReport => "application-support-report",
        ForgeQueryEvidenceScope::ApplicationEvidenceIdentityBoundaryClosure => {
            "application-evidence-identity-boundary-closure"
        }
        ForgeQueryEvidenceScope::ApplicationStopClassBoundaryClosure => {
            "application-stop-class-boundary-closure"
        }
        ForgeQueryEvidenceScope::ApplicationSessionLabelBoundaryClosure => {
            "application-session-label-boundary-closure"
        }
        ForgeQueryEvidenceScope::ApplicationIdentityBoundaryClosure => {
            "application-identity-boundary-closure"
        }
        ForgeQueryEvidenceScope::ApplicationConsumerKitFamilyClosure => {
            "application-consumer-kit-family-closure"
        }
        ForgeQueryEvidenceScope::ApplicationConsumerKitHostileCertification => {
            "application-consumer-kit-hostile-certification"
        }
        ForgeQueryEvidenceScope::ApplicationConsumerKitReferenceResidue => {
            "application-consumer-kit-reference-residue"
        }
        ForgeQueryEvidenceScope::ApplicationConsumerKitClosure => {
            "application-consumer-kit-closure"
        }
        _ => unreachable!("graph/application scope helper called with unrelated scope"),
    }
}
