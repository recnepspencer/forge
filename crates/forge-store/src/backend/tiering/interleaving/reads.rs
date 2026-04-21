use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};
use crate::failure::StoreError;
use crate::live_query::StableBasisHandle;
use crate::tiering::{
    ColdRecallLease, InterleavedReadParityReport, PlacementResolvedReadHandle, ResidentReadLease,
};

use super::shared::{observation_for_artifacts, record_interleaving_observation};

pub(crate) fn resolve_resident_read_handle(
    lease: &ResidentReadLease,
) -> crate::PlacementResolvedReadHandle {
    PlacementResolvedReadHandle::from_resident_lease(lease)
}

pub(crate) fn resolve_cold_recall_read_handle(
    lease: &ColdRecallLease,
) -> crate::PlacementResolvedReadHandle {
    PlacementResolvedReadHandle::from_cold_recall_lease(lease)
}

pub(crate) fn observe_placement_read_interleaving<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    handle: &PlacementResolvedReadHandle,
) -> Result<InterleavedReadParityReport, StoreError> {
    let observation =
        observation_for_artifacts(backend.state(), vec![artifact_key_for_read_handle(handle)?]);
    let report = InterleavedReadParityReport::new(
        observation,
        handle.execution_origin(),
        handle.placement_path(),
        handle.tier_miss_outcome(),
        None,
        true,
    );
    record_interleaving_observation(backend.counters(), report.observation(), false, true);
    Ok(report)
}

pub(crate) fn observe_stable_basis_interleaving<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    basis: &StableBasisHandle,
) -> Result<InterleavedReadParityReport, StoreError> {
    let observation = observation_for_artifacts(
        backend.state(),
        vec![
            format!("authoritative_branch_head:{}", basis.branch_id().0),
            format!("stable_basis:{}", basis.stable_basis_id().as_str()),
        ],
    );
    let report = InterleavedReadParityReport::new(
        observation,
        crate::PlacementExecutionOrigin::Foreground,
        crate::RetainedReadPlacementPath::WarmResident,
        crate::TierMissOutcome::WarmHit,
        Some(basis.foreground_isolation().clone()),
        true,
    );
    record_interleaving_observation(backend.counters(), report.observation(), false, true);
    Ok(report)
}

fn artifact_key_for_read_handle(
    handle: &PlacementResolvedReadHandle,
) -> Result<String, StoreError> {
    let artifact_key = match handle.artifact_ref().artifact_family() {
        crate::PlacementArtifactFamily::AuthoritativeBranchHead => format!(
            "authoritative_branch_head:{}",
            handle.artifact_ref().artifact_id()
        ),
        crate::PlacementArtifactFamily::StableBasis => {
            format!("stable_basis:{}", handle.artifact_ref().artifact_id())
        }
        crate::PlacementArtifactFamily::SnapshotFamily => {
            format!("snapshot:{}", handle.artifact_ref().artifact_id())
        }
        crate::PlacementArtifactFamily::BranchDeltaFamily => {
            format!("branch_delta:{}", handle.artifact_ref().artifact_id())
        }
        crate::PlacementArtifactFamily::Milestone6LayoutFamily => {
            format!("milestone6_layout:{}", handle.artifact_ref().artifact_id())
        }
        crate::PlacementArtifactFamily::RetainedAuthority => format!(
            "retained_authority:{}",
            handle
                .artifact_ref()
                .retained_basis_label()
                .unwrap_or(handle.artifact_ref().artifact_id())
        ),
    };
    Ok(artifact_key)
}
