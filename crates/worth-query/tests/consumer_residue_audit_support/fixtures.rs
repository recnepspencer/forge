use worth_query::facade::consumer_kit::WorthQueryConsumerResidueClass;

pub struct HostileClassCase {
    pub label: &'static str,
    pub class: WorthQueryConsumerResidueClass,
    pub detection_key: &'static str,
    pub replacement_lane: &'static str,
    pub line_needle: &'static str,
    pub source: &'static str,
}

pub struct FalsePositiveCase {
    pub label: &'static str,
    pub source: &'static str,
}

pub const HOSTILE_CLASS_CASES: &[HostileClassCase] = &[
    case(
        "runtime-schema",
        WorthQueryConsumerResidueClass::RuntimeSchemaAdapter,
        "impl WorthQueryRuntimeSchemaAdapter",
        "in-memory-test-backend",
        "impl WorthQueryRuntimeSchemaAdapter",
        RUNTIME_SCHEMA_SOURCE,
    ),
    case(
        "runtime-source",
        WorthQueryConsumerResidueClass::RuntimeSourceAdapter,
        "impl WorthQueryRuntimeSourceAdapter",
        "in-memory-test-backend",
        "impl WorthQueryRuntimeSourceAdapter",
        RUNTIME_SOURCE_SOURCE,
    ),
    case(
        "runtime-write-authority",
        WorthQueryConsumerResidueClass::RuntimeWriteAuthorityAdapter,
        "impl WorthQueryRuntimeWriteAuthorityAdapter",
        "in-memory-test-backend",
        "impl WorthQueryRuntimeWriteAuthorityAdapter",
        RUNTIME_WRITE_AUTHORITY_SOURCE,
    ),
    case(
        "runtime-signal-sink",
        WorthQueryConsumerResidueClass::RuntimeSignalSinkAdapter,
        "impl WorthQueryRuntimeSignalSinkAdapter",
        "in-memory-test-backend",
        "impl WorthQueryRuntimeSignalSinkAdapter",
        RUNTIME_SIGNAL_SINK_SOURCE,
    ),
    case(
        "runtime-snapshot-identity",
        WorthQueryConsumerResidueClass::RuntimeSnapshotIdentityAdapter,
        "impl WorthQueryRuntimeSnapshotIdentityAdapter",
        "in-memory-test-backend",
        "impl WorthQueryRuntimeSnapshotIdentityAdapter",
        RUNTIME_SNAPSHOT_IDENTITY_SOURCE,
    ),
    case(
        "runtime-subscription",
        WorthQueryConsumerResidueClass::RuntimeSubscriptionActivationAdapter,
        "impl WorthQueryRuntimeSubscriptionActivationAdapter",
        "in-memory-test-backend",
        "impl WorthQueryRuntimeSubscriptionActivationAdapter",
        RUNTIME_SUBSCRIPTION_SOURCE,
    ),
    case(
        "runtime-preview-basis",
        WorthQueryConsumerResidueClass::RuntimePreviewBasisAdapter,
        "impl WorthQueryRuntimePreviewBasisAdapter",
        "in-memory-test-backend",
        "impl WorthQueryRuntimePreviewBasisAdapter",
        RUNTIME_PREVIEW_BASIS_SOURCE,
    ),
    case(
        "runtime-inspector",
        WorthQueryConsumerResidueClass::RuntimeInspectorEvidenceAdapter,
        "impl WorthQueryRuntimeInspectorEvidenceAdapter",
        "in-memory-test-backend",
        "impl WorthQueryRuntimeInspectorEvidenceAdapter",
        RUNTIME_INSPECTOR_SOURCE,
    ),
    case(
        "runtime-bridge",
        WorthQueryConsumerResidueClass::RuntimeBridgeHandAssembly,
        "RuntimeBridge::",
        "in-memory-test-backend",
        "RuntimeBridge::",
        RUNTIME_BRIDGE_SOURCE,
    ),
    case(
        "fabricated-mutation",
        WorthQueryConsumerResidueClass::FabricatedMutationReceipt,
        "WorthQueryMutationReceipt::from_authoritative_parts",
        "in-memory-test-backend",
        "from_authoritative_parts",
        FABRICATED_MUTATION_SOURCE,
    ),
    case(
        "fabricated-bridge-mutation",
        WorthQueryConsumerResidueClass::FabricatedBridgeMutationReceipt,
        "WorthQueryMutationReceipt::from_bridge_authoritative_parts",
        "in-memory-test-backend",
        "from_bridge_authoritative_parts",
        FABRICATED_BRIDGE_MUTATION_SOURCE,
    ),
    case(
        "fabricated-write-authority",
        WorthQueryConsumerResidueClass::FabricatedWriteAuthorityReceipt,
        "WriteAuthorityExecutionReceipt",
        "in-memory-test-backend",
        "WriteAuthorityExecutionReceipt",
        FABRICATED_WRITE_AUTHORITY_SOURCE,
    ),
    case(
        "local-query-report",
        WorthQueryConsumerResidueClass::LocalQueryReport,
        "local-query-report-struct",
        "evidence-report-kit",
        "LocalQueryReport",
        LOCAL_QUERY_REPORT_SOURCE,
    ),
    case(
        "local-query-proof",
        WorthQueryConsumerResidueClass::LocalQueryProof,
        "local-query-proof-struct",
        "evidence-report-kit",
        "LocalQueryProof",
        LOCAL_QUERY_PROOF_SOURCE,
    ),
    case(
        "raw-support-row",
        WorthQueryConsumerResidueClass::RawSupportSnapshotRow,
        "WorthQuerySupportSnapshotRow",
        "support-pinning",
        "WorthQuerySupportSnapshotRow",
        RAW_SUPPORT_ROW_SOURCE,
    ),
    case(
        "support-row-search",
        WorthQueryConsumerResidueClass::SupportMatrixRowSearch,
        "row_for_family",
        "support-pinning",
        "row_for_family",
        SUPPORT_ROW_SEARCH_SOURCE,
    ),
    case(
        "debug-proof",
        WorthQueryConsumerResidueClass::DebugDerivedQueryProof,
        "format-debug-query-proof",
        "evidence-report-kit",
        "format!",
        DEBUG_BINDING_SOURCE,
    ),
    case(
        "joined-proof",
        WorthQueryConsumerResidueClass::DelimiterJoinedQueryProof,
        "delimiter-joined-query-proof",
        "evidence-report-kit",
        ".join",
        DELIMITER_JOIN_SOURCE,
    ),
    case(
        "formatted-proof",
        WorthQueryConsumerResidueClass::DelimiterFormattedQueryProof,
        "delimiter-formatted-query-proof",
        "evidence-report-kit",
        "format!",
        DELIMITER_FORMAT_SOURCE,
    ),
    case(
        "decomposed-projection-attempt",
        WorthQueryConsumerResidueClass::DecomposedProjectionConsumptionAttempt,
        "ProjectionFactConsumptionAttempt",
        "downstream-authority-adoption",
        "ProjectionFactConsumptionAttempt",
        "fn residue(value: ProjectionFactConsumptionAttempt) { let _ = value; }",
    ),
    case(
        "local-consumption-identity",
        WorthQueryConsumerResidueClass::LocalQueryMeasurementConsumptionIdentity,
        "ConsumerMeasurementConsumptionIdentity",
        "downstream-authority-adoption",
        "ConsumerMeasurementConsumptionIdentity",
        "struct ConsumerMeasurementConsumptionIdentity;",
    ),
    case(
        "local-contract-binding",
        WorthQueryConsumerResidueClass::LocalProjectionContractBinding,
        "bind_projection_contract(",
        "downstream-authority-adoption",
        "bind_projection_contract(",
        "fn bind_projection_contract() {}",
    ),
    case(
        "local-basis-digest-compatibility",
        WorthQueryConsumerResidueClass::LocalQueryBasisDigestCompatibility,
        "contract.basis_digest() != Some(",
        "downstream-authority-adoption",
        "contract.basis_digest() != Some(",
        "fn residue(contract: Contract) { let _ = contract.basis_digest() != Some(\"basis\"); }",
    ),
    case(
        "legacy-prerequisite-assembly",
        WorthQueryConsumerResidueClass::LegacyProjectionPrerequisiteAssembly,
        "with_query_prerequisites_from_projection_consumption",
        "downstream-authority-adoption",
        "with_query_prerequisites_from_projection_consumption",
        "fn with_query_prerequisites_from_projection_consumption() {}",
    ),
    case(
        "direct-internal-query-import",
        WorthQueryConsumerResidueClass::DirectInternalQueryImport,
        "worth_query::projection_consumption",
        "downstream-authority-adoption",
        "worth_query::projection_consumption",
        "use worth_query::projection_consumption::ProjectionConsumptionReceipt;",
    ),
    case(
        "legacy-query-basis-lifecycle",
        WorthQueryConsumerResidueClass::LegacyQueryBasisLifecycle,
        "query_basis_lifecycle",
        "downstream-authority-adoption",
        "query_basis_lifecycle",
        "use worth_query::query_basis_lifecycle::RawBasisIntent;",
    ),
];

