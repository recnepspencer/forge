use std::collections::BTreeSet;

use crate::WorthQueryEvidenceIdentity;

use super::detection::scan_consumer_residue_source;
use super::evidence::derive_consumer_residue_certification_case_identity;
use super::registry::{
    worth_query_consumer_residue_registry, WorthQueryConsumerResidueClass,
    WorthQueryConsumerResidueDetection,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerResidueCertificationCaseEvidence {
    case_id: &'static str,
    checked_source_count: usize,
    checked_class_count: usize,
    finding_count: usize,
    satisfied: bool,
    case_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryConsumerResidueCertificationCaseEvidence {
    fn new(
        case_id: &'static str,
        checked_source_count: usize,
        checked_class_count: usize,
        finding_count: usize,
        satisfied: bool,
    ) -> Self {
        let case_identity = derive_consumer_residue_certification_case_identity(
            case_id,
            checked_source_count,
            checked_class_count,
            finding_count,
            satisfied,
        );
        Self {
            case_id,
            checked_source_count,
            checked_class_count,
            finding_count,
            satisfied,
            case_identity,
        }
    }

    pub fn case_id(&self) -> &'static str {
        self.case_id
    }

    pub fn checked_source_count(&self) -> usize {
        self.checked_source_count
    }

    pub fn checked_class_count(&self) -> usize {
        self.checked_class_count
    }

    pub fn finding_count(&self) -> usize {
        self.finding_count
    }

    pub fn satisfied(&self) -> bool {
        self.satisfied
    }

    pub fn case_digest(&self) -> &str {
        self.case_identity.as_str()
    }

    pub fn case_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.case_identity
    }
}

pub fn worth_query_consumer_residue_certification_evidence(
) -> Vec<WorthQueryConsumerResidueCertificationCaseEvidence> {
    vec![
        proof_folklore_authority_evidence(),
        false_positive_honesty_evidence(),
    ]
}

fn proof_folklore_authority_evidence() -> WorthQueryConsumerResidueCertificationCaseEvidence {
    let mut observed_classes = BTreeSet::new();
    let mut finding_count = 0;
    for (index, (class, source)) in HOSTILE_CERTIFICATION_SOURCES.iter().enumerate() {
        let classification = scan_consumer_residue_source(
            "consumer-residue-certification.hostile",
            &format!("consumer-residue-certification-hostile-{index}.rs"),
            source,
            false,
            None,
        )
        .expect("consumer residue hostile certification source must parse");
        finding_count += classification.findings.len();
        if classification
            .findings
            .iter()
            .any(|finding| finding.residue_class() == *class)
        {
            observed_classes.insert(*class);
        }
    }
    let required_classes = worth_query_consumer_residue_registry()
        .iter()
        .map(|row| row.class())
        .collect::<BTreeSet<_>>();
    let ast_rows_present = worth_query_consumer_residue_registry()
        .iter()
        .filter(|row| row.detection() == WorthQueryConsumerResidueDetection::Ast)
        .count()
        >= 7;
    let satisfied = observed_classes == required_classes && ast_rows_present;
    WorthQueryConsumerResidueCertificationCaseEvidence::new(
        "consumer-residue-proof-folklore-authority",
        HOSTILE_CERTIFICATION_SOURCES.len(),
        observed_classes.len(),
        finding_count,
        satisfied,
    )
}

fn false_positive_honesty_evidence() -> WorthQueryConsumerResidueCertificationCaseEvidence {
    let finding_count = FALSE_POSITIVE_CERTIFICATION_SOURCES
        .iter()
        .enumerate()
        .map(|(index, (path, source))| {
            scan_consumer_residue_source(
                "consumer-residue-certification.false-positive",
                &format!("consumer-residue-certification-clean-{index}/{path}"),
                source,
                false,
                None,
            )
            .expect("consumer residue false-positive certification source must parse")
            .findings
            .len()
        })
        .sum::<usize>();
    WorthQueryConsumerResidueCertificationCaseEvidence::new(
        "consumer-residue-false-positive-honesty",
        FALSE_POSITIVE_CERTIFICATION_SOURCES.len(),
        0,
        finding_count,
        finding_count == 0,
    )
}

const HOSTILE_CERTIFICATION_SOURCES: &[(WorthQueryConsumerResidueClass, &str)] = &[
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
        "struct WorthUiQueryMeasurementConsumptionIdentity;",
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
];

const FALSE_POSITIVE_CERTIFICATION_SOURCES: &[(&str, &str)] = &[
    ("lib.rs", "#[doc = \"RuntimeBridge::new and LocalQueryReport\"]\n/// WorthQuerySupportSnapshotRow\n/* WriteAuthorityExecutionReceipt */\nfn clean() {}"),
    ("lib.rs", "fn clean() { let _ = \"RuntimeBridge::new\"; let _ = r#\"LocalQueryReport || format!(\\\"{:?}\\\")\"#; let _ = '\\''; }"),
    ("lib.rs", "fn clean(values: Vec<String>, item: String) { let diagnostic = format!(\"{:?}\", item); let joined = values.join(\"||\"); let display = format!(\"{}||{}\", diagnostic, joined); let _ = display; }"),
    ("lib.rs", "fn clean(rows: DomainRows) { let _ = rows.row_for_family(\"domain\"); } struct DomainRows; impl DomainRows { fn row_for_family(&self, _: &str) -> Option<String> { None } }"),
    ("runtime/backend/physical_boundary.rs", "struct StorageDomainAdapter;"),
];
