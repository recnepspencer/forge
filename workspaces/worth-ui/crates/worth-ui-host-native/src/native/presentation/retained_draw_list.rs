use std::collections::HashMap;

use worth_ui_host_contract::{
    UiMountedLogicalDamage, UiMountedPaintCommand, UiMountedPaintCommandIdentity,
    UiMountedPaintOrderIdentity,
};

use super::damage_index::{UiNativeDamageIndex, UiNativeDamageIndexDenial};
use super::retained_order::{UiNativeRetainedOrder, UiNativeRetainedOrderDenial};

#[path = "retained_draw_list/command_store.rs"]
mod command_store;
#[path = "retained_draw_list/complete.rs"]
mod complete;
#[path = "retained_draw_list/delta_transaction.rs"]
mod delta_transaction;
#[path = "retained_draw_list/denial.rs"]
mod denial;
#[path = "retained_draw_list/lifecycle.rs"]
mod lifecycle;
#[path = "retained_draw_list/mutation.rs"]
mod mutation;
#[path = "retained_draw_list/replay.rs"]
mod replay;

pub(super) use delta_transaction::UiNativeRetainedDeltaUndo;
pub(super) use denial::UiNativeRetainedDrawListDenial;

pub(crate) struct UiNativeRetainedDrawList {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    baseline: worth_ui_host_contract::UiHostSurfaceBaselineIdentity,
    commands: command_store::UiNativeRetainedCommandStore,
    order: UiNativeRetainedOrder<UiMountedPaintOrderIdentity>,
    order_integrity: worth_ui_host_contract::UiMountedPaintOrderIntegrity,
    damage: UiNativeDamageIndex<UiMountedPaintCommandIdentity>,
    glyph_runs:
        HashMap<UiMountedPaintCommandIdentity, Box<[worth_ui_host_contract::UiGlyphRunView]>>,
    regions: super::retained_regions::UiNativeRetainedRegions,
    identity_overlay: super::identity_overlay::UiNativeRetainedIdentityOverlay,
    last_paint_attribution: Option<(usize, UiNativeRetainedPresentationAttribution)>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct UiNativeRetainedPresentationAttribution {
    pub(super) color: worth_ui_host_contract::UiMountedRgba8,
    pub(super) bounds: worth_ui_host_contract::UiMountedCanonicalBox,
    pub(super) mounted_instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    pub(super) node_receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
}

#[derive(Debug, PartialEq)]
pub(crate) struct UiNativeRetainedReplayPlan {
    pub(super) baseline_rgba8: [u8; 4],
    pub(super) regions: Box<[UiNativeRetainedReplayRegion]>,
    pub(super) counters: UiNativeRetainedMutationCounters,
    pub(super) identity_overlay_effect: bool,
}

#[derive(Debug, PartialEq)]
pub(super) struct UiNativeRetainedReplayRegion {
    pub(super) damage: UiMountedLogicalDamage,
    pub(super) replay: Box<[UiMountedPaintCommandIdentity]>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct UiNativeRetainedMutationCounters {
    pub(super) draw_mutations: u64,
    pub(super) order_mutations: u64,
    pub(super) order_index_lookups: u64,
    pub(super) order_index_node_touches: u64,
    pub(super) order_index_rotations: u64,
    pub(super) order_index_high_water: u64,
    pub(super) damage_rows_carried: u64,
    pub(super) damage_regions: u64,
    pub(super) damage_index_branch_aabb_probes: u64,
    pub(super) damage_index_leaf_command_bounds_probes: u64,
    pub(super) damage_index_stored_records: u64,
    pub(super) damage_index_high_water: u64,
    pub(super) damage_region_command_checks: u64,
    pub(super) replayed_commands: u64,
    pub(super) retained_command_scans: u64,
}

impl UiNativeRetainedDrawList {
    pub(crate) const fn frame(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.frame
    }

    #[cfg(test)]
    pub(super) fn apply_delta(
        &mut self,
        delta: &worth_ui_host_contract::UiMountedPresentationDelta,
    ) -> Result<UiNativeRetainedReplayPlan, UiNativeRetainedDrawListDenial> {
        let (plan, _) = self.stage_delta(delta, &[])?;
        Ok(plan)
    }

    pub(super) fn command(
        &self,
        identity: UiMountedPaintCommandIdentity,
    ) -> Option<&UiMountedPaintCommand> {
        self.commands.get(&identity)
    }

    pub(super) fn glyph_runs(
        &self,
        identity: UiMountedPaintCommandIdentity,
    ) -> &[worth_ui_host_contract::UiGlyphRunView] {
        self.glyph_runs
            .get(&identity)
            .map(Box::as_ref)
            .unwrap_or_default()
    }

    pub(super) fn all_glyph_runs(&self) -> Vec<worth_ui_host_contract::UiGlyphRunView> {
        self.glyph_runs
            .values()
            .flat_map(|runs| runs.iter().copied())
            .collect()
    }

    pub(crate) fn realized_regions(
        &self,
    ) -> Option<Vec<worth_ui_host_contract::UiHostRealizedRegion>> {
        self.regions.realized(self.order.ordered())
    }

    pub(super) fn identity_overlay_operations(
        &self,
        basis: super::raster::UiNativeRasterBasis,
    ) -> Result<
        Vec<super::UiNativeRasterOperation>,
        worth_ui_host_contract::UiHostSurfacePresentationDenial,
    > {
        self.identity_overlay.raster_operations(basis)
    }

    pub(crate) const fn identity_overlay_active(&self) -> bool {
        self.identity_overlay.is_active()
    }

    pub(super) fn top_paint_attribution(
        &self,
    ) -> Option<(usize, UiNativeRetainedPresentationAttribution)> {
        self.current_top_paint_attribution()
            .or(self.last_paint_attribution)
            .map(|(ordinal, mut attribution)| {
                attribution.node_receipt = self.regions.current_receipt(attribution.node_receipt);
                (ordinal, attribution)
            })
    }

    fn current_top_paint_attribution(
        &self,
    ) -> Option<(usize, UiNativeRetainedPresentationAttribution)> {
        let (ordinal, identity) = self.order.ordered().enumerate().last()?;
        let attribution = match self.commands.get(&identity.command())? {
            UiMountedPaintCommand::FilledRect { mechanic, .. } => {
                UiNativeRetainedPresentationAttribution {
                    color: mechanic.color(),
                    bounds: mechanic.bounds(),
                    mounted_instance: mechanic.mounted_instance(),
                    node_receipt: mechanic.node_receipt(),
                }
            }
            UiMountedPaintCommand::SemanticText { mechanic, .. } => {
                UiNativeRetainedPresentationAttribution {
                    color: mechanic.foregrounds().first()?.color(),
                    bounds: mechanic.bounds(),
                    mounted_instance: mechanic.mounted_instance(),
                    node_receipt: mechanic.node_receipt(),
                }
            }
        };
        Some((ordinal, attribution))
    }

    fn retain_current_paint_attribution(&mut self) {
        if let Some(attribution) = self.current_top_paint_attribution() {
            self.last_paint_attribution = Some(attribution);
        }
    }
}

#[cfg(test)]
#[path = "retained_draw_list_tests.rs"]
pub(super) mod tests;
