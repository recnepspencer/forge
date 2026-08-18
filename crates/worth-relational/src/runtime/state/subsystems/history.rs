use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::branch::{RelationalBranchCellCheckpoint, RelationalBranchReferenceCell};
use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::{BranchId, CommitId, VersionNode};
use crate::history::RelationalCommitCatalog;
use crate::identity::data::VersionId;
use crate::publication::patch::data::PatchStreamPosition;
use crate::runtime::state::subsystems::RuntimeSubsystem;

#[path = "history_recovery.rs"]
mod history_recovery;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RelationalPhase4ReferenceCostCounters {
    pub branch_cell_lookups: u64,
    pub catalog_lookups: u64,
    /// Count of immutable catalog-artifact materializations. This is backed
    /// by the catalog's only envelope-to-artifact boundary, so a fork cannot
    /// hide eager reconstruction behind an unchanged fork-local counter.
    pub artifact_clones: u64,
    pub reference_allocations: u64,
    pub branch_cell_contacts: u64,
    /// Number of operations that inspect the whole branch population. Phase 4
    /// fork paths must leave this at zero; later sharing/retention lanes own
    /// any population-wide maintenance accounting.
    pub branch_population_scans: u64,
}

#[derive(Debug, Clone)]
pub(crate) struct HistorySubsystem {
    pub(crate) runtime_instance_id: u64,
    pub(crate) main_branch: BranchId,
    branch_cells: BTreeMap<BranchId, RelationalBranchReferenceCell>,
    pub(crate) commit_catalog: RelationalCommitCatalog,
    pub(crate) phase4_costs: RelationalPhase4ReferenceCostCounters,
    /// Population-wide reference traversal is deliberately hidden behind
    /// named diagnostics/checkpoint methods.  Keeping the counter separate
    /// lets those shared-`&self` methods record a scan without making the
    /// operational counters interior-mutable.
    branch_population_scans: Arc<AtomicU64>,
    pub(crate) commit_graph: BTreeMap<crate::history::data::CommitId, VersionNode>,
    pub(crate) commit_envelopes:
        BTreeMap<crate::history::data::CommitId, Arc<CanonicalCommitEnvelope>>,
    pub(crate) patch_stream_index: BTreeMap<PatchStreamPosition, crate::history::data::CommitId>,
    pub(crate) next_commit_id: u64,
    pub(crate) next_version_id: u64,
}

impl HistorySubsystem {
    fn build_with_main_branch(main_branch: BranchId) -> Self {
        let main_cell = RelationalBranchReferenceCell::empty(0, main_branch.clone())
            .expect("configured main branch must have a valid identity");
        Self {
            runtime_instance_id: 0,
            main_branch: main_branch.clone(),
            branch_cells: BTreeMap::from([(main_branch.clone(), main_cell)]),
            commit_catalog: RelationalCommitCatalog::default(),
            phase4_costs: RelationalPhase4ReferenceCostCounters::default(),
            branch_population_scans: Arc::new(AtomicU64::new(0)),
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

    pub(crate) fn advance_commit_sequence(&mut self) -> Result<(), &'static str> {
        self.next_commit_id = self
            .next_commit_id
            .checked_add(1)
            .ok_or("commit id sequence overflow")?;
        self.next_version_id = self
            .next_version_id
            .checked_add(1)
            .ok_or("version id sequence overflow")?;
        Ok(())
    }

    pub(crate) fn advance_metadata_commit_sequence(&mut self) -> Result<(), &'static str> {
        self.next_commit_id = self
            .next_commit_id
            .checked_add(1)
            .ok_or("metadata commit id sequence overflow")?;
        Ok(())
    }

    pub(crate) fn prepare_recovery_sequence(&mut self, commit_id: CommitId, version_id: VersionId) {
        self.next_commit_id = self.next_commit_id.max(commit_id.0);
        self.next_version_id = self.next_version_id.max(version_id.0);
    }

    pub(crate) fn set_runtime_instance_id(&mut self, runtime_instance_id: u64) {
        self.runtime_instance_id = runtime_instance_id;
        self.record_branch_population_scan();
        let cells = std::mem::take(&mut self.branch_cells);
        self.branch_cells = cells
            .into_values()
            .map(|cell| {
                let rebound = cell
                    .rebind_runtime(runtime_instance_id)
                    .expect("existing branch identities must remain valid when rebound");
                (rebound.identity().branch_id().clone(), rebound)
            })
            .collect();
    }

