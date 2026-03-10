use super::super::fixture::FintechWorld;
use super::case_truth::ProbeStage;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ObservabilityProbe {
    pub(crate) stage: ProbeStage,
    pub(crate) latest_commit_id: Option<u64>,
    pub(crate) latest_patch_present: bool,
    pub(crate) latest_replay_present: bool,
    pub(crate) diagnostics_artifact_count: usize,
    pub(crate) publication_snapshot_id: Option<u64>,
}

pub(crate) fn capture_observability_probe(
    world: &FintechWorld,
    stage: ProbeStage,
) -> ObservabilityProbe {
    ObservabilityProbe {
        stage,
        latest_commit_id: world
            .runtime
            .latest_commit()
            .map(|commit| commit.commit_id.0),
        latest_patch_present: world.runtime.latest_patch().is_some(),
        latest_replay_present: world.runtime.latest_replay().is_some(),
        diagnostics_artifact_count: world.runtime.diagnostics().artifacts().len(),
        publication_snapshot_id: world
            .runtime
            .latest_publication_bundle()
            .map(|bundle| bundle.snapshot.snapshot_id.0),
    }
}
