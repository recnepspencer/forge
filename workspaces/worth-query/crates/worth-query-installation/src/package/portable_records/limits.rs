//! Explicit cold-path budgets for typed package export.

/// Default maximum number of logical package records.
pub const WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECORDS: u32 = 65_536;
/// Default maximum logical export bytes, including record framing and source meaning.
pub const WORTH_QUERY_PORTABLE_PACKAGE_MAX_LOGICAL_EXPORT_BYTES: u64 = 64 * 1_024 * 1_024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryPortablePackageExportLimits {
    maximum_records: u32,
    maximum_logical_export_bytes: u64,
}

impl WorthQueryPortablePackageExportLimits {
    pub const DEFAULT: Self = Self {
        maximum_records: WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECORDS,
        maximum_logical_export_bytes: WORTH_QUERY_PORTABLE_PACKAGE_MAX_LOGICAL_EXPORT_BYTES,
    };

    pub const fn new(maximum_records: u32, maximum_logical_export_bytes: u64) -> Self {
        Self {
            maximum_records,
            maximum_logical_export_bytes,
        }
    }

    pub const fn maximum_records(self) -> u32 {
        self.maximum_records
    }

    pub const fn maximum_logical_export_bytes(self) -> u64 {
        self.maximum_logical_export_bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPortablePackageExportDenialKind {
    RecordCountExceeded,
    LogicalExportBytesExceeded,
    IncompleteRecordClosure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPortablePackageExportDenial {
    kind: WorthQueryPortablePackageExportDenialKind,
}

impl WorthQueryPortablePackageExportDenial {
    pub(crate) const fn new(kind: WorthQueryPortablePackageExportDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthQueryPortablePackageExportDenialKind {
        self.kind
    }
}
