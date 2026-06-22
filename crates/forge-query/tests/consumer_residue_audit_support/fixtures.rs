use forge_query::facade::consumer_kit::ForgeQueryConsumerResidueClass;

pub struct HostileClassCase {
    pub label: &'static str,
    pub class: ForgeQueryConsumerResidueClass,
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
        ForgeQueryConsumerResidueClass::RuntimeSchemaAdapter,
        "impl ForgeQueryRuntimeSchemaAdapter",
        "in-memory-test-backend",
        "impl ForgeQueryRuntimeSchemaAdapter",
        RUNTIME_SCHEMA_SOURCE,
    ),
    case(
        "runtime-source",
        ForgeQueryConsumerResidueClass::RuntimeSourceAdapter,
        "impl ForgeQueryRuntimeSourceAdapter",
        "in-memory-test-backend",
        "impl ForgeQueryRuntimeSourceAdapter",
        RUNTIME_SOURCE_SOURCE,
    ),
    case(
        "runtime-write-authority",
        ForgeQueryConsumerResidueClass::RuntimeWriteAuthorityAdapter,
        "impl ForgeQueryRuntimeWriteAuthorityAdapter",
        "in-memory-test-backend",
        "impl ForgeQueryRuntimeWriteAuthorityAdapter",
        RUNTIME_WRITE_AUTHORITY_SOURCE,
    ),
    case(
        "runtime-signal-sink",
        ForgeQueryConsumerResidueClass::RuntimeSignalSinkAdapter,
        "impl ForgeQueryRuntimeSignalSinkAdapter",
        "in-memory-test-backend",
        "impl ForgeQueryRuntimeSignalSinkAdapter",
        RUNTIME_SIGNAL_SINK_SOURCE,
    ),
    case(
        "runtime-snapshot-identity",
        ForgeQueryConsumerResidueClass::RuntimeSnapshotIdentityAdapter,
        "impl ForgeQueryRuntimeSnapshotIdentityAdapter",
        "in-memory-test-backend",
        "impl ForgeQueryRuntimeSnapshotIdentityAdapter",
        RUNTIME_SNAPSHOT_IDENTITY_SOURCE,
    ),
    case(
        "runtime-subscription",
        ForgeQueryConsumerResidueClass::RuntimeSubscriptionActivationAdapter,
        "impl ForgeQueryRuntimeSubscriptionActivationAdapter",
        "in-memory-test-backend",
        "impl ForgeQueryRuntimeSubscriptionActivationAdapter",
        RUNTIME_SUBSCRIPTION_SOURCE,
    ),
    case(
        "runtime-preview-basis",
        ForgeQueryConsumerResidueClass::RuntimePreviewBasisAdapter,
        "impl ForgeQueryRuntimePreviewBasisAdapter",
        "in-memory-test-backend",
        "impl ForgeQueryRuntimePreviewBasisAdapter",
        RUNTIME_PREVIEW_BASIS_SOURCE,
    ),
    case(
        "runtime-inspector",
        ForgeQueryConsumerResidueClass::RuntimeInspectorEvidenceAdapter,
        "impl ForgeQueryRuntimeInspectorEvidenceAdapter",
        "in-memory-test-backend",
        "impl ForgeQueryRuntimeInspectorEvidenceAdapter",
        RUNTIME_INSPECTOR_SOURCE,
    ),
    case(
        "runtime-bridge",
        ForgeQueryConsumerResidueClass::RuntimeBridgeHandAssembly,
        "RuntimeBridge::",
        "in-memory-test-backend",
        "RuntimeBridge::",
        RUNTIME_BRIDGE_SOURCE,
    ),
    case(
        "fabricated-mutation",
        ForgeQueryConsumerResidueClass::FabricatedMutationReceipt,
        "ForgeQueryMutationReceipt::from_authoritative_parts",
        "in-memory-test-backend",
        "from_authoritative_parts",
        FABRICATED_MUTATION_SOURCE,
    ),
    case(
        "fabricated-bridge-mutation",
        ForgeQueryConsumerResidueClass::FabricatedBridgeMutationReceipt,
        "ForgeQueryMutationReceipt::from_bridge_authoritative_parts",
        "in-memory-test-backend",
        "from_bridge_authoritative_parts",
        FABRICATED_BRIDGE_MUTATION_SOURCE,
    ),
    case(
        "fabricated-write-authority",
        ForgeQueryConsumerResidueClass::FabricatedWriteAuthorityReceipt,
        "WriteAuthorityExecutionReceipt",
        "in-memory-test-backend",
        "WriteAuthorityExecutionReceipt",
        FABRICATED_WRITE_AUTHORITY_SOURCE,
    ),
    case(
        "local-query-report",
        ForgeQueryConsumerResidueClass::LocalQueryReport,
        "local-query-report-struct",
        "evidence-report-kit",
        "LocalQueryReport",
        LOCAL_QUERY_REPORT_SOURCE,
    ),
    case(
        "local-query-proof",
        ForgeQueryConsumerResidueClass::LocalQueryProof,
        "local-query-proof-struct",
        "evidence-report-kit",
        "LocalQueryProof",
        LOCAL_QUERY_PROOF_SOURCE,
    ),
    case(
        "raw-support-row",
        ForgeQueryConsumerResidueClass::RawSupportSnapshotRow,
        "ForgeQuerySupportSnapshotRow",
        "support-pinning",
        "ForgeQuerySupportSnapshotRow",
        RAW_SUPPORT_ROW_SOURCE,
    ),
    case(
        "support-row-search",
        ForgeQueryConsumerResidueClass::SupportMatrixRowSearch,
        "row_for_family",
        "support-pinning",
        "row_for_family",
        SUPPORT_ROW_SEARCH_SOURCE,
    ),
    case(
        "debug-proof",
        ForgeQueryConsumerResidueClass::DebugDerivedQueryProof,
        "format-debug-query-proof",
        "evidence-report-kit",
        "format!",
        DEBUG_BINDING_SOURCE,
    ),
    case(
        "joined-proof",
        ForgeQueryConsumerResidueClass::DelimiterJoinedQueryProof,
        "delimiter-joined-query-proof",
        "evidence-report-kit",
        ".join",
        DELIMITER_JOIN_SOURCE,
    ),
    case(
        "formatted-proof",
        ForgeQueryConsumerResidueClass::DelimiterFormattedQueryProof,
        "delimiter-formatted-query-proof",
        "evidence-report-kit",
        "format!",
        DELIMITER_FORMAT_SOURCE,
    ),
];

