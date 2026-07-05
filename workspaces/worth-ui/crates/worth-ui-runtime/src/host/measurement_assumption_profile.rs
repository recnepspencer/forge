use worth_ui_host_contract::{
    WorthUiHostCapabilityObservationGeneration, WorthUiHostCapabilityReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostMeasurementAssumptionProfile {
    capability_observation_generation: WorthUiHostCapabilityObservationGeneration,
    capability_profile_digest: u64,
    viewport_assumption_digest: u64,
    dpi_assumption_digest: u64,
    font_assumption_digest: u64,
    adapter_profile_digest: u64,
}

impl UiHostMeasurementAssumptionProfile {
    pub const fn new(
        capability_observation_generation: WorthUiHostCapabilityObservationGeneration,
        capability_profile_digest: u64,
        viewport_assumption_digest: u64,
        dpi_assumption_digest: u64,
        font_assumption_digest: u64,
        adapter_profile_digest: u64,
    ) -> Self {
        Self {
            capability_observation_generation,
            capability_profile_digest,
            viewport_assumption_digest,
            dpi_assumption_digest,
            font_assumption_digest,
            adapter_profile_digest,
        }
    }

    pub fn from_capability_report(
        capability_report: &WorthUiHostCapabilityReport,
        viewport_assumption_digest: u64,
        dpi_assumption_digest: u64,
        font_assumption_digest: u64,
        adapter_profile_digest: u64,
    ) -> Self {
        Self::new(
            capability_report.observation_generation(),
            capability_report.profile_identity_digest(),
            viewport_assumption_digest,
            dpi_assumption_digest,
            font_assumption_digest,
            adapter_profile_digest,
        )
    }

    pub fn profile_identity_digest(self) -> u64 {
        stable_text_digest("worth-ui-host-measurement-assumption-profile")
            ^ self
                .capability_observation_generation
                .as_u64()
                .rotate_left(7)
            ^ self.capability_profile_digest.rotate_left(13)
            ^ self.viewport_assumption_digest.rotate_left(17)
            ^ self.dpi_assumption_digest.rotate_left(23)
            ^ self.font_assumption_digest.rotate_left(29)
            ^ self.adapter_profile_digest.rotate_left(31)
    }

    pub fn capability_observation_generation(self) -> WorthUiHostCapabilityObservationGeneration {
        self.capability_observation_generation
    }

    pub fn capability_profile_digest(self) -> u64 {
        self.capability_profile_digest
    }

    pub fn viewport_assumption_digest(self) -> u64 {
        self.viewport_assumption_digest
    }

    pub fn dpi_assumption_digest(self) -> u64 {
        self.dpi_assumption_digest
    }

    pub fn font_assumption_digest(self) -> u64 {
        self.font_assumption_digest
    }

    pub fn adapter_profile_digest(self) -> u64 {
        self.adapter_profile_digest
    }
}

fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}