pub const SYNTAX_ROLE_CASES: &[HostileClassCase] = &[
    case(
        "proof-assignment",
        WorthQueryConsumerResidueClass::DebugDerivedQueryProof,
        "format-debug-query-proof",
        "evidence-report-kit",
        "query_proof =",
        DEBUG_ASSIGNMENT_SOURCE,
    ),
    case(
        "proof-return",
        WorthQueryConsumerResidueClass::DebugDerivedQueryProof,
        "format-debug-query-proof",
        "evidence-report-kit",
        "return format!",
        DEBUG_RETURN_SOURCE,
    ),
    case(
        "proof-field",
        WorthQueryConsumerResidueClass::DebugDerivedQueryProof,
        "format-debug-query-proof",
        "evidence-report-kit",
        "evidence_proof:",
        DEBUG_FIELD_SOURCE,
    ),
    case(
        "support-var-search",
        WorthQueryConsumerResidueClass::SupportMatrixRowSearch,
        "row_for_family",
        "support-pinning",
        "row_for_family",
        SUPPORT_VARIABLE_ROW_SEARCH_SOURCE,
    ),
];

pub const FALSE_POSITIVE_CASES: &[FalsePositiveCase] = &[
    FalsePositiveCase {
        label: "comments-and-docs",
        source: COMMENTS_AND_DOCS_SOURCE,
    },
    FalsePositiveCase {
        label: "strings-and-raw-strings",
        source: STRINGS_SOURCE,
    },
    FalsePositiveCase {
        label: "ordinary-formatting",
        source: ORDINARY_FORMATTING_SOURCE,
    },
    FalsePositiveCase {
        label: "unrelated-row-search",
        source: UNRELATED_ROW_SEARCH_SOURCE,
    },
];

