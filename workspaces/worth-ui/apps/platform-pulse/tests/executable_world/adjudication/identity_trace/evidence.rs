use super::{
    float_pair, project_axis, ExecutableVisualComparisonEvidence, ExecutableVisualIdentityFailure,
    ExecutableVisualRetirementEvidence, ExecutableVisualSnapshotEvidence,
    ExecutableVisualTraceEvidence, PLATFORM_PULSE_CANONICAL_LOGICAL_EXTENT, TARGET_LOGICAL_INSET,
};
use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseVisualComparison, PlatformPulseVisualPointTrace, PlatformPulseVisualSnapshotCaptured,
    PlatformPulseVisualSnapshotRetired,
};

impl ExecutableVisualSnapshotEvidence {
    pub(crate) fn sequence(&self) -> u64 {
        self.sequence
    }

    pub(crate) fn snapshot(&self) -> &PlatformPulseVisualSnapshotCaptured {
        &self.snapshot
    }

    pub(crate) fn physical_extent(&self) -> [u32; 2] {
        self.snapshot.coordinates().client_physical_dimensions()
    }

    pub(crate) fn project_logical_point(
        &self,
        logical: [u32; 2],
    ) -> Result<[u32; 2], ExecutableVisualIdentityFailure> {
        let scale = float_pair(self.snapshot.coordinates().scale_bits());
        let translation = float_pair(self.snapshot.coordinates().translation_bits());
        Ok([
            project_axis(logical[0], scale[0], translation[0])?,
            project_axis(logical[1], scale[1], translation[1])?,
        ])
    }

    pub(crate) fn expected_target_region(
        &self,
    ) -> Result<[u32; 4], ExecutableVisualIdentityFailure> {
        let logical_right = PLATFORM_PULSE_CANONICAL_LOGICAL_EXTENT[0] - TARGET_LOGICAL_INSET[0];
        let logical_bottom = PLATFORM_PULSE_CANONICAL_LOGICAL_EXTENT[1] - TARGET_LOGICAL_INSET[1];
        let [left, top] = self.project_logical_point(TARGET_LOGICAL_INSET)?;
        let [right, bottom] = self.project_logical_point([logical_right, logical_bottom])?;
        Ok([left, top, right, bottom])
    }
}

impl ExecutableVisualTraceEvidence {
    pub(crate) fn sequence(&self) -> u64 {
        self.sequence
    }

    pub(crate) fn trace(&self) -> &PlatformPulseVisualPointTrace {
        &self.trace
    }
}

impl ExecutableVisualRetirementEvidence {
    pub(crate) fn sequence(self) -> u64 {
        self.sequence
    }

    pub(crate) fn retirement(self) -> PlatformPulseVisualSnapshotRetired {
        self.retirement
    }
}

impl ExecutableVisualComparisonEvidence {
    pub(crate) fn sequence(self) -> u64 {
        self.sequence
    }

    pub(crate) fn comparison(self) -> PlatformPulseVisualComparison {
        self.comparison
    }
}
