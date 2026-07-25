mod fixture_sources;

use std::collections::BTreeSet;

use crate::WorthQueryEvidenceIdentity;

use super::detection::scan_consumer_residue_source;
use super::evidence::derive_consumer_residue_certification_case_identity;
use super::registry::{worth_query_consumer_residue_registry, WorthQueryConsumerResidueDetection};
use fixture_sources::{FALSE_POSITIVE_CERTIFICATION_SOURCES, HOSTILE_CERTIFICATION_SOURCES};

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
