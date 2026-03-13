use std::collections::BTreeMap;
use std::sync::Arc;

use crate::history::data::{BranchId, CommitId, VersionNode};
use crate::identity::data::VersionId;
use crate::logic::runtime::state::subsystems::RuntimeSubsystem;
use crate::publication::data::diff::PatchStreamPosition;
use crate::replay::data::CanonicalCommitEnvelope;

#[derive(Debug, Clone)]
pub(crate) struct HistorySubsystem {
    pub(crate) branch_heads: BTreeMap<BranchId, Option<crate::history::data::CommitReference>>,
    pub(crate) commit_graph: BTreeMap<crate::history::data::CommitId, VersionNode>,
    pub(crate) commit_envelopes:
        BTreeMap<crate::history::data::CommitId, Arc<CanonicalCommitEnvelope>>,
    pub(crate) patch_stream_index: BTreeMap<PatchStreamPosition, crate::history::data::CommitId>,
    pub(crate) next_commit_id: u64,
    pub(crate) next_version_id: u64,
}

impl HistorySubsystem {
    fn build_with_main_branch(main_branch: BranchId) -> Self {
        Self {
            branch_heads: BTreeMap::from([(main_branch, None)]),
            commit_graph: BTreeMap::new(),
            commit_envelopes: BTreeMap::new(),
            patch_stream_index: BTreeMap::new(),
            next_commit_id: 1,
            next_version_id: 1,
        }
    }

    pub(crate) fn preview_next_commit_id(&self) -> CommitId {
        CommitId(self.next_commit_id)
    }

    pub(crate) fn preview_next_version_id(&self) -> VersionId {
        VersionId(self.next_version_id)
    }

    pub(crate) fn current_version_id(&self) -> VersionId {
        VersionId(self.next_version_id.saturating_sub(1))
    }

    pub(crate) fn advance_commit_sequence(&mut self) {
        self.next_commit_id += 1;
        self.next_version_id += 1;
    }
}

impl RuntimeSubsystem for HistorySubsystem {
    type Config = BranchId;

    fn new(config: &Self::Config) -> Self {
        Self::build_with_main_branch(config.clone())
    }

    fn fork(&self) -> Self {
        self.clone()
    }
}
