#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryConsumerResidueClass {
    RuntimeSchemaAdapter,
    RuntimeSourceAdapter,
    RuntimeWriteAuthorityAdapter,
    RuntimeSignalSinkAdapter,
    RuntimeSnapshotIdentityAdapter,
    RuntimeSubscriptionActivationAdapter,
    RuntimePreviewBasisAdapter,
    RuntimeInspectorEvidenceAdapter,
    RuntimeBridgeHandAssembly,
    FabricatedMutationReceipt,
    FabricatedBridgeMutationReceipt,
    FabricatedWriteAuthorityReceipt,
    LocalQueryReport,
    LocalQueryProof,
    RawSupportSnapshotRow,
    SupportMatrixRowSearch,
    DebugDerivedQueryProof,
    DelimiterJoinedQueryProof,
    DelimiterFormattedQueryProof,
    DecomposedProjectionConsumptionAttempt,
    IndependentlyPairableProjectionConsumptionParts,
    LegacyProjectionFactConsumptionCall,
    LegacyProjectionDeclarationCall,
    LegacyProjectionIntentAdmissionCall,
    LocalQueryMeasurementConsumptionIdentity,
    LocalProjectionContractBinding,
    LocalQueryBasisDigestCompatibility,
    LegacyProjectionPrerequisiteAssembly,
    DirectInternalQueryImport,
    LegacyQueryBasisLifecycle,
    RawDomainStringAuthority,
    ConsumerAuthoredContextDigest,
    ApplicationFacadeDomainAuthority,
    IndependentOperationRegistry,
    CallerSuppliedOperationRegistry,
    QueryPhaseMaterializerImport,
    ConsumerSemanticDomainAdapter,
    LocalQueryOperationRegistry,
    LocalQueryDependencyGraph,
    LocalQueryRecomputePolicy,
    LocalQuerySharingRegistry,
    LocalQuerySupportMirror,
    LocalQueryInvalidationMirror,
    LocalQueryBasisCompatibilityMirror,
    LocalQueryPatchPosture,
    RawChangeDataCaptureInterpretation,
    RawSignalImport,
    RawRuntimeBridgeImport,
    LocalQueryConditionEvaluator,
    OrphanQueryLifecycleJoin,
}

impl WorthQueryConsumerResidueClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeSchemaAdapter => "runtime-schema-adapter",
            Self::RuntimeSourceAdapter => "runtime-source-adapter",
            Self::RuntimeWriteAuthorityAdapter => "runtime-write-authority-adapter",
            Self::RuntimeSignalSinkAdapter => "runtime-signal-sink-adapter",
            Self::RuntimeSnapshotIdentityAdapter => "runtime-snapshot-identity-adapter",
            Self::RuntimeSubscriptionActivationAdapter => "runtime-subscription-activation-adapter",
            Self::RuntimePreviewBasisAdapter => "runtime-preview-basis-adapter",
            Self::RuntimeInspectorEvidenceAdapter => "runtime-inspector-evidence-adapter",
            Self::RuntimeBridgeHandAssembly => "runtime-bridge-hand-assembly",
            Self::FabricatedMutationReceipt => "fabricated-mutation-receipt",
            Self::FabricatedBridgeMutationReceipt => "fabricated-bridge-mutation-receipt",
            Self::FabricatedWriteAuthorityReceipt => "fabricated-write-authority-receipt",
            Self::LocalQueryReport => "local-query-report",
            Self::LocalQueryProof => "local-query-proof",
            Self::RawSupportSnapshotRow => "raw-support-snapshot-row",
            Self::SupportMatrixRowSearch => "support-matrix-row-search",
            Self::DebugDerivedQueryProof => "debug-derived-query-proof",
            Self::DelimiterJoinedQueryProof => "delimiter-joined-query-proof",
            Self::DelimiterFormattedQueryProof => "delimiter-formatted-query-proof",
            Self::DecomposedProjectionConsumptionAttempt => {
                "decomposed-projection-consumption-attempt"
            }
            Self::IndependentlyPairableProjectionConsumptionParts => {
                "independently-pairable-projection-consumption-parts"
            }
            Self::LegacyProjectionFactConsumptionCall => "legacy-projection-fact-consumption-call",
            Self::LegacyProjectionDeclarationCall => "legacy-projection-declaration-call",
            Self::LegacyProjectionIntentAdmissionCall => "legacy-projection-intent-admission-call",
            Self::LocalQueryMeasurementConsumptionIdentity => {
                "local-query-measurement-consumption-identity"
            }
            Self::LocalProjectionContractBinding => "local-projection-contract-binding",
            Self::LocalQueryBasisDigestCompatibility => "local-query-basis-digest-compatibility",
            Self::LegacyProjectionPrerequisiteAssembly => "legacy-projection-prerequisite-assembly",
            Self::DirectInternalQueryImport => "direct-internal-query-import",
            Self::LegacyQueryBasisLifecycle => "legacy-query-basis-lifecycle",
            Self::RawDomainStringAuthority => "raw-domain-string-authority",
            Self::ConsumerAuthoredContextDigest => "consumer-authored-context-digest",
            Self::ApplicationFacadeDomainAuthority => "application-facade-domain-authority",
            Self::IndependentOperationRegistry => "independent-operation-registry",
            Self::CallerSuppliedOperationRegistry => "caller-supplied-operation-registry",
            Self::QueryPhaseMaterializerImport => "query-phase-materializer-import",
            Self::ConsumerSemanticDomainAdapter => "consumer-semantic-domain-adapter",
            Self::LocalQueryOperationRegistry => "local-query-operation-registry",
            Self::LocalQueryDependencyGraph => "local-query-dependency-graph",
            Self::LocalQueryRecomputePolicy => "local-query-recompute-policy",
            Self::LocalQuerySharingRegistry => "local-query-sharing-registry",
            Self::LocalQuerySupportMirror => "local-query-support-mirror",
            Self::LocalQueryInvalidationMirror => "local-query-invalidation-mirror",
            Self::LocalQueryBasisCompatibilityMirror => "local-query-basis-compatibility-mirror",
            Self::LocalQueryPatchPosture => "local-query-patch-posture",
            Self::RawChangeDataCaptureInterpretation => "raw-change-data-capture-interpretation",
            Self::RawSignalImport => "raw-signal-import",
            Self::RawRuntimeBridgeImport => "raw-runtime-bridge-import",
            Self::LocalQueryConditionEvaluator => "local-query-condition-evaluator",
            Self::OrphanQueryLifecycleJoin => "orphan-query-lifecycle-join",
        }
    }

    pub(crate) fn is_test_backend_residue(self) -> bool {
        matches!(
            self,
            Self::RuntimeSchemaAdapter
                | Self::RuntimeSourceAdapter
                | Self::RuntimeWriteAuthorityAdapter
                | Self::RuntimeSignalSinkAdapter
                | Self::RuntimeSnapshotIdentityAdapter
                | Self::RuntimeSubscriptionActivationAdapter
                | Self::RuntimePreviewBasisAdapter
                | Self::RuntimeInspectorEvidenceAdapter
                | Self::RuntimeBridgeHandAssembly
                | Self::FabricatedMutationReceipt
                | Self::FabricatedBridgeMutationReceipt
                | Self::FabricatedWriteAuthorityReceipt
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConsumerResidueDetection {
    ExactText,
    Ast,
}

impl WorthQueryConsumerResidueDetection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExactText => "exact-text",
            Self::Ast => "ast",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerResidueRegistryRow {
    class: WorthQueryConsumerResidueClass,
    detection: WorthQueryConsumerResidueDetection,
    detection_key: &'static str,
    explanation: &'static str,
    replacement_lane: &'static str,
}

impl WorthQueryConsumerResidueRegistryRow {
    pub(crate) const fn new(
        class: WorthQueryConsumerResidueClass,
        detection: WorthQueryConsumerResidueDetection,
        detection_key: &'static str,
        explanation: &'static str,
        replacement_lane: &'static str,
    ) -> Self {
        Self {
            class,
            detection,
            detection_key,
            explanation,
            replacement_lane,
        }
    }

    pub fn class(&self) -> WorthQueryConsumerResidueClass {
        self.class
    }

    pub fn detection(&self) -> WorthQueryConsumerResidueDetection {
        self.detection
    }

    pub fn detection_key(&self) -> &'static str {
        self.detection_key
    }

    pub fn explanation(&self) -> &'static str {
        self.explanation
    }

    pub fn replacement_lane(&self) -> &'static str {
        self.replacement_lane
    }
}
