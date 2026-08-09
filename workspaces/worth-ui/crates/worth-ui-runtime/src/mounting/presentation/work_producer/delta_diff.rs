use worth_ui_host_contract::{
    UiMountedLogicalDamage, UiMountedPaintCommandChange, UiMountedPaintCommandIdentity,
    UiMountedPaintOrderEdit,
};

use super::{CommandSnapshot, UiMountedPresentationState};

pub(super) fn command_changes(
    predecessor: &UiMountedPresentationState,
    successor: &UiMountedPresentationState,
) -> (
    Vec<UiMountedPaintCommandChange>,
    Vec<UiMountedLogicalDamage>,
) {
    let mut changes = Vec::new();
    let mut damage = Vec::new();
    for order in predecessor.order.iter() {
        let identity = order.command();
        if !successor.commands.contains_key(&identity) {
            push_change(
                identity,
                predecessor.commands.get(&identity),
                None,
                &mut changes,
                &mut damage,
            );
        }
    }
    for order in successor.order.iter() {
        let identity = order.command();
        let before = predecessor.commands.get(&identity);
        let after = successor.commands.get(&identity);
        if before.is_none() || before != after {
            push_change(identity, before, after, &mut changes, &mut damage);
        }
    }
    (changes, damage)
}

fn push_change(
    identity: UiMountedPaintCommandIdentity,
    before: Option<&CommandSnapshot>,
    after: Option<&CommandSnapshot>,
    changes: &mut Vec<UiMountedPaintCommandChange>,
    damage: &mut Vec<UiMountedLogicalDamage>,
) {
    match (before, after) {
        (None, Some(command)) => {
            changes.push(UiMountedPaintCommandChange::Insert(
                command.to_command(identity),
            ));
            append_damage(command, damage);
        }
        (Some(command), None) => {
            changes.push(UiMountedPaintCommandChange::Remove(identity));
            append_damage(command, damage);
        }
        (Some(old), Some(new)) if !old.same_presentation_meaning(new) => {
            changes.push(UiMountedPaintCommandChange::Replace(
                new.to_command(identity),
            ));
            append_damage(old, damage);
            append_damage(new, damage);
        }
        (Some(_), Some(_)) => {}
        (None, None) => unreachable!("identity comes from either command map"),
    }
}

fn append_damage(command: &CommandSnapshot, damage: &mut Vec<UiMountedLogicalDamage>) {
    damage.extend(
        command
            .visible_bounds()
            .map(UiMountedLogicalDamage::from_runtime_mounting),
    );
}

pub(super) fn order_edits(
    predecessor: &UiMountedPresentationState,
    successor: &UiMountedPresentationState,
) -> Vec<UiMountedPaintOrderEdit> {
    let mut edits = predecessor
        .order
        .iter()
        .filter(|identity| !successor.commands.contains_key(&identity.command()))
        .copied()
        .map(UiMountedPaintOrderEdit::remove)
        .collect::<Vec<_>>();
    edits.extend(
        successor
            .order
            .iter()
            .enumerate()
            .filter_map(|(index, identity)| {
                let expected = index.checked_sub(1).map(|i| successor.order[i]);
                let retained = predecessor
                    .commands
                    .get(&identity.command())
                    .map(CommandSnapshot::previous_order);
                (retained != Some(expected))
                    .then(|| UiMountedPaintOrderEdit::place_after(*identity, expected))
            }),
    );
    edits
}
