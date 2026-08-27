use std::collections::{BTreeMap, HashMap};
#[cfg(test)]
use std::sync::atomic::AtomicBool;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::branch::{
    RelationalBranchBasisRegistryMetrics, RelationalBranchReferenceCell,
    RelationalBranchReferenceRegistry, RelationalBranchRootIdentityIssuer,
};
use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::{BranchId, CommitId, VersionNode};
use crate::history::RelationalCommitCatalog;
use crate::identity::data::VersionId;
use crate::publication::patch::data::PatchStreamPosition;

#[path = "canonical_publication_routes.rs"]
mod canonical_publication_routes;
#[path = "history_branch_cells.rs"]
mod history_branch_cells;
#[path = "history_branch_lifecycle.rs"]
mod history_branch_lifecycle;
#[path = "history_canonical_routes.rs"]
mod history_canonical_routes;
#[path = "history_catalog_recovery.rs"]
mod history_catalog_recovery;
#[path = "history_construction.rs"]
mod history_construction;
#[path = "history_cost_recording.rs"]
mod history_cost_recording;
#[path = "history_costs.rs"]
mod history_costs;
#[path = "history_head_versions.rs"]
mod history_head_versions;
pub(crate) use history_head_versions::BranchHeadVersionIndexAuthority;
#[path = "history_publication.rs"]
mod history_publication;
#[path = "history_recovery.rs"]
mod history_recovery;
#[path = "history_recovery_lineage.rs"]
mod history_recovery_lineage;
#[path = "history_recovery_validation.rs"]
mod history_recovery_validation;
#[path = "history_retention.rs"]
mod history_retention;
#[path = "history_root_capture.rs"]
mod history_root_capture;
pub use canonical_publication_routes::RelationalPatchPositionReservationCounters;
pub(crate) use canonical_publication_routes::{
    readmit_positioned_canonical_commit, CanonicalCheckpointAdmissionError,
    CanonicalPositionAdmission, CanonicalPublicationRecordError, PerformedCheckpointSelection,
    PreparedCanonicalPublicationRoute, RelationalCanonicalPublicationRoutes,
};
pub use history_costs::RelationalBranchSharingCostCounters;
pub(crate) use history_costs::RelationalForkMaterializationCost;
pub(crate) use history_publication::PreparedRecoveredVersionedArtifactPublication;
pub(crate) use history_publication::PreparedVersionedArtifactAccelerators;
pub(crate) use history_publication::PreparedVersionedArtifactPublication;

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

#[derive(Debug)]
pub(crate) struct HistorySubsystem {
    pub(crate) runtime_instance_id: u64,
    pub(crate) main_branch: BranchId,
    branch_cells: RelationalBranchReferenceRegistry,
    pub(crate) commit_catalog: RelationalCommitCatalog,
    pub(crate) phase4_costs: RelationalPhase4ReferenceCostCounters,
    pub(crate) sharing_costs: RelationalBranchSharingCostCounters,
    branch_sharing_costs:
        HashMap<crate::branch::RelationalBranchIdentity, RelationalBranchSharingCostCounters>,
    root_identity_issuer: RelationalBranchRootIdentityIssuer,
    /// Population-wide reference traversal is deliberately hidden behind
    /// named diagnostics/checkpoint methods.  Keeping the counter separate
    /// lets those shared-`&self` methods record a scan without making the
    /// operational counters interior-mutable.
    branch_population_scans: Arc<AtomicU64>,
    branch_head_versions: history_head_versions::BranchHeadVersionIndexAuthority,
    basis_registry_metrics: Arc<RelationalBranchBasisRegistryMetrics>,
    retention_owner: crate::history::retention::RelationalBranchRetentionOwner,
    retired_branch_names: std::collections::HashSet<BranchId>,
    #[cfg(test)]
    root_capture_sabotage: Arc<AtomicBool>,
    /// Durable recovery/diagnostic sidecar. Currentness and fork identity
    /// read the catalog, not this map.
    pub(crate) commit_graph: BTreeMap<crate::history::data::CommitId, VersionNode>,
    /// Durable recovery sidecar holding the same sealed envelope the catalog
    /// already admitted. It cannot mint a branch cell or a fork basis.
    pub(crate) commit_envelopes:
        BTreeMap<crate::history::data::CommitId, Arc<CanonicalCommitEnvelope>>,
    pub(crate) patch_stream_index: BTreeMap<PatchStreamPosition, crate::history::data::CommitId>,
    pub(crate) next_commit_id: u64,
    pub(crate) next_version_id: u64,
    commit_identity_allocator: Arc<AtomicU64>,
    version_identity_allocator: Arc<AtomicU64>,
    canonical_publication_routes: Arc<RelationalCanonicalPublicationRoutes>,
}

