use crate::{PhysicalAlignmentSite, PhysicalFieldWidthKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalBinaryFormatError {
    AlignmentMismatch(PhysicalAlignmentSite),
    ByteOrderMismatch,
    DowngradeRefused,
    FieldWidthMismatch(PhysicalFieldWidthKind),
    ForwardMigrationNotAdmission,
    ForwardPreservationNotAdmission,
    GoldenHeaderLengthMismatch { expected: usize, actual: usize },
    HostEndianRejected,
    MagicMismatch,
    MissingAlignment(PhysicalAlignmentSite),
    MissingByteOrder,
    MissingFieldWidth(PhysicalFieldWidthKind),
    MissingForwardCompatibilityPolicy,
    MissingMagic,
    MissingPageSize,
    MissingReservedFieldPolicy,
    MissingVersion,
    RustLayoutRejected,
    SerdeOrderRejected,
    UnknownReservedFieldPolicy,
    UnsupportedForwardCompatibilityPolicy,
    UnsupportedPageSize(u32),
    VersionMismatch,
}
