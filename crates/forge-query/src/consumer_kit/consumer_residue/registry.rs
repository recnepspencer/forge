#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryConsumerResidueClass {
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
}

impl ForgeQueryConsumerResidueClass {
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
pub enum ForgeQueryConsumerResidueDetection {
    ExactText,
    Ast,
}

impl ForgeQueryConsumerResidueDetection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExactText => "exact-text",
            Self::Ast => "ast",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryConsumerResidueRegistryRow {
    class: ForgeQueryConsumerResidueClass,
    detection: ForgeQueryConsumerResidueDetection,
    detection_key: &'static str,
    explanation: &'static str,
    replacement_lane: &'static str,
}

impl ForgeQueryConsumerResidueRegistryRow {
    pub(crate) const fn new(
        class: ForgeQueryConsumerResidueClass,
        detection: ForgeQueryConsumerResidueDetection,
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

    pub fn class(&self) -> ForgeQueryConsumerResidueClass {
        self.class
    }

    pub fn detection(&self) -> ForgeQueryConsumerResidueDetection {
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

pub fn forge_query_consumer_residue_registry() -> &'static [ForgeQueryConsumerResidueRegistryRow] {
    CONSUMER_RESIDUE_REGISTRY
}

pub fn forge_query_test_backend_residue_classes() -> Vec<ForgeQueryConsumerResidueClass> {
    CONSUMER_RESIDUE_REGISTRY
        .iter()
        .map(ForgeQueryConsumerResidueRegistryRow::class)
        .filter(|class| class.is_test_backend_residue())
        .collect()
}

pub(crate) fn registry_row_for_class(
    class: ForgeQueryConsumerResidueClass,
) -> &'static ForgeQueryConsumerResidueRegistryRow {
    CONSUMER_RESIDUE_REGISTRY
        .iter()
        .find(|row| row.class() == class)
        .expect("every consumer residue class must have a registry row")
}

const CONSUMER_RESIDUE_REGISTRY: &[ForgeQueryConsumerResidueRegistryRow] = &[
    registry_row(
        ForgeQueryConsumerResidueClass::RuntimeSchemaAdapter,
        ForgeQueryConsumerResidueDetection::ExactText,
        "impl ForgeQueryRuntimeSchemaAdapter",
        "consumer reimplements the runtime schema adapter",
        "in-memory-test-backend",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::RuntimeSourceAdapter,
        ForgeQueryConsumerResidueDetection::ExactText,
        "impl ForgeQueryRuntimeSourceAdapter",
        "consumer reimplements the runtime source adapter",
        "in-memory-test-backend",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::RuntimeWriteAuthorityAdapter,
        ForgeQueryConsumerResidueDetection::ExactText,
        "impl ForgeQueryRuntimeWriteAuthorityAdapter",
        "consumer reimplements write authority instead of using the kit",
        "in-memory-test-backend",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::RuntimeSignalSinkAdapter,
        ForgeQueryConsumerResidueDetection::ExactText,
        "impl ForgeQueryRuntimeSignalSinkAdapter",
        "consumer reimplements the runtime signal sink adapter",
        "in-memory-test-backend",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::RuntimeSnapshotIdentityAdapter,
        ForgeQueryConsumerResidueDetection::ExactText,
        "impl ForgeQueryRuntimeSnapshotIdentityAdapter",
        "consumer reimplements runtime snapshot identity",
        "in-memory-test-backend",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::RuntimeSubscriptionActivationAdapter,
        ForgeQueryConsumerResidueDetection::ExactText,
        "impl ForgeQueryRuntimeSubscriptionActivationAdapter",
        "consumer reimplements subscription activation",
        "in-memory-test-backend",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::RuntimePreviewBasisAdapter,
        ForgeQueryConsumerResidueDetection::ExactText,
        "impl ForgeQueryRuntimePreviewBasisAdapter",
        "consumer reimplements preview basis support",
        "in-memory-test-backend",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::RuntimeInspectorEvidenceAdapter,
        ForgeQueryConsumerResidueDetection::ExactText,
        "impl ForgeQueryRuntimeInspectorEvidenceAdapter",
        "consumer reimplements inspector evidence support",
        "in-memory-test-backend",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::RuntimeBridgeHandAssembly,
        ForgeQueryConsumerResidueDetection::ExactText,
        "RuntimeBridge::",
        "consumer hand-assembles runtime bridge internals",
        "in-memory-test-backend",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::FabricatedMutationReceipt,
        ForgeQueryConsumerResidueDetection::ExactText,
        "ForgeQueryMutationReceipt::from_authoritative_parts",
        "consumer fabricates mutation receipts",
        "in-memory-test-backend",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::FabricatedBridgeMutationReceipt,
        ForgeQueryConsumerResidueDetection::ExactText,
        "ForgeQueryMutationReceipt::from_bridge_authoritative_parts",
        "consumer fabricates bridge mutation receipts",
        "in-memory-test-backend",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::FabricatedWriteAuthorityReceipt,
        ForgeQueryConsumerResidueDetection::ExactText,
        "WriteAuthorityExecutionReceipt",
        "consumer fabricates write authority receipts",
        "in-memory-test-backend",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::LocalQueryReport,
        ForgeQueryConsumerResidueDetection::Ast,
        "local-query-report-struct",
        "consumer defines a local Query report instead of sealed evidence",
        "evidence-report-kit",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::LocalQueryProof,
        ForgeQueryConsumerResidueDetection::Ast,
        "local-query-proof-struct",
        "consumer defines a local Query proof instead of sealed evidence",
        "evidence-report-kit",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::RawSupportSnapshotRow,
        ForgeQueryConsumerResidueDetection::Ast,
        "ForgeQuerySupportSnapshotRow",
        "consumer treats raw support rows as proof",
        "support-pinning",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::SupportMatrixRowSearch,
        ForgeQueryConsumerResidueDetection::Ast,
        "row_for_family",
        "consumer searches support matrix rows instead of pinning support",
        "support-pinning",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::DebugDerivedQueryProof,
        ForgeQueryConsumerResidueDetection::Ast,
        "format-debug-query-proof",
        "consumer derives Query proof from debug text",
        "evidence-report-kit",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::DelimiterJoinedQueryProof,
        ForgeQueryConsumerResidueDetection::Ast,
        "delimiter-joined-query-proof",
        "consumer derives Query proof from delimiter-joined strings",
        "evidence-report-kit",
    ),
    registry_row(
        ForgeQueryConsumerResidueClass::DelimiterFormattedQueryProof,
        ForgeQueryConsumerResidueDetection::Ast,
        "delimiter-formatted-query-proof",
        "consumer derives Query proof from delimiter-formatted strings",
        "evidence-report-kit",
    ),
];

const fn registry_row(
    class: ForgeQueryConsumerResidueClass,
    detection: ForgeQueryConsumerResidueDetection,
    detection_key: &'static str,
    explanation: &'static str,
    replacement_lane: &'static str,
) -> ForgeQueryConsumerResidueRegistryRow {
    ForgeQueryConsumerResidueRegistryRow::new(
        class,
        detection,
        detection_key,
        explanation,
        replacement_lane,
    )
}