const fn case(
    label: &'static str,
    class: WorthQueryConsumerResidueClass,
    detection_key: &'static str,
    replacement_lane: &'static str,
    line_needle: &'static str,
    source: &'static str,
) -> HostileClassCase {
    HostileClassCase {
        label,
        class,
        detection_key,
        replacement_lane,
        line_needle,
        source,
    }
}

const RUNTIME_SCHEMA_SOURCE: &str = "struct Fake; impl WorthQueryRuntimeSchemaAdapter for Fake {}";
const RUNTIME_SOURCE_SOURCE: &str = "struct Fake; impl WorthQueryRuntimeSourceAdapter for Fake {}";
const RUNTIME_WRITE_AUTHORITY_SOURCE: &str =
    "struct Fake; impl WorthQueryRuntimeWriteAuthorityAdapter for Fake {}";
const RUNTIME_SIGNAL_SINK_SOURCE: &str =
    "struct Fake; impl WorthQueryRuntimeSignalSinkAdapter for Fake {}";
const RUNTIME_SNAPSHOT_IDENTITY_SOURCE: &str =
    "struct Fake; impl WorthQueryRuntimeSnapshotIdentityAdapter for Fake {}";
const RUNTIME_SUBSCRIPTION_SOURCE: &str =
    "struct Fake; impl WorthQueryRuntimeSubscriptionActivationAdapter for Fake {}";
