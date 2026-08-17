use std::collections::HashSet;

use worth_ui_host_contract::{
    UiHostSurfacePresentationDenial, UiMountedFrameConsumptionView, UiMountedPaintCommand,
    UiMountedPaintCommandChange, UiMountedPaintCommandIdentity, UiMountedPaintOrderEdit,
    UiMountedPresentationDelta,
};

use super::super::recorded_frame::UiHeadlessRecordedFrame;
use super::super::{UiHeadlessRecorderCapacity, UiHeadlessRetainedPresentation};

pub(super) fn apply(
    view: &UiMountedFrameConsumptionView<'_>,
    capacity: UiHeadlessRecorderCapacity,
    current: &mut UiHeadlessRetainedPresentation,
    delta: &UiMountedPresentationDelta,
) -> Result<UiHeadlessRecordedFrame, UiHostSurfacePresentationDenial> {
    validate_delta(current, delta, capacity)?;
    let nodes = super::node_delta::UiHeadlessNodeMutation::prepare(
        current,
        delta.changes(),
        delta.nodes(),
        delta.auxiliary().unwrap_or(&current.auxiliary),
        capacity.mechanics_per_frame(),
    )?;
    let undo = DeltaUndo::capture(current, delta, nodes);
    undo.nodes.apply(current);
    for change in delta.changes() {
        if apply_command_change(&mut current.commands, change).is_err() {
            undo.restore(current);
            return Err(malformed());
        }
    }
    if current
        .order
        .apply(delta.order(), delta.order_integrity())
        .is_err()
    {
        undo.restore(current);
        return Err(malformed());
    }
    current.frame = view.frame();
    if let Some(auxiliary) = delta.auxiliary() {
        current.auxiliary = auxiliary.clone();
    }
    UiHeadlessRecordedFrame::delta(view, delta, capacity)
}

struct DeltaUndo {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    commands: Vec<(UiMountedPaintCommandIdentity, Option<UiMountedPaintCommand>)>,
    order: super::super::retained_order::UiHeadlessRetainedOrderSnapshot,
    auxiliary: Option<worth_ui_host_contract::UiMountedPresentationAuxiliaryState>,
    nodes: super::node_delta::UiHeadlessNodeMutation,
}

impl DeltaUndo {
    fn capture(
        current: &UiHeadlessRetainedPresentation,
        delta: &UiMountedPresentationDelta,
        nodes: super::node_delta::UiHeadlessNodeMutation,
    ) -> Self {
        Self {
            frame: current.frame,
            commands: delta
                .changes()
                .iter()
                .map(|change| {
                    let identity = change_identity(change);
                    (identity, current.commands.get(&identity).cloned())
                })
                .collect(),
            order: current
                .order
                .snapshot(delta.order().iter().map(|edit| edit.identity())),
            auxiliary: delta.auxiliary().map(|_| current.auxiliary.clone()),
            nodes,
        }
    }

    fn restore(self, current: &mut UiHeadlessRetainedPresentation) {
        self.nodes.restore(current);
        for (identity, _) in &self.commands {
            current.commands.remove(identity);
        }
        for (identity, command) in self.commands {
            if let Some(command) = command {
                current.commands.insert(identity, command);
            }
        }
        current
            .order
            .restore(self.order)
            .expect("a staged headless order must restore exactly");
        current.frame = self.frame;
        if let Some(auxiliary) = self.auxiliary {
            current.auxiliary = auxiliary;
        }
    }
}

