mod model;

pub use model::{
    WorthQueryConsumerResidueClass, WorthQueryConsumerResidueDetection,
    WorthQueryConsumerResidueRegistryRow,
};

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
        "ConsumerMeasurementConsumptionIdentity",
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
    registry_row(
        WorthQueryConsumerResidueClass::LegacyQueryBasisLifecycle,
        WorthQueryConsumerResidueDetection::ExactText,
        "query_basis_lifecycle",
        "consumer imports or reconstructs the deleted parallel basis lifecycle",
        "downstream-authority-adoption",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::RawDomainStringAuthority,
        WorthQueryConsumerResidueDetection::ExactText,
        "worth_query_domain(",
        "consumer starts domain authority from an uninstalled string-authored root",
        "installed-domain-handle",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::ConsumerAuthoredContextDigest,
        WorthQueryConsumerResidueDetection::ExactText,
        "fn context_identity_digest(",
        "consumer authors a representation digest instead of typed context identity fields",
        "installed-domain-context-identity",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::ApplicationFacadeDomainAuthority,
        WorthQueryConsumerResidueDetection::ExactText,
        "WorthQueryApplicationFacade",
        "consumer retains executable domain authority outside runtime installation",
        "installed-domain-handle",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::IndependentOperationRegistry,
        WorthQueryConsumerResidueDetection::ExactText,
        "WorthQueryGraphReadOperationRegistry",
        "consumer owns an operation registry independently of installed package authority",
        "installed-domain-execution-index",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::CallerSuppliedOperationRegistry,
        WorthQueryConsumerResidueDetection::ExactText,
        "with_operation_registry(",
        "consumer injects operation authority into an ordinary execution path",
        "installed-domain-execution-index",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::QueryPhaseMaterializerImport,
        WorthQueryConsumerResidueDetection::Ast,
        "worth-query-phase-materializer-import",
        "consumer imports Query transition machinery instead of using installed capabilities",
        "installed-domain-capability",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::ConsumerSemanticDomainAdapter,
        WorthQueryConsumerResidueDetection::Ast,
        "consumer-semantic-domain-adapter",
        "consumer inserts a semantic adapter between package declaration and installed authority",
        "installed-domain-extension",
    ),
];

#[rustfmt::skip]
const fn registry_row(class: WorthQueryConsumerResidueClass, detection: WorthQueryConsumerResidueDetection, detection_key: &'static str, explanation: &'static str, replacement_lane: &'static str) -> WorthQueryConsumerResidueRegistryRow {
    WorthQueryConsumerResidueRegistryRow::new(class, detection, detection_key, explanation, replacement_lane)
}
