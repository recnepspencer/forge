use super::{
    UiMountedLogicalDamage, UiMountedPaintCommand, UiMountedPaintOrderIdentity,
    UiMountedPaintOrderIntegrity, UiMountedPresentationAffinity,
    UiMountedPresentationAuxiliaryState,
};

#[derive(Debug, PartialEq)]
pub struct UiMountedPresentationInitial {
    pub(super) affinity: UiMountedPresentationAffinity,
    pub(super) projection: crate::UiMountedProjectionView,
    pub(super) auxiliary: UiMountedPresentationAuxiliaryState,
    pub(super) commands: Box<[UiMountedPaintCommand]>,
    pub(super) order: Box<[UiMountedPaintOrderIdentity]>,
    pub(super) order_integrity: UiMountedPaintOrderIntegrity,
    pub(super) damage: Box<[UiMountedLogicalDamage]>,
}

#[doc(hidden)]
pub struct UiMountedPresentationInitialInput {
    pub successor: crate::UiMountedFrameIdentity,
    pub surface: crate::UiSemanticSurfaceIdentity,
    pub binding: crate::UiSurfaceBindingGeneration,
    pub content: crate::UiMountedContentGeneration,
    pub baseline: crate::UiHostSurfaceBaselineIdentity,
    pub projection: crate::UiMountedProjectionView,
    pub commands: Vec<UiMountedPaintCommand>,
    pub order: Vec<UiMountedPaintOrderIdentity>,
    pub order_integrity: UiMountedPaintOrderIntegrity,
    pub damage: Vec<UiMountedLogicalDamage>,
}

impl UiMountedPresentationInitial {
    #[doc(hidden)]
    pub fn from_inert_mechanics(input: UiMountedPresentationInitialInput) -> Self {
        let auxiliary =
            UiMountedPresentationAuxiliaryState::from_runtime_mounting(&input.projection);
        let affinity = UiMountedPresentationAffinity::from_runtime(
            super::affinity::UiMountedPresentationAffinityInput {
                predecessor: None,
                successor: input.successor,
                surface: input.surface,
                binding: input.binding,
                content: input.content,
                baseline: input.baseline,
            },
        );
        Self {
            affinity,
            projection: input.projection,
            auxiliary,
            commands: input.commands.into_boxed_slice(),
            order: input.order.into_boxed_slice(),
            order_integrity: input.order_integrity,
            damage: input.damage.into_boxed_slice(),
        }
    }

    pub const fn affinity(&self) -> UiMountedPresentationAffinity {
        self.affinity
    }

    pub fn projection(&self) -> &crate::UiMountedProjectionView {
        &self.projection
    }

    pub fn auxiliary(&self) -> &UiMountedPresentationAuxiliaryState {
        &self.auxiliary
    }

    pub fn commands(&self) -> &[UiMountedPaintCommand] {
        &self.commands
    }

    pub fn order(&self) -> &[UiMountedPaintOrderIdentity] {
        &self.order
    }

    pub const fn order_integrity(&self) -> UiMountedPaintOrderIntegrity {
        self.order_integrity
    }

    pub fn damage(&self) -> &[UiMountedLogicalDamage] {
        &self.damage
    }
}