fn validate_delta(
    current: &UiHeadlessRetainedPresentation,
    delta: &UiMountedPresentationDelta,
    capacity: UiHeadlessRecorderCapacity,
) -> Result<(), UiHostSurfacePresentationDenial> {
    if !super::affinity_matches(current, delta.affinity()) {
        return Err(malformed());
    }
    let mut inserted = HashSet::new();
    let mut removed = HashSet::new();
    let mut seen = HashSet::new();
    for change in delta.changes() {
        let identity = change_identity(change);
        if !seen.insert(identity) {
            return Err(malformed());
        }
        match change {
            UiMountedPaintCommandChange::Insert(_) if !current.commands.contains_key(&identity) => {
                inserted.insert(identity);
            }
            UiMountedPaintCommandChange::Replace(_) if current.commands.contains_key(&identity) => {
            }
            UiMountedPaintCommandChange::Remove(_) if current.commands.contains_key(&identity) => {
                removed.insert(identity);
            }
            _ => return Err(malformed()),
        }
    }
    let final_count = current
        .commands
        .len()
        .checked_add(inserted.len())
        .and_then(|count| count.checked_sub(removed.len()))
        .ok_or_else(malformed)?;
    if final_count > capacity.mechanics_per_frame() {
        return Err(UiHostSurfacePresentationDenial::CapacityExceeded);
    }
    validate_delta_order(current, delta.order(), &inserted, &removed)
}

fn validate_delta_order(
    current: &UiHeadlessRetainedPresentation,
    edits: &[UiMountedPaintOrderEdit],
    inserted: &HashSet<UiMountedPaintCommandIdentity>,
    removed: &HashSet<UiMountedPaintCommandIdentity>,
) -> Result<(), UiHostSurfacePresentationDenial> {
    let mut seen = HashSet::new();
    let mut removal_edits = HashSet::new();
    let mut placements = HashSet::new();
    for edit in edits {
        let command = edit.identity().command();
        if !seen.insert(command) {
            return Err(malformed());
        }
        if edit.is_removal() {
            if !removed.contains(&command) || !current.order.contains(edit.identity()) {
                return Err(malformed());
            }
            removal_edits.insert(command);
            continue;
        }
        if !final_command_exists(current, command, inserted, removed)
            || edit.predecessor().is_some_and(|predecessor| {
                predecessor == edit.identity()
                    || !final_command_exists(current, predecessor.command(), inserted, removed)
            })
        {
            return Err(malformed());
        }
        placements.insert(command);
    }
    if removal_edits != *removed
        || !inserted
            .iter()
            .all(|identity| placements.contains(identity))
    {
        return Err(malformed());
    }
    Ok(())
}

fn final_command_exists(
    current: &UiHeadlessRetainedPresentation,
    identity: UiMountedPaintCommandIdentity,
    inserted: &HashSet<UiMountedPaintCommandIdentity>,
    removed: &HashSet<UiMountedPaintCommandIdentity>,
) -> bool {
    !removed.contains(&identity)
        && (inserted.contains(&identity) || current.commands.contains_key(&identity))
}

fn apply_command_change(
    commands: &mut super::super::retained_command_store::UiHeadlessRetainedCommandStore,
    change: &UiMountedPaintCommandChange,
) -> Result<(), UiHostSurfacePresentationDenial> {
    match change {
        UiMountedPaintCommandChange::Insert(command) => {
            if commands
                .insert(command.identity(), command.clone())
                .is_some()
            {
                return Err(malformed());
            }
        }
        UiMountedPaintCommandChange::Replace(command) => {
            if !commands.contains_key(&command.identity()) {
                return Err(malformed());
            }
            commands.insert(command.identity(), command.clone());
        }
        UiMountedPaintCommandChange::Remove(identity) => {
            if commands.remove(identity).is_none() {
                return Err(malformed());
            }
        }
    }
    Ok(())
}

fn change_identity(change: &UiMountedPaintCommandChange) -> UiMountedPaintCommandIdentity {
    match change {
        UiMountedPaintCommandChange::Insert(command)
        | UiMountedPaintCommandChange::Replace(command) => command.identity(),
        UiMountedPaintCommandChange::Remove(identity) => *identity,
    }
}

fn malformed() -> UiHostSurfacePresentationDenial {
    UiHostSurfacePresentationDenial::MalformedProjection
}