const RUNTIME_PREVIEW_BASIS_SOURCE: &str =
    "struct Fake; impl WorthQueryRuntimePreviewBasisAdapter for Fake {}";
const RUNTIME_INSPECTOR_SOURCE: &str =
    "struct Fake; impl WorthQueryRuntimeInspectorEvidenceAdapter for Fake {}";
pub const RUNTIME_BRIDGE_SOURCE: &str = "fn residue() { let _ = RuntimeBridge::new(); }";
const FABRICATED_MUTATION_SOURCE: &str =
    "fn residue() { let _ = WorthQueryMutationReceipt::from_authoritative_parts(); }";
const FABRICATED_BRIDGE_MUTATION_SOURCE: &str =
    "fn residue() { let _ = WorthQueryMutationReceipt::from_bridge_authoritative_parts(); }";
const FABRICATED_WRITE_AUTHORITY_SOURCE: &str =
    "fn residue() { let _ = WriteAuthorityExecutionReceipt; }";
pub const LOCAL_QUERY_REPORT_SOURCE: &str = "struct LocalQueryReport;";
const LOCAL_QUERY_PROOF_SOURCE: &str = "struct LocalQueryProof;";
const RAW_SUPPORT_ROW_SOURCE: &str =
    "fn residue(row: WorthQuerySupportSnapshotRow) { let _ = row; }";
const SUPPORT_ROW_SEARCH_SOURCE: &str = "fn residue(snapshot: Snapshot) { let _ = snapshot.runtime_support_matrix().row_for_family(\"write\"); }";
const SUPPORT_VARIABLE_ROW_SEARCH_SOURCE: &str = "fn residue(support_matrix: SupportMatrix) { let _ = support_matrix.row_for_family(\"write\"); }";
pub const DEBUG_BINDING_SOURCE: &str = "fn residue(query_receipt: String) { let query_proof = format!(\"{:?}\", query_receipt); let _ = query_proof; }";
const DEBUG_ASSIGNMENT_SOURCE: &str = "fn residue(query_receipt: String) { let mut query_proof = String::new(); query_proof = format!(\"{:?}\", query_receipt); }";
const DEBUG_RETURN_SOURCE: &str =
    "fn query_proof(receipt: String) -> String { return format!(\"{:?}\", receipt); }";
const DEBUG_FIELD_SOURCE: &str = "struct Local { evidence_proof: String } fn residue(receipt: String) { let _ = Local { evidence_proof: format!(\"{:?}\", receipt) }; }";
const DELIMITER_JOIN_SOURCE: &str =
    "fn residue(parts: Vec<String>) { let query_proof = parts.join(\"||\"); let _ = query_proof; }";
const DELIMITER_FORMAT_SOURCE: &str = "fn residue(left: String, right: String) { let query_proof = format!(\"{}||{}\", left, right); let _ = query_proof; }";
pub const CLEAN_SOURCE: &str = "fn clean(value: String) -> String { value }";
const COMMENTS_AND_DOCS_SOURCE: &str = "#[doc = \"RuntimeBridge::new and LocalQueryReport\"]\n/// WorthQuerySupportSnapshotRow\n/* WriteAuthorityExecutionReceipt */\nfn clean() {}";
const STRINGS_SOURCE: &str = "fn clean() { let _ = \"RuntimeBridge::new\"; let _ = r#\"LocalQueryReport || format!(\\\"{:?}\\\")\"#; let _ = '\\''; }";
const ORDINARY_FORMATTING_SOURCE: &str = "fn clean(values: Vec<String>, item: String) { let diagnostic = format!(\"{:?}\", item); let joined = values.join(\"||\"); let display = format!(\"{}||{}\", diagnostic, joined); let _ = display; }";
const UNRELATED_ROW_SEARCH_SOURCE: &str = "fn clean(rows: DomainRows) { let _ = rows.row_for_family(\"domain\"); } struct DomainRows; impl DomainRows { fn row_for_family(&self, _: &str) -> Option<String> { None } }";
