pub use worth_store_buffer_pool::PhysicalResidencyDimension;

/// Why a physical residency declaration could not become an admitted policy.
///
/// Every variant is a pre-construction configuration failure. No pool or
/// serving runtime is created when this denial is returned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRecordResidencyPolicyDenial {
    MissingRequiredDimension(PhysicalResidencyDimension),
    CategoryExceedsTotal {
        dimension: PhysicalResidencyDimension,
        declared: u64,
        total: u64,
    },
    ScopeExceedsOperation {
        scope: super::PhysicalOperationAllocationScope,
        declared: u64,
        operation: u64,
    },
    CountExceedsFrameEntries {
        dimension: PhysicalResidencyDimension,
        declared: u32,
        frame_entries: u32,
    },
    PageExceedsResidentBytes {
        page: u64,
        resident: u64,
    },
    PageExceedsOperationBytes {
        page: u64,
        operation: u64,
    },
    PageExceedsDirtyReplacementBytes {
        page: u64,
        dirty_replacement: u64,
    },
}

impl From<worth_store_buffer_pool::PhysicalResidencyLimitsAdmissionDenial>
    for PhysicalRecordResidencyPolicyDenial
{
    fn from(denial: worth_store_buffer_pool::PhysicalResidencyLimitsAdmissionDenial) -> Self {
        use worth_store_buffer_pool::PhysicalResidencyLimitsAdmissionDenial as Lower;
        match denial {
            Lower::Missing(dimension) => Self::MissingRequiredDimension(dimension),
            Lower::CategoryExceedsTotal {
                dimension,
                declared,
                total,
            } => Self::CategoryExceedsTotal {
                dimension,
                declared,
                total,
            },
            Lower::ScopeExceedsOperation {
                scope,
                declared,
                operation,
            } => Self::ScopeExceedsOperation {
                scope,
                declared,
                operation,
            },
            Lower::CountExceedsFrameEntries {
                dimension,
                declared,
                frame_entries,
            } => Self::CountExceedsFrameEntries {
                dimension,
                declared,
                frame_entries,
            },
            Lower::PageExceedsResidentBytes { page, resident } => {
                Self::PageExceedsResidentBytes { page, resident }
            }
            Lower::PageExceedsOperationBytes { page, operation } => {
                Self::PageExceedsOperationBytes { page, operation }
            }
            Lower::PageExceedsDirtyReplacementBytes {
                page,
                dirty_replacement,
            } => Self::PageExceedsDirtyReplacementBytes {
                page,
                dirty_replacement,
            },
        }
    }
}
