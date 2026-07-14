//! Freeze snapshot authority — immutable capability truth after registration freeze.

pub use crate::capability::{
    CapabilitySnapshot, CapabilitySnapshotDigest, CapabilitySnapshotIndex, FrozenCapabilityFamily,
    FrozenCommandCapabilities, FrozenCommandProjectionCapabilities, FrozenCommandProjectionEntry,
    FrozenComponentCapabilities, FrozenIconCapabilities, FrozenIconEntry,
    FrozenMosaicPlacementCapabilities, FrozenMosaicRegionCapabilities,
    FrozenMosaicSizingCapabilities, FrozenMosaicStateCapabilities, FrozenMosaicStateSlotEntry,
    FrozenNativeCapabilities, FrozenNativeCapabilityEntry, FrozenPluginSlotCapabilities,
    FrozenPluginSlotEntry, FrozenRuntimeOutcomeProjectionCapabilities,
    FrozenRuntimeOutcomeProjectionEntry, FrozenSettingCapabilities, FrozenSettingEntry,
    FrozenSurfaceCapabilities, FrozenTaskPresentationCapabilities, FrozenTaskPresentationEntry,
    FrozenThemeTokenCapabilities, FrozenThemeTokenEntry, FrozenViewBindingCapabilities,
    FrozenViewBindingEntry, RegisteredCapabilitySet, SnapshotFamilyIndex, SnapshotFreezeReport,
    SnapshotLookupCounters, SnapshotLookupReport, SnapshotMetrics,
    SnapshotReferenceValidationReport, SnapshotReferenceViolation, SnapshotReferenceViolationKind,
};
