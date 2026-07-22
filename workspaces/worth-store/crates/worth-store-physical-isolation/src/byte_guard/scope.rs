use crate::CurrentGenerationPhysicalReference;
use worth_store_buffer_pool::PhysicalFrameKey;
#[cfg(feature = "legacy-certification-models")]
use worth_store_buffer_pool::ResidentFrameToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalByteGuardScope {
    ResidentFrame {
        reference: CurrentGenerationPhysicalReference,
        frame: PhysicalFrameKey,
    },
    #[cfg(feature = "legacy-certification-models")]
    LegacyResidentFrame {
        reference: CurrentGenerationPhysicalReference,
        token: ResidentFrameToken,
    },
    MmapView {
        reference: CurrentGenerationPhysicalReference,
    },
    ExtentWindow {
        reference: CurrentGenerationPhysicalReference,
    },
    OwnedReadBuffer {
        reference: CurrentGenerationPhysicalReference,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalByteGuardScopeKind {
    ResidentFrame,
    MmapView,
    ExtentWindow,
    OwnedReadBuffer,
}

impl PhysicalByteGuardScope {
    pub const fn for_resident_frame(
        reference: CurrentGenerationPhysicalReference,
        frame: PhysicalFrameKey,
    ) -> Self {
        Self::ResidentFrame { reference, frame }
    }

    #[cfg(feature = "legacy-certification-models")]
    pub const fn for_legacy_resident_frame(
        reference: CurrentGenerationPhysicalReference,
        token: ResidentFrameToken,
    ) -> Self {
        Self::LegacyResidentFrame { reference, token }
    }

    pub const fn for_mmap_view(reference: CurrentGenerationPhysicalReference) -> Self {
        Self::MmapView { reference }
    }

    pub const fn for_extent_window(reference: CurrentGenerationPhysicalReference) -> Self {
        Self::ExtentWindow { reference }
    }

    pub const fn for_owned_read_buffer(reference: CurrentGenerationPhysicalReference) -> Self {
        Self::OwnedReadBuffer { reference }
    }

    pub const fn reference(self) -> CurrentGenerationPhysicalReference {
        match self {
            Self::ResidentFrame { reference, .. }
            | Self::MmapView { reference }
            | Self::ExtentWindow { reference }
            | Self::OwnedReadBuffer { reference } => reference,
            #[cfg(feature = "legacy-certification-models")]
            Self::LegacyResidentFrame { reference, .. } => reference,
        }
    }

    pub const fn resident_frame(self) -> Option<PhysicalFrameKey> {
        match self {
            Self::ResidentFrame { frame, .. } => Some(frame),
            _ => None,
        }
    }

    #[cfg(feature = "legacy-certification-models")]
    pub const fn legacy_resident_frame_token(self) -> Option<ResidentFrameToken> {
        match self {
            Self::LegacyResidentFrame { token, .. } => Some(token),
            _ => None,
        }
    }

    pub const fn kind(self) -> PhysicalByteGuardScopeKind {
        match self {
            Self::ResidentFrame { .. } => PhysicalByteGuardScopeKind::ResidentFrame,
            #[cfg(feature = "legacy-certification-models")]
            Self::LegacyResidentFrame { .. } => PhysicalByteGuardScopeKind::ResidentFrame,
            Self::MmapView { .. } => PhysicalByteGuardScopeKind::MmapView,
            Self::ExtentWindow { .. } => PhysicalByteGuardScopeKind::ExtentWindow,
            Self::OwnedReadBuffer { .. } => PhysicalByteGuardScopeKind::OwnedReadBuffer,
        }
    }
}
