use super::scoped::FoundationalMergeScope;
use super::strategy::{
    FoundationalMergeBaseSelectionBasis, FoundationalMergeBasis, FoundationalStrategyBasis,
    FoundationalTransitionCorrespondenceBasis, FoundationalTransitionRemapBasis,
    FoundationalTransitionStrategyContractBasis, FoundationalTransitionStrategyDescriptorDigest,
    FoundationalTransitionStrategyIdentity,
};
use super::vocabulary::{
    FoundationalMergeConstructionDenial, FoundationalMergeIntent,
    FoundationalMergeStructuralSummary,
};
use crate::transitions::{FoundationalBranchId, FoundationalStagedBranchArtifact};

#[derive(Debug, Clone)]
pub struct FoundationalMergeBuilder<T> {
    source: FoundationalStagedBranchArtifact<T>,
    target_branch: Option<FoundationalBranchId>,
    intent: Option<FoundationalMergeIntent>,
    summary: Option<FoundationalMergeStructuralSummary>,
    scope: Option<FoundationalMergeScope>,
    merge_basis: Option<FoundationalMergeBasis>,
    merge_base_selection_basis: Option<FoundationalMergeBaseSelectionBasis>,
    strategy_identity: Option<FoundationalTransitionStrategyIdentity>,
    strategy_descriptor_digest: Option<FoundationalTransitionStrategyDescriptorDigest>,
    strategy_contract_basis: Option<FoundationalTransitionStrategyContractBasis>,
    strategy_basis: Option<FoundationalStrategyBasis>,
    correspondence_basis: Option<FoundationalTransitionCorrespondenceBasis>,
    remap_basis: Option<FoundationalTransitionRemapBasis>,
}

