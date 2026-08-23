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
        self.frame = delta.affinity().successor();
        self.retain_current_paint_attribution();
        match self.replay_plan(delta.damage(), delta.changes().len(), delta.order().len()) {
            Ok(plan) => Ok((plan, undo)),
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
        self.last_paint_attribution = undo.last_paint_attribution;
        self.frame = undo.frame;
        Ok(())
    }
}
