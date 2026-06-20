use std::collections::BTreeSet;

use crate::ForgeQueryEvidenceIdentity;

use super::detection::scan_consumer_residue_source;
use super::evidence::derive_consumer_residue_certification_case_identity;
use super::registry::{
    forge_query_consumer_residue_registry, ForgeQueryConsumerResidueClass,
    ForgeQueryConsumerResidueDetection,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConsumerResidueCertificationCaseEvidence {
    case_id: &'static str,
    checked_source_count: usize,
    checked_class_count: usize,
    finding_count: usize,
    satisfied: bool,
    case_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryConsumerResidueCertificationCaseEvidence {
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

    pub fn case_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.case_identity
    }
}

pub fn forge_query_consumer_residue_certification_evidence(
) -> Vec<ForgeQueryConsumerResidueCertificationCaseEvidence> {
    vec![
        proof_folklore_authority_evidence(),
        false_positive_honesty_evidence(),
    ]
}

fn proof_folklore_authority_evidence() -> ForgeQueryConsumerResidueCertificationCaseEvidence {
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
    let required_classes = forge_query_consumer_residue_registry()
        .iter()
        .map(|row| row.class())
        .collect::<BTreeSet<_>>();
    let ast_rows_present = forge_query_consumer_residue_registry()
        .iter()
        .filter(|row| row.detection() == ForgeQueryConsumerResidueDetection::Ast)
        .count()
        >= 7;
    let satisfied = observed_classes == required_classes && ast_rows_present;
    ForgeQueryConsumerResidueCertificationCaseEvidence::new(
        "consumer-residue-proof-folklore-authority",
        HOSTILE_CERTIFICATION_SOURCES.len(),
        observed_classes.len(),
        finding_count,
        satisfied,
    )
}

fn false_positive_honesty_evidence() -> ForgeQueryConsumerResidueCertificationCaseEvidence {
    let finding_count = FALSE_POSITIVE_CERTIFICATION_SOURCES
        .iter()
        .enumerate()
        .map(|(index, source)| {
            scan_consumer_residue_source(
                "consumer-residue-certification.false-positive",
                &format!("consumer-residue-certification-clean-{index}.rs"),
                source,
                false,
                None,
            )
            .expect("consumer residue false-positive certification source must parse")
            .findings
            .len()
        })
        .sum::<usize>();
    ForgeQueryConsumerResidueCertificationCaseEvidence::new(
        "consumer-residue-false-positive-honesty",
        FALSE_POSITIVE_CERTIFICATION_SOURCES.len(),
        0,
        finding_count,
        finding_count == 0,
    )
}

const HOSTILE_CERTIFICATION_SOURCES: &[(ForgeQueryConsumerResidueClass, &str)] = &[
    (
        ForgeQueryConsumerResidueClass::RuntimeSchemaAdapter,
        "struct Fake; impl ForgeQueryRuntimeSchemaAdapter for Fake {}",
    ),
    (
        ForgeQueryConsumerResidueClass::RuntimeSourceAdapter,
        "struct Fake; impl ForgeQueryRuntimeSourceAdapter for Fake {}",
    ),
    (
        ForgeQueryConsumerResidueClass::RuntimeWriteAuthorityAdapter,
        "struct Fake; impl ForgeQueryRuntimeWriteAuthorityAdapter for Fake {}",
    ),
    (
        ForgeQueryConsumerResidueClass::RuntimeSignalSinkAdapter,
        "struct Fake; impl ForgeQueryRuntimeSignalSinkAdapter for Fake {}",
    ),
    (
        ForgeQueryConsumerResidueClass::RuntimeSnapshotIdentityAdapter,
        "struct Fake; impl ForgeQueryRuntimeSnapshotIdentityAdapter for Fake {}",
    ),
    (
        ForgeQueryConsumerResidueClass::RuntimeSubscriptionActivationAdapter,
        "struct Fake; impl ForgeQueryRuntimeSubscriptionActivationAdapter for Fake {}",
    ),
    (
        ForgeQueryConsumerResidueClass::RuntimePreviewBasisAdapter,
        "struct Fake; impl ForgeQueryRuntimePreviewBasisAdapter for Fake {}",
    ),
    (
        ForgeQueryConsumerResidueClass::RuntimeInspectorEvidenceAdapter,
        "struct Fake; impl ForgeQueryRuntimeInspectorEvidenceAdapter for Fake {}",
    ),
    (
        ForgeQueryConsumerResidueClass::RuntimeBridgeHandAssembly,
        "fn residue() { let _ = RuntimeBridge::new(); }",
    ),
    (
        ForgeQueryConsumerResidueClass::FabricatedMutationReceipt,
        "fn residue() { let _ = ForgeQueryMutationReceipt::from_authoritative_parts(); }",
    ),
    (
        ForgeQueryConsumerResidueClass::FabricatedBridgeMutationReceipt,
        "fn residue() { let _ = ForgeQueryMutationReceipt::from_bridge_authoritative_parts(); }",
    ),
    (
        ForgeQueryConsumerResidueClass::FabricatedWriteAuthorityReceipt,
        "fn residue() { let _ = WriteAuthorityExecutionReceipt; }",
    ),
    (
        ForgeQueryConsumerResidueClass::LocalQueryReport,
        "struct LocalQueryReport;",
    ),
    (
        ForgeQueryConsumerResidueClass::LocalQueryProof,
        "struct LocalQueryProof;",
    ),
    (
        ForgeQueryConsumerResidueClass::RawSupportSnapshotRow,
        "fn residue(row: ForgeQuerySupportSnapshotRow) { let _ = row; }",
    ),
    (
        ForgeQueryConsumerResidueClass::SupportMatrixRowSearch,
        "fn residue(support_matrix: SupportMatrix) { let _ = support_matrix.row_for_family(\"write\"); }",
    ),
    (
        ForgeQueryConsumerResidueClass::DebugDerivedQueryProof,
        "fn residue(receipt: String) { let query_proof = format!(\"{:?}\", receipt); let _ = query_proof; }",
    ),
    (
        ForgeQueryConsumerResidueClass::DelimiterJoinedQueryProof,
        "fn residue(parts: Vec<String>) { let query_proof = parts.join(\"||\"); let _ = query_proof; }",
    ),
    (
        ForgeQueryConsumerResidueClass::DelimiterFormattedQueryProof,
        "fn residue(left: String, right: String) { let query_proof = format!(\"{}||{}\", left, right); let _ = query_proof; }",
    ),
];

const FALSE_POSITIVE_CERTIFICATION_SOURCES: &[&str] = &[
    "#[doc = \"RuntimeBridge::new and LocalQueryReport\"]\n/// ForgeQuerySupportSnapshotRow\n/* WriteAuthorityExecutionReceipt */\nfn clean() {}",
    "fn clean() { let _ = \"RuntimeBridge::new\"; let _ = r#\"LocalQueryReport || format!(\\\"{:?}\\\")\"#; let _ = '\\''; }",
    "fn clean(values: Vec<String>, item: String) { let diagnostic = format!(\"{:?}\", item); let joined = values.join(\"||\"); let display = format!(\"{}||{}\", diagnostic, joined); let _ = display; }",
    "fn clean(rows: DomainRows) { let _ = rows.row_for_family(\"domain\"); } struct DomainRows; impl DomainRows { fn row_for_family(&self, _: &str) -> Option<String> { None } }",
];
