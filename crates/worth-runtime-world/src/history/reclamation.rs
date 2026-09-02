use crate::identity::{CompositeCommitIdentity, RuntimeWorldOwnerIdentity};

use super::catalog::CompositeHistoryCatalogDenial;

/// Explicit maintenance input. Product heads and retained obligations are
/// represented by live catalog-owned protection obligations, not copied
/// ancestry supplied with this request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CompositeHistoryReclamationRequest {
    owner: RuntimeWorldOwnerIdentity,
    candidate_commits: Vec<CompositeCommitIdentity>,
    maximum_reclaims: usize,
    age_ticks: u64,
}

impl CompositeHistoryReclamationRequest {
    pub(crate) fn new(
        owner: RuntimeWorldOwnerIdentity,
        candidate_commits: Vec<CompositeCommitIdentity>,
        maximum_reclaims: usize,
        age_ticks: u64,
    ) -> Self {
        Self {
            owner,
            candidate_commits,
            maximum_reclaims,
            age_ticks,
        }
    }

    pub(crate) const fn owner(&self) -> RuntimeWorldOwnerIdentity {
        self.owner
    }

    pub(crate) fn candidate_commits(&self) -> &[CompositeCommitIdentity] {
        &self.candidate_commits
    }

    pub(crate) const fn maximum_reclaims(&self) -> usize {
        self.maximum_reclaims
    }

    pub(crate) const fn age_ticks(&self) -> u64 {
        self.age_ticks
    }
}

/// Reclamation is an observation of a bounded maintenance batch, not a
/// promise that every requested candidate can be removed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HistoryReclamationOutcome {
    maximum_reclaims: usize,
    examined: usize,
    skipped_protected: usize,
    skipped_too_young: usize,
    skipped_with_descendant_dependencies: usize,
    reclaimed: Vec<CompositeCommitIdentity>,
    metadata_bytes_reclaimed: usize,
}

impl HistoryReclamationOutcome {
    pub(crate) fn new(maximum_reclaims: usize) -> Self {
        Self {
            maximum_reclaims,
            examined: 0,
            skipped_protected: 0,
            skipped_too_young: 0,
            skipped_with_descendant_dependencies: 0,
            reclaimed: Vec::new(),
            metadata_bytes_reclaimed: 0,
        }
    }

    pub(crate) fn examined_one(&mut self) {
        self.examined += 1;
    }

    pub(crate) fn record_skipped_protected(&mut self) {
        self.skipped_protected += 1;
    }

    pub(crate) fn record_skipped_too_young(&mut self) {
        self.skipped_too_young += 1;
    }

    pub(crate) fn record_skipped_with_descendant_dependencies(&mut self) {
        self.skipped_with_descendant_dependencies += 1;
    }

    pub(crate) fn reclaimed_one(
        &mut self,
        identity: CompositeCommitIdentity,
        metadata_bytes: usize,
    ) {
        debug_assert!(self.reclaimed.len() < self.maximum_reclaims);
        self.metadata_bytes_reclaimed = self
            .metadata_bytes_reclaimed
            .checked_add(metadata_bytes)
            .expect("a bounded reclamation outcome fits addressable memory");
        self.reclaimed.push(identity);
    }

    pub(crate) const fn maximum_reclaims(&self) -> usize {
        self.maximum_reclaims
    }

    pub(crate) const fn examined(&self) -> usize {
        self.examined
    }

    pub(crate) const fn skipped_protected(&self) -> usize {
        self.skipped_protected
    }

    pub(crate) const fn skipped_too_young(&self) -> usize {
        self.skipped_too_young
    }

    pub(crate) const fn skipped_with_descendant_dependencies(&self) -> usize {
        self.skipped_with_descendant_dependencies
    }

    pub(crate) fn reclaimed_commits(&self) -> &[CompositeCommitIdentity] {
        &self.reclaimed
    }

    pub(crate) const fn metadata_bytes_reclaimed(&self) -> usize {
        self.metadata_bytes_reclaimed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HistoryReclamationDenial {
    Catalog(CompositeHistoryCatalogDenial),
    ForeignCandidate {
        expected: RuntimeWorldOwnerIdentity,
        actual: RuntimeWorldOwnerIdentity,
    },
    DuplicateCandidate(CompositeCommitIdentity),
    UnknownCandidate(CompositeCommitIdentity),
}
