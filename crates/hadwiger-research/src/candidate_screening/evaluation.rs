use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;

use super::{CandidateScreeningInvariantCatalog, CandidateScreeningInvariantFamily};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CandidateScreeningVerdict {
    Passed,
    Rejected,
    Priority,
}

impl CandidateScreeningVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Rejected => "rejected",
            Self::Priority => "priority",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CandidateScreeningEvaluationMode {
    DirectGraphAlgorithm,
    CheckedCertificate,
    SolverBackedCertificate,
}

impl CandidateScreeningEvaluationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DirectGraphAlgorithm => "direct_graph_algorithm",
            Self::CheckedCertificate => "checked_certificate",
            Self::SolverBackedCertificate => "solver_backed_certificate",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateScreeningCertificate {
    family: CandidateScreeningInvariantFamily,
    subject: HadwigerArtifactReference,
    certificate_id: String,
    verdict: CandidateScreeningVerdict,
    checked_basis: String,
}

impl CandidateScreeningCertificate {
    pub fn checked(
        family: CandidateScreeningInvariantFamily,
        subject: HadwigerArtifactReference,
        certificate_id: impl Into<String>,
        verdict: CandidateScreeningVerdict,
        checked_basis: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            family,
            subject,
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            verdict,
            checked_basis: require_non_empty(checked_basis, "checked_basis")?,
        })
    }

    pub fn family(&self) -> CandidateScreeningInvariantFamily {
        self.family
    }

    pub fn subject(&self) -> &HadwigerArtifactReference {
        &self.subject
    }

    pub fn verdict(&self) -> CandidateScreeningVerdict {
        self.verdict
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.family.as_str(),
            self.subject.stable_token(),
            self.certificate_id,
            self.verdict.as_str(),
            self.checked_basis
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateScreeningEvaluation {
    core: HadwigerArtifactCore,
    family: CandidateScreeningInvariantFamily,
    subject: HadwigerArtifactReference,
    verdict: CandidateScreeningVerdict,
    mode: CandidateScreeningEvaluationMode,
    evidence: String,
}

impl CandidateScreeningEvaluation {
    pub(crate) fn new(
        catalog: &CandidateScreeningInvariantCatalog,
        family: CandidateScreeningInvariantFamily,
        subject: HadwigerArtifactReference,
        verdict: CandidateScreeningVerdict,
        mode: CandidateScreeningEvaluationMode,
        evidence: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let evidence = require_non_empty(evidence, "screening_evidence")?;
        let core = artifact_core(
            HadwigerArtifactKind::CandidateScreeningEvaluation,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "candidate_screening_evaluation".to_string(),
            },
            vec![catalog.reference(), subject.clone()],
            vec![
                HadwigerArtifactPayloadEntry::text("family", family.as_str()),
                HadwigerArtifactPayloadEntry::text("subject", subject.stable_token()),
                HadwigerArtifactPayloadEntry::text("verdict", verdict.as_str()),
                HadwigerArtifactPayloadEntry::text("mode", mode.as_str()),
                HadwigerArtifactPayloadEntry::text("evidence", evidence.clone()),
            ],
        )?;
        Ok(Self {
            core,
            family,
            subject,
            verdict,
            mode,
            evidence,
        })
    }

    pub fn family(&self) -> CandidateScreeningInvariantFamily {
        self.family
    }

    pub fn subject(&self) -> &HadwigerArtifactReference {
        &self.subject
    }

    pub fn verdict(&self) -> CandidateScreeningVerdict {
        self.verdict
    }

    pub fn mode(&self) -> CandidateScreeningEvaluationMode {
        self.mode
    }

    pub fn evidence(&self) -> &str {
        &self.evidence
    }

    pub fn rejects_candidate(&self) -> bool {
        self.verdict == CandidateScreeningVerdict::Rejected
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(CandidateScreeningEvaluation, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateScreeningEvaluationReport {
    core: HadwigerArtifactCore,
    evaluations: Vec<CandidateScreeningEvaluation>,
}

impl CandidateScreeningEvaluationReport {
    pub(crate) fn new(
        catalog: &CandidateScreeningInvariantCatalog,
        mut evaluations: Vec<CandidateScreeningEvaluation>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        evaluations.sort_by_key(|evaluation| evaluation.reference().stable_token());
        let mut parents = vec![catalog.reference()];
        parents.extend(
            evaluations
                .iter()
                .map(CandidateScreeningEvaluation::reference),
        );
        let core = artifact_core(
            HadwigerArtifactKind::CandidateScreeningEvaluationReport,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "candidate_screening_evaluation_report".to_string(),
            },
            parents,
            report_payload(&evaluations),
        )?;
        Ok(Self { core, evaluations })
    }

    pub fn evaluations(&self) -> &[CandidateScreeningEvaluation] {
        &self.evaluations
    }

    pub fn rejected_count(&self) -> usize {
        self.evaluations
            .iter()
            .filter(|evaluation| evaluation.rejects_candidate())
            .count()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(CandidateScreeningEvaluationReport, core);

fn report_payload(
    evaluations: &[CandidateScreeningEvaluation],
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![HadwigerArtifactPayloadEntry::unsigned(
        "evaluation_count",
        evaluations.len() as u128,
    )];
    for evaluation in evaluations {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "evaluation",
            evaluation.reference().stable_token(),
        ));
    }
    payload
}