impl HistorySubsystem {
    pub(super) fn build_with_main_branch(main_branch: BranchId) -> Self {
        let basis_registry_metrics = Arc::new(RelationalBranchBasisRegistryMetrics::default());
        let mut main_cell = RelationalBranchReferenceCell::empty(0, main_branch.clone())
            .expect("configured main branch must have a valid identity");
        main_cell.bind_basis_registry_metrics(Arc::clone(&basis_registry_metrics));
        Self {
            runtime_instance_id: 0,
            main_branch: main_branch.clone(),
            branch_cells: RelationalBranchReferenceRegistry::from_main(main_cell),
            commit_catalog: RelationalCommitCatalog::default(),
            phase4_costs: RelationalPhase4ReferenceCostCounters::default(),
            sharing_costs: RelationalBranchSharingCostCounters::default(),
            branch_sharing_costs: HashMap::new(),
            root_identity_issuer: RelationalBranchRootIdentityIssuer::default(),
            branch_population_scans: Arc::new(AtomicU64::new(0)),
            branch_head_versions: Default::default(),
            basis_registry_metrics,
            retention_owner: crate::history::retention::RelationalBranchRetentionOwner::new(0),
            retired_branch_names: std::collections::HashSet::new(),
            #[cfg(test)]
            root_capture_sabotage: Arc::new(AtomicBool::new(false)),
            commit_graph: BTreeMap::new(),
            commit_envelopes: BTreeMap::new(),
            patch_stream_index: BTreeMap::new(),
            next_commit_id: 1,
            next_version_id: 1,
            commit_identity_allocator: Arc::new(AtomicU64::new(1)),
            version_identity_allocator: Arc::new(AtomicU64::new(1)),
            canonical_publication_routes: Arc::new(RelationalCanonicalPublicationRoutes::default()),
        }
    }

    pub(crate) fn preview_next_commit_id(&self) -> CommitId {
        CommitId(self.next_commit_id)
    }

    #[cfg(test)]
    pub(crate) fn preview_next_version_id(&self) -> VersionId {
        VersionId(self.next_version_id)
    }

    pub(crate) fn reserve_commit_id(&self) -> Option<CommitId> {
        reserve_identity(&self.commit_identity_allocator, self.next_commit_id).map(CommitId)
    }

    pub(crate) fn reserve_version_id(&self) -> Option<VersionId> {
        reserve_identity(&self.version_identity_allocator, self.next_version_id).map(VersionId)
    }

    pub(crate) fn current_version_id(&self) -> VersionId {
        VersionId(self.next_version_id.saturating_sub(1))
    }

    pub(crate) fn move_branch_head_version(
        &self,
        previous: Option<VersionId>,
        next: Option<VersionId>,
    ) {
        self.branch_head_versions.move_head(previous, next);
    }

    pub(crate) fn oldest_branch_head_version(&self) -> Option<VersionId> {
        self.branch_head_versions.oldest()
    }

    pub(crate) fn branch_head_version_index(&self) -> BranchHeadVersionIndexAuthority {
        self.branch_head_versions.clone()
    }

    pub(crate) fn prepare_recovery_sequence(&mut self, commit_id: CommitId, version_id: VersionId) {
        self.next_commit_id = commit_id.0;
        self.next_version_id = version_id.0;
        self.commit_identity_allocator = Arc::new(AtomicU64::new(commit_id.0));
        self.version_identity_allocator = Arc::new(AtomicU64::new(version_id.0));
    }

    pub(crate) fn install_recovered_sequence_floor(
        &mut self,
        next_commit_id: u64,
        next_version_id: u64,
    ) {
        self.next_commit_id = next_commit_id;
        self.next_version_id = next_version_id;
        self.commit_identity_allocator = Arc::new(AtomicU64::new(next_commit_id));
        self.version_identity_allocator = Arc::new(AtomicU64::new(next_version_id));
    }

