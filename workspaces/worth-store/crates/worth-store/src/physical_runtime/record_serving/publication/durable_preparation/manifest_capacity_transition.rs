/// Explicit root-planning posture for a durable physical record mutation.
///
/// Ordinary preparation preserves the current manifest capacity. Reconstruction
/// is a distinct effect-relevant request and must be selected before
/// idempotency admission, WAL binding, or data planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalManifestCapacityTransition {
    PreserveCurrent,
    ReconstructToRequested,
}

impl PhysicalManifestCapacityTransition {
    pub(in crate::physical_runtime::record_serving) const fn identity_code(self) -> u8 {
        match self {
            Self::PreserveCurrent => 1,
            Self::ReconstructToRequested => 2,
        }
    }
}
