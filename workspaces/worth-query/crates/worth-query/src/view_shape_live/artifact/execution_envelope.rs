use crate::live::LiveExecutionEnvelope;

use super::super::counters::ViewShapeLiveCounters;
use super::live_view::LiveViewShapeArtifact;
use super::patches::ViewShapePatchEnvelope;
use super::replay::ViewShapeReplayBundle;
use super::report::ViewShapeLiveReport;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveViewShapeExecutionEnvelope {
    report: ViewShapeLiveReport,
    patch_envelope: ViewShapePatchEnvelope,
    replay_bundle: ViewShapeReplayBundle,
    counters: ViewShapeLiveCounters,
    core_execution: Option<LiveExecutionEnvelope>,
    next_live_view: LiveViewShapeArtifact,
}

impl LiveViewShapeExecutionEnvelope {
    pub fn report(&self) -> &ViewShapeLiveReport {
        &self.report
    }

    pub fn patch_envelope(&self) -> &ViewShapePatchEnvelope {
        &self.patch_envelope
    }

    pub fn replay_bundle(&self) -> &ViewShapeReplayBundle {
        &self.replay_bundle
    }

    pub fn counters(&self) -> &ViewShapeLiveCounters {
        &self.counters
    }

    pub fn core_execution(&self) -> Option<&LiveExecutionEnvelope> {
        self.core_execution.as_ref()
    }

    pub fn next_live_view(&self) -> &LiveViewShapeArtifact {
        &self.next_live_view
    }
    #[cfg(test)]
    pub(crate) fn new(
        report: ViewShapeLiveReport,
        patch_envelope: ViewShapePatchEnvelope,
        replay_bundle: ViewShapeReplayBundle,
        counters: ViewShapeLiveCounters,
        core_execution: Option<LiveExecutionEnvelope>,
        next_live_view: LiveViewShapeArtifact,
    ) -> Self {
        Self {
            report,
            patch_envelope,
            replay_bundle,
            counters,
            core_execution,
            next_live_view,
        }
    }
}
