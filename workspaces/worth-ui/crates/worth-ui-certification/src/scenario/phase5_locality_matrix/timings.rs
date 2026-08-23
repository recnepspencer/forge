use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

#[derive(Clone, Default)]
pub(super) struct Phase5LocalityApplicationTimingRecorder {
    state: Arc<Phase5LocalityApplicationTimingState>,
}

#[derive(Default)]
struct Phase5LocalityApplicationTimingState {
    fixture_materialization_micros: AtomicU64,
    owner_installation_micros: AtomicU64,
    builder_registration_micros: AtomicU64,
    application_completion_micros: AtomicU64,
}

#[derive(Clone, Copy)]
pub(super) struct Phase5LocalityApplicationTimingSnapshot {
    pub(super) fixture_materialization_micros: u64,
    pub(super) owner_installation_micros: u64,
    pub(super) builder_registration_micros: u64,
    pub(super) application_completion_micros: u64,
}

impl Phase5LocalityApplicationTimingRecorder {
    pub(super) fn record_fixture_materialization(&self, elapsed: std::time::Duration) {
        store(&self.state.fixture_materialization_micros, elapsed);
        report("fixture-materialization", elapsed);
    }

    pub(super) fn record_owner_installation(&self, elapsed: std::time::Duration) {
        store(&self.state.owner_installation_micros, elapsed);
        report("owner-installation", elapsed);
    }

    pub(super) fn record_builder_registration(&self, elapsed: std::time::Duration) {
        store(&self.state.builder_registration_micros, elapsed);
        report("builder-registration", elapsed);
    }

    pub(super) fn record_application_completion(&self, elapsed: std::time::Duration) {
        store(&self.state.application_completion_micros, elapsed);
        report("application-completion", elapsed);
    }

    pub(super) fn snapshot(&self) -> Phase5LocalityApplicationTimingSnapshot {
        Phase5LocalityApplicationTimingSnapshot {
            fixture_materialization_micros: load(&self.state.fixture_materialization_micros),
            owner_installation_micros: load(&self.state.owner_installation_micros),
            builder_registration_micros: load(&self.state.builder_registration_micros),
            application_completion_micros: load(&self.state.application_completion_micros),
        }
    }
}

fn store(target: &AtomicU64, elapsed: std::time::Duration) {
    target.store(
        u64::try_from(elapsed.as_micros()).unwrap_or(u64::MAX),
        Ordering::Relaxed,
    );
}

fn load(source: &AtomicU64) -> u64 {
    source.load(Ordering::Relaxed)
}

fn report(phase: &str, elapsed: std::time::Duration) {
    eprintln!(
        "phase5-locality timing phase={phase} elapsed_us={}",
        elapsed.as_micros(),
    );
}
