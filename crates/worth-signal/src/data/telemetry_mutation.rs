use std::ops::{Deref, DerefMut};

use super::telemetry::RuntimeTelemetry;

/// A graph-owned telemetry mutation available only after the optional
/// telemetry surface has been admitted. Callers must treat `None` from
/// `SignalGraph::telemetry_mut` as a hard gate: no telemetry value is
/// constructed, copied, or updated on that path.
pub struct RuntimeTelemetryMutation<'a> {
    telemetry: &'a mut RuntimeTelemetry,
}

impl<'a> RuntimeTelemetryMutation<'a> {
    pub(crate) fn active(telemetry: &'a mut RuntimeTelemetry) -> Self {
        Self { telemetry }
    }
}

impl Deref for RuntimeTelemetryMutation<'_> {
    type Target = RuntimeTelemetry;

    fn deref(&self) -> &Self::Target {
        self.telemetry
    }
}

impl DerefMut for RuntimeTelemetryMutation<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.telemetry
    }
}
