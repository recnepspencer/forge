use forge_query::facade::{
    ForgeQueryAdmissionContributionAuthoring, ForgeQueryContributionComposedOrchestrationInput,
    ForgeQueryContributionIntent, ForgeQueryGroupedContributionComposition,
    ForgeQueryGroupedContributionInput, ForgeQueryGroupedContributionStop,
    ForgeQuerySupportContributionAuthoring,
};

use crate::discovery_loop::{DiscoveryFrontier, ResearchEvidenceCorpus};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::domain_declarations::{AdvisoryNoteDeclaration, HadwigerResearchDeclarationInput};
use crate::query_entry::{HadwigerResearchDomainEntry, HadwigerResearchHandle};
use crate::research_graph_invariants::{
    HadwigerResearchInvariantCatalog, ResearchGraphInvariantFamily,
};

use super::artifacts::{
    proposal_artifacts, source_digest_from_artifacts, AgentAdvisoryArtifact,
    AgentAdvisoryContributionRecord, AgentAdvisoryError, AgentExperimentProposalScreening,
    AgentExplorationAdmissionChecked, AgentGroupedContributionStopKind,
};
use super::batch::{AgentBatchEntry, AgentExplorationBatch};
use super::source::AgentSourceRecord;
use super::suggestions::{AgentAdmissionAdvisory, AgentAdvisoryKind, AgentPromotionPathDescriptor};

pub fn admit_agent_exploration_batch_checked(
    _handle: &HadwigerResearchHandle,
    corpus: &ResearchEvidenceCorpus,
    batch: AgentExplorationBatch,
) -> Result<AgentExplorationAdmissionChecked, AgentAdvisoryError> {
    for entry in batch.entries() {
        for reference in entry.cited_references() {
            if !corpus.has_reference(reference) {
                return Err(AgentAdvisoryError::EvidenceNotInCorpus {
                    reference_token: reference.stable_token(),
                });
            }
        }
    }
    let mut artifacts = Vec::new();
    for entry in batch.entries().iter().cloned() {
        artifacts.push(artifact_from_batch_entry(batch.source().clone(), entry)?);
    }
    Ok(AgentExplorationAdmissionChecked::new(batch, artifacts))
}

pub fn materialize_agent_declaration_advisory_checked(
    handle: &HadwigerResearchHandle,
    declaration: AdvisoryNoteDeclaration,
    advisory: AgentAdmissionAdvisory,
) -> Result<AgentAdvisoryContributionRecord, AgentAdvisoryError> {
    let source =
        AgentSourceRecord::declaration_advisory(advisory.candidate_id(), advisory.detail());
    let admission = ForgeQueryContributionIntent::admission(query_admission(advisory.clone()));
    let support = ForgeQueryContributionIntent::support(query_support(&advisory));
    let input = ForgeQueryContributionComposedOrchestrationInput::new(declaration.clone())
        .with_contribution(admission)
        .with_contribution(support);
    let proof = handle.orchestrate_declaration_with_contributions_proof(input);
    let query_contribution_digest = proof
        .contribution_digest()
        .map(str::to_string)
        .ok_or(AgentAdvisoryError::MissingQueryContributionDigest)?;
    let advisory_artifact = AgentAdvisoryArtifact::new(
        format!("declaration-advisory:{}", advisory.candidate_id()),
        advisory.kind(),
        source,
        Vec::new(),
        advisory.detail(),
        AgentPromotionPathDescriptor::NoDirectPromotion,
    )?;
    AgentAdvisoryContributionRecord::new(advisory_artifact, query_contribution_digest)
        .map_err(AgentAdvisoryError::from)
}

pub fn screen_agent_experiment_proposals_checked(
    _handle: &HadwigerResearchHandle,
    corpus: &ResearchEvidenceCorpus,
    frontier: &DiscoveryFrontier,
    catalog: &HadwigerResearchInvariantCatalog,
    checked: AgentExplorationAdmissionChecked,
) -> Result<AgentExperimentProposalScreening, AgentAdvisoryError> {
    let proposals = proposal_artifacts(&checked);
    let mut accepted = Vec::new();
    let mut blocked = Vec::new();
    let mut reasons = Vec::new();
    for proposal in proposals {
        if proposal_passes_existing_gates(corpus, frontier, catalog) {
            accepted.push(proposal);
        } else {
            reasons.push("phase7_suppression_or_reactivation_required".to_string());
            blocked.push(proposal);
        }
    }
    let context = vec![
        format!("corpus:{}", corpus.corpus_digest().stable_token()),
        format!("frontier:{}", frontier.artifact_digest().stable_token()),
        format!("catalog:{}", catalog.artifact_digest().stable_token()),
        format!("catalog_rules:{}", catalog.rules().len()),
    ];
    AgentExperimentProposalScreening::new(
        source_digest_from_artifacts(&accepted, &checked),
        accepted,
        blocked,
        reasons,
        context,
    )
    .map_err(AgentAdvisoryError::from)
}

