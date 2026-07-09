#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TierMovementStabilityCounterSnapshot {
    stability_admissions: u64,
    chunk_placeholders: u64,
    denied_s7_promotions: u64,
    denied_s6_qos_promotions: u64,
    missing_epoch_denials: u64,
    stale_generation_denials: u64,
    copied_label_denials: u64,
}

impl TierMovementStabilityCounterSnapshot {
    pub const fn with_stability_admission(mut self) -> Self {
        self.stability_admissions += 1;
        self
    }

    pub const fn with_chunk_placeholder(mut self) -> Self {
        self.chunk_placeholders += 1;
        self
    }

    pub const fn with_s7_promotion_denial(mut self) -> Self {
        self.denied_s7_promotions += 1;
        self
    }

    pub const fn with_s6_qos_denial(mut self) -> Self {
        self.denied_s6_qos_promotions += 1;
        self
    }

    pub const fn with_missing_epoch_denial(mut self) -> Self {
        self.missing_epoch_denials += 1;
        self
    }

    pub const fn with_stale_generation_denial(mut self) -> Self {
        self.stale_generation_denials += 1;
        self
    }

    pub const fn with_copied_label_denial(mut self) -> Self {
        self.copied_label_denials += 1;
        self
    }

    pub const fn stability_admissions(self) -> u64 {
        self.stability_admissions
    }

    pub const fn chunk_placeholders(self) -> u64 {
        self.chunk_placeholders
    }

    pub const fn denied_s7_promotions(self) -> u64 {
        self.denied_s7_promotions
    }

    pub const fn denied_s6_qos_promotions(self) -> u64 {
        self.denied_s6_qos_promotions
    }

    pub const fn missing_epoch_denials(self) -> u64 {
        self.missing_epoch_denials
    }

    pub const fn stale_generation_denials(self) -> u64 {
        self.stale_generation_denials
    }

    pub const fn copied_label_denials(self) -> u64 {
        self.copied_label_denials
    }
}
