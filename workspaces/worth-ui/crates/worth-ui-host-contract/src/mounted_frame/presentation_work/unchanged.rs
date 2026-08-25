use super::UiMountedPresentationAffinity;

#[derive(Debug, PartialEq)]
pub struct UiMountedPresentationUnchanged {
    pub(super) affinity: UiMountedPresentationAffinity,
    pub(super) production_cost: crate::UiMountedPresentationProductionCost,
}

#[doc(hidden)]
pub struct UiMountedPresentationUnchangedInput {
    pub predecessor: crate::UiMountedFrameIdentity,
    pub successor: crate::UiMountedFrameIdentity,
    pub surface: crate::UiSemanticSurfaceIdentity,
    pub binding: crate::UiSurfaceBindingGeneration,
    pub content: crate::UiMountedContentGeneration,
    pub baseline: crate::UiHostSurfaceBaselineIdentity,
    pub production_cost: crate::UiMountedPresentationProductionCost,
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
                receipt_affinity: None,
            },
        );
        Self {
            affinity,
            production_cost: input.production_cost,
        }
    }

    pub const fn affinity(&self) -> UiMountedPresentationAffinity {
        self.affinity
    }

    #[doc(hidden)]
    pub const fn with_successor_receipt_affinity(
        mut self,
        affinity: Option<crate::UiMountedNodeReceiptAffinity>,
    ) -> Self {
        self.affinity = self.affinity.with_receipt_affinity(affinity);
        self
    }

    pub const fn production_cost(&self) -> crate::UiMountedPresentationProductionCost {
        self.production_cost
    }
}