pub fn materialize_agent_grouped_advisory_checked<I>(
    handle: &HadwigerResearchHandle,
    input: ForgeQueryGroupedContributionInput<HadwigerResearchDomainEntry, I>,
    advisory_artifact: AgentAdvisoryArtifact,
) -> Result<AgentAdvisoryContributionRecord, AgentAdvisoryError>
where
    I: HadwigerResearchDeclarationInput + Clone,
{
    let composition = handle
        .grouped_contributions_checked(input)
        .map_err(|stop| AgentAdvisoryError::GroupedContributionStopped {
            stop_kind: grouped_stop_kind(stop),
        })?;
    let grouped_digest = grouped_contribution_digest(&composition);
    AgentAdvisoryContributionRecord::new(advisory_artifact, grouped_digest)
        .map_err(AgentAdvisoryError::from)
}

fn grouped_stop_kind<I>(
    stop: ForgeQueryGroupedContributionStop<HadwigerResearchDomainEntry, I>,
) -> AgentGroupedContributionStopKind
where
    I: HadwigerResearchDeclarationInput,
{
    match stop {
        ForgeQueryGroupedContributionStop::DeclarationStopped(_) => {
            AgentGroupedContributionStopKind::DeclarationStopped
        }
        ForgeQueryGroupedContributionStop::MemberStopped(_, _) => {
            AgentGroupedContributionStopKind::MemberStopped
        }
        ForgeQueryGroupedContributionStop::WrongWorld(_)
        | ForgeQueryGroupedContributionStop::WrongHandle(_) => {
            AgentGroupedContributionStopKind::MemberStopped
        }
    }
}

fn proposal_passes_existing_gates(
    corpus: &ResearchEvidenceCorpus,
    frontier: &DiscoveryFrontier,
    catalog: &HadwigerResearchInvariantCatalog,
) -> bool {
    corpus.reusable_negative_evidence().is_empty()
        && !frontier.admits_theorem_authority()
        && !frontier.registers_query_invariant_authority()
        && catalog_has_phase8_gate_coverage(catalog)
}

fn catalog_has_phase8_gate_coverage(catalog: &HadwigerResearchInvariantCatalog) -> bool {
    [
        ResearchGraphInvariantFamily::FailureResidency,
        ResearchGraphInvariantFamily::SuppressionRelation,
        ResearchGraphInvariantFamily::HypothesisLifecycle,
        ResearchGraphInvariantFamily::BranchPromotion,
        ResearchGraphInvariantFamily::ExecutableExperimentAdmission,
    ]
    .into_iter()
    .all(|family| catalog.has_rule_family(family))
}

fn artifact_from_batch_entry(
    source: AgentSourceRecord,
    entry: AgentBatchEntry,
) -> Result<AgentAdvisoryArtifact, AgentAdvisoryError> {
    let (id, kind, refs, detail, promotion) = match entry {
        AgentBatchEntry::Motif(value) => value.into_advisory_parts(),
        AgentBatchEntry::Invariant(value) => value.into_advisory_parts(),
        AgentBatchEntry::Experiment(value) => value.into_advisory_parts(),
        AgentBatchEntry::Repair(value) => value.into_advisory_parts(),
    };
    AgentAdvisoryArtifact::new(id, kind, source, refs, detail, promotion)
        .map_err(AgentAdvisoryError::from)
}

fn grouped_contribution_digest<I>(
    composition: &ForgeQueryGroupedContributionComposition<HadwigerResearchDomainEntry, I>,
) -> String
where
    I: HadwigerResearchDeclarationInput,
{
    let mut member_digests = composition
        .members()
        .iter()
        .map(|(context, member)| {
            format!(
                "{}:{}:{}:{}",
                context.member_index(),
                context.shared_contribution_count(),
                context.member_contribution_count(),
                member
                    .composition_identity()
                    .terminal_projection_for_reporting()
            )
        })
        .collect::<Vec<_>>();
    member_digests.sort();
    format!(
        "grouped_query_contributions:{}:{}",
        composition.declaration().group_digest(),
        member_digests.join("|")
    )
}

fn query_admission(advisory: AgentAdmissionAdvisory) -> ForgeQueryAdmissionContributionAuthoring {
    let semantic_code = format!("hadwiger.agent.{}", advisory.kind().as_str());
    match advisory.kind() {
        AgentAdvisoryKind::AdmissionViolation => {
            ForgeQueryAdmissionContributionAuthoring::violation_at_stage(
                "agent_advisory",
                semantic_code,
                advisory.detail(),
            )
        }
        _ => ForgeQueryAdmissionContributionAuthoring::advisory_at_stage(
            "agent_advisory",
            semantic_code,
            advisory.detail(),
        ),
    }
}

fn query_support(advisory: &AgentAdmissionAdvisory) -> ForgeQuerySupportContributionAuthoring {
    ForgeQuerySupportContributionAuthoring::declaration_support(
        format!("hadwiger.agent.{}.support", advisory.kind().as_str()),
        advisory.detail(),
    )
}
