use super::super::WorthQueryConsumerResidueClass;

pub(super) const HOSTILE_CERTIFICATION_SOURCES: &[(WorthQueryConsumerResidueClass, &str)] = &[
    (
        WorthQueryConsumerResidueClass::RuntimeSchemaAdapter,
        "struct Fake; impl WorthQueryRuntimeSchemaAdapter for Fake {}",
    ),
    (
        WorthQueryConsumerResidueClass::RuntimeSourceAdapter,
        "struct Fake; impl WorthQueryRuntimeSourceAdapter for Fake {}",
    ),
    (
        WorthQueryConsumerResidueClass::RuntimeWriteAuthorityAdapter,
        "struct Fake; impl WorthQueryRuntimeWriteAuthorityAdapter for Fake {}",
    ),
    (
        WorthQueryConsumerResidueClass::RuntimeSignalSinkAdapter,
        "struct Fake; impl WorthQueryRuntimeSignalSinkAdapter for Fake {}",
    ),
    (
        WorthQueryConsumerResidueClass::RuntimeSnapshotIdentityAdapter,
        "struct Fake; impl WorthQueryRuntimeSnapshotIdentityAdapter for Fake {}",
    ),
    (
        WorthQueryConsumerResidueClass::RuntimeSubscriptionActivationAdapter,
        "struct Fake; impl WorthQueryRuntimeSubscriptionActivationAdapter for Fake {}",
    ),
    (
        WorthQueryConsumerResidueClass::RuntimePreviewBasisAdapter,
        "struct Fake; impl WorthQueryRuntimePreviewBasisAdapter for Fake {}",
    ),
    (
        WorthQueryConsumerResidueClass::RuntimeInspectorEvidenceAdapter,
        "struct Fake; impl WorthQueryRuntimeInspectorEvidenceAdapter for Fake {}",
    ),
    (
        WorthQueryConsumerResidueClass::RuntimeBridgeHandAssembly,
        "fn residue() { let _ = RuntimeBridge::new(); }",
    ),
    (
        WorthQueryConsumerResidueClass::FabricatedMutationReceipt,
        "fn residue() { let _ = WorthQueryMutationReceipt::from_authoritative_parts(); }",
    ),
    (
        WorthQueryConsumerResidueClass::FabricatedBridgeMutationReceipt,
        "fn residue() { let _ = WorthQueryMutationReceipt::from_bridge_authoritative_parts(); }",
    ),
    (
        WorthQueryConsumerResidueClass::FabricatedWriteAuthorityReceipt,
        "fn residue() { let _ = WriteAuthorityExecutionReceipt; }",
    ),
    (
        WorthQueryConsumerResidueClass::LocalQueryReport,
        "struct LocalQueryReport;",
    ),
    (
        WorthQueryConsumerResidueClass::LocalQueryProof,
        "struct LocalQueryProof;",
    ),
    (
        WorthQueryConsumerResidueClass::RawSupportSnapshotRow,
        "fn residue(row: WorthQuerySupportSnapshotRow) { let _ = row; }",
    ),
    (
        WorthQueryConsumerResidueClass::SupportMatrixRowSearch,
        "fn residue(support_matrix: SupportMatrix) { let _ = support_matrix.row_for_family(\"write\"); }",
    ),
    (
        WorthQueryConsumerResidueClass::DebugDerivedQueryProof,
        "fn residue(receipt: String) { let query_proof = format!(\"{:?}\", receipt); let _ = query_proof; }",
    ),
    (
        WorthQueryConsumerResidueClass::DelimiterJoinedQueryProof,
        "fn residue(parts: Vec<String>) { let query_proof = parts.join(\"||\"); let _ = query_proof; }",
    ),
    (
        WorthQueryConsumerResidueClass::DelimiterFormattedQueryProof,
        "fn residue(left: String, right: String) { let query_proof = format!(\"{}||{}\", left, right); let _ = query_proof; }",
    ),
    (
        WorthQueryConsumerResidueClass::DecomposedProjectionConsumptionAttempt,
        "fn residue(value: ProjectionFactConsumptionAttempt) { let _ = value; }",
    ),
    (
        WorthQueryConsumerResidueClass::IndependentlyPairableProjectionConsumptionParts,
        "fn residue(value: CompletedProjectionFactConsumption) { let _ = value; }",
    ),
    (
        WorthQueryConsumerResidueClass::LegacyProjectionFactConsumptionCall,
        "fn residue(result: Result) { let _ = result.consume_projection_facts(); }",
    ),
    (
        WorthQueryConsumerResidueClass::LegacyProjectionDeclarationCall,
        "fn residue(receipt: Receipt) { let _ = receipt.declare_projection_fact_consumption(); }",
    ),
    (
        WorthQueryConsumerResidueClass::LegacyProjectionIntentAdmissionCall,
        "fn residue(declaration: Declaration) { let _ = worth_query_projection_consumption_intent(declaration); }",
    ),
    (
        WorthQueryConsumerResidueClass::LocalQueryMeasurementConsumptionIdentity,
        "struct ConsumerMeasurementConsumptionIdentity;",
    ),
    (
        WorthQueryConsumerResidueClass::LocalProjectionContractBinding,
        "fn bind_projection_contract() {}",
    ),
    (
        WorthQueryConsumerResidueClass::LocalQueryBasisDigestCompatibility,
        "fn residue(contract: Contract) { let _ = contract.basis_digest() != Some(\"basis\"); }",
    ),
    (
        WorthQueryConsumerResidueClass::LegacyProjectionPrerequisiteAssembly,
        "fn with_query_prerequisites_from_projection_consumption() {}",
    ),
    (
        WorthQueryConsumerResidueClass::DirectInternalQueryImport,
        "use worth_query::projection_consumption::ProjectionConsumptionReceipt;",
    ),
    (
        WorthQueryConsumerResidueClass::LegacyQueryBasisLifecycle,
        "use worth_query::query_basis_lifecycle::RawBasisIntent;",
    ),
    (
        WorthQueryConsumerResidueClass::RawDomainStringAuthority,
        "fn residue() { let _ = worth_query_domain(\"raw-domain\"); }",
    ),
    (
        WorthQueryConsumerResidueClass::ConsumerAuthoredContextDigest,
        "impl Context { fn context_identity_digest(&self) -> String { String::new() } }",
    ),
    (
        WorthQueryConsumerResidueClass::ApplicationFacadeDomainAuthority,
        "fn residue(value: WorthQueryApplicationFacade) { let _ = value; }",
    ),
    (
        WorthQueryConsumerResidueClass::IndependentOperationRegistry,
        "fn residue(value: WorthQueryGraphReadOperationRegistry) { let _ = value; }",
    ),
    (
        WorthQueryConsumerResidueClass::CallerSuppliedOperationRegistry,
        "fn residue(runtime: Runtime, registry: Registry) { let _ = runtime.with_operation_registry(registry); }",
    ),
    (
        WorthQueryConsumerResidueClass::QueryPhaseMaterializerImport,
        "use worth_query::facade::runtime::{materialize_canonical_admission_artifact};",
    ),
    (
        WorthQueryConsumerResidueClass::ConsumerSemanticDomainAdapter,
        "struct HadwigerDomainAuthorityAdapter;",
    ),
    (
        WorthQueryConsumerResidueClass::LocalQueryOperationRegistry,
        "struct WorthUiQueryOperationRegistry;",
    ),
    (
        WorthQueryConsumerResidueClass::LocalQueryDependencyGraph,
        "struct WorthUiQueryDependencyGraph;",
    ),
    (
        WorthQueryConsumerResidueClass::LocalQueryRecomputePolicy,
        "enum WorthUiQueryRecomputePolicy { Full }",
    ),
    (
        WorthQueryConsumerResidueClass::LocalQuerySharingRegistry,
        "struct WorthUiQuerySharingRegistry;",
    ),
    (
        WorthQueryConsumerResidueClass::LocalQuerySupportMirror,
        "struct WorthUiQuerySupportMirror;",
    ),
    (
        WorthQueryConsumerResidueClass::LocalQueryInvalidationMirror,
        "struct WorthUiQueryInvalidationMirror;",
    ),
    (
        WorthQueryConsumerResidueClass::LocalQueryBasisCompatibilityMirror,
        "struct WorthUiQueryBasisCompatibility;",
    ),
    (
        WorthQueryConsumerResidueClass::LocalQueryPatchPosture,
        "enum WorthUiQueryPatchPosture { Reset }",
    ),
    (
        WorthQueryConsumerResidueClass::RawChangeDataCaptureInterpretation,
        "fn interpret_change_data_capture() {}",
    ),
    (
        WorthQueryConsumerResidueClass::RawSignalImport,
        "use worth_signal::facade::SignalConditionalNode;",
    ),
    (
        WorthQueryConsumerResidueClass::RawRuntimeBridgeImport,
        "use worth_runtime_bridge::facade::BridgeConditionalInstallationRequest;",
    ),
    (
        WorthQueryConsumerResidueClass::LocalQueryConditionEvaluator,
        "struct WorthUiQueryConditionEvaluator;",
    ),
    (
        WorthQueryConsumerResidueClass::OrphanQueryLifecycleJoin,
        "struct WorthUiQueryLifecycleJoin;",
    ),
];

pub(super) const FALSE_POSITIVE_CERTIFICATION_SOURCES: &[(&str, &str)] = &[
    (
        "lib.rs",
        "#[doc = \"RuntimeBridge::new and LocalQueryReport\"]\n/// WorthQuerySupportSnapshotRow\n/* WriteAuthorityExecutionReceipt */\nfn clean() {}",
    ),
    (
        "lib.rs",
        "fn clean() { let _ = \"RuntimeBridge::new\"; let _ = r#\"LocalQueryReport || format!(\\\"{:?}\\\")\"#; let _ = '\\''; }",
    ),
    (
        "lib.rs",
        "fn clean(values: Vec<String>, item: String) { let diagnostic = format!(\"{:?}\", item); let joined = values.join(\"||\"); let display = format!(\"{}||{}\", diagnostic, joined); let _ = display; }",
    ),
    (
        "lib.rs",
        "fn clean(rows: DomainRows) { let _ = rows.row_for_family(\"domain\"); } struct DomainRows; impl DomainRows { fn row_for_family(&self, _: &str) -> Option<String> { None } }",
    ),
    (
        "runtime/backend/physical_boundary.rs",
        "struct StorageDomainAdapter;",
    ),
];
