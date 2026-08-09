use std::collections::HashMap;

use worth_ui_host_contract::{
    UiMountedDrawableReference, UiMountedPaintCommandIdentity, UiMountedPaintOrderIdentity,
    UiMountedProjectionView,
};

use super::CommandSnapshot;

pub(super) fn commands_and_total_order(
    projection: &UiMountedProjectionView,
) -> (
    HashMap<UiMountedPaintCommandIdentity, CommandSnapshot>,
    Box<[UiMountedPaintOrderIdentity]>,
) {
    let mut commands = HashMap::new();
    let mut sources = Vec::new();
    for (node_ordinal, node) in projection.nodes().iter().enumerate() {
        for (local_ordinal, reference) in node.drawables().iter().enumerate() {
            let (identity, command) = command_for(projection, *reference);
            assert!(
                commands.insert(identity, command).is_none(),
                "a mounted paint command has duplicate authored order sources"
            );
            sources.push(AuthoredOrderSource {
                layer: commands[&identity].layer(),
                node_ordinal,
                local_ordinal,
                identity,
            });
        }
    }
    sources.sort_by_key(|source| (source.layer, source.node_ordinal, source.local_ordinal));
    let order = sources
        .into_iter()
        .map(|source| UiMountedPaintOrderIdentity::for_command(source.identity))
        .collect();
    (commands, order)
}

fn command_for(
    projection: &UiMountedProjectionView,
    reference: UiMountedDrawableReference,
) -> (UiMountedPaintCommandIdentity, CommandSnapshot) {
    match reference {
        UiMountedDrawableReference::FilledRect(reference) => {
            let mechanic = projection.filled_rects().rows()[usize::from(reference.index())];
            (
                UiMountedPaintCommandIdentity::filled_rect(&mechanic),
                CommandSnapshot::FilledRect {
                    table_index: reference.index(),
                    mechanic,
                    previous_order: None,
                },
            )
        }
        UiMountedDrawableReference::SemanticText(reference) => {
            let mechanic =
                projection.semantic_text().rows()[usize::from(reference.index())].clone();
            (
                UiMountedPaintCommandIdentity::semantic_text(&mechanic),
                CommandSnapshot::SemanticText {
                    table_index: reference.index(),
                    mechanic,
                    previous_order: None,
                },
            )
        }
    }
}

struct AuthoredOrderSource {
    layer: u32,
    node_ordinal: usize,
    local_ordinal: usize,
    identity: UiMountedPaintCommandIdentity,
}
