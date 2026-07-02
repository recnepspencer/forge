use crate::CurrentGenerationPhysicalReference;
use forge_store_buffer_pool::ResidentFrameToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalByteGuardScope {
    ResidentFrame {
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
        token: ResidentFrameToken,
    ) -> Self {
        Self::ResidentFrame { reference, token }
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
        }
    }

    pub const fn resident_frame_token(self) -> Option<ResidentFrameToken> {
        match self {
            Self::ResidentFrame { token, .. } => Some(token),
            Self::MmapView { .. } | Self::ExtentWindow { .. } | Self::OwnedReadBuffer { .. } => {
                None
            }
        }
    }

    pub const fn kind(self) -> PhysicalByteGuardScopeKind {
        match self {
            Self::ResidentFrame { .. } => PhysicalByteGuardScopeKind::ResidentFrame,
            Self::MmapView { .. } => PhysicalByteGuardScopeKind::MmapView,
            Self::ExtentWindow { .. } => PhysicalByteGuardScopeKind::ExtentWindow,
            Self::OwnedReadBuffer { .. } => PhysicalByteGuardScopeKind::OwnedReadBuffer,
        }
    }
}
