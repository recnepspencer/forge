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
    let publication = world.runtime.publication();
    let observation = publication.observation_snapshot();

    ObservabilityProbe {
        stage,
        latest_commit_id: world
            .runtime
            .history()
            .latest_commit()
            .map(|commit| commit.commit_id.0),
        latest_patch_present: observation.latest_patch_present,
        latest_replay_present: observation.latest_replay_present,
        diagnostics_artifact_count: observation.diagnostics_artifact_count,
        publication_snapshot_id: observation.publication_snapshot_id.map(|id| id.0),
    }
}
