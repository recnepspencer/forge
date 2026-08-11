use crate::logic::transaction::runtime::state::{
    temporal_certification_builder, temporal_certification_bundle,
    temporal_certification_bundle_parity_report, temporal_replay_parity_report,
    ReconstructabilityRecord, TemporalCertificationBuilder, TemporalCertificationBundle,
    TemporalCertificationBundleParityReport, TemporalCertificationRecord,
    TemporalReplayParityReport,
};

use super::super::runtime_state::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn temporal_replay_parity_report(
        &mut self,
        expected: &ReconstructabilityRecord,
        replayed: &ReconstructabilityRecord,
    ) -> TemporalReplayParityReport {
        self.telemetry.temporal.temporal_replay_parity_check_count += 1;
        temporal_replay_parity_report(&expected.temporal, &replayed.temporal)
    }

    pub fn temporal_certification_bundle(
        &self,
        records: impl IntoIterator<Item = TemporalCertificationRecord>,
    ) -> TemporalCertificationBundle {
        temporal_certification_bundle(records)
    }

    pub fn temporal_certification_builder(&self) -> TemporalCertificationBuilder {
        temporal_certification_builder()
    }

    pub fn temporal_certification_bundle_parity_report(
        &mut self,
        expected: &TemporalCertificationBundle,
        replayed: &TemporalCertificationBundle,
    ) -> TemporalCertificationBundleParityReport {
        self.telemetry.temporal.temporal_replay_parity_check_count += 1;
        temporal_certification_bundle_parity_report(expected, replayed)
    }
}
