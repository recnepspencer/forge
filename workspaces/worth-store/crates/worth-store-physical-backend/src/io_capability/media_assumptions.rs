use super::vocabulary::BackendCapabilityKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendMediaAssumptionSet {
    bits: u32,
}

impl BackendMediaAssumptionSet {
    const BUFFERED_FILE: u32 = 1 << 0;
    const DIRECT_IO_ALIGNMENT: u32 = 1 << 1;
    const MMAP_COHERENCE: u32 = 1 << 2;
    const ASYNC_ORDERING: u32 = 1 << 3;
    const FSYNC_DURABILITY: u32 = 1 << 4;
    const DIRECTORY_SYNC: u32 = 1 << 5;
    const DURABLE_RENAME: u32 = 1 << 6;
    const SECURE_FRAME_IO: u32 = 1 << 7;
    const FDATASYNC_DURABILITY: u32 = 1 << 8;
    const SECTOR_ATOMICITY: u32 = 1 << 9;
    const PAGE_CACHE_POLICY: u32 = 1 << 10;
    const FLUSH_ORDERING: u32 = 1 << 11;
    const MMAP_TYPED_FAULTS: u32 = 1 << 12;
    const MMAP_STORE_TRACKED_WRITEBACK: u32 = 1 << 13;
    const MMAP_SHARED_VISIBILITY: u32 = 1 << 14;
    const MMAP_TYPED_TRUNCATE: u32 = 1 << 15;
    const MMAP_TYPED_PUNCH_HOLE: u32 = 1 << 16;
    const MIXED_PAGE_CACHE_INVALIDATION: u32 = 1 << 17;
    const MIXED_STORE_WRITEBACK_SEQUENCING: u32 = 1 << 18;
    const TRIM_POSTURE: u32 = 1 << 19;
    const PUNCH_HOLE_POSTURE: u32 = 1 << 20;
    const SPARSE_POSTURE: u32 = 1 << 21;
    const COLD_TIER_IO_POSTURE: u32 = 1 << 22;
    const MMAP_ACCESS_POLICY: u32 = Self::MMAP_COHERENCE
        | Self::MMAP_TYPED_FAULTS
        | Self::MMAP_STORE_TRACKED_WRITEBACK
        | Self::MMAP_SHARED_VISIBILITY
        | Self::MMAP_TYPED_TRUNCATE
        | Self::MMAP_TYPED_PUNCH_HOLE;
    const MIXED_ACCESS_COHERENCE: u32 =
        Self::MIXED_PAGE_CACHE_INVALIDATION | Self::MIXED_STORE_WRITEBACK_SEQUENCING;

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn platform_file_defaults() -> Self {
        Self {
            bits: Self::BUFFERED_FILE
                | Self::FSYNC_DURABILITY
                | Self::FDATASYNC_DURABILITY
                | Self::DIRECTORY_SYNC
                | Self::DURABLE_RENAME
                | Self::FLUSH_ORDERING,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_direct_io_alignment(self) -> Self {
        Self {
            bits: self.bits | Self::DIRECT_IO_ALIGNMENT,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_fdatasync_durability(self) -> Self {
        Self {
            bits: self.bits | Self::FDATASYNC_DURABILITY,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_sector_atomicity(self) -> Self {
        Self {
            bits: self.bits | Self::SECTOR_ATOMICITY,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_page_cache_policy(self) -> Self {
        Self {
            bits: self.bits | Self::PAGE_CACHE_POLICY,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_flush_ordering(self) -> Self {
        Self {
            bits: self.bits | Self::FLUSH_ORDERING,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_mmap_coherence(self) -> Self {
        Self {
            bits: self.bits | Self::MMAP_ACCESS_POLICY,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_mmap_mapping_coherence(self) -> Self {
        Self {
            bits: self.bits | Self::MMAP_COHERENCE,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_mmap_typed_faults(self) -> Self {
        Self {
            bits: self.bits | Self::MMAP_TYPED_FAULTS,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_mmap_store_tracked_writeback(self) -> Self {
        Self {
            bits: self.bits | Self::MMAP_STORE_TRACKED_WRITEBACK,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_mmap_shared_visibility(self) -> Self {
        Self {
            bits: self.bits | Self::MMAP_SHARED_VISIBILITY,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_mmap_typed_truncate(self) -> Self {
        Self {
            bits: self.bits | Self::MMAP_TYPED_TRUNCATE,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_mmap_typed_punch_hole(self) -> Self {
        Self {
            bits: self.bits | Self::MMAP_TYPED_PUNCH_HOLE,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_mixed_access_coherence(self) -> Self {
        Self {
            bits: self.bits | Self::MIXED_ACCESS_COHERENCE,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_trim_posture(self) -> Self {
        Self {
            bits: self.bits | Self::TRIM_POSTURE,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_punch_hole_posture(self) -> Self {
        Self {
            bits: self.bits | Self::PUNCH_HOLE_POSTURE,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_sparse_posture(self) -> Self {
        Self {
            bits: self.bits | Self::SPARSE_POSTURE,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_cold_tier_io_posture(self) -> Self {
        Self {
            bits: self.bits | Self::COLD_TIER_IO_POSTURE,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_async_ordering(self) -> Self {
        Self {
            bits: self.bits | Self::ASYNC_ORDERING,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_secure_frame_io(self) -> Self {
        Self {
            bits: self.bits | Self::SECURE_FRAME_IO,
        }
    }

    pub const fn supports(self, kind: BackendCapabilityKind) -> bool {
        let required = match kind {
            BackendCapabilityKind::BufferedFile => Self::BUFFERED_FILE,
            BackendCapabilityKind::DirectIo => {
                Self::DIRECT_IO_ALIGNMENT | Self::SECTOR_ATOMICITY | Self::PAGE_CACHE_POLICY
            }
            BackendCapabilityKind::Mmap => Self::MMAP_ACCESS_POLICY | Self::PAGE_CACHE_POLICY,
            BackendCapabilityKind::AsyncIo => Self::ASYNC_ORDERING | Self::FLUSH_ORDERING,
            BackendCapabilityKind::Fsync => {
                Self::FSYNC_DURABILITY | Self::FDATASYNC_DURABILITY | Self::FLUSH_ORDERING
            }
            BackendCapabilityKind::DirectorySync => Self::DIRECTORY_SYNC | Self::FLUSH_ORDERING,
            BackendCapabilityKind::DurableRename => {
                Self::DURABLE_RENAME | Self::DIRECTORY_SYNC | Self::FLUSH_ORDERING
            }
            BackendCapabilityKind::SecureFrameIo => {
                Self::SECURE_FRAME_IO | Self::PAGE_CACHE_POLICY | Self::FLUSH_ORDERING
            }
        };
        (self.bits & required) == required
    }

    pub const fn supports_page_cache_policy(self) -> bool {
        (self.bits & Self::PAGE_CACHE_POLICY) == Self::PAGE_CACHE_POLICY
    }

    pub const fn supports_direct_io_alignment(self) -> bool {
        (self.bits & (Self::DIRECT_IO_ALIGNMENT | Self::SECTOR_ATOMICITY))
            == (Self::DIRECT_IO_ALIGNMENT | Self::SECTOR_ATOMICITY)
    }

    pub const fn supports_admitted_mmap_access_policy(self) -> bool {
        (self.bits & (Self::MMAP_ACCESS_POLICY | Self::PAGE_CACHE_POLICY))
            == (Self::MMAP_ACCESS_POLICY | Self::PAGE_CACHE_POLICY)
    }

    pub const fn supports_mixed_access_coherence(self) -> bool {
        (self.bits & (Self::PAGE_CACHE_POLICY | Self::MIXED_ACCESS_COHERENCE))
            == (Self::PAGE_CACHE_POLICY | Self::MIXED_ACCESS_COHERENCE)
    }

    pub const fn supports_trim_posture(self) -> bool {
        (self.bits & Self::TRIM_POSTURE) == Self::TRIM_POSTURE
    }

    pub const fn supports_punch_hole_posture(self) -> bool {
        (self.bits & Self::PUNCH_HOLE_POSTURE) == Self::PUNCH_HOLE_POSTURE
    }

    pub const fn supports_sparse_posture(self) -> bool {
        (self.bits & Self::SPARSE_POSTURE) == Self::SPARSE_POSTURE
    }

    pub const fn supports_cold_tier_io_posture(self) -> bool {
        (self.bits & Self::COLD_TIER_IO_POSTURE) == Self::COLD_TIER_IO_POSTURE
    }
}
