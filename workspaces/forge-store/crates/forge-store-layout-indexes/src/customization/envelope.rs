use crate::artifact_family::ArtifactFamilyAccessLane;

use super::S8FutureLayoutCapabilityRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8FutureLayoutWorkloadEnvelope {
    ForegroundLowFanout,
    ForegroundBoundedTraversal,
    BackgroundRebuildProjection,
    VerifierCorpusInspection,
}

impl S8FutureLayoutWorkloadEnvelope {
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

    pub const fn supports_capability(self, capability: S8FutureLayoutCapabilityRequest) -> bool {
        match (self, capability) {
            (Self::ForegroundLowFanout, S8FutureLayoutCapabilityRequest::PointLookup { .. }) => {
                true
            }
            (
                Self::ForegroundBoundedTraversal,
                S8FutureLayoutCapabilityRequest::PointLookup { .. }
                | S8FutureLayoutCapabilityRequest::OrderedRange { .. }
                | S8FutureLayoutCapabilityRequest::PrefixTraversal { .. }
                | S8FutureLayoutCapabilityRequest::BlobStreaming { .. },
            ) => true,
            (
                Self::BackgroundRebuildProjection,
                S8FutureLayoutCapabilityRequest::RebuildableProjection { .. },
            ) => true,
            (
                Self::VerifierCorpusInspection,
                S8FutureLayoutCapabilityRequest::VerifierDeclaredScan { .. },
            ) => true,
            _ => false,
        }
    }
}