pub fn foundational_merge<T>(
    source: FoundationalStagedBranchArtifact<T>,
) -> FoundationalMergeBuilder<T> {
    FoundationalMergeBuilder {
        source,
        target_branch: None,
        intent: None,
        summary: None,
        scope: None,
        merge_basis: None,
        merge_base_selection_basis: None,
        strategy_identity: None,
        strategy_descriptor_digest: None,
        strategy_contract_basis: None,
        strategy_basis: None,
        correspondence_basis: None,
        remap_basis: None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalMergeCandidate<T> {
    source: FoundationalStagedBranchArtifact<T>,
    target_branch: FoundationalBranchId,
    intent: FoundationalMergeIntent,
    summary: FoundationalMergeStructuralSummary,
    scope: FoundationalMergeScope,
    merge_basis: FoundationalMergeBasis,
    merge_base_selection_basis: FoundationalMergeBaseSelectionBasis,
    strategy_identity: FoundationalTransitionStrategyIdentity,
    strategy_descriptor_digest: FoundationalTransitionStrategyDescriptorDigest,
    strategy_contract_basis: FoundationalTransitionStrategyContractBasis,
    strategy_basis: FoundationalStrategyBasis,
    correspondence_basis: Option<FoundationalTransitionCorrespondenceBasis>,
    remap_basis: Option<FoundationalTransitionRemapBasis>,
}

impl<T> FoundationalMergeBuilder<T> {
    pub fn into_target_branch(mut self, target_branch: FoundationalBranchId) -> Self {
        self.target_branch = Some(target_branch);
        self
    }

    pub fn with_intent(mut self, intent: FoundationalMergeIntent) -> Self {
        self.intent = Some(intent);
        self
    }

    pub fn with_structural_summary(mut self, summary: FoundationalMergeStructuralSummary) -> Self {
        self.summary = Some(summary);
        self
    }

    pub fn with_scope(mut self, scope: FoundationalMergeScope) -> Self {
        self.scope = Some(scope);
        self
    }

    pub fn with_merge_basis(mut self, merge_basis: FoundationalMergeBasis) -> Self {
        self.merge_basis = Some(merge_basis);
        self
    }

    pub fn with_merge_base_selection_basis(
        mut self,
        basis: FoundationalMergeBaseSelectionBasis,
    ) -> Self {
        self.merge_base_selection_basis = Some(basis);
        self
    }

    pub fn under_strategy(mut self, strategy: FoundationalTransitionStrategyIdentity) -> Self {
        self.strategy_identity = Some(strategy);
        self
    }

    pub fn with_strategy_descriptor_digest(
        mut self,
        digest: FoundationalTransitionStrategyDescriptorDigest,
    ) -> Self {
        self.strategy_descriptor_digest = Some(digest);
        self
    }

    pub fn with_strategy_contract_basis(
        mut self,
        basis: FoundationalTransitionStrategyContractBasis,
    ) -> Self {
        self.strategy_contract_basis = Some(basis);
        self
    }

    pub fn with_strategy_basis(mut self, basis: FoundationalStrategyBasis) -> Self {
        self.strategy_basis = Some(basis);
        self
    }

    pub fn under_correspondence_basis(
        mut self,
        basis: FoundationalTransitionCorrespondenceBasis,
    ) -> Self {
        self.correspondence_basis = Some(basis);
        self
    }

    pub fn under_remap_basis(mut self, basis: FoundationalTransitionRemapBasis) -> Self {
        self.remap_basis = Some(basis);
        self
    }

    pub fn plan(
        self,
    ) -> Result<FoundationalMergeCandidate<T>, FoundationalMergeConstructionDenial> {
        let target_branch = self
            .target_branch
            .ok_or(FoundationalMergeConstructionDenial::MissingTargetBranch)?;
        if self.source.branch_id() == &target_branch {
            return Err(FoundationalMergeConstructionDenial::SourceAndTargetBranchMustDiffer);
        }
        let merge_basis = self
            .merge_basis
            .ok_or(FoundationalMergeConstructionDenial::MissingMergeBasis)?;
        if merge_basis.source_branch() != self.source.branch_id() {
            return Err(FoundationalMergeConstructionDenial::MergeBasisSourceBranchMismatch);
        }
        if merge_basis.target_branch() != &target_branch {
            return Err(FoundationalMergeConstructionDenial::MergeBasisTargetBranchMismatch);
        }
        if self
            .source
            .comparison_basis()
            .is_some_and(|basis| basis.compared_against_branch() != &target_branch)
        {
            return Err(FoundationalMergeConstructionDenial::ComparisonBasisTargetBranchMismatch);
        }

        Ok(FoundationalMergeCandidate {
            source: self.source,
            target_branch,
            intent: self
                .intent
                .ok_or(FoundationalMergeConstructionDenial::MissingIntent)?,
            summary: self
                .summary
                .ok_or(FoundationalMergeConstructionDenial::MissingStructuralSummary)?,
            scope: self
                .scope
                .unwrap_or_else(FoundationalMergeScope::full_branch),
            merge_basis,
            merge_base_selection_basis: self
                .merge_base_selection_basis
                .ok_or(FoundationalMergeConstructionDenial::MissingMergeBaseSelectionBasis)?,
            strategy_identity: self
                .strategy_identity
                .ok_or(FoundationalMergeConstructionDenial::MissingStrategyIdentity)?,
            strategy_descriptor_digest: self
                .strategy_descriptor_digest
                .ok_or(FoundationalMergeConstructionDenial::MissingStrategyDescriptorDigest)?,
            strategy_contract_basis: self
                .strategy_contract_basis
                .ok_or(FoundationalMergeConstructionDenial::MissingStrategyContractBasis)?,
            strategy_basis: self
                .strategy_basis
                .ok_or(FoundationalMergeConstructionDenial::MissingStrategyBasis)?,
            correspondence_basis: self.correspondence_basis,
            remap_basis: self.remap_basis,
        })
    }
}

impl<T> FoundationalMergeCandidate<T> {
    pub fn source_branch(&self) -> &FoundationalBranchId {
        self.source.branch_id()
    }

    pub fn target_branch(&self) -> &FoundationalBranchId {
        &self.target_branch
    }

    pub const fn intent(&self) -> FoundationalMergeIntent {
        self.intent
    }

    pub const fn structural_summary(&self) -> FoundationalMergeStructuralSummary {
        self.summary
    }

    pub fn scope(&self) -> &FoundationalMergeScope {
        &self.scope
    }

    pub fn merge_basis(&self) -> &FoundationalMergeBasis {
        &self.merge_basis
    }

    pub const fn merge_base_selection_basis(&self) -> FoundationalMergeBaseSelectionBasis {
        self.merge_base_selection_basis
    }

    pub fn strategy_identity(&self) -> &FoundationalTransitionStrategyIdentity {
        &self.strategy_identity
    }

    pub const fn strategy_descriptor_digest(
        &self,
    ) -> FoundationalTransitionStrategyDescriptorDigest {
        self.strategy_descriptor_digest
    }

    pub const fn strategy_contract_basis(&self) -> FoundationalTransitionStrategyContractBasis {
        self.strategy_contract_basis
    }

    pub const fn strategy_basis(&self) -> FoundationalStrategyBasis {
        self.strategy_basis
    }

    pub const fn correspondence_basis(&self) -> Option<FoundationalTransitionCorrespondenceBasis> {
        self.correspondence_basis
    }

    pub const fn remap_basis(&self) -> Option<FoundationalTransitionRemapBasis> {
        self.remap_basis
    }

    pub fn payload(&self) -> &T {
        self.source.payload()
    }

    pub fn staged_branch(&self) -> &FoundationalStagedBranchArtifact<T> {
        &self.source
    }
}
