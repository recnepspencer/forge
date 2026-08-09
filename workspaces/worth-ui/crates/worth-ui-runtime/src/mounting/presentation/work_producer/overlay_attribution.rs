use std::collections::HashSet;

use worth_ui_host_contract::{UiMountedEffectFamily, UiMountedPaintCommandChange};

use super::UiMountedPresentationState;

pub(super) fn refresh_commands(
    predecessor: &UiMountedPresentationState,
    successor: &UiMountedPresentationState,
    auxiliary_changed: bool,
    changes: &mut Vec<UiMountedPaintCommandChange>,
) {
    let overlay_changed = auxiliary_changed
        && (predecessor
            .effects
            .contains(&UiMountedEffectFamily::IdentityOverlay)
            || successor
                .effects
                .contains(&UiMountedEffectFamily::IdentityOverlay));
    if !overlay_changed {
        return;
    }
    let carried = changes
        .iter()
        .map(|change| match change {
            UiMountedPaintCommandChange::Insert(command)
            | UiMountedPaintCommandChange::Replace(command) => command.identity(),
            UiMountedPaintCommandChange::Remove(identity) => *identity,
        })
        .collect::<HashSet<_>>();
    changes.extend(
        successor
            .order
            .iter()
            .map(|order| order.command())
            .filter(|identity| !carried.contains(identity))
            .map(|identity| {
                UiMountedPaintCommandChange::Replace(
                    successor.commands[&identity].to_command(identity),
                )
            }),
    );
}
