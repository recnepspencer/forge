use forge_query::facade::{
    ForgeQueryAdmissionContributionAuthoring, ForgeQueryContributionComposedOrchestrationInput,
    ForgeQueryContributionIntent, ForgeQuerySupportContributionAuthoring,
};

use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::{GraphVersion, HadwigerCanonicalArtifact};
use crate::domain_declarations::AdvisoryNoteDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::finite_graph_view::FiniteGraphView;
use super::CandidateScreeningError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CandidateScreeningAdvisoryPosture {
    Deprioritize,
    Neutral,
}

impl CandidateScreeningAdvisoryPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deprioritize => "deprioritize",
            Self::Neutral => "neutral",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateScreeningAdvisoryArtifact {
    core: HadwigerArtifactCore,
    graph_reference: HadwigerArtifactReference,
    maximum_degree: usize,
    threshold: usize,
    posture: CandidateScreeningAdvisoryPosture,
    detail: String,
}

impl CandidateScreeningAdvisoryArtifact {
    fn maximum_degree_sanity(
        graph_reference: HadwigerArtifactReference,
        maximum_degree: usize,
        threshold: usize,
        posture: CandidateScreeningAdvisoryPosture,
        detail: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let detail = require_non_empty(detail, "screening_advisory_detail")?;
        let core = artifact_core(
            HadwigerArtifactKind::CandidateScreeningAdvisoryArtifact,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "maximum_degree_sanity_advisory".to_string(),
            },
            vec![graph_reference.clone()],
            vec![
                HadwigerArtifactPayloadEntry::text("advisory_kind", "maximum_degree_sanity"),
                HadwigerArtifactPayloadEntry::unsigned("maximum_degree", maximum_degree as u128),
                HadwigerArtifactPayloadEntry::unsigned("threshold", threshold as u128),
                HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
                HadwigerArtifactPayloadEntry::text("detail", detail.clone()),
            ],
        )?;
        Ok(Self {
            core,
            graph_reference,
            maximum_degree,
            threshold,
            posture,
            detail,
        })
    }

    pub fn graph_reference(&self) -> &HadwigerArtifactReference {
        &self.graph_reference
    }

    pub fn maximum_degree(&self) -> usize {
        self.maximum_degree
    }

    pub fn threshold(&self) -> usize {
        self.threshold
    }

    pub fn posture(&self) -> CandidateScreeningAdvisoryPosture {
        self.posture
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(CandidateScreeningAdvisoryArtifact, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateScreeningAdvisoryContributionRecord {
    core: HadwigerArtifactCore,
    advisory_artifact: CandidateScreeningAdvisoryArtifact,
    query_contribution_digest: String,
}

impl CandidateScreeningAdvisoryContributionRecord {
    fn new(
        advisory_artifact: CandidateScreeningAdvisoryArtifact,
        query_contribution_digest: String,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let query_contribution_digest =
            require_non_empty(query_contribution_digest, "query_contribution_digest")?;
        let core = artifact_core(
            HadwigerArtifactKind::CandidateScreeningAdvisoryContributionRecord,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "candidate_screening_advisory_contribution_record".to_string(),
            },
            vec![advisory_artifact.reference()],
            vec![HadwigerArtifactPayloadEntry::text(
                "query_contribution_digest",
                query_contribution_digest.clone(),
            )],
        )?;
        Ok(Self {
            core,
            advisory_artifact,
            query_contribution_digest,
        })
    }

    pub fn advisory_artifact(&self) -> &CandidateScreeningAdvisoryArtifact {
        &self.advisory_artifact
    }

    pub fn query_contribution_digest(&self) -> Option<&str> {
        Some(&self.query_contribution_digest)
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(CandidateScreeningAdvisoryContributionRecord, core);

pub fn advise_maximum_degree_sanity_checked(
    handle: &HadwigerResearchHandle,
    declaration: AdvisoryNoteDeclaration,
    graph: &GraphVersion,
    threshold: usize,
) -> Result<CandidateScreeningAdvisoryContributionRecord, CandidateScreeningError> {
    let maximum_degree = FiniteGraphView::from_graph_version(graph).maximum_degree();
    let posture = if maximum_degree <= threshold {
        CandidateScreeningAdvisoryPosture::Deprioritize
    } else {
        CandidateScreeningAdvisoryPosture::Neutral
    };
    let detail = format!(
        "maximum_degree={maximum_degree};sanity_threshold={threshold};posture={}",
        posture.as_str()
    );
    let input = ForgeQueryContributionComposedOrchestrationInput::new(declaration)
        .with_contribution(ForgeQueryContributionIntent::admission(
            ForgeQueryAdmissionContributionAuthoring::advisory_at_stage(
                "candidate_screening",
                "hadwiger.screening.maximum_degree_sanity",
                detail.clone(),
            ),
        ))
        .with_contribution(ForgeQueryContributionIntent::support(
            ForgeQuerySupportContributionAuthoring::declaration_support(
                "hadwiger.screening.maximum_degree_sanity.support",
                detail.clone(),
            ),
        ));
    let proof = handle.orchestrate_declaration_with_contributions_proof(input);
    let query_contribution_digest = proof.contribution_digest().map(str::to_string).ok_or(
        CandidateScreeningError::QueryContributionDigestMissing {
            advisory: "maximum_degree_sanity",
        },
    )?;
    let advisory_artifact = CandidateScreeningAdvisoryArtifact::maximum_degree_sanity(
        graph.reference(),
        maximum_degree,
        threshold,
        posture,
        detail,
    )?;
    CandidateScreeningAdvisoryContributionRecord::new(advisory_artifact, query_contribution_digest)
        .map_err(Into::into)
}
