#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedPresentationAffinity {
    predecessor: Option<crate::UiMountedFrameIdentity>,
    successor: crate::UiMountedFrameIdentity,
    surface: crate::UiSemanticSurfaceIdentity,
    binding: crate::UiSurfaceBindingGeneration,
    content: crate::UiMountedContentGeneration,
    baseline: crate::UiHostSurfaceBaselineIdentity,
}

pub(super) struct UiMountedPresentationAffinityInput {
    pub predecessor: Option<crate::UiMountedFrameIdentity>,
    pub successor: crate::UiMountedFrameIdentity,
    pub surface: crate::UiSemanticSurfaceIdentity,
    pub binding: crate::UiSurfaceBindingGeneration,
    pub content: crate::UiMountedContentGeneration,
    pub baseline: crate::UiHostSurfaceBaselineIdentity,
}

impl UiMountedPresentationAffinity {
    pub(super) const fn from_runtime(input: UiMountedPresentationAffinityInput) -> Self {
        Self {
            predecessor: input.predecessor,
            successor: input.successor,
            surface: input.surface,
            binding: input.binding,
            content: input.content,
            baseline: input.baseline,
        }
    }

    pub const fn predecessor(self) -> Option<crate::UiMountedFrameIdentity> {
        self.predecessor
    }

    pub const fn successor(self) -> crate::UiMountedFrameIdentity {
        self.successor
    }

    pub const fn surface(self) -> crate::UiSemanticSurfaceIdentity {
        self.surface
    }

    pub const fn binding(self) -> crate::UiSurfaceBindingGeneration {
        self.binding
    }

    pub const fn content(self) -> crate::UiMountedContentGeneration {
        self.content
    }

    pub const fn baseline(self) -> crate::UiHostSurfaceBaselineIdentity {
        self.baseline
    }
}
