use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::{HadwigerArtifactDigest, HadwigerCanonicalArtifact};
use crate::explanations::{
    HadwigerPartialAdmissionExplanation, HadwigerQueryRecoveryExplanation,
    HadwigerRejectionExplanation, HadwigerReusableNegativeEvidence,
};

use super::graph_memory::GraphResidentFailure;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HadwigerDiscoveryEvidenceReference {
    Artifact {
        reference: HadwigerArtifactReference,
    },
    QueryRecovery {
        recovery_token: String,
        stop_family: &'static str,
    },
}

impl HadwigerDiscoveryEvidenceReference {
    pub fn artifact(reference: HadwigerArtifactReference) -> Self {
        Self::Artifact { reference }
    }

    pub(crate) fn query_recovery(explanation: &HadwigerQueryRecoveryExplanation) -> Self {
        let brief = explanation
            .query_recovery_brief()
            .expect("query recovery explanations always retain a brief");
        Self::QueryRecovery {
            recovery_token: format!(
                "{:?}:{:?}:{:?}:{}",
                brief.stop_family(),
                brief.stop_kind(),
                brief.recommended_action(),
                brief.reason()
            ),
            stop_family: explanation.stop_family().as_str(),
        }
    }

    pub fn stable_token(&self) -> String {
        match self {
            Self::Artifact { reference } => format!("artifact:{}", reference.stable_token()),
            Self::QueryRecovery {
                recovery_token,
                stop_family,
            } => format!("query_recovery:{stop_family}:{recovery_token}"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchEvidenceCorpus {
    core: HadwigerArtifactCore,
    corpus_id: String,
    evidence_references: Vec<HadwigerDiscoveryEvidenceReference>,
    reusable_negative_evidence: Vec<HadwigerReusableNegativeEvidence>,
    graph_resident_failures: Vec<GraphResidentFailure>,
}

impl ResearchEvidenceCorpus {
    pub fn builder(corpus_id: impl Into<String>) -> ResearchEvidenceCorpusBuilder {
        ResearchEvidenceCorpusBuilder {
            corpus_id: corpus_id.into(),
            evidence_references: Vec::new(),
            reusable_negative_evidence: Vec::new(),
            graph_resident_failures: Vec::new(),
        }
    }

    pub fn corpus_id(&self) -> &str {
        &self.corpus_id
    }

    pub fn corpus_digest(&self) -> &HadwigerArtifactDigest {
        self.artifact_digest()
    }

    pub fn evidence_references(&self) -> &[HadwigerDiscoveryEvidenceReference] {
        &self.evidence_references
    }

    pub fn graph_resident_failures(&self) -> &[GraphResidentFailure] {
        &self.graph_resident_failures
    }

    pub fn reusable_negative_evidence(&self) -> &[HadwigerReusableNegativeEvidence] {
        &self.reusable_negative_evidence
    }

    pub fn has_reference(&self, reference: &HadwigerArtifactReference) -> bool {
        self.evidence_references.iter().any(|evidence| {
            matches!(evidence, HadwigerDiscoveryEvidenceReference::Artifact { reference: stored } if stored == reference)
        })
    }

    pub fn rejected_evidence_available(&self) -> bool {
        self.evidence_references.iter().any(|evidence| {
            evidence
                .stable_token()
                .contains(HadwigerArtifactKind::RejectionExplanation.as_str())
        })
    }

    pub fn has_query_recovery_evidence(&self) -> bool {
        self.evidence_references.iter().any(|evidence| {
            matches!(
                evidence,
                HadwigerDiscoveryEvidenceReference::QueryRecovery { .. }
            )
        })
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(ResearchEvidenceCorpus, core);

pub struct ResearchEvidenceCorpusBuilder {
    corpus_id: String,
    evidence_references: Vec<HadwigerDiscoveryEvidenceReference>,
    reusable_negative_evidence: Vec<HadwigerReusableNegativeEvidence>,
    graph_resident_failures: Vec<GraphResidentFailure>,
}

impl ResearchEvidenceCorpusBuilder {
    pub fn with_graph_version(mut self, reference: HadwigerArtifactReference) -> Self {
        self.evidence_references
            .push(HadwigerDiscoveryEvidenceReference::artifact(reference));
        self
    }

    pub fn with_retained_artifact(mut self, reference: HadwigerArtifactReference) -> Self {
        self.evidence_references
            .push(HadwigerDiscoveryEvidenceReference::artifact(reference));
        self
    }

    pub fn with_checker_rejection(
        mut self,
        explanation: HadwigerRejectionExplanation,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let negative = explanation.reusable_negative_evidence().ok_or(
            HadwigerArtifactShapeError::EmptyField {
                field: "reusable_negative_evidence",
            },
        )?;
        self.evidence_references
            .push(HadwigerDiscoveryEvidenceReference::artifact(
                explanation.reference(),
            ));
        self.evidence_references
            .push(HadwigerDiscoveryEvidenceReference::artifact(
                negative.reference(),
            ));
        self.reusable_negative_evidence.push(negative.clone());
        Ok(self)
    }

    pub fn with_partial_admission(
        mut self,
        explanation: HadwigerPartialAdmissionExplanation,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        self.evidence_references
            .push(HadwigerDiscoveryEvidenceReference::artifact(
                explanation.reference(),
            ));
        for reference in explanation.surviving_evidence() {
            self.evidence_references
                .push(HadwigerDiscoveryEvidenceReference::artifact(
                    reference.clone(),
                ));
        }
        Ok(self)
    }

    pub fn with_query_recovery(mut self, explanation: HadwigerQueryRecoveryExplanation) -> Self {
        self.evidence_references
            .push(HadwigerDiscoveryEvidenceReference::artifact(
                explanation.reference(),
            ));
        self.evidence_references
            .push(HadwigerDiscoveryEvidenceReference::query_recovery(
                &explanation,
            ));
        self
    }

    pub fn with_graph_resident_failure(mut self, failure: GraphResidentFailure) -> Self {
        self.evidence_references
            .push(HadwigerDiscoveryEvidenceReference::artifact(
                failure.reference(),
            ));
        self.graph_resident_failures.push(failure);
        self
    }

    pub fn finish(self) -> Result<ResearchEvidenceCorpus, HadwigerArtifactShapeError> {
        let corpus_id = require_non_empty(self.corpus_id, "corpus_id")?;
        let mut evidence_references = self.evidence_references;
        evidence_references.sort_by_key(HadwigerDiscoveryEvidenceReference::stable_token);
        evidence_references.dedup();
        let mut reusable_negative_evidence = self.reusable_negative_evidence;
        reusable_negative_evidence.sort_by_key(|evidence| evidence.reference().stable_token());
        reusable_negative_evidence.dedup();
        let mut graph_resident_failures = self.graph_resident_failures;
        graph_resident_failures.sort_by_key(GraphResidentFailure::stable_token);
        graph_resident_failures.dedup();
        let core = artifact_core(
            HadwigerArtifactKind::ResearchEvidenceCorpus,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "research_evidence_corpus".to_string(),
            },
            Vec::new(),
            corpus_payload(
                &corpus_id,
                &evidence_references,
                &reusable_negative_evidence,
                &graph_resident_failures,
            ),
        )?;
        Ok(ResearchEvidenceCorpus {
            core,
            corpus_id,
            evidence_references,
            reusable_negative_evidence,
            graph_resident_failures,
        })
    }
}

fn corpus_payload(
    corpus_id: &str,
    evidence_references: &[HadwigerDiscoveryEvidenceReference],
    reusable_negative_evidence: &[HadwigerReusableNegativeEvidence],
    graph_resident_failures: &[GraphResidentFailure],
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![HadwigerArtifactPayloadEntry::text("corpus_id", corpus_id)];
    for evidence in evidence_references {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "evidence_reference",
            evidence.stable_token(),
        ));
    }
    for evidence in reusable_negative_evidence {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "reusable_negative_evidence",
            evidence.reference().stable_token(),
        ));
    }
    for failure in graph_resident_failures {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "graph_resident_failure",
            failure.stable_token(),
        ));
    }
    payload
}
