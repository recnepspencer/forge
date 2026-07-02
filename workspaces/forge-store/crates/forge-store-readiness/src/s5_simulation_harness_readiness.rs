#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S5CorrectnessNonClaimEvidence {
    ShapeProbeOnly,
}

impl S5CorrectnessNonClaimEvidence {
    pub const fn shape_probe_only() -> Self {
        Self::ShapeProbeOnly
    }
}
