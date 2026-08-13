use std::collections::HashSet;

use worth_ui_host_contract::{
    UiHostSurfacePresentationDenial, UiMountedInstanceIdentity, UiMountedPaintCommand,
    UiMountedPaintCommandChange, UiMountedPresentationAuxiliaryState,
    UiMountedPresentationNodeChange, UiMountedPresentationNodePaint,
};

use super::super::UiHeadlessRetainedPresentation;

pub(super) struct UiHeadlessNodeMutation {
    before: Vec<(UiMountedInstanceIdentity, u64)>,
    after: Vec<(UiMountedInstanceIdentity, Option<u64>)>,
}

impl UiHeadlessNodeMutation {
    pub(super) fn prepare(
        current: &UiHeadlessRetainedPresentation,
        command_changes: &[UiMountedPaintCommandChange],
        changes: &[UiMountedPresentationNodeChange],
        auxiliary: &UiMountedPresentationAuxiliaryState,
        capacity: usize,
    ) -> Result<Self, UiHostSurfacePresentationDenial> {
        let mut seen = HashSet::with_capacity(changes.len());
        let mut target_positions = HashSet::with_capacity(changes.len());
        let affected = changes
            .iter()
            .map(|change| change.mounted_instance())
            .collect::<HashSet<_>>();
        let mut before = Vec::with_capacity(changes.len());
        let mut after = Vec::with_capacity(changes.len());
        let mut inserted = 0usize;
        let mut removed = 0usize;
        for change in changes {
            let instance = change.mounted_instance();
            if !seen.insert(instance) {
                return Err(malformed());
            }
            let existing = current.node_positions.get(&instance).copied();
            if let Some(position) = existing {
                before.push((instance, position));
            }
            match change {
                UiMountedPresentationNodeChange::Remove(_) => {
                    existing.ok_or_else(malformed)?;
                    removed += 1;
                    after.push((instance, None));
                }
                UiMountedPresentationNodeChange::Upsert(state) => {
                    validate_paint(current, command_changes, auxiliary, state.paint())?;
                    inserted += usize::from(existing.is_none());
                    let position = state.authored_position();
                    if !target_positions.insert(position)
                        || current
                            .node_by_position
                            .get(&position)
                            .is_some_and(|owner| !affected.contains(owner))
                    {
                        return Err(malformed());
                    }
                    after.push((instance, Some(position)));
                }
            }
        }
        let count = current
            .node_positions
            .len()
            .checked_add(inserted)
            .and_then(|count| count.checked_sub(removed))
            .ok_or_else(malformed)?;
        if count > capacity {
            return Err(UiHostSurfacePresentationDenial::CapacityExceeded);
        }
        Ok(Self { before, after })
    }

    pub(super) fn apply(&self, current: &mut UiHeadlessRetainedPresentation) {
        for (instance, position) in &self.before {
            current.node_positions.remove(instance);
            current.node_by_position.remove(position);
        }
        for (instance, position) in &self.after {
            if let Some(position) = position {
                current.node_positions.insert(*instance, *position);
                current.node_by_position.insert(*position, *instance);
            }
        }
    }

    pub(super) fn restore(self, current: &mut UiHeadlessRetainedPresentation) {
        for (instance, position) in &self.after {
            current.node_positions.remove(instance);
            if let Some(position) = position {
                current.node_by_position.remove(position);
            }
        }
        for (instance, position) in self.before {
            current.node_positions.insert(instance, position);
            current.node_by_position.insert(position, instance);
        }
    }
}

fn validate_paint(
    current: &UiHeadlessRetainedPresentation,
    changes: &[UiMountedPaintCommandChange],
    auxiliary: &UiMountedPresentationAuxiliaryState,
    paint: UiMountedPresentationNodePaint,
) -> Result<(), UiHostSurfacePresentationDenial> {
    match paint {
        UiMountedPresentationNodePaint::Command(identity) => {
            let changed = changes.iter().find_map(|change| match change {
                UiMountedPaintCommandChange::Insert(command)
                | UiMountedPaintCommandChange::Replace(command)
                    if command.identity() == identity =>
                {
                    Some(Some(command))
                }
                UiMountedPaintCommandChange::Remove(removed) if *removed == identity => Some(None),
                _ => None,
            });
            let command = match changed {
                Some(Some(command)) => Some(command),
                Some(None) => None,
                None => current.commands.get(&identity),
            };
            if !matches!(command, Some(UiMountedPaintCommand::FilledRect { .. })) {
                return Err(malformed());
            }
        }
        UiMountedPresentationNodePaint::CountOnlyBatch(index)
            if usize::from(index) >= auxiliary.paint_batch_count() =>
        {
            return Err(malformed());
        }
        UiMountedPresentationNodePaint::CountOnlyBatch(_)
        | UiMountedPresentationNodePaint::Omitted(_) => {}
    }
    Ok(())
}

fn malformed() -> UiHostSurfacePresentationDenial {
    UiHostSurfacePresentationDenial::MalformedProjection
}