pub const SYNTAX_ROLE_CASES: &[HostileClassCase] = &[
    case(
        "proof-assignment",
        ForgeQueryConsumerResidueClass::DebugDerivedQueryProof,
        "format-debug-query-proof",
        "evidence-report-kit",
        "query_proof =",
        DEBUG_ASSIGNMENT_SOURCE,
    ),
    case(
        "proof-return",
        ForgeQueryConsumerResidueClass::DebugDerivedQueryProof,
        "format-debug-query-proof",
        "evidence-report-kit",
        "return format!",
        DEBUG_RETURN_SOURCE,
    ),
    case(
        "proof-field",
        ForgeQueryConsumerResidueClass::DebugDerivedQueryProof,
        "format-debug-query-proof",
        "evidence-report-kit",
        "evidence_proof:",
        DEBUG_FIELD_SOURCE,
    ),
    case(
        "support-var-search",
        ForgeQueryConsumerResidueClass::SupportMatrixRowSearch,
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
    class: ForgeQueryConsumerResidueClass,
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

const RUNTIME_SCHEMA_SOURCE: &str = "struct Fake; impl ForgeQueryRuntimeSchemaAdapter for Fake {}";
const RUNTIME_SOURCE_SOURCE: &str = "struct Fake; impl ForgeQueryRuntimeSourceAdapter for Fake {}";
const RUNTIME_WRITE_AUTHORITY_SOURCE: &str =
    "struct Fake; impl ForgeQueryRuntimeWriteAuthorityAdapter for Fake {}";
const RUNTIME_SIGNAL_SINK_SOURCE: &str =
    "struct Fake; impl ForgeQueryRuntimeSignalSinkAdapter for Fake {}";
const RUNTIME_SNAPSHOT_IDENTITY_SOURCE: &str =
    "struct Fake; impl ForgeQueryRuntimeSnapshotIdentityAdapter for Fake {}";
const RUNTIME_SUBSCRIPTION_SOURCE: &str =
    "struct Fake; impl ForgeQueryRuntimeSubscriptionActivationAdapter for Fake {}";
const RUNTIME_PREVIEW_BASIS_SOURCE: &str =
    "struct Fake; impl ForgeQueryRuntimePreviewBasisAdapter for Fake {}";
const RUNTIME_INSPECTOR_SOURCE: &str =
    "struct Fake; impl ForgeQueryRuntimeInspectorEvidenceAdapter for Fake {}";
pub const RUNTIME_BRIDGE_SOURCE: &str = "fn residue() { let _ = RuntimeBridge::new(); }";
const FABRICATED_MUTATION_SOURCE: &str =
    "fn residue() { let _ = ForgeQueryMutationReceipt::from_authoritative_parts(); }";
const FABRICATED_BRIDGE_MUTATION_SOURCE: &str =
    "fn residue() { let _ = ForgeQueryMutationReceipt::from_bridge_authoritative_parts(); }";
const FABRICATED_WRITE_AUTHORITY_SOURCE: &str =
    "fn residue() { let _ = WriteAuthorityExecutionReceipt; }";
pub const LOCAL_QUERY_REPORT_SOURCE: &str = "struct LocalQueryReport;";
const LOCAL_QUERY_PROOF_SOURCE: &str = "struct LocalQueryProof;";
const RAW_SUPPORT_ROW_SOURCE: &str =
    "fn residue(row: ForgeQuerySupportSnapshotRow) { let _ = row; }";
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
const COMMENTS_AND_DOCS_SOURCE: &str = "#[doc = \"RuntimeBridge::new and LocalQueryReport\"]\n/// ForgeQuerySupportSnapshotRow\n/* WriteAuthorityExecutionReceipt */\nfn clean() {}";
const STRINGS_SOURCE: &str = "fn clean() { let _ = \"RuntimeBridge::new\"; let _ = r#\"LocalQueryReport || format!(\\\"{:?}\\\")\"#; let _ = '\\''; }";
const ORDINARY_FORMATTING_SOURCE: &str = "fn clean(values: Vec<String>, item: String) { let diagnostic = format!(\"{:?}\", item); let joined = values.join(\"||\"); let display = format!(\"{}||{}\", diagnostic, joined); let _ = display; }";
const UNRELATED_ROW_SEARCH_SOURCE: &str = "fn clean(rows: DomainRows) { let _ = rows.row_for_family(\"domain\"); } struct DomainRows; impl DomainRows { fn row_for_family(&self, _: &str) -> Option<String> { None } }";
