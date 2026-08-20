use worth_ui_host_contract::{
    UiMountedPaintCommand, UiMountedPaintOrderIdentity, UiMountedPaintOrderIntegrity,
    UiMountedPresentationInitial, UiMountedPresentationReconstruction,
};

use super::mutation::visible_bounds;
use super::{
    command_store::UiNativeRetainedCommandStore, UiNativeRetainedDrawList,
    UiNativeRetainedDrawListDenial,
};
use crate::native::presentation::damage_index::UiNativeDamageIndex;
use crate::native::presentation::retained_order::UiNativeRetainedOrder;

impl UiNativeRetainedDrawList {
    pub(in crate::native::presentation) fn initial(
        initial: &UiMountedPresentationInitial,
        glyph_runs: &[worth_ui_host_contract::UiGlyphRunView],
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
            glyph_runs,
        )
    }

    pub(in crate::native::presentation) fn reconstruction(
        work: &UiMountedPresentationReconstruction,
        glyph_runs: &[worth_ui_host_contract::UiGlyphRunView],
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
            glyph_runs,
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
        source_glyph_runs: &[worth_ui_host_contract::UiGlyphRunView],
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
        let semantic_identities = source_commands
            .iter()
            .filter_map(|command| match command {
                UiMountedPaintCommand::SemanticText { identity, .. } => Some(*identity),
                UiMountedPaintCommand::FilledRect { .. } => None,
            })
            .collect::<std::collections::HashSet<_>>();
        if source_glyph_runs
            .iter()
            .any(|run| !semantic_identities.contains(&run.mechanic()))
        {
            return Err(UiNativeRetainedDrawListDenial::CommandMismatch);
        }
        let glyph_runs = semantic_identities
            .into_iter()
            .map(|identity| {
                (
                    identity,
                    source_glyph_runs
                        .iter()
                        .copied()
                        .filter(|run| run.mechanic() == identity)
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                )
            })
            .collect();
        let mut retained = Self {
            frame,
            surface,
            binding,
            baseline,
            commands,
            order: UiNativeRetainedOrder::initial(source_order.iter().copied())?,
            order_integrity,
            damage,
            glyph_runs,
            last_paint_attribution: None,
        };
        retained.retain_current_paint_attribution();
        Ok(retained)
    }
}
