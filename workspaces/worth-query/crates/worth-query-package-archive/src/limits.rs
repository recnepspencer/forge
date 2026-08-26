use worth_query_installation::facade::{
    WORTH_QUERY_PORTABLE_PACKAGE_MAX_LOGICAL_EXPORT_BYTES,
    WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECONSTRUCTION_CANONICAL_BYTES,
    WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECONSTRUCTION_ENTRIES,
    WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECORDS,
};

use crate::record::RECORD_FRAME_HEADER_BYTES;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryPackageArchiveLimits {
    maximum_archive_bytes: u64,
    maximum_manifest_frame_bytes: u64,
    maximum_records: u32,
    maximum_logical_bytes: u64,
    maximum_nested_entries: u64,
    maximum_nesting_depth: u32,
    maximum_canonical_work_bytes: u64,
}

impl WorthQueryPackageArchiveLimits {
    pub const DEFAULT: Self = Self {
        maximum_archive_bytes: 4_096
            + (WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECORDS as u64 * RECORD_FRAME_HEADER_BYTES)
            + WORTH_QUERY_PORTABLE_PACKAGE_MAX_LOGICAL_EXPORT_BYTES,
        maximum_manifest_frame_bytes: 4_096,
        maximum_records: WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECORDS,
        maximum_logical_bytes: WORTH_QUERY_PORTABLE_PACKAGE_MAX_LOGICAL_EXPORT_BYTES,
        maximum_nested_entries: WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECONSTRUCTION_ENTRIES,
        maximum_nesting_depth: 128,
        maximum_canonical_work_bytes:
            WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECONSTRUCTION_CANONICAL_BYTES,
    };

    pub const fn new(
        maximum_manifest_frame_bytes: u64,
        maximum_records: u32,
        maximum_logical_bytes: u64,
        maximum_canonical_work_bytes: u64,
    ) -> Self {
        Self {
            maximum_archive_bytes: Self::DEFAULT.maximum_archive_bytes,
            maximum_manifest_frame_bytes,
            maximum_records,
            maximum_logical_bytes,
            maximum_nested_entries: Self::DEFAULT.maximum_nested_entries,
            maximum_nesting_depth: Self::DEFAULT.maximum_nesting_depth,
            maximum_canonical_work_bytes,
        }
    }

    pub const fn with_maximum_archive_bytes(mut self, maximum_archive_bytes: u64) -> Self {
        self.maximum_archive_bytes = maximum_archive_bytes;
        self
    }

    pub const fn with_maximum_nested_entries(mut self, maximum_nested_entries: u64) -> Self {
        self.maximum_nested_entries = maximum_nested_entries;
        self
    }

    pub const fn with_maximum_nesting_depth(mut self, maximum_nesting_depth: u32) -> Self {
        self.maximum_nesting_depth = maximum_nesting_depth;
        self
    }

    pub const fn maximum_archive_bytes(self) -> u64 {
        self.maximum_archive_bytes
    }
    pub const fn maximum_manifest_frame_bytes(self) -> u64 {
        self.maximum_manifest_frame_bytes
    }
    pub const fn maximum_records(self) -> u32 {
        self.maximum_records
    }
    pub const fn maximum_logical_bytes(self) -> u64 {
        self.maximum_logical_bytes
    }
    pub const fn maximum_nested_entries(self) -> u64 {
        self.maximum_nested_entries
    }
    pub const fn maximum_nesting_depth(self) -> u32 {
        self.maximum_nesting_depth
    }
    pub const fn maximum_canonical_work_bytes(self) -> u64 {
        self.maximum_canonical_work_bytes
    }

    pub(crate) const fn narrowed(self) -> Self {
        Self {
            maximum_archive_bytes: if self.maximum_archive_bytes
                < Self::DEFAULT.maximum_archive_bytes
            {
                self.maximum_archive_bytes
            } else {
                Self::DEFAULT.maximum_archive_bytes
            },
            maximum_manifest_frame_bytes: if self.maximum_manifest_frame_bytes
                < Self::DEFAULT.maximum_manifest_frame_bytes
            {
                self.maximum_manifest_frame_bytes
            } else {
                Self::DEFAULT.maximum_manifest_frame_bytes
            },
            maximum_records: if self.maximum_records < Self::DEFAULT.maximum_records {
                self.maximum_records
            } else {
                Self::DEFAULT.maximum_records
            },
            maximum_logical_bytes: if self.maximum_logical_bytes
                < Self::DEFAULT.maximum_logical_bytes
            {
                self.maximum_logical_bytes
            } else {
                Self::DEFAULT.maximum_logical_bytes
            },
            maximum_nested_entries: if self.maximum_nested_entries
                < Self::DEFAULT.maximum_nested_entries
            {
                self.maximum_nested_entries
            } else {
                Self::DEFAULT.maximum_nested_entries
            },
            maximum_nesting_depth: if self.maximum_nesting_depth
                < Self::DEFAULT.maximum_nesting_depth
            {
                self.maximum_nesting_depth
            } else {
                Self::DEFAULT.maximum_nesting_depth
            },
            maximum_canonical_work_bytes: if self.maximum_canonical_work_bytes
                < Self::DEFAULT.maximum_canonical_work_bytes
            {
                self.maximum_canonical_work_bytes
            } else {
                Self::DEFAULT.maximum_canonical_work_bytes
            },
        }
    }
}
