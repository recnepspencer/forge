use worth_foundational::{
    BoundaryArtifactField, BoundaryArtifactLocator, BoundaryMismatchLocator, BoundarySourceLocator,
    FoundationalBoundaryEvidenceLocality,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalArtifactKind {
    WalFrame,
    PageImage,
    CheckpointManifest,
    CompactionProduct,
    RootPointer,
    CrashRecoveryRuntime,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalFaultFieldKind {
    Header,
    ChecksumProtectedPayload,
    LengthField,
    GenerationField,
    RootPointer,
    SlotState,
    RuntimeIsolation,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExpectedFaultLocalization {
    PreDecodeBoundary,
    PhysicalIntegrityBoundary,
    FreshRuntimeRecoveryBoundary,
    ProductionDriverBoundary,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalFaultOffset {
    byte_offset: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalArtifactFaultLocus {
    artifact_kind: PhysicalArtifactKind,
    field_kind: PhysicalFaultFieldKind,
    artifact_locator: BoundaryArtifactLocator,
    source_locator: BoundarySourceLocator,
    mismatch_locator: BoundaryMismatchLocator,
    offset: Option<PhysicalFaultOffset>,
    expected_localization: ExpectedFaultLocalization,
    expected_locality: FoundationalBoundaryEvidenceLocality,
    ambiguous: bool,
}

impl PhysicalFaultOffset {
    pub const fn at(byte_offset: u64) -> Self {
        Self { byte_offset }
    }

    pub const fn byte_offset(self) -> u64 {
        self.byte_offset
    }
}

impl PhysicalArtifactFaultLocus {
    pub fn wal_frame(
        artifact_locator: BoundaryArtifactLocator,
        field_kind: PhysicalFaultFieldKind,
        offset: PhysicalFaultOffset,
        expected_localization: ExpectedFaultLocalization,
    ) -> Self {
        Self::located(
            PhysicalArtifactKind::WalFrame,
            field_kind,
            artifact_locator,
            Some(offset),
            expected_localization,
        )
    }

    pub fn page_image(
        artifact_locator: BoundaryArtifactLocator,
        field_kind: PhysicalFaultFieldKind,
        offset: PhysicalFaultOffset,
        expected_localization: ExpectedFaultLocalization,
    ) -> Self {
        Self::located(
            PhysicalArtifactKind::PageImage,
            field_kind,
            artifact_locator,
            Some(offset),
            expected_localization,
        )
    }

    pub fn root_pointer(
        artifact_locator: BoundaryArtifactLocator,
        expected_localization: ExpectedFaultLocalization,
    ) -> Self {
        Self::located(
            PhysicalArtifactKind::RootPointer,
            PhysicalFaultFieldKind::RootPointer,
            artifact_locator,
            None,
            expected_localization,
        )
    }

    pub fn crash_recovery_runtime(
        artifact_locator: BoundaryArtifactLocator,
        expected_localization: ExpectedFaultLocalization,
    ) -> Self {
        Self::located(
            PhysicalArtifactKind::CrashRecoveryRuntime,
            PhysicalFaultFieldKind::RuntimeIsolation,
            artifact_locator,
            None,
            expected_localization,
        )
    }

    pub(crate) fn ambiguous_for_denial() -> Self {
        Self {
            ambiguous: true,
            ..Self::located(
                PhysicalArtifactKind::FutureExtensionSlot,
                PhysicalFaultFieldKind::FutureExtensionSlot,
                BoundaryArtifactLocator::new(
                    worth_foundational::BoundaryArtifactId::new(0),
                    BoundaryArtifactField::Basis,
                ),
                None,
                ExpectedFaultLocalization::FutureExtensionSlot,
            )
        }
    }

    fn located(
        artifact_kind: PhysicalArtifactKind,
        field_kind: PhysicalFaultFieldKind,
        artifact_locator: BoundaryArtifactLocator,
        offset: Option<PhysicalFaultOffset>,
        expected_localization: ExpectedFaultLocalization,
    ) -> Self {
        Self {
            artifact_kind,
            field_kind,
            artifact_locator,
            source_locator: BoundarySourceLocator::boundary_artifact(artifact_locator),
            mismatch_locator: BoundaryMismatchLocator::boundary_artifact(artifact_locator),
            offset,
            expected_localization,
            expected_locality: FoundationalBoundaryEvidenceLocality::Current,
            ambiguous: false,
        }
    }

    pub const fn artifact_kind(&self) -> PhysicalArtifactKind {
        self.artifact_kind
    }

    pub const fn field_kind(&self) -> PhysicalFaultFieldKind {
        self.field_kind
    }

    pub const fn artifact_locator(&self) -> &BoundaryArtifactLocator {
        &self.artifact_locator
    }

    pub const fn source_locator(&self) -> &BoundarySourceLocator {
        &self.source_locator
    }

    pub const fn mismatch_locator(&self) -> &BoundaryMismatchLocator {
        &self.mismatch_locator
    }

    pub const fn offset(&self) -> Option<PhysicalFaultOffset> {
        self.offset
    }

    pub const fn expected_localization(&self) -> ExpectedFaultLocalization {
        self.expected_localization
    }

    pub const fn expected_locality(&self) -> FoundationalBoundaryEvidenceLocality {
        self.expected_locality
    }

    pub(crate) const fn is_ambiguous(&self) -> bool {
        self.ambiguous
    }
}
