use std::collections::HashSet;

use worth_ui_host_contract::{UiMountedEffectFamily, UiMountedPaintCommandChange};

use super::UiMountedPresentationState;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct OverlayRefreshCost {
    pub(super) commands_considered: usize,
    pub(super) command_lookups: usize,
    pub(super) order_items_scanned: usize,
}

pub(super) fn refresh_commands(
    predecessor: &UiMountedPresentationState,
    successor: &UiMountedPresentationState,
    auxiliary_changed: bool,
    changes: &mut Vec<UiMountedPaintCommandChange>,
) -> OverlayRefreshCost {
    let overlay_changed = auxiliary_changed
        && (predecessor
            .effects
            .contains(&UiMountedEffectFamily::IdentityOverlay)
            || successor
                .effects
                .contains(&UiMountedEffectFamily::IdentityOverlay));
    if !overlay_changed {
        return OverlayRefreshCost::default();
    }
    let carried = changes
        .iter()
        .map(|change| match change {
            UiMountedPaintCommandChange::Insert(command)
            | UiMountedPaintCommandChange::Replace {
                successor: command, ..
            } => command.identity(),
            UiMountedPaintCommandChange::Remove(identity) => *identity,
        })
        .collect::<HashSet<_>>();
    let mut command_lookups = 0;
    let order = successor.order();
    changes.extend(
        order
            .iter()
            .map(|order| order.command())
            .filter_map(|identity| {
                if carried.contains(&identity) {
                    return None;
                }
                command_lookups += 1;
                Some(UiMountedPaintCommandChange::replacement(
                    identity,
                    successor.command(identity).clone(),
                ))
            }),
    );
    OverlayRefreshCost {
        commands_considered: order.len(),
        command_lookups,
        order_items_scanned: order.len(),
    }
}
