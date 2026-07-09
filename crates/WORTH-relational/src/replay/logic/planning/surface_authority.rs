use std::collections::BTreeSet;

use crate::capabilities::HistorySource;
use crate::history::data::{BranchId, CommitId};
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::{CanonicalCommitEnvelope, ReplayObservableSurface};

use super::super::derived_index_surface::{
    derived_index_surface_is_promised, DERIVED_INDEX_SURFACE,
};

pub(in crate::replay::logic) fn promised_replay_surfaces(
    runtime: &RelationalRuntime,
    original: &CanonicalCommitEnvelope,
    original_closure: &[CommitId],
    recovered: Option<&CanonicalCommitEnvelope>,
) -> Vec<ReplayObservableSurface> {
    let mut surfaces = vec![
        ReplayObservableSurface::Patch,
        ReplayObservableSurface::Diagnostics,
        ReplayObservableSurface::History,
        ReplayObservableSurface::BranchHead,
    ];
    if snapshot_surface_is_authoritative(runtime, original, original_closure) {
        surfaces.push(ReplayObservableSurface::Snapshot);
    }
    if original.has_lineage_authority()
        || recovered.is_some_and(|envelope| envelope.has_lineage_authority())
    {
        surfaces.push(ReplayObservableSurface::Lineage);
    }
    if original.strategy_artifacts.is_some()
        || recovered.is_some_and(|envelope| envelope.strategy_artifacts.is_some())
    {
        surfaces.push(ReplayObservableSurface::Strategy);
    }
    if derived_index_surface_is_promised(original, recovered) {
        surfaces.push(DERIVED_INDEX_SURFACE);
    }
    surfaces
}

fn snapshot_surface_is_authoritative(
    history: &impl HistorySource,
    envelope: &CanonicalCommitEnvelope,
    closure: &[CommitId],
) -> bool {
    if envelope.branch_context != BranchId("main".to_string()) {
        return false;
    }
    !history.has_committed_version_at_or_before_outside_closure(
        envelope.commit.version_id,
        &closure.iter().copied().collect::<BTreeSet<_>>(),
    )
}
