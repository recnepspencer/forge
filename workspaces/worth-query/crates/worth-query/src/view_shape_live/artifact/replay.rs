use crate::live::LiveReplayBundle;

use super::super::counters::ViewShapeLiveCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewShapeReplayBundle {
    delivery_digest: String,
    replay_digest: String,
    core: Option<LiveReplayBundle>,
    counters: ViewShapeLiveCounters,
}

impl ViewShapeReplayBundle {
    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn core(&self) -> Option<&LiveReplayBundle> {
        self.core.as_ref()
    }

    pub fn counters(&self) -> &ViewShapeLiveCounters {
        &self.counters
    }
    #[cfg(test)]
    pub(crate) fn new(
        delivery_digest: impl Into<String>,
        replay_digest: impl Into<String>,
        core: Option<LiveReplayBundle>,
        counters: ViewShapeLiveCounters,
    ) -> Self {
        Self {
            delivery_digest: delivery_digest.into(),
            replay_digest: replay_digest.into(),
            core,
            counters,
        }
    }
}
