//! Caller-narrowable work limits for typed record reconstruction.

use crate::package::{
    WORTH_QUERY_PORTABLE_PACKAGE_MAX_LOGICAL_EXPORT_BYTES, WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECORDS,
};

/// Default aggregate nested-entry ceiling for one reconstructed package.
pub const WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECONSTRUCTION_ENTRIES: u64 = 1_048_576;
/// Default aggregate canonical-work ceiling for one reconstructed package.
pub const WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECONSTRUCTION_CANONICAL_BYTES: u64 =
    WORTH_QUERY_PORTABLE_PACKAGE_MAX_LOGICAL_EXPORT_BYTES;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryPortablePackageReconstructionLimits {
    maximum_records: u32,
    maximum_logical_bytes: u64,
    maximum_nested_entries: u64,
    maximum_canonical_work_bytes: u64,
    canonical_query_limits: worth_query_declaration::facade::canonicalization::WorthQueryPortableCanonicalQueryReadmissionLimits,
}

impl WorthQueryPortablePackageReconstructionLimits {
    /// Repository-wide record-allocation ceiling for typed intake.
    pub const DEFAULT: Self = Self {
        maximum_records: WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECORDS,
        maximum_logical_bytes: WORTH_QUERY_PORTABLE_PACKAGE_MAX_LOGICAL_EXPORT_BYTES,
        maximum_nested_entries: WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECONSTRUCTION_ENTRIES,
        maximum_canonical_work_bytes:
            WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECONSTRUCTION_CANONICAL_BYTES,
        canonical_query_limits: worth_query_declaration::facade::canonicalization::WorthQueryPortableCanonicalQueryReadmissionLimits::DEFAULT,
    };

    pub const fn new(maximum_records: u32) -> Self {
        Self {
            maximum_records,
            ..Self::DEFAULT
        }
    }

    pub const fn with_work_bounds(
        mut self,
        maximum_logical_bytes: u64,
        maximum_nested_entries: u64,
        maximum_canonical_work_bytes: u64,
    ) -> Self {
        self.maximum_logical_bytes = maximum_logical_bytes;
        self.maximum_nested_entries = maximum_nested_entries;
        self.maximum_canonical_work_bytes = maximum_canonical_work_bytes;
        self
    }

    pub const fn with_canonical_query_limits(
        mut self,
        canonical_query_limits: worth_query_declaration::facade::canonicalization::WorthQueryPortableCanonicalQueryReadmissionLimits,
    ) -> Self {
        self.canonical_query_limits = canonical_query_limits;
        self
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

    pub const fn maximum_canonical_work_bytes(self) -> u64 {
        self.maximum_canonical_work_bytes
    }

    pub const fn canonical_query_limits(
        self,
    ) -> worth_query_declaration::facade::canonicalization::WorthQueryPortableCanonicalQueryReadmissionLimits{
        self.canonical_query_limits
    }

    pub(super) const fn narrowed(self) -> Self {
        Self {
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
            maximum_canonical_work_bytes: if self.maximum_canonical_work_bytes
                < Self::DEFAULT.maximum_canonical_work_bytes
            {
                self.maximum_canonical_work_bytes
            } else {
                Self::DEFAULT.maximum_canonical_work_bytes
            },
            canonical_query_limits: self.canonical_query_limits,
        }
    }
}