    pub(crate) fn fork_for_runtime(&self, runtime_instance_id: u64) -> Self {
        let mut fork = self.clone();
        fork.branch_population_scans = Arc::new(AtomicU64::new(
            self.branch_population_scans.load(Ordering::Relaxed),
        ));
        fork.set_runtime_instance_id(runtime_instance_id);
        fork
    }

    pub(crate) fn branch_cell(
        &self,
        branch_id: &BranchId,
    ) -> Option<&RelationalBranchReferenceCell> {
        self.branch_cells.get(branch_id)
    }

    pub(crate) fn phase4_costs(&self) -> RelationalPhase4ReferenceCostCounters {
        let mut costs = self.phase4_costs;
        costs.artifact_clones = self.history_materialization_count();
        costs.branch_population_scans = self.branch_population_scans.load(Ordering::Relaxed);
        costs
    }

    pub(crate) fn branch_count(&self) -> usize {
        self.branch_cells.len()
    }

    fn history_materialization_count(&self) -> u64 {
        self.commit_catalog.materialization_count()
    }

    pub(crate) fn branch_cell_mut(
        &mut self,
        branch_id: &BranchId,
    ) -> Option<&mut RelationalBranchReferenceCell> {
        self.branch_cells.get_mut(branch_id)
    }

    pub(crate) fn insert_branch_cell(&mut self, cell: RelationalBranchReferenceCell) {
        self.branch_cells
            .insert(cell.identity().branch_id().clone(), cell);
    }

    pub(crate) fn has_branch(&self, branch_id: &BranchId) -> bool {
        self.branch_cells.contains_key(branch_id)
    }

    /// Require a branch cell already admitted by an exact checkpoint. Replay
    /// may not synthesize a cell from a legacy branch-head projection.
    pub(crate) fn require_recovered_branch(&self, branch_id: &BranchId) -> Result<(), String> {
        self.has_branch(branch_id)
            .then_some(())
            .ok_or_else(|| format!("recovery checkpoint omitted branch cell `{}`", branch_id.0))
    }

    pub(crate) fn admit_recovered_branch_cell(
        &mut self,
        checkpoint: RelationalBranchCellCheckpoint,
        expected_branch_id: &BranchId,
    ) -> Result<(), String> {
        let cell =
            RelationalBranchReferenceCell::from_checkpoint(self.runtime_instance_id, checkpoint)
                .map_err(|denial| format!("invalid durable branch-cell state: {denial:?}"))?;
        let branch_id = cell.identity().branch_id().clone();
        if &branch_id != expected_branch_id {
            return Err(format!(
                "recovery branch-cell `{}` does not match envelope branch `{}`",
                branch_id.0, expected_branch_id.0
            ));
        }
        history_recovery::validate_tail_branch_cell(self, &cell)?;
        if let Some(existing) = self.branch_cell(&branch_id) {
            let existing_checkpoint = existing.checkpoint();
            let incoming_checkpoint = cell.checkpoint();
            if !history_recovery::branch_cell_truth_matches(
                &existing_checkpoint,
                &incoming_checkpoint,
            ) {
                return Err(format!(
                    "recovery branch-cell state conflicts for `{}`",
                    branch_id.0
                ));
            }
            return Ok(());
        }
        self.insert_branch_cell(cell);
        Ok(())
    }

    pub(crate) fn branch_cells_snapshot(&self) -> Vec<RelationalBranchCellCheckpoint> {
        self.branch_population_values()
            .map(RelationalBranchReferenceCell::checkpoint)
            .collect()
    }

    pub(crate) fn branch_ids_snapshot(&self) -> Vec<BranchId> {
        self.record_branch_population_scan();
        self.branch_cells.keys().cloned().collect()
    }

    fn record_branch_population_scan(&self) {
        self.branch_population_scans.fetch_add(1, Ordering::Relaxed);
    }

    fn branch_population_values(&self) -> impl Iterator<Item = &RelationalBranchReferenceCell> {
        self.record_branch_population_scan();
        self.branch_cells.values()
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
