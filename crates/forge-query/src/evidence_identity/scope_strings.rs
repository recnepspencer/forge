use super::consumer_scope_strings::consumer_kit_evidence_scope_as_str;
use super::graph_application_scope_strings::graph_application_evidence_scope_as_str;
use super::scope::ForgeQueryEvidenceScope;

pub(crate) fn evidence_scope_as_str(scope: ForgeQueryEvidenceScope) -> &'static str {
    match scope {
        ForgeQueryEvidenceScope::RuntimePublicSupportMatrixRow => {
            "runtime-public-support-matrix-row"
        }
        ForgeQueryEvidenceScope::RuntimePublicSupportMatrix => "runtime-public-support-matrix",
        ForgeQueryEvidenceScope::RuntimePublicApiFamilyContract => {
            "runtime-public-api-family-contract"
        }
        ForgeQueryEvidenceScope::RuntimePublicApiContract => "runtime-public-api-contract",
        ForgeQueryEvidenceScope::RuntimePublicApiTranscriptEvidence => {
            "runtime-public-api-transcript-evidence"
        }
        ForgeQueryEvidenceScope::RuntimeSubscriptionBudget => "runtime-subscription-budget",
        ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact => {
            "runtime-hostile-certification-artifact"
        }
        ForgeQueryEvidenceScope::RuntimeStateSnapshot => "runtime-state-snapshot",
        ForgeQueryEvidenceScope::WorkflowContextBinding => "workflow-context-binding",
        ForgeQueryEvidenceScope::WorkflowMutationLowering => "workflow-mutation-lowering",
        ForgeQueryEvidenceScope::SubscriptionActivationReceipt => "subscription-activation-receipt",
        ForgeQueryEvidenceScope::SignalInvalidationRoutingReceipt => {
            "signal-invalidation-routing-receipt"
        }
        ForgeQueryEvidenceScope::LowerRuntimeCapabilitySubject => {
            "lower-runtime-capability-subject"
        }
        ForgeQueryEvidenceScope::LowerRuntimeRouteSubject => "lower-runtime-route-subject",
        ForgeQueryEvidenceScope::LowerRuntimeCapabilityRequest => {
            "lower-runtime-capability-request"
        }
        ForgeQueryEvidenceScope::LowerRuntimeCapabilityEligibility => {
            "lower-runtime-capability-eligibility"
        }
        ForgeQueryEvidenceScope::LowerRuntimeRoutePlan => "lower-runtime-route-plan",
        ForgeQueryEvidenceScope::PreviewPromotionContinuation => "preview-promotion-continuation",
        ForgeQueryEvidenceScope::LowerRuntimeReadmissionReceipt => {
            "lower-runtime-readmission-receipt"
        }
        ForgeQueryEvidenceScope::LowerRuntimeBoundaryExecutionReceipt => {
            "lower-runtime-boundary-execution-receipt"
        }
        ForgeQueryEvidenceScope::LowerRuntimeBoundaryAuthority => {
            "lower-runtime-boundary-authority"
        }
        ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence => "lower-runtime-boundary-evidence",
        ForgeQueryEvidenceScope::LowerRuntimeBoundaryEnvelope => "lower-runtime-boundary-envelope",
        ForgeQueryEvidenceScope::DeclarationBridgeRoutingDigest => {
            "declaration-bridge-routing-digest"
        }
        ForgeQueryEvidenceScope::DeclarationBridgeLoweringIdentity => {
            "declaration-bridge-lowering-identity"
        }
        ForgeQueryEvidenceScope::ContinuationExecutionReadmissionEvidence => {
            "continuation-execution-readmission-evidence"
        }
        ForgeQueryEvidenceScope::ContinuationLinkedArtifacts => "continuation-linked-artifacts",
        ForgeQueryEvidenceScope::ContinuationPreparedDigest => "continuation-prepared-digest",
        ForgeQueryEvidenceScope::ContinuationExecutionTranscript => {
            "continuation-execution-transcript"
        }
        ForgeQueryEvidenceScope::ContinuationExecutionDigest => "continuation-execution-digest",
        ForgeQueryEvidenceScope::ViewShapePlanDigest => "view-shape-plan-digest",
        ForgeQueryEvidenceScope::BasisDigest => "basis-digest",
        ForgeQueryEvidenceScope::BridgeGroupedTruthViewDigest => "bridge-grouped-truth-view-digest",
        ForgeQueryEvidenceScope::ReadGraphDigest => "read-graph-digest",
        ForgeQueryEvidenceScope::SessionLabelIdentity => "session-label-identity",
        ForgeQueryEvidenceScope::ResolvedSnapshotBasis => "resolved-snapshot-basis",
        ForgeQueryEvidenceScope::BasisAdmissionEvidenceRow => "basis-admission-evidence-row",
        ForgeQueryEvidenceScope::PreviewBasisAdmission => "preview-basis-admission",
        ForgeQueryEvidenceScope::BranchBasisAdmission => "branch-basis-admission",
        ForgeQueryEvidenceScope::RawBasisIntent => "raw-basis-intent",
        ForgeQueryEvidenceScope::QueryContextCompatibilityBasisLabel => {
            "query-context-compatibility-basis-label"
        }
        ForgeQueryEvidenceScope::BridgeLowerRuntimeEvidenceReference => {
            "bridge-lower-runtime-evidence-reference"
        }
        ForgeQueryEvidenceScope::BridgeLowerRuntimeBasisBinding => {
            "bridge-lower-runtime-basis-binding"
        }
        ForgeQueryEvidenceScope::ContinuationReadmissionBasis => "continuation-readmission-basis",
        ForgeQueryEvidenceScope::ContinuationReadmissionLowerRuntimeBinding => {
            "continuation-readmission-lower-runtime-binding"
        }
        ForgeQueryEvidenceScope::ContinuationReadmissionSourceBasis => {
            "continuation-readmission-source-basis"
        }
        ForgeQueryEvidenceScope::SharedReadGeneration => "shared-read-generation",
        ForgeQueryEvidenceScope::PreviewIntentAdmission => "preview-intent-admission",
        ForgeQueryEvidenceScope::PreviewIntentReceipt => "preview-intent-receipt",
        ForgeQueryEvidenceScope::IntentExecutionProvenanceChain => {
            "intent-execution-provenance-chain"
        }
        ForgeQueryEvidenceScope::AuthoritativeIntentReceipt => "authoritative-intent-receipt",
        ForgeQueryEvidenceScope::EffectIntentReceipt => "effect-intent-receipt",
        ForgeQueryEvidenceScope::WriteReceiptCommitIdentity => "write-receipt-commit-identity",
        ForgeQueryEvidenceScope::JournalPositionIdentity => "journal-position-identity",
        ForgeQueryEvidenceScope::JournalSegmentIdentity => "journal-segment-identity",
        ForgeQueryEvidenceScope::JournalReplayOutcome => "journal-replay-outcome",
        ForgeQueryEvidenceScope::WriteReceiptSnapshotIdentity => "write-receipt-snapshot-identity",
        ForgeQueryEvidenceScope::WriteReceiptEntityIdentity => "write-receipt-entity-identity",
        ForgeQueryEvidenceScope::AuthoredCommandEntityIdentity => {
            "authored-command-entity-identity"
        }
        ForgeQueryEvidenceScope::ExistingTruthResolvedTargetIdentity => {
            "existing-truth-resolved-target-identity"
        }
        ForgeQueryEvidenceScope::ProjectionConsumptionIdentity => "projection-consumption-identity",
        ForgeQueryEvidenceScope::ProjectionConsumptionCertificationIdentity => {
            "projection-consumption-certification-identity"
        }
        ForgeQueryEvidenceScope::DomainCapabilityIdentity => "domain-capability-identity",
        ForgeQueryEvidenceScope::DomainCapabilityCertificationIdentity => {
            "domain-capability-certification-identity"
        }
        ForgeQueryEvidenceScope::ProjectionConsumedContinuityAuthorityIdentity => {
            "projection-consumed-continuity-authority-identity"
        }
        ForgeQueryEvidenceScope::RuntimeBridgeWritebackAuthority => {
            "runtime-bridge-writeback-authority"
        }
        ForgeQueryEvidenceScope::MutationEvidenceAuthorityIdentity => {
            "mutation-evidence-authority-identity"
        }
        ForgeQueryEvidenceScope::MutationEvidenceTargetCollectionIdentity => {
            "mutation-evidence-target-collection-identity"
        }
        ForgeQueryEvidenceScope::MutationEvidenceSymbolIdentity => {
            "mutation-evidence-symbol-identity"
        }
        ForgeQueryEvidenceScope::MutationEvidenceSourceDigest => "mutation-evidence-source-digest",
        ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest => {
            "mutation-evidence-aggregate-digest"
        }
        ForgeQueryEvidenceScope::EffectTriggerCommitIdentity => "effect-trigger-commit-identity",
        ForgeQueryEvidenceScope::PreviewIntentBasisEvidence => "preview-intent-basis-evidence",
        ForgeQueryEvidenceScope::PreviewIntentReceiptInspectionBasis => {
            "preview-intent-receipt-inspection-basis"
        }
        ForgeQueryEvidenceScope::PreviewIntentReceiptInspection => {
            "preview-intent-receipt-inspection"
        }
        ForgeQueryEvidenceScope::IntentInspectionDeliveryCounters => {
            "intent-inspection-delivery-counters"
        }
        ForgeQueryEvidenceScope::IntentReceiptInspection => "intent-receipt-inspection",
        ForgeQueryEvidenceScope::IntentDenialInspection => "intent-denial-inspection",
        ForgeQueryEvidenceScope::EffectIntentReceiptPhase => "effect-intent-receipt-phase",
        ForgeQueryEvidenceScope::EffectIntentReceiptInspection => {
            "effect-intent-receipt-inspection"
        }
        ForgeQueryEvidenceScope::FeedbackPhaseGraph => "feedback-phase-graph",
        ForgeQueryEvidenceScope::FeedbackPhaseGraphInspection => "feedback-phase-graph-inspection",
        ForgeQueryEvidenceScope::BranchIntentReceiptInspectionBasis => {
            "branch-intent-receipt-inspection-basis"
        }
        ForgeQueryEvidenceScope::BranchIntentReceiptInspection => {
            "branch-intent-receipt-inspection"
        }
        ForgeQueryEvidenceScope::GenericInspectionIntentSeed => "generic-inspection-intent-seed",
        ForgeQueryEvidenceScope::AuthoritativeMutationIntentSeed => {
            "authoritative-mutation-intent-seed"
        }
        ForgeQueryEvidenceScope::AuthoritativeMutationBatchIntentSeed => {
            "authoritative-mutation-batch-intent-seed"
        }
        ForgeQueryEvidenceScope::AuthoritativeMutationExecutionHandoff => {
            "authoritative-mutation-execution-handoff"
        }
        ForgeQueryEvidenceScope::BranchIntentAdmission => "branch-intent-admission",
        ForgeQueryEvidenceScope::BranchIntentReceipt => "branch-intent-receipt",
        ForgeQueryEvidenceScope::IntentDenialEvidence => "intent-denial-evidence",
        ForgeQueryEvidenceScope::IntentExecutionFailureEvidence => {
            "intent-execution-failure-evidence"
        }
        ForgeQueryEvidenceScope::PreviewCloseoutEvidence => "preview-closeout-evidence",
        ForgeQueryEvidenceScope::PreviewPromotionDenialEvidence => {
            "preview-promotion-denial-evidence"
        }
        ForgeQueryEvidenceScope::PreviewExecutionEvidence => "preview-execution-evidence",
        ForgeQueryEvidenceScope::PreviewPromotionRebinding => "preview-promotion-rebinding",
        ForgeQueryEvidenceScope::PreviewWriteReceiptIdentity => "preview-write-receipt-identity",
        ForgeQueryEvidenceScope::WriteReceiptInspectionArtifact => {
            "write-receipt-inspection-artifact"
        }
        ForgeQueryEvidenceScope::WriteReceiptDeclaredAspectOperation => {
            "write-receipt-declared-aspect-operation"
        }
        ForgeQueryEvidenceScope::WriteReceiptMutationMetadataEntry => {
            "write-receipt-mutation-metadata-entry"
        }
        ForgeQueryEvidenceScope::BatchWriteReceipt => "batch-write-receipt",
        ForgeQueryEvidenceScope::BatchWriteReceiptInspectionArtifact => {
            "batch-write-receipt-inspection-artifact"
        }
        ForgeQueryEvidenceScope::BatchWriteReceiptComponent => "batch-write-receipt-component",
        ForgeQueryEvidenceScope::BatchWriteReceiptSymbolicAspectResolution => {
            "batch-write-receipt-symbolic-aspect-resolution"
        }
        ForgeQueryEvidenceScope::BatchWriteReceiptGraphResolution => {
            "batch-write-receipt-graph-resolution"
        }
        ForgeQueryEvidenceScope::RetainedExistingTruthAssertionEvidence => {
            "retained-existing-truth-assertion-evidence"
        }
        ForgeQueryEvidenceScope::LiveArtifactBundle => "live-artifact-bundle",
        ForgeQueryEvidenceScope::GroupedExecutionSurfaceArtifact => {
            "grouped-execution-surface-artifact"
        }
        ForgeQueryEvidenceScope::DerivedMaterializationBundle => "derived-materialization-bundle",
        ForgeQueryEvidenceScope::PreviewBindingInspectionArtifact => {
            "preview-binding-inspection-artifact"
        }
        ForgeQueryEvidenceScope::PreviewOutcomeInspectionArtifact => {
            "preview-outcome-inspection-artifact"
        }
        ForgeQueryEvidenceScope::CausalObservationReceipt => "causal-observation-receipt",
        ForgeQueryEvidenceScope::CausalObservationQuery => "causal-observation-query",
        ForgeQueryEvidenceScope::CausalObservationBasis => "causal-observation-basis",
        ForgeQueryEvidenceScope::CausalObservationTarget => "causal-observation-target",
        ForgeQueryEvidenceScope::CausalResultShapeContext => "causal-result-shape-context",
        ForgeQueryEvidenceScope::CausalQueryObservationReceipt => {
            "causal-query-observation-receipt"
        }
        ForgeQueryEvidenceScope::CausalObservationAnchor => "causal-observation-anchor",
        ForgeQueryEvidenceScope::CausalObservationAnchorCounters => {
            "causal-observation-anchor-counters"
        }
        ForgeQueryEvidenceScope::CausalObservationAnchorFailure => {
            "causal-observation-anchor-failure"
        }
        ForgeQueryEvidenceScope::CausalEvidenceReference => "causal-evidence-reference",
        ForgeQueryEvidenceScope::CausalEvidenceReferenceReceipt => {
            "causal-evidence-reference-receipt"
        }
        ForgeQueryEvidenceScope::CausalEvidenceReferenceResolutionCounters => {
            "causal-evidence-reference-resolution-counters"
        }
        ForgeQueryEvidenceScope::CausalEvidenceReferenceResolutionDenial => {
            "causal-evidence-reference-resolution-denial"
        }
        ForgeQueryEvidenceScope::CausalEvidenceReferenceIndex => "causal-evidence-reference-index",
        ForgeQueryEvidenceScope::CausalEvidenceReferenceIndexRecord => {
            "causal-evidence-reference-index-record"
        }
        ForgeQueryEvidenceScope::CausalEvidenceReferenceIndexError => {
            "causal-evidence-reference-index-error"
        }
        ForgeQueryEvidenceScope::CausalInspectionTarget => "causal-inspection-target",
        ForgeQueryEvidenceScope::CausalInspectionRequest => "causal-inspection-request",
        ForgeQueryEvidenceScope::CausalInspectionRequestFailure => {
            "causal-inspection-request-failure"
        }
        ForgeQueryEvidenceScope::CausalInspectionAdmissionSubject => {
            "causal-inspection-admission-subject"
        }
        ForgeQueryEvidenceScope::CausalInspectionAdmissionDecision => {
            "causal-inspection-admission-decision"
        }
        ForgeQueryEvidenceScope::CausalInspectionDecisionTraceRow => {
            "causal-inspection-decision-trace-row"
        }
        ForgeQueryEvidenceScope::CausalInspectionDecisionTraceIndex => {
            "causal-inspection-decision-trace-index"
        }
        ForgeQueryEvidenceScope::CausalInspectionAdmissionCounters => {
            "causal-inspection-admission-counters"
        }
        ForgeQueryEvidenceScope::CausalInspectionAdmissionReceipt => {
            "causal-inspection-admission-receipt"
        }
        ForgeQueryEvidenceScope::CausalInspectionOutcome => "causal-inspection-outcome",
        ForgeQueryEvidenceScope::CausalInspectionMaterializedDetail => {
            "causal-inspection-materialized-detail"
        }
        ForgeQueryEvidenceScope::CausalInspectionDeniedArtifactDetail => {
            "causal-inspection-denied-artifact-detail"
        }
        ForgeQueryEvidenceScope::CausalInspectionArtifact => "causal-inspection-artifact",
        ForgeQueryEvidenceScope::CausalInspectionArtifactIdentity => {
            "causal-inspection-artifact-identity"
        }
        ForgeQueryEvidenceScope::CausalInspectionPerformanceSnapshot => {
            "causal-inspection-performance-snapshot"
        }
        ForgeQueryEvidenceScope::CausalInspectionPerformanceSlope => {
            "causal-inspection-performance-slope"
        }
        ForgeQueryEvidenceScope::CausalInspectionPerformanceScaleSlope => {
            "causal-inspection-performance-scale-slope"
        }
        ForgeQueryEvidenceScope::CausalInspectionPerformanceCertificationBundle => {
            "causal-inspection-performance-certification-bundle"
        }
        ForgeQueryEvidenceScope::CausalInspectionCertificationError => {
            "causal-inspection-certification-error"
        }
        ForgeQueryEvidenceScope::CausalInspectionCertificationFailureEvidence => {
            "causal-inspection-certification-failure-evidence"
        }
        ForgeQueryEvidenceScope::RuntimePublicApiNamingRow => "runtime-public-api-naming-row",
        ForgeQueryEvidenceScope::RuntimePublicApiNamingContract => {
            "runtime-public-api-naming-contract"
        }
        ForgeQueryEvidenceScope::ConsumerEvidenceReportField
        | ForgeQueryEvidenceScope::ConsumerEvidenceReport
        | ForgeQueryEvidenceScope::ConsumerEvidenceReportFieldInventory
        | ForgeQueryEvidenceScope::ConsumerEvidenceReportDigestParticipation
        | ForgeQueryEvidenceScope::ConsumerEvidenceReportAdoptionFinding
        | ForgeQueryEvidenceScope::ConsumerEvidenceReportAdoptionResidue
        | ForgeQueryEvidenceScope::ConsumerEvidenceReportAdoptionReport
        | ForgeQueryEvidenceScope::ConsumerBoundaryAuditFinding
        | ForgeQueryEvidenceScope::ConsumerBoundaryAuditReport
        | ForgeQueryEvidenceScope::ConsumerBoundaryAuditCoverage
        | ForgeQueryEvidenceScope::ConsumerBoundaryAuditSourceInventory
        | ForgeQueryEvidenceScope::ConsumerSupportSnapshotSchema
        | ForgeQueryEvidenceScope::ConsumerSupportSnapshotRow
        | ForgeQueryEvidenceScope::ConsumerSupportSnapshotDocument
        | ForgeQueryEvidenceScope::ConsumerSupportPinContractSchema
        | ForgeQueryEvidenceScope::ConsumerSupportPinVocabulary
        | ForgeQueryEvidenceScope::ConsumerSupportPinRequirement
        | ForgeQueryEvidenceScope::ConsumerSupportPinObservedRow
        | ForgeQueryEvidenceScope::ConsumerSupportPinContract
        | ForgeQueryEvidenceScope::ConsumerSupportPinContractDocument
        | ForgeQueryEvidenceScope::ConsumerSupportPinFinding
        | ForgeQueryEvidenceScope::ConsumerSupportPinReport
        | ForgeQueryEvidenceScope::ConsumerResidueFinding
        | ForgeQueryEvidenceScope::ConsumerResidueReport
        | ForgeQueryEvidenceScope::ConsumerTestBackendResidueFinding
        | ForgeQueryEvidenceScope::ConsumerTestBackendResidueReport
        | ForgeQueryEvidenceScope::ConsumerGraphReadBypassFinding
        | ForgeQueryEvidenceScope::ConsumerGraphReadBypassReport
        | ForgeQueryEvidenceScope::ConsumerGraphReadBypassResidue => {
            consumer_kit_evidence_scope_as_str(scope)
        }
        ForgeQueryEvidenceScope::GraphCompositionDomainInvariantDenial
        | ForgeQueryEvidenceScope::GraphCompositionInvariantViolation
        | ForgeQueryEvidenceScope::GraphTouchDescriptor
        | ForgeQueryEvidenceScope::GraphTouchDescriptorRow
        | ForgeQueryEvidenceScope::GraphObligationRuleIdentity
        | ForgeQueryEvidenceScope::GraphObligationDispatchContext
        | ForgeQueryEvidenceScope::GraphObligationDispatchPlan
        | ForgeQueryEvidenceScope::GraphObligationDispatchEnvelope
        | ForgeQueryEvidenceScope::GraphObligationExecutionBudget
        | ForgeQueryEvidenceScope::GraphObligationExecutorContract
        | ForgeQueryEvidenceScope::GraphObligationExecutionInput
        | ForgeQueryEvidenceScope::GraphObligationExecutionContext
        | ForgeQueryEvidenceScope::GraphObligationStateLoadPlan
        | ForgeQueryEvidenceScope::GraphObligationStateLoadCounters
        | ForgeQueryEvidenceScope::GraphObligationExecutionResultRow
        | ForgeQueryEvidenceScope::GraphObligationExecutionResultEnvelope
        | ForgeQueryEvidenceScope::GraphObligationReduction
        | ForgeQueryEvidenceScope::GraphObligationDenialProjection
        | ForgeQueryEvidenceScope::GraphObligationDenialProjectionRow
        | ForgeQueryEvidenceScope::GraphObligationAttachmentEvidence
        | ForgeQueryEvidenceScope::GraphObligationDenialAttachmentProjection
        | ForgeQueryEvidenceScope::GraphObligationDenialAttachmentProjectionRow
        | ForgeQueryEvidenceScope::GraphObligationMaterializedDispatch
        | ForgeQueryEvidenceScope::GraphObligationSupportMatrixRow
        | ForgeQueryEvidenceScope::GraphObligationSupportMatrix
        | ForgeQueryEvidenceScope::GraphObligationTouchSelector
        | ForgeQueryEvidenceScope::GraphObligationOperatingWorldSelector
        | ForgeQueryEvidenceScope::GraphObligationOperatingWorldDescriptor
        | ForgeQueryEvidenceScope::GraphObligationSupportPosture
        | ForgeQueryEvidenceScope::GraphObligationRegistration
        | ForgeQueryEvidenceScope::GraphObligationRegistrationCatalog
        | ForgeQueryEvidenceScope::GraphObligationIndex
        | ForgeQueryEvidenceScope::GraphObligationIndexEntry
        | ForgeQueryEvidenceScope::GraphObligationIndexComplexityContract
        | ForgeQueryEvidenceScope::GraphObligationIndexBuildCounters
        | ForgeQueryEvidenceScope::GraphObligationSelection
        | ForgeQueryEvidenceScope::GraphObligationSelectionCounters
        | ForgeQueryEvidenceScope::GraphObligationIndexSupportRow
        | ForgeQueryEvidenceScope::ReadDomainInvariantDenial
        | ForgeQueryEvidenceScope::ReadInvariantViolation
        | ForgeQueryEvidenceScope::ApplicationSupportSectionPosture
        | ForgeQueryEvidenceScope::ApplicationSupportReport
        | ForgeQueryEvidenceScope::ApplicationEvidenceIdentityBoundaryClosure
        | ForgeQueryEvidenceScope::ApplicationStopClassBoundaryClosure
        | ForgeQueryEvidenceScope::ApplicationSessionLabelBoundaryClosure
        | ForgeQueryEvidenceScope::ApplicationIdentityBoundaryClosure
        | ForgeQueryEvidenceScope::ApplicationConsumerKitFamilyClosure
        | ForgeQueryEvidenceScope::ApplicationConsumerKitHostileCertification
        | ForgeQueryEvidenceScope::ApplicationConsumerKitReferenceResidue
        | ForgeQueryEvidenceScope::ApplicationConsumerKitClosure => {
            graph_application_evidence_scope_as_str(scope)
        }
    }
}
