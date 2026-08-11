use crate::live::LiveExecutionEnvelope;

use super::super::artifact::{
    LiveViewShapeArtifact, LiveViewShapeExecutionEnvelope, ViewShapeLiveReport,
    ViewShapePatchEnvelope, ViewShapeReplayBundle,
};
use super::super::counters::ViewShapeLiveCounters;

pub(super) struct LiveExecutionAssembly {
    pub(super) patch_envelope: ViewShapePatchEnvelope,
    pub(super) counters: ViewShapeLiveCounters,
    pub(super) core_execution: Option<LiveExecutionEnvelope>,
    pub(super) next_live_view: LiveViewShapeArtifact,
}

pub(super) fn assemble_live_execution_envelope(
    assembly: LiveExecutionAssembly,
) -> LiveViewShapeExecutionEnvelope {
    let family = assembly.patch_envelope.family();
    let report = ViewShapeLiveReport::new(
        family,
        assembly.patch_envelope.delivery_digest(),
        assembly.patch_envelope.replay_digest(),
    );
    let replay_bundle = ViewShapeReplayBundle::new(
        assembly.patch_envelope.delivery_digest(),
        assembly.patch_envelope.replay_digest(),
        assembly
            .core_execution
            .as_ref()
            .map(|execution| execution.replay_bundle().clone()),
        assembly.counters.clone(),
    );

    LiveViewShapeExecutionEnvelope::new(
        report,
        assembly.patch_envelope,
        replay_bundle,
        assembly.counters,
        assembly.core_execution,
        assembly.next_live_view,
    )
}
