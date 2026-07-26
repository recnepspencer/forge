use super::WorthQueryProviderWorkReport;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryLegacyProviderWorkClaim {
    completed_work_units: u64,
    applied_effect_count: u64,
    produced_artifact_count: usize,
    retained_artifact_count: usize,
    disposed_artifact_count: usize,
    scratch_bytes: usize,
    retained_bytes: usize,
}

impl WorthQueryLegacyProviderWorkClaim {
    pub const fn new(
        completed_work_units: u64,
        applied_effect_count: u64,
        scratch_bytes: usize,
        retained_bytes: usize,
    ) -> Self {
        Self {
            completed_work_units,
            applied_effect_count,
            produced_artifact_count: 0,
            retained_artifact_count: 0,
            disposed_artifact_count: 0,
            scratch_bytes,
            retained_bytes,
        }
    }

    pub fn with_artifact_disposition(
        mut self,
        produced: usize,
        retained: usize,
        disposed: usize,
    ) -> Option<Self> {
        if retained.checked_add(disposed) != Some(produced) {
            return None;
        }
        self.produced_artifact_count = produced;
        self.retained_artifact_count = retained;
        self.disposed_artifact_count = disposed;
        Some(self)
    }

    pub(crate) fn into_report(self) -> WorthQueryProviderWorkReport {
        WorthQueryProviderWorkReport::new(
            self.completed_work_units,
            self.applied_effect_count,
            self.scratch_bytes,
            self.retained_bytes,
        )
        .with_artifact_disposition(
            self.produced_artifact_count,
            self.retained_artifact_count,
            self.disposed_artifact_count,
        )
        .expect("legacy claim validates its artifact disposition before conversion")
    }
}
