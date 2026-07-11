#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalIsolationCorrectnessNonClaimEvidence {
    ShapeProbeOnly,
}

impl PhysicalIsolationCorrectnessNonClaimEvidence {
    pub const fn shape_probe_only() -> Self {
        Self::ShapeProbeOnly
    }
}
