#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccessPolicyBufferLifecycleKind {
    PinnedS2Lease,
    DirtyPageTracked,
    DirtyMmapPage,
    Missing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessPolicyBufferLifecycle {
    kind: AccessPolicyBufferLifecycleKind,
    _seal: AccessPolicyBufferLifecycleSeal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AccessPolicyBufferLifecycleSeal;

impl AccessPolicyBufferLifecycle {
    pub(crate) const fn pinned_physical_substrate_lease() -> Self {
        Self::new(AccessPolicyBufferLifecycleKind::PinnedS2Lease)
    }

    #[allow(dead_code)]
    pub(crate) const fn dirty_page_tracked() -> Self {
        Self::new(AccessPolicyBufferLifecycleKind::DirtyPageTracked)
    }

    #[allow(dead_code)]
    pub(crate) const fn dirty_mmap_page() -> Self {
        Self::new(AccessPolicyBufferLifecycleKind::DirtyMmapPage)
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn for_certification_pinned_physical_substrate_lease() -> Self {
        Self::pinned_physical_substrate_lease()
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn for_certification_dirty_page_tracked() -> Self {
        Self::dirty_page_tracked()
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn for_certification_dirty_mmap_page() -> Self {
        Self::dirty_mmap_page()
    }

    const fn new(kind: AccessPolicyBufferLifecycleKind) -> Self {
        Self {
            kind,
            _seal: AccessPolicyBufferLifecycleSeal,
        }
    }

    pub const fn kind(self) -> AccessPolicyBufferLifecycleKind {
        self.kind
    }
}
