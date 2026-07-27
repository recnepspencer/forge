use std::sync::Arc;

use crate::domain_computation::provider_session::{
    WorthQueryFreshDecisionReadSet, WorthQuerySessionEffectAuthority,
};
use crate::execution_digest::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProvisionalProposalBasis {
    identity: Arc<str>,
    source_occurrence: Arc<str>,
    search_occurrence: Arc<str>,
    candidate_identity: Arc<str>,
    transformation_evidence: Arc<str>,
    semantic_basis_identity: Arc<str>,
    target_generation: u64,
    installed_policy_identity: Arc<str>,
    correspondence_identity: Arc<str>,
    identity_consequence_identity: Arc<str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProvisionalProposalBasisParts {
    pub source_occurrence: String,
    pub search_occurrence: String,
    pub candidate_identity: String,
    pub transformation_evidence: String,
    pub semantic_basis_identity: String,
    pub target_generation: u64,
    pub installed_policy_identity: String,
    pub correspondence_identity: String,
    pub identity_consequence_identity: String,
}

impl WorthQueryProvisionalProposalBasis {
    pub(crate) fn new(
        parts: WorthQueryProvisionalProposalBasisParts,
    ) -> Result<Self, super::WorthQueryProvisionalFailure> {
        let text = [
            &parts.source_occurrence,
            &parts.search_occurrence,
            &parts.candidate_identity,
            &parts.transformation_evidence,
            &parts.semantic_basis_identity,
            &parts.installed_policy_identity,
            &parts.correspondence_identity,
            &parts.identity_consequence_identity,
        ];
        if text
            .into_iter()
            .any(|value| value.trim().is_empty() || value.trim() != value)
        {
            return Err(super::WorthQueryProvisionalFailure::invalid_program(
                "proposal basis fields must be non-empty canonical text",
            ));
        }
        let identity = hash_parts(&[
            "worth_query_provisional_proposal_basis_v1".to_owned(),
            parts.source_occurrence.clone(),
            parts.search_occurrence.clone(),
            parts.candidate_identity.clone(),
            parts.transformation_evidence.clone(),
            parts.semantic_basis_identity.clone(),
            parts.target_generation.to_string(),
            parts.installed_policy_identity.clone(),
            parts.correspondence_identity.clone(),
            parts.identity_consequence_identity.clone(),
        ]);
        Ok(Self {
            identity: identity.into(),
            source_occurrence: parts.source_occurrence.into(),
            search_occurrence: parts.search_occurrence.into(),
            candidate_identity: parts.candidate_identity.into(),
            transformation_evidence: parts.transformation_evidence.into(),
            semantic_basis_identity: parts.semantic_basis_identity.into(),
            target_generation: parts.target_generation,
            installed_policy_identity: parts.installed_policy_identity.into(),
            correspondence_identity: parts.correspondence_identity.into(),
            identity_consequence_identity: parts.identity_consequence_identity.into(),
        })
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn semantic_basis_identity(&self) -> &str {
        &self.semantic_basis_identity
    }

    pub fn target_generation(&self) -> u64 {
        self.target_generation
    }

    pub fn dimensions(&self) -> [&str; 8] {
        [
            &self.source_occurrence,
            &self.search_occurrence,
            &self.candidate_identity,
            &self.transformation_evidence,
            &self.installed_policy_identity,
            &self.correspondence_identity,
            &self.identity_consequence_identity,
            &self.semantic_basis_identity,
        ]
    }
}

impl WorthQuerySessionEffectAuthority<'_> {
    pub fn admit_proposal_basis(
        &self,
        read_set: &WorthQueryFreshDecisionReadSet,
        parts: WorthQueryProvisionalProposalBasisParts,
    ) -> Result<WorthQueryProvisionalProposalBasis, super::WorthQueryProvisionalFailure> {
        if !read_set.belongs_to(self.binding().canonical_identity())
            || parts.semantic_basis_identity != self.plan().basis_identity()
        {
            return Err(super::WorthQueryProvisionalFailure::new(
                super::WorthQueryProvisionalDenialKind::ProposalBasisMismatch,
                "proposal does not belong to the exact session and semantic basis",
            ));
        }
        let proposal = WorthQueryProvisionalProposalBasis::new(parts)?;
        if proposal
            .dimensions()
            .into_iter()
            .take(7)
            .any(|identity| !read_set.contains_locator(identity))
        {
            return Err(super::WorthQueryProvisionalFailure::new(
                super::WorthQueryProvisionalDenialKind::ProposalBasisMismatch,
                "proposal provenance was not observed in the admitted decision read-set",
            ));
        }
        Ok(proposal)
    }
}
