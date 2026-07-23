/// Observed buffer state relevant to physical access-mode admission.
///
/// This is deliberately an observation rather than mutation authority. The
/// Store-owned composition boundary remains responsible for obtaining it from
/// a live residency lease before asking the backend to admit an access mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccessPolicyBufferLifecycleKind {
    PinnedPhysicalLease,
    DirtyPageTracked,
    DirtyMmapPage,
    Missing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessPolicyBufferLifecycle {
    kind: AccessPolicyBufferLifecycleKind,
}

impl AccessPolicyBufferLifecycle {
    pub const fn from_pinned_residency_observation() -> Self {
        Self::new(AccessPolicyBufferLifecycleKind::PinnedPhysicalLease)
    }

    pub const fn from_dirty_page_observation() -> Self {
        Self::new(AccessPolicyBufferLifecycleKind::DirtyPageTracked)
    }

    pub const fn from_dirty_mmap_observation() -> Self {
        Self::new(AccessPolicyBufferLifecycleKind::DirtyMmapPage)
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn for_certification_pinned_physical_substrate_lease() -> Self {
        Self::from_pinned_residency_observation()
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn for_certification_dirty_page_tracked() -> Self {
        Self::from_dirty_page_observation()
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn for_certification_dirty_mmap_page() -> Self {
        Self::from_dirty_mmap_observation()
    }

    const fn new(kind: AccessPolicyBufferLifecycleKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> AccessPolicyBufferLifecycleKind {
        self.kind
    }
}
