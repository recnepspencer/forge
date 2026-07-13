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

pub fn worth_query_consumer_residue_registry() -> &'static [WorthQueryConsumerResidueRegistryRow] {
    CONSUMER_RESIDUE_REGISTRY
}

pub fn worth_query_test_backend_residue_classes() -> Vec<WorthQueryConsumerResidueClass> {
    CONSUMER_RESIDUE_REGISTRY
        .iter()
        .map(WorthQueryConsumerResidueRegistryRow::class)
        .filter(|class| class.is_test_backend_residue())
        .collect()
}

pub(crate) fn registry_row_for_class(
    class: WorthQueryConsumerResidueClass,
) -> &'static WorthQueryConsumerResidueRegistryRow {
    CONSUMER_RESIDUE_REGISTRY
        .iter()
        .find(|row| row.class() == class)
        .expect("every consumer residue class must have a registry row")
}

const CONSUMER_RESIDUE_REGISTRY: &[WorthQueryConsumerResidueRegistryRow] = &[
    registry_row(
        WorthQueryConsumerResidueClass::RuntimeSchemaAdapter,
        WorthQueryConsumerResidueDetection::ExactText,
        "impl WorthQueryRuntimeSchemaAdapter",
        "consumer reimplements the runtime schema adapter",
        "in-memory-test-backend",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::RuntimeSourceAdapter,
        WorthQueryConsumerResidueDetection::ExactText,
        "impl WorthQueryRuntimeSourceAdapter",
        "consumer reimplements the runtime source adapter",
        "in-memory-test-backend",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::RuntimeWriteAuthorityAdapter,
        WorthQueryConsumerResidueDetection::ExactText,
        "impl WorthQueryRuntimeWriteAuthorityAdapter",
        "consumer reimplements write authority instead of using the kit",
        "in-memory-test-backend",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::RuntimeSignalSinkAdapter,
        WorthQueryConsumerResidueDetection::ExactText,
        "impl WorthQueryRuntimeSignalSinkAdapter",
        "consumer reimplements the runtime signal sink adapter",
        "in-memory-test-backend",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::RuntimeSnapshotIdentityAdapter,
        WorthQueryConsumerResidueDetection::ExactText,
        "impl WorthQueryRuntimeSnapshotIdentityAdapter",
        "consumer reimplements runtime snapshot identity",
        "in-memory-test-backend",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::RuntimeSubscriptionActivationAdapter,
        WorthQueryConsumerResidueDetection::ExactText,
        "impl WorthQueryRuntimeSubscriptionActivationAdapter",
        "consumer reimplements subscription activation",
        "in-memory-test-backend",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::RuntimePreviewBasisAdapter,
        WorthQueryConsumerResidueDetection::ExactText,
        "impl WorthQueryRuntimePreviewBasisAdapter",
        "consumer reimplements preview basis support",
        "in-memory-test-backend",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::RuntimeInspectorEvidenceAdapter,
        WorthQueryConsumerResidueDetection::ExactText,
        "impl WorthQueryRuntimeInspectorEvidenceAdapter",
        "consumer reimplements inspector evidence support",
        "in-memory-test-backend",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::RuntimeBridgeHandAssembly,
        WorthQueryConsumerResidueDetection::ExactText,
        "RuntimeBridge::",
        "consumer hand-assembles runtime bridge internals",
        "in-memory-test-backend",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::FabricatedMutationReceipt,
        WorthQueryConsumerResidueDetection::ExactText,
        "WorthQueryMutationReceipt::from_authoritative_parts",
        "consumer fabricates mutation receipts",
        "in-memory-test-backend",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::FabricatedBridgeMutationReceipt,
        WorthQueryConsumerResidueDetection::ExactText,
        "WorthQueryMutationReceipt::from_bridge_authoritative_parts",
        "consumer fabricates bridge mutation receipts",
        "in-memory-test-backend",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::FabricatedWriteAuthorityReceipt,
        WorthQueryConsumerResidueDetection::ExactText,
        "WriteAuthorityExecutionReceipt",
        "consumer fabricates write authority receipts",
        "in-memory-test-backend",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::LocalQueryReport,
        WorthQueryConsumerResidueDetection::Ast,
        "local-query-report-struct",
        "consumer defines a local Query report instead of sealed evidence",
        "evidence-report-kit",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::LocalQueryProof,
        WorthQueryConsumerResidueDetection::Ast,
        "local-query-proof-struct",
        "consumer defines a local Query proof instead of sealed evidence",
        "evidence-report-kit",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::RawSupportSnapshotRow,
        WorthQueryConsumerResidueDetection::Ast,
        "WorthQuerySupportSnapshotRow",
        "consumer treats raw support rows as proof",
        "support-pinning",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::SupportMatrixRowSearch,
        WorthQueryConsumerResidueDetection::Ast,
        "row_for_family",
        "consumer searches support matrix rows instead of pinning support",
        "support-pinning",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::DebugDerivedQueryProof,
        WorthQueryConsumerResidueDetection::Ast,
        "format-debug-query-proof",
        "consumer derives Query proof from debug text",
        "evidence-report-kit",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::DelimiterJoinedQueryProof,
        WorthQueryConsumerResidueDetection::Ast,
        "delimiter-joined-query-proof",
        "consumer derives Query proof from delimiter-joined strings",
        "evidence-report-kit",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::DelimiterFormattedQueryProof,
        WorthQueryConsumerResidueDetection::Ast,
        "delimiter-formatted-query-proof",
        "consumer derives Query proof from delimiter-formatted strings",
        "evidence-report-kit",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::DecomposedProjectionConsumptionAttempt,
        WorthQueryConsumerResidueDetection::ExactText,
        "ProjectionFactConsumptionAttempt",
        "consumer accepts a decomposed attempt instead of Query's sealed authority outcome",
        "downstream-authority-adoption",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::IndependentlyPairableProjectionConsumptionParts,
        WorthQueryConsumerResidueDetection::ExactText,
        "CompletedProjectionFactConsumption",
        "consumer retains completed consumption parts that can be paired outside Query authority",
        "downstream-authority-adoption",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::LegacyProjectionFactConsumptionCall,
        WorthQueryConsumerResidueDetection::ExactText,
        ".consume_projection_facts(",
        "consumer invokes the decomposed fact-consumption lifecycle instead of requesting sealed authority",
        "downstream-authority-adoption",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::LegacyProjectionDeclarationCall,
        WorthQueryConsumerResidueDetection::ExactText,
        ".declare_projection_fact_consumption(",
        "consumer authors the intermediate projection lifecycle outside Query authority",
        "downstream-authority-adoption",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::LegacyProjectionIntentAdmissionCall,
        WorthQueryConsumerResidueDetection::ExactText,
        "worth_query_projection_consumption_intent(",
        "consumer routes through the retired projection-consumption intent instead of requesting sealed authority",
        "downstream-authority-adoption",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::LocalQueryMeasurementConsumptionIdentity,
        WorthQueryConsumerResidueDetection::ExactText,
        "WorthUiQueryMeasurementConsumptionIdentity",
        "consumer mints a local mirror of Query consumption identity",
        "downstream-authority-adoption",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::LocalProjectionContractBinding,
        WorthQueryConsumerResidueDetection::ExactText,
        "bind_projection_contract(",
        "consumer binds a decomposed projection contract instead of retaining Query authority",
        "downstream-authority-adoption",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::LocalQueryBasisDigestCompatibility,
        WorthQueryConsumerResidueDetection::ExactText,
        "contract.basis_digest() != Some(",
        "consumer reopens Query basis compatibility through a reporting digest",
        "downstream-authority-adoption",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::LegacyProjectionPrerequisiteAssembly,
        WorthQueryConsumerResidueDetection::ExactText,
        "with_query_prerequisites_from_projection_consumption",
        "consumer reconstructs prerequisites from decomposed projection consumption",
        "downstream-authority-adoption",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::DirectInternalQueryImport,
        WorthQueryConsumerResidueDetection::ExactText,
        "worth_query::projection_consumption",
        "consumer imports Query implementation topology instead of the curated facade",
        "downstream-authority-adoption",
    ),
];

const fn registry_row(
    class: WorthQueryConsumerResidueClass,
    detection: WorthQueryConsumerResidueDetection,
    detection_key: &'static str,
    explanation: &'static str,
    replacement_lane: &'static str,
) -> WorthQueryConsumerResidueRegistryRow {
    WorthQueryConsumerResidueRegistryRow::new(
        class,
        detection,
        detection_key,
        explanation,
        replacement_lane,
    )
}
