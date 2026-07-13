use crate::catalog::ArtifactFamilyAccessLane;

use super::FutureLayoutCapabilityRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FutureLayoutWorkloadEnvelope {
    ForegroundLowFanout,
    ForegroundBoundedTraversal,
    BackgroundRebuildProjection,
    VerifierCorpusInspection,
}

impl FutureLayoutWorkloadEnvelope {
    pub const fn foreground_low_fanout() -> Self {
        Self::ForegroundLowFanout
    }

    pub const fn foreground_bounded_traversal() -> Self {
        Self::ForegroundBoundedTraversal
    }

    pub const fn background_rebuild_projection() -> Self {
        Self::BackgroundRebuildProjection
    }

    pub const fn verifier_corpus_inspection() -> Self {
        Self::VerifierCorpusInspection
    }

    pub const fn admitted_lane(self) -> ArtifactFamilyAccessLane {
        match self {
            Self::ForegroundLowFanout | Self::ForegroundBoundedTraversal => {
                ArtifactFamilyAccessLane::HotPath
            }
            Self::BackgroundRebuildProjection => ArtifactFamilyAccessLane::MaintenancePath,
            Self::VerifierCorpusInspection => ArtifactFamilyAccessLane::VerifierPath,
        }
    }

    pub const fn supports_capability(self, capability: FutureLayoutCapabilityRequest) -> bool {
        match (self, capability) {
            (Self::ForegroundLowFanout, FutureLayoutCapabilityRequest::PointLookup { .. }) => true,
            (
                Self::ForegroundBoundedTraversal,
                FutureLayoutCapabilityRequest::PointLookup { .. }
                | FutureLayoutCapabilityRequest::OrderedRange { .. }
                | FutureLayoutCapabilityRequest::PrefixTraversal { .. }
                | FutureLayoutCapabilityRequest::BlobStreaming { .. },
            ) => true,
            (
                Self::BackgroundRebuildProjection,
                FutureLayoutCapabilityRequest::RebuildableProjection { .. },
            ) => true,
            (
                Self::VerifierCorpusInspection,
                FutureLayoutCapabilityRequest::VerifierDeclaredScan { .. },
            ) => true,
            _ => false,
        }
    }
}
