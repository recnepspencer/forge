use std::collections::{HashMap, HashSet};

use worth_ui_host_contract::{
    UiMountedCanonicalBox, UiMountedLogicalDamage, UiMountedPaintCommand,
    UiMountedPaintCommandChange, UiMountedPaintCommandIdentity, UiMountedPaintOrderEdit,
    UiMountedPaintOrderIdentity, UiMountedPresentationDelta, UiMountedPresentationInitial,
};

use super::damage_index::{UiNativeDamageIndex, UiNativeDamageIndexDenial};
use super::retained_order::{UiNativeRetainedOrder, UiNativeRetainedOrderDenial};

#[path = "retained_draw_list/delta_transaction.rs"]
mod delta_transaction;
#[path = "retained_draw_list/denial.rs"]
mod denial;
#[path = "retained_draw_list/lifecycle.rs"]
mod lifecycle;
#[path = "retained_draw_list/replay.rs"]
mod replay;

pub(super) use denial::UiNativeRetainedDrawListDenial;

pub(crate) struct UiNativeRetainedDrawList {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    baseline: worth_ui_host_contract::UiHostSurfaceBaselineIdentity,
    commands: HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand>,
    order: UiNativeRetainedOrder<UiMountedPaintOrderIdentity>,
    order_integrity: worth_ui_host_contract::UiMountedPaintOrderIntegrity,
    damage: UiNativeDamageIndex<UiMountedPaintCommandIdentity>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct UiNativeRetainedReplayPlan {
    pub(super) baseline_rgba8: [u8; 4],
    pub(super) clear_regions: Box<[UiMountedLogicalDamage]>,
    pub(super) replay: Box<[UiMountedPaintCommandIdentity]>,
    pub(super) counters: UiNativeRetainedMutationCounters,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct UiNativeRetainedMutationCounters {
    pub(super) draw_mutations: u64,
    pub(super) order_mutations: u64,
    pub(super) damage_regions: u64,
    pub(super) damage_cell_probes: u64,
    pub(super) damage_candidate_probes: u64,
    pub(super) replayed_commands: u64,
    pub(super) retained_command_scans: u64,
}

impl UiNativeRetainedDrawList {
    pub(super) fn initial(
        initial: &UiMountedPresentationInitial,
    ) -> Result<Self, UiNativeRetainedDrawListDenial> {
        if initial.affinity().baseline().transparent_rgba8() != [0, 0, 0, 0]
            || !initial.order_integrity().admits(initial.order())
        {
            return Err(UiNativeRetainedDrawListDenial::BaselineUnavailable);
        }
        let mut commands = HashMap::with_capacity(initial.commands().len());
        let mut damage = UiNativeDamageIndex::new();
        for command in initial.commands() {
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
        if initial.order().len() != commands.len()
            || initial
                .order()
                .iter()
                .any(|identity| !commands.contains_key(&identity.command()))
        {
            return Err(UiNativeRetainedDrawListDenial::OrderMismatch);
        }
        Ok(Self {
            frame: initial.affinity().successor(),
            surface: initial.affinity().surface(),
            binding: initial.affinity().binding(),
            baseline: initial.affinity().baseline(),
            commands,
            order: UiNativeRetainedOrder::initial(initial.order().iter().copied())?,
            order_integrity: initial.order_integrity(),
            damage,
        })
    }

    #[cfg(test)]
    pub(super) fn apply_delta(
        &mut self,
        delta: &UiMountedPresentationDelta,
    ) -> Result<UiNativeRetainedReplayPlan, UiNativeRetainedDrawListDenial> {
        let (plan, _) = self.stage_delta(delta)?;
        Ok(plan)
    }

    pub(super) fn command(
        &self,
        identity: UiMountedPaintCommandIdentity,
    ) -> Option<&UiMountedPaintCommand> {
        self.commands.get(&identity)
    }

    fn validate_affinity(
        &self,
        delta: &UiMountedPresentationDelta,
    ) -> Result<(), UiNativeRetainedDrawListDenial> {
        let affinity = delta.affinity();
        if affinity.predecessor() != Some(self.frame)
            || affinity.surface() != self.surface
            || affinity.binding() != self.binding
            || affinity.baseline() != self.baseline
        {
            return Err(UiNativeRetainedDrawListDenial::AffinityMismatch);
        }
        Ok(())
    }

    fn validate_changes(
        &self,
        changes: &[UiMountedPaintCommandChange],
    ) -> Result<DeltaMembership, UiNativeRetainedDrawListDenial> {
        let mut membership = DeltaMembership::default();
        let mut seen = HashSet::new();
        for change in changes {
            let identity = change_identity(change);
            if !seen.insert(identity) {
                return Err(UiNativeRetainedDrawListDenial::CommandMismatch);
            }
            match change {
                UiMountedPaintCommandChange::Insert(_)
                    if !self.commands.contains_key(&identity) =>
                {
                    membership.inserted.insert(identity);
                }
                UiMountedPaintCommandChange::Replace(_)
                    if self.commands.contains_key(&identity) => {}
                UiMountedPaintCommandChange::Remove(_) if self.commands.contains_key(&identity) => {
                    membership.removed.insert(identity);
                }
                _ => return Err(UiNativeRetainedDrawListDenial::CommandMismatch),
            }
        }
        Ok(membership)
    }

    fn validate_damage(
        &self,
        changes: &[UiMountedPaintCommandChange],
        regions: &[UiMountedLogicalDamage],
    ) -> Result<(), UiNativeRetainedDrawListDenial> {
        for change in changes {
            if let UiMountedPaintCommandChange::Insert(command)
            | UiMountedPaintCommandChange::Replace(command) = change
            {
                if let Some(bounds) = visible_bounds(command) {
                    self.damage.validate_bounds(bounds)?;
                }
            }
        }
        for region in regions {
            self.damage.validate_bounds(region.bounds())?;
        }
        Ok(())
    }

    fn validate_order_edits(
        &self,
        edits: &[UiMountedPaintOrderEdit],
        membership: &DeltaMembership,
    ) -> Result<(), UiNativeRetainedDrawListDenial> {
        let mut removals = HashSet::new();
        let mut placements = HashSet::new();
        for edit in edits {
            let identity = edit.identity();
            if edit.is_removal() {
                if !self.order.contains(identity)
                    || !membership.removed.contains(&identity.command())
                    || !removals.insert(identity.command())
                {
                    return Err(UiNativeRetainedDrawListDenial::OrderMismatch);
                }
            } else {
                if !self.final_command_exists(identity.command(), membership)
                    || !placements.insert(identity.command())
                {
                    return Err(UiNativeRetainedDrawListDenial::OrderMismatch);
                }
                if edit.predecessor().is_some_and(|predecessor| {
                    predecessor == identity
                        || !self.final_command_exists(predecessor.command(), membership)
                }) {
                    return Err(UiNativeRetainedDrawListDenial::OrderMismatch);
                }
            }
        }
        if removals != membership.removed
            || !membership
                .inserted
                .iter()
                .all(|identity| placements.contains(identity))
        {
            return Err(UiNativeRetainedDrawListDenial::OrderMismatch);
        }
        Ok(())
    }

    fn final_command_exists(
        &self,
        identity: UiMountedPaintCommandIdentity,
        membership: &DeltaMembership,
    ) -> bool {
        !membership.removed.contains(&identity)
            && (membership.inserted.contains(&identity) || self.commands.contains_key(&identity))
    }

    fn apply_changes(
        &mut self,
        changes: &[UiMountedPaintCommandChange],
    ) -> Result<(), UiNativeRetainedDrawListDenial> {
        for change in changes {
            match change {
                UiMountedPaintCommandChange::Insert(command) => self.insert(command.clone())?,
                UiMountedPaintCommandChange::Replace(command) => self.replace(command.clone())?,
                UiMountedPaintCommandChange::Remove(identity) => self.remove(*identity)?,
            }
        }
        Ok(())
    }

    fn insert(
        &mut self,
        command: UiMountedPaintCommand,
    ) -> Result<(), UiNativeRetainedDrawListDenial> {
        if let Some(bounds) = visible_bounds(&command) {
            self.damage.insert(command.identity(), bounds)?;
        }
        self.commands.insert(command.identity(), command);
        Ok(())
    }

    fn replace(
        &mut self,
        command: UiMountedPaintCommand,
    ) -> Result<(), UiNativeRetainedDrawListDenial> {
        let identity = command.identity();
        let old = self
            .commands
            .get(&identity)
            .ok_or(UiNativeRetainedDrawListDenial::CommandMismatch)?;
        update_damage(
            &mut self.damage,
            identity,
            visible_bounds(old),
            visible_bounds(&command),
        )?;
        self.commands.insert(identity, command);
        Ok(())
    }

    fn remove(
        &mut self,
        identity: UiMountedPaintCommandIdentity,
    ) -> Result<(), UiNativeRetainedDrawListDenial> {
        let command = self
            .commands
            .remove(&identity)
            .ok_or(UiNativeRetainedDrawListDenial::CommandMismatch)?;
        if visible_bounds(&command).is_some() {
            self.damage.remove(identity)?;
        }
        Ok(())
    }

    fn apply_order_edits(
        &mut self,
        edits: &[UiMountedPaintOrderEdit],
    ) -> Result<(), UiNativeRetainedDrawListDenial> {
        for edit in edits {
            if edit.is_removal() {
                let (predecessor, successor) = self.order.neighbors(edit.identity())?;
                self.order_integrity = self
                    .order_integrity
                    .remove_edge(predecessor, edit.identity(), successor)
                    .ok_or(UiNativeRetainedDrawListDenial::OrderMismatch)?;
                self.order.remove(edit.identity())?;
            } else {
                if self.order.contains(edit.identity()) {
                    let (predecessor, successor) = self.order.neighbors(edit.identity())?;
                    self.order_integrity = self
                        .order_integrity
                        .remove_edge(predecessor, edit.identity(), successor)
                        .ok_or(UiNativeRetainedDrawListDenial::OrderMismatch)?;
                    self.order.remove(edit.identity())?;
                }
                let successor = self.order.successor_after(edit.predecessor())?;
                self.order_integrity = self
                    .order_integrity
                    .insert_edge(edit.predecessor(), edit.identity(), successor)
                    .ok_or(UiNativeRetainedDrawListDenial::OrderMismatch)?;
                self.order
                    .place_after(edit.identity(), edit.predecessor())?;
            }
        }
        Ok(())
    }
}

#[derive(Default)]
struct DeltaMembership {
    inserted: HashSet<UiMountedPaintCommandIdentity>,
    removed: HashSet<UiMountedPaintCommandIdentity>,
}

fn change_identity(change: &UiMountedPaintCommandChange) -> UiMountedPaintCommandIdentity {
    match change {
        UiMountedPaintCommandChange::Insert(command)
        | UiMountedPaintCommandChange::Replace(command) => command.identity(),
        UiMountedPaintCommandChange::Remove(identity) => *identity,
    }
}

fn visible_bounds(command: &UiMountedPaintCommand) -> Option<UiMountedCanonicalBox> {
    let bounds = command.bounds();
    let clip = command.clip_bounds();
    if bounds.coordinate_space() != clip.coordinate_space() {
        return None;
    }
    let x = bounds.x().max(clip.x());
    let y = bounds.y().max(clip.y());
    let right = (bounds.x() + bounds.width()).min(clip.x() + clip.width());
    let bottom = (bounds.y() + bounds.height()).min(clip.y() + clip.height());
    UiMountedCanonicalBox::canonicalize(worth_ui_host_contract::UiMountedCanonicalBoxInput {
        x,
        y,
        width: right - x,
        height: bottom - y,
        coordinate_space: bounds.coordinate_space(),
    })
    .ok()
}

fn update_damage(
    index: &mut UiNativeDamageIndex<UiMountedPaintCommandIdentity>,
    identity: UiMountedPaintCommandIdentity,
    old: Option<UiMountedCanonicalBox>,
    new: Option<UiMountedCanonicalBox>,
) -> Result<(), UiNativeRetainedDrawListDenial> {
    match (old, new) {
        (Some(_), Some(bounds)) => index.replace(identity, bounds)?,
        (Some(_), None) => index.remove(identity)?,
        (None, Some(bounds)) => index.insert(identity, bounds)?,
        (None, None) => {}
    }
    Ok(())
}

#[cfg(test)]
#[path = "retained_draw_list_tests.rs"]
mod tests;
