use worth_ui_host_contract::{
    UiMountedPaintCommand, UiMountedPaintCommandIdentity, UiMountedPresentationDelta,
};

use super::mutation::{change_identity, visible_bounds};
use super::{UiNativeRetainedDrawList, UiNativeRetainedDrawListDenial, UiNativeRetainedReplayPlan};
use crate::native::presentation::retained_order::UiNativeRetainedOrderSnapshot;

pub(crate) struct UiNativeRetainedDeltaUndo {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    commands: Vec<(UiMountedPaintCommandIdentity, Option<UiMountedPaintCommand>)>,
    glyph_runs: Vec<(
        UiMountedPaintCommandIdentity,
        Option<Box<[worth_ui_host_contract::UiGlyphRunView]>>,
    )>,
    order: UiNativeRetainedOrderSnapshot<worth_ui_host_contract::UiMountedPaintOrderIdentity>,
    order_integrity: worth_ui_host_contract::UiMountedPaintOrderIntegrity,
    regions: super::super::retained_regions::UiNativeRetainedRegions,
    identity_overlay: super::super::identity_overlay::UiNativeRetainedIdentityOverlay,
    last_paint_attribution: Option<(usize, super::UiNativeRetainedPresentationAttribution)>,
}

impl UiNativeRetainedDrawList {
    pub(crate) fn stage_delta(
        &mut self,
        delta: &UiMountedPresentationDelta,
        glyph_runs: &[worth_ui_host_contract::UiGlyphRunView],
    ) -> Result<
        (UiNativeRetainedReplayPlan, UiNativeRetainedDeltaUndo),
        UiNativeRetainedDrawListDenial,
    > {
        self.order.take_cost();
        self.validate_affinity(delta)?;
        let membership = self.validate_changes(delta.changes())?;
        self.validate_order_edits(delta.order(), &membership)?;
        self.validate_damage(delta.changes(), delta.damage())?;
        let undo = UiNativeRetainedDeltaUndo {
            frame: self.frame,
            commands: delta
                .changes()
                .iter()
                .map(|change| {
                    let identity = change_identity(change);
                    (identity, self.commands.get(&identity).cloned())
                })
                .collect(),
            glyph_runs: delta
                .changes()
                .iter()
                .map(|change| {
                    let identity = change_identity(change);
                    (identity, self.glyph_runs.get(&identity).cloned())
                })
                .collect(),
            order: self
                .order
                .snapshot(delta.order().iter().map(|edit| edit.identity())),
            order_integrity: self.order_integrity,
            regions: self.regions.clone(),
            identity_overlay: self.identity_overlay,
            last_paint_attribution: self.last_paint_attribution,
        };
        if let Err(error) = self
            .apply_changes(delta.changes(), glyph_runs)
            .and_then(|_| self.apply_order_edits(delta.order()))
        {
            self.rollback_delta(undo)
                .expect("a prevalidated retained delta must roll back exactly");
            return Err(error);
        }
        if self.order_integrity != delta.order_integrity() {
            self.rollback_delta(undo)
                .expect("an invalid order receipt must preserve retained state");
            return Err(UiNativeRetainedDrawListDenial::OrderMismatch);
        }
        self.regions.apply_paint_changes(delta.changes());
        if self.regions.apply_node_changes(delta).is_err() {
            self.rollback_delta(undo)
                .expect("an invalid realized-region delta must roll back exactly");
            return Err(UiNativeRetainedDrawListDenial::CommandMismatch);
        }
        let predecessor_identity_overlay = self.identity_overlay;
        let identity_overlay_effect = match self.identity_overlay.apply_delta(delta) {
            Ok(changed) => changed,
            Err(_) => {
                self.rollback_delta(undo)
                    .expect("an invalid identity-overlay delta must roll back exactly");
                return Err(UiNativeRetainedDrawListDenial::CommandMismatch);
            }
        };
        let region_result = delta.auxiliary().map_or(Ok(()), |auxiliary| {
            let projection = auxiliary
                .reconstruct(self.commands.as_map())
                .map_err(|_| UiNativeRetainedDrawListDenial::CommandMismatch)?;
            self.regions
                .replace_hit_tests(&projection)
                .map_err(|_| UiNativeRetainedDrawListDenial::CommandMismatch)?;
            Ok(())
        });
        if let Err(error) = region_result {
            self.rollback_delta(undo)
                .expect("an invalid realized-region delta must roll back exactly");
            return Err(error);
        }
        self.regions
            .rebind_receipt_affinity(delta.affinity().receipt_affinity());
        self.frame = delta.affinity().successor();
        self.retain_current_paint_attribution();
        let mut replay_damage = delta.damage().to_vec();
        let overlay_damage =
            match super::super::identity_overlay::UiNativeRetainedIdentityOverlay::transition_damage(
                predecessor_identity_overlay,
                self.identity_overlay,
            ) {
                Ok(damage) => damage,
                Err(_) => {
                    self.rollback_delta(undo)
                        .expect("invalid overlay damage must roll back exactly");
                    return Err(UiNativeRetainedDrawListDenial::CommandMismatch);
                }
            };
        replay_damage.extend(overlay_damage);
        match self.replay_plan(&replay_damage, delta.changes().len(), delta.order().len()) {
            Ok(mut plan) => {
                plan.identity_overlay_effect = identity_overlay_effect;
                Ok((plan, undo))
            }
            Err(error) => {
                self.rollback_delta(undo)
                    .expect("a prevalidated retained delta must roll back exactly");
                Err(error)
            }
        }
    }

    pub(crate) fn rollback_delta(
        &mut self,
        undo: UiNativeRetainedDeltaUndo,
    ) -> Result<(), UiNativeRetainedDrawListDenial> {
        for (identity, _) in &undo.commands {
            if let Some(current) = self.commands.remove(identity) {
                if visible_bounds(&current).is_some() {
                    self.damage.remove(*identity)?;
                }
            }
        }
        for (identity, _) in &undo.glyph_runs {
            self.glyph_runs.remove(identity);
        }
        for (identity, runs) in undo.glyph_runs {
            if let Some(runs) = runs {
                self.glyph_runs.insert(identity, runs);
            }
        }
        for (identity, command) in undo.commands {
            let Some(command) = command else {
                continue;
            };
            if let Some(bounds) = visible_bounds(&command) {
                self.damage.insert(identity, bounds)?;
            }
            self.commands.insert(identity, command);
        }
        self.order.restore(undo.order)?;
        self.order_integrity = undo.order_integrity;
        self.regions = undo.regions;
        self.identity_overlay = undo.identity_overlay;
        self.last_paint_attribution = undo.last_paint_attribution;
        self.frame = undo.frame;
        Ok(())
    }
}