    pub(crate) fn prepare_replay_target_sequence(
        &mut self,
        commit_id: CommitId,
        version_id: VersionId,
    ) -> Result<(), &'static str> {
        if self.next_commit_id > commit_id.0 || self.next_version_id > version_id.0 {
            return Err("replay basis sequence has advanced beyond the committed target");
        }
        self.next_commit_id = commit_id.0;
        self.next_version_id = version_id.0;
        self.commit_identity_allocator = Arc::new(AtomicU64::new(commit_id.0));
        self.version_identity_allocator = Arc::new(AtomicU64::new(version_id.0));
        Ok(())
    }

    pub(crate) fn set_runtime_instance_id(&mut self, runtime_instance_id: u64) {
        self.runtime_instance_id = runtime_instance_id;
        self.basis_registry_metrics = Arc::new(RelationalBranchBasisRegistryMetrics::default());
        self.record_branch_population_scan();
        let cells = self.branch_cells.take_all();
        self.branch_cells.restore_all(
            cells
                .into_values()
                .map(|cell| {
                    let mut rebound = cell
                        .rebind_runtime(runtime_instance_id)
                        .expect("existing branch identities must remain valid when rebound");
                    rebound.bind_basis_registry_metrics(Arc::clone(&self.basis_registry_metrics));
                    (rebound.identity().branch_id().clone(), rebound)
                })
                .collect(),
        );
        self.reset_retention_owner(runtime_instance_id);
    }

    pub(crate) fn fork_snapshot(
        &self,
    ) -> Result<Self, crate::runtime::RelationalRuntimeForkDenial> {
        let _fork_guard = self.canonical_publication_routes.enter_fork()?;
        if let Some(commit_id) = self
            .canonical_publication_routes
            .has_unsettled_performed_publication()
        {
            return Err(
                crate::runtime::RelationalRuntimeForkDenial::PerformedPublicationRequiresSettlement {
                    commit_id,
                },
            );
        }
        let positioned = self.canonical_publication_routes.performed_snapshot();
        let mut fork = self.detached_owner_snapshot();
        if let Some(maximum) = positioned
            .iter()
            .map(|commit| commit.envelope().commit.commit_id.0)
            .max()
        {
            fork.next_commit_id =
                fork.next_commit_id.max(maximum.checked_add(1).ok_or(
                    crate::runtime::RelationalRuntimeForkDenial::IdentityCapacityExhausted,
                )?);
        }
        if let Some(maximum) = positioned
            .iter()
            .map(|commit| commit.envelope().commit.version_id.0)
            .max()
        {
            fork.next_version_id =
                fork.next_version_id.max(maximum.checked_add(1).ok_or(
                    crate::runtime::RelationalRuntimeForkDenial::IdentityCapacityExhausted,
                )?);
        }
        fork.commit_identity_allocator = Arc::new(AtomicU64::new(fork.next_commit_id));
        fork.version_identity_allocator = Arc::new(AtomicU64::new(fork.next_version_id));
        let routes = Arc::new(RelationalCanonicalPublicationRoutes::default());
        for commit in positioned {
            routes.install_recovered(commit).map_err(|_| {
                crate::runtime::RelationalRuntimeForkDenial::CanonicalInventoryInvalid
            })?;
        }
        fork.canonical_publication_routes = routes;
        Ok(fork)
    }

    pub(crate) fn bind_fork_runtime(&mut self, runtime_instance_id: u64) {
        self.set_runtime_instance_id(runtime_instance_id);
        self.phase4_costs = RelationalPhase4ReferenceCostCounters::default();
        self.sharing_costs = RelationalBranchSharingCostCounters::default();
        self.branch_sharing_costs.clear();
        self.branch_population_scans = Arc::new(AtomicU64::new(0));
        self.retired_branch_names.clear();
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

    pub(crate) fn branch_population_scan_count(&self) -> u64 {
        self.branch_population_scans.load(Ordering::Relaxed)
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

    pub(crate) fn insert_branch_cell(&mut self, mut cell: RelationalBranchReferenceCell) {
        cell.bind_basis_registry_metrics(Arc::clone(&self.basis_registry_metrics));
        self.branch_cells.insert(cell);
    }

    pub(crate) fn basis_registry_metrics(&self) -> (u64, u64, u64) {
        self.basis_registry_metrics.snapshot()
    }

    pub(crate) fn has_branch(&self, branch_id: &BranchId) -> bool {
        self.branch_cells.contains(branch_id)
    }

    /// Require a branch cell already admitted by an exact checkpoint. Replay
    /// may not synthesize a cell from a legacy branch-head projection.
    pub(crate) fn require_recovered_branch(&self, branch_id: &BranchId) -> Result<(), String> {
        self.has_branch(branch_id)
            .then_some(())
            .ok_or_else(|| format!("recovery checkpoint omitted branch cell `{}`", branch_id.0))
    }
}

fn reserve_identity(allocator: &AtomicU64, legacy_floor: u64) -> Option<u64> {
    allocator.fetch_max(legacy_floor, Ordering::Relaxed);
    allocator
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |next| {
            next.checked_add(1)
        })
        .ok()
}
