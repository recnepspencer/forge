use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::{UiMountedDrawableReference, UiMountedNodeProjectionView};
use crate::{
    UiMountedInstanceIdentity, UiMountedPaintCommand, UiMountedPaintCommandIdentity,
    UiMountedPaintOrderIdentity,
};

pub(super) struct PresentationSources {
    pub(super) commands: Vec<UiMountedPaintCommand>,
    pub(super) order: Vec<UiMountedPaintOrderIdentity>,
    pub(super) command_indices: HashMap<UiMountedPaintCommandIdentity, usize>,
    pub(super) commands_by_instance:
        HashMap<UiMountedInstanceIdentity, Arc<[UiMountedPaintCommandIdentity]>>,
    pub(super) order_predecessors:
        HashMap<UiMountedPaintCommandIdentity, Option<UiMountedPaintOrderIdentity>>,
    pub(super) order_positions: HashMap<UiMountedPaintCommandIdentity, usize>,
}

impl PresentationSources {
    pub(super) fn admit(
        nodes: &[UiMountedNodeProjectionView],
        filled_rects: &super::super::UiMountedFilledRectTable,
        semantic_text: &super::super::UiMountedSemanticTextTable,
        commands: Vec<UiMountedPaintCommand>,
        order: Vec<UiMountedPaintOrderIdentity>,
    ) -> Self {
        let command_indices = command_indices(&commands);
        validate_drawable_set(nodes, filled_rects, semantic_text, &command_indices);
        let commands_by_instance =
            commands_by_instance(nodes, &commands, filled_rects, semantic_text);
        let (order_predecessors, order_positions) = order_indexes(&order, &command_indices);
        Self {
            commands,
            order,
            command_indices,
            commands_by_instance,
            order_predecessors,
            order_positions,
        }
    }
}

fn command_indices(
    commands: &[UiMountedPaintCommand],
) -> HashMap<UiMountedPaintCommandIdentity, usize> {
    let indexes = commands
        .iter()
        .enumerate()
        .map(|(index, command)| (command.identity(), index))
        .collect::<HashMap<_, _>>();
    assert_eq!(indexes.len(), commands.len());
    indexes
}

fn validate_drawable_set(
    nodes: &[UiMountedNodeProjectionView],
    filled_rects: &super::super::UiMountedFilledRectTable,
    semantic_text: &super::super::UiMountedSemanticTextTable,
    command_indices: &HashMap<UiMountedPaintCommandIdentity, usize>,
) {
    let expected = nodes
        .iter()
        .flat_map(UiMountedNodeProjectionView::drawables)
        .copied()
        .map(|reference| command_for(reference, filled_rects, semantic_text).identity())
        .collect::<HashSet<_>>();
    assert_eq!(expected.len(), command_indices.len());
    assert!(expected
        .iter()
        .all(|identity| command_indices.contains_key(identity)));
}

fn commands_by_instance(
    nodes: &[UiMountedNodeProjectionView],
    commands: &[UiMountedPaintCommand],
    _filled_rects: &super::super::UiMountedFilledRectTable,
    _semantic_text: &super::super::UiMountedSemanticTextTable,
) -> HashMap<UiMountedInstanceIdentity, Arc<[UiMountedPaintCommandIdentity]>> {
    let mut by_instance = nodes
        .iter()
        .map(|node| (node.mounted_instance(), Vec::new()))
        .collect::<HashMap<_, _>>();
    for command in commands {
        validate_command(command);
        by_instance
            .get_mut(&command.identity().mounted_instance())
            .expect("authored command belongs to an admitted node")
            .push(command.identity());
    }
    by_instance
        .into_iter()
        .map(|(instance, commands)| (instance, commands.into()))
        .collect()
}

fn order_indexes(
    order: &[UiMountedPaintOrderIdentity],
    command_indices: &HashMap<UiMountedPaintCommandIdentity, usize>,
) -> (
    HashMap<UiMountedPaintCommandIdentity, Option<UiMountedPaintOrderIdentity>>,
    HashMap<UiMountedPaintCommandIdentity, usize>,
) {
    assert_eq!(order.len(), command_indices.len());
    let ordered = order
        .iter()
        .map(|identity| identity.command())
        .collect::<HashSet<_>>();
    assert_eq!(ordered.len(), order.len());
    assert!(ordered
        .iter()
        .all(|identity| command_indices.contains_key(identity)));
    let mut previous = None;
    let mut predecessors = HashMap::new();
    let mut positions = HashMap::new();
    for (position, identity) in order.iter().enumerate() {
        predecessors.insert(identity.command(), previous);
        positions.insert(identity.command(), position);
        previous = Some(*identity);
    }
    (predecessors, positions)
}

fn command_for(
    reference: UiMountedDrawableReference,
    filled_rects: &super::super::UiMountedFilledRectTable,
    semantic_text: &super::super::UiMountedSemanticTextTable,
) -> UiMountedPaintCommand {
    match reference {
        UiMountedDrawableReference::FilledRect(reference) => {
            let mechanic = *filled_rects
                .resolve(reference)
                .expect("authored rectangle reference resolves");
            UiMountedPaintCommand::FilledRect {
                identity: UiMountedPaintCommandIdentity::filled_rect(&mechanic),
                mechanic,
            }
        }
        UiMountedDrawableReference::SemanticText(reference) => {
            let mechanic = semantic_text
                .resolve(reference)
                .expect("authored text reference resolves")
                .clone();
            UiMountedPaintCommand::SemanticText {
                identity: UiMountedPaintCommandIdentity::semantic_text(&mechanic),
                mechanic,
            }
        }
    }
}

fn validate_command(
    command: &UiMountedPaintCommand,
) {
    match command {
        UiMountedPaintCommand::FilledRect {
            identity,
            mechanic,
        } => {
            assert_eq!(
                *identity,
                UiMountedPaintCommandIdentity::filled_rect(mechanic)
            );
        }
        UiMountedPaintCommand::SemanticText {
            identity,
            mechanic,
        } => {
            assert_eq!(
                *identity,
                UiMountedPaintCommandIdentity::semantic_text(mechanic)
            );
        }
    }
}
