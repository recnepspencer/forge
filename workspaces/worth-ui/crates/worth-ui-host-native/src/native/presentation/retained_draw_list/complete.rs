use worth_ui_host_contract::{
    UiMountedPaintCommand, UiMountedPaintOrderIdentity, UiMountedPaintOrderIntegrity,
    UiMountedPresentationInitial, UiMountedPresentationReconstruction,
};

use super::{
    command_store::UiNativeRetainedCommandStore, visible_bounds, UiNativeRetainedDrawList,
    UiNativeRetainedDrawListDenial,
};
use crate::native::presentation::damage_index::UiNativeDamageIndex;
use crate::native::presentation::retained_order::UiNativeRetainedOrder;

impl UiNativeRetainedDrawList {
    pub(in crate::native::presentation) fn initial(
        initial: &UiMountedPresentationInitial,
    ) -> Result<Self, UiNativeRetainedDrawListDenial> {
        if initial.affinity().baseline().transparent_rgba8() != [0, 0, 0, 0]
            || !initial.order_integrity().admits(initial.order())
        {
            return Err(UiNativeRetainedDrawListDenial::BaselineUnavailable);
        }
        Self::from_complete(
            initial.affinity().successor(),
            initial.affinity().surface(),
            initial.affinity().binding(),
            initial.affinity().baseline(),
            initial.commands(),
            initial.order(),
            initial.order_integrity(),
        )
    }

    pub(in crate::native::presentation) fn reconstruction(
        work: &UiMountedPresentationReconstruction,
    ) -> Result<Self, UiNativeRetainedDrawListDenial> {
        if work.affinity().predecessor().is_none()
            || work.affinity().baseline().transparent_rgba8() != [0, 0, 0, 0]
            || !work.order_integrity().admits(work.order())
        {
            return Err(UiNativeRetainedDrawListDenial::BaselineUnavailable);
        }
        Self::from_complete(
            work.affinity().successor(),
            work.affinity().surface(),
            work.affinity().binding(),
            work.affinity().baseline(),
            work.commands(),
            work.order(),
            work.order_integrity(),
        )
    }

    pub(super) fn from_complete(
        frame: worth_ui_host_contract::UiMountedFrameIdentity,
        surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
        baseline: worth_ui_host_contract::UiHostSurfaceBaselineIdentity,
        source_commands: &[UiMountedPaintCommand],
        source_order: &[UiMountedPaintOrderIdentity],
        order_integrity: UiMountedPaintOrderIntegrity,
    ) -> Result<Self, UiNativeRetainedDrawListDenial> {
        let mut commands = UiNativeRetainedCommandStore::with_capacity(source_commands.len());
        let mut damage = UiNativeDamageIndex::new();
        for command in source_commands {
            if commands
                .insert(command.identity(), command.clone())
                .is_some()
            {
                return Err(UiNativeRetainedDrawListDenial::CommandMismatch);
            }
            if let Some(bounds) = visible_bounds(command) {
                damage.insert(command.identity(), bounds)?;
            }
        }
        if source_order.len() != commands.len()
            || source_order
                .iter()
                .any(|identity| !commands.contains(&identity.command()))
        {
            return Err(UiNativeRetainedDrawListDenial::OrderMismatch);
        }
        Ok(Self {
            frame,
            surface,
            binding,
            baseline,
            commands,
            order: UiNativeRetainedOrder::initial(source_order.iter().copied())?,
            order_integrity,
            damage,
        })
    }
}
