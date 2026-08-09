use super::UiMountedPresentationAffinity;

#[derive(Debug, PartialEq)]
pub struct UiMountedPresentationUnchanged {
    pub(super) affinity: UiMountedPresentationAffinity,
}

#[doc(hidden)]
pub struct UiMountedPresentationUnchangedInput {
    pub predecessor: crate::UiMountedFrameIdentity,
    pub successor: crate::UiMountedFrameIdentity,
    pub surface: crate::UiSemanticSurfaceIdentity,
    pub binding: crate::UiSurfaceBindingGeneration,
    pub content: crate::UiMountedContentGeneration,
    pub baseline: crate::UiHostSurfaceBaselineIdentity,
}

impl UiMountedPresentationUnchanged {
    #[doc(hidden)]
    pub fn from_inert_mechanics(input: UiMountedPresentationUnchangedInput) -> Self {
        let affinity = UiMountedPresentationAffinity::from_runtime(
            super::affinity::UiMountedPresentationAffinityInput {
                predecessor: Some(input.predecessor),
                successor: input.successor,
                surface: input.surface,
                binding: input.binding,
                content: input.content,
                baseline: input.baseline,
            },
        );
        Self { affinity }
    }

    pub const fn affinity(&self) -> UiMountedPresentationAffinity {
        self.affinity
    }
}
