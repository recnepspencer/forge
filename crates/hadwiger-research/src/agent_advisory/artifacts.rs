use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;

use super::batch::AgentExplorationBatch;
use super::source::AgentSourceRecord;
use super::suggestions::{AgentAdvisoryKind, AgentPromotionPathDescriptor};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AgentAdvisoryError {
    Shape(HadwigerArtifactShapeError),
    EvidenceNotInCorpus {
        reference_token: String,
    },
    MissingQueryContributionDigest,
    QueryContributionStopped {
        stop_kind: AgentQueryContributionStopKind,
    },
    GroupedContributionStopped {
        stop_kind: AgentGroupedContributionStopKind,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentQueryContributionStopKind {
    Deferred,
    DeclarationDenied,
    ContributionDenied,
    Stale,
    RebindRequired,
    Unsupported,
    Failed,
}

impl From<HadwigerArtifactShapeError> for AgentAdvisoryError {
    fn from(value: HadwigerArtifactShapeError) -> Self {
        Self::Shape(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentGroupedContributionStopKind {
    DeclarationStopped,
    MemberStopped,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentAdvisoryArtifact {
    core: HadwigerArtifactCore,
    advisory_id: String,
    advisory_kind: AgentAdvisoryKind,
    source: AgentSourceRecord,
    cited_evidence: Vec<HadwigerArtifactReference>,
    detail: String,
    promotion_path: AgentPromotionPathDescriptor,
}

impl AgentAdvisoryArtifact {
    pub(crate) fn new(
        advisory_id: impl Into<String>,
        advisory_kind: AgentAdvisoryKind,
        source: AgentSourceRecord,
        mut cited_evidence: Vec<HadwigerArtifactReference>,
        detail: impl Into<String>,
        promotion_path: AgentPromotionPathDescriptor,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let advisory_id = require_non_empty(advisory_id, "advisory_id")?;
        let detail = require_non_empty(detail, "detail")?;
        cited_evidence.sort_by_key(HadwigerArtifactReference::stable_token);
        cited_evidence.dedup();
        let core = artifact_core(
            HadwigerArtifactKind::AgentAdvisoryArtifact,
            HadwigerArtifactAuthorityOwner::AgentAdvisory,
            HadwigerArtifactSourceReference::AgentAdvisory {
                source_digest: source.source_digest(),
            },
            cited_evidence.clone(),
            artifact_payload(
                &advisory_id,
                advisory_kind,
                &source,
                &detail,
                promotion_path,
            ),
        )?;
        Ok(Self {
            core,
            advisory_id,
            advisory_kind,
            source,
            cited_evidence,
            detail,
            promotion_path,
        })
    }

    pub fn advisory_id(&self) -> &str {
        &self.advisory_id
    }

    pub fn advisory_kind(&self) -> AgentAdvisoryKind {
        self.advisory_kind
    }

    pub fn source(&self) -> &AgentSourceRecord {
        &self.source
    }

    pub fn cited_evidence(&self) -> &[HadwigerArtifactReference] {
        &self.cited_evidence
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn promotion_path(&self) -> AgentPromotionPathDescriptor {
        self.promotion_path
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }

    pub fn admits_checker_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(AgentAdvisoryArtifact, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentAdvisoryContributionRecord {
    core: HadwigerArtifactCore,
    advisory_artifact: AgentAdvisoryArtifact,
    query_contribution_digest: String,
}

impl AgentAdvisoryContributionRecord {
    pub(crate) fn new(
        advisory_artifact: AgentAdvisoryArtifact,
        query_contribution_digest: String,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let query_contribution_digest =
            require_non_empty(query_contribution_digest, "query_contribution_digest")?;
        let core = artifact_core(
            HadwigerArtifactKind::AgentAdvisoryContributionRecord,
            HadwigerArtifactAuthorityOwner::AgentAdvisory,
            HadwigerArtifactSourceReference::AgentAdvisory {
                source_digest: advisory_artifact.source().source_digest(),
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

    pub fn advisory_artifact(&self) -> &AgentAdvisoryArtifact {
        &self.advisory_artifact
    }

    pub fn query_contribution_digest(&self) -> Option<&str> {
        Some(&self.query_contribution_digest)
    }
}

impl_hadwiger_artifact!(AgentAdvisoryContributionRecord, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentExplorationAdmissionChecked {
    batch: AgentExplorationBatch,
    advisory_artifacts: Vec<AgentAdvisoryArtifact>,
}

impl AgentExplorationAdmissionChecked {
    pub(crate) fn new(
        batch: AgentExplorationBatch,
        mut advisory_artifacts: Vec<AgentAdvisoryArtifact>,
    ) -> Self {
        advisory_artifacts.sort_by_key(|artifact| artifact.reference().stable_token());
        Self {
            batch,
            advisory_artifacts,
        }
    }

    pub fn batch(&self) -> &AgentExplorationBatch {
        &self.batch
    }

    pub fn advisory_artifacts(&self) -> &[AgentAdvisoryArtifact] {
        &self.advisory_artifacts
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentExperimentProposalScreening {
    core: HadwigerArtifactCore,
    accepted_proposals: Vec<AgentAdvisoryArtifact>,
    blocked_proposals: Vec<AgentAdvisoryArtifact>,
    blocked_reasons: Vec<String>,
}

impl AgentExperimentProposalScreening {
    pub(crate) fn new(
        source_digest: String,
        accepted_proposals: Vec<AgentAdvisoryArtifact>,
        blocked_proposals: Vec<AgentAdvisoryArtifact>,
        blocked_reasons: Vec<String>,
        context_tokens: Vec<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let mut parents = accepted_proposals
            .iter()
            .chain(blocked_proposals.iter())
            .map(AgentAdvisoryArtifact::reference)
            .collect::<Vec<_>>();
        parents.sort_by_key(HadwigerArtifactReference::stable_token);
        parents.dedup();
        let mut payload = context_tokens
            .into_iter()
            .map(|token| HadwigerArtifactPayloadEntry::text("context", token))
            .collect::<Vec<_>>();
        for reason in &blocked_reasons {
            payload.push(HadwigerArtifactPayloadEntry::text("blocked_reason", reason));
        }
        let core = artifact_core(
            HadwigerArtifactKind::AgentExperimentProposalScreening,
            HadwigerArtifactAuthorityOwner::AgentAdvisory,
            HadwigerArtifactSourceReference::AgentAdvisory { source_digest },
            parents,
            payload,
        )?;
        Ok(Self {
            core,
            accepted_proposals,
            blocked_proposals,
            blocked_reasons,
        })
    }

    pub fn accepted_proposals(&self) -> &[AgentAdvisoryArtifact] {
        &self.accepted_proposals
    }

    pub fn blocked_proposals(&self) -> &[AgentAdvisoryArtifact] {
        &self.blocked_proposals
    }

    pub fn blocked_reasons(&self) -> &[String] {
        &self.blocked_reasons
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(AgentExperimentProposalScreening, core);

pub(crate) fn proposal_artifacts(
    checked: &AgentExplorationAdmissionChecked,
) -> Vec<AgentAdvisoryArtifact> {
    checked
        .advisory_artifacts()
        .iter()
        .filter(|artifact| artifact.advisory_kind() == AgentAdvisoryKind::ExperimentProposal)
        .cloned()
        .collect()
}

pub(crate) fn source_digest_from_artifacts(
    artifacts: &[AgentAdvisoryArtifact],
    fallback: &AgentExplorationAdmissionChecked,
) -> String {
    artifacts
        .first()
        .map(|artifact| artifact.source().source_digest())
        .unwrap_or_else(|| fallback.batch().source().source_digest())
}

fn artifact_payload(
    advisory_id: &str,
    advisory_kind: AgentAdvisoryKind,
    source: &AgentSourceRecord,
    detail: &str,
    promotion_path: AgentPromotionPathDescriptor,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text("advisory_id", advisory_id),
        HadwigerArtifactPayloadEntry::text("advisory_kind", advisory_kind.as_str()),
        HadwigerArtifactPayloadEntry::text("source", source.stable_token()),
        HadwigerArtifactPayloadEntry::text("detail", detail),
        HadwigerArtifactPayloadEntry::text("promotion_path", promotion_path.as_str()),
    ]
}
