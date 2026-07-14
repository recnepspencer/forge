#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalMutationShape {
    ObservationOnly,
    PointRewrite,
    LogStructuredAppend,
    CompactionRewrite,
}

impl PhysicalMutationShape {
    pub const fn requires_write_ordering_proof(self) -> bool {
        !matches!(self, Self::ObservationOnly)
    }
}
