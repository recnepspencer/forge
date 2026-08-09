use std::collections::{HashMap, HashSet};

use worth_ui_host_contract::{
    UiHostPresentationCostInput, UiHostPresentationCostReport, UiHostSurfacePresentationDenial,
    UiMountedFrameConsumptionView, UiMountedPaintCommand, UiMountedPaintCommandChange,
    UiMountedPaintCommandIdentity, UiMountedPaintOrderEdit, UiMountedPaintOrderIdentity,
    UiMountedPresentationWorkView,
};

use super::{UiHeadlessRecorderCapacity, UiHeadlessRetainedPresentation};
use crate::headless_translation::translate_headless_frame;

pub(super) fn prepare_candidate(
    view: &UiMountedFrameConsumptionView<'_>,
    capacity: UiHeadlessRecorderCapacity,
    current: Option<&UiHeadlessRetainedPresentation>,
) -> Result<UiHeadlessRetainedPresentation, UiHostSurfacePresentationDenial> {
    match view.presentation_work() {
        UiMountedPresentationWorkView::Initial(initial) => {
            if current.is_some() {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
            let commands = validated_initial_commands(initial)?;
            Ok(UiHeadlessRetainedPresentation {
                frame: view.frame(),
                commands,
                order: initial.order().into(),
                auxiliary: initial.auxiliary().clone(),
                transcript: translate_headless_frame(
                    view,
                    initial.projection(),
                    capacity,
                    initial.order(),
                    initial.damage(),
                )?,
            })
        }
        UiMountedPresentationWorkView::Delta(delta) => {
            let current = current.ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?;
            if delta.affinity().predecessor() != Some(current.frame) {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
            let mut commands = current.commands.clone();
            for change in delta.changes() {
                apply_command_change(&mut commands, change)?;
            }
            let mut order = current.order.to_vec();
            apply_order_edits(&mut order, delta.order())?;
            validate_order(&commands, &order, delta.order_integrity())?;
            let command_transcript = current.transcript.successor_delta(
                view,
                delta.changes(),
                &order,
                delta.damage(),
            )?;
            refresh_table_indices(&mut commands, &command_transcript)?;
            let auxiliary = delta
                .auxiliary()
                .cloned()
                .unwrap_or_else(|| current.auxiliary.clone());
            let transcript = match delta.auxiliary() {
                None => command_transcript,
                Some(_) => {
                    let projection = auxiliary
                        .reconstruct(&commands)
                        .map_err(|_| UiHostSurfacePresentationDenial::MalformedProjection)?;
                    crate::headless_translation::translate_auxiliary_delta(
                        view,
                        &projection,
                        &command_transcript,
                        capacity,
                        &order,
                        delta.damage(),
                    )?
                }
            };
            Ok(UiHeadlessRetainedPresentation {
                frame: view.frame(),
                transcript,
                commands,
                order: order.into_boxed_slice(),
                auxiliary,
            })
        }
        UiMountedPresentationWorkView::Unchanged(unchanged) => {
            let current = current.ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?;
            if unchanged.affinity().predecessor() != Some(current.frame) {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
            Ok(UiHeadlessRetainedPresentation {
                frame: view.frame(),
                commands: current.commands.clone(),
                order: current.order.clone(),
                auxiliary: current.auxiliary.clone(),
                transcript: current.transcript.successor_unchanged(view),
            })
        }
    }
}

fn validated_initial_commands(
    initial: &worth_ui_host_contract::UiMountedPresentationInitial,
) -> Result<
    HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand>,
    UiHostSurfacePresentationDenial,
> {
    let mut commands = HashMap::with_capacity(initial.commands().len());
    for command in initial.commands() {
        if commands
            .insert(command.identity(), command.clone())
            .is_some()
        {
            return Err(UiHostSurfacePresentationDenial::MalformedProjection);
        }
    }
    validate_initial_projection(&commands, initial.projection())?;
    validate_order(&commands, initial.order(), initial.order_integrity())?;
    Ok(commands)
}

fn validate_initial_projection(
    commands: &HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand>,
    projection: &worth_ui_host_contract::UiMountedProjectionView,
) -> Result<(), UiHostSurfacePresentationDenial> {
    let expected_count =
        projection.filled_rects().rows().len() + projection.semantic_text().rows().len();
    if commands.len() != expected_count {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    let aligned = commands.values().all(|command| match command {
        UiMountedPaintCommand::FilledRect {
            identity,
            table_index,
            mechanic,
        } => {
            *identity == UiMountedPaintCommandIdentity::filled_rect(mechanic)
                && projection
                    .filled_rects()
                    .rows()
                    .get(usize::from(*table_index))
                    == Some(mechanic)
        }
        UiMountedPaintCommand::SemanticText {
            identity,
            table_index,
            mechanic,
        } => {
            *identity == UiMountedPaintCommandIdentity::semantic_text(mechanic)
                && projection
                    .semantic_text()
                    .rows()
                    .get(usize::from(*table_index))
                    == Some(mechanic)
        }
    });
    if !aligned {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(())
}

fn refresh_table_indices(
    commands: &mut HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand>,
    transcript: &crate::UiHeadlessMountedFrameTranscript,
) -> Result<(), UiHostSurfacePresentationDenial> {
    for (index, row) in transcript.filled_rects().iter().enumerate() {
        refresh_table_index(commands, row.command_identity(), index)?;
    }
    for (index, row) in transcript.semantic_text().iter().enumerate() {
        refresh_table_index(commands, row.command_identity(), index)?;
    }
    let represented = commands.values().all(|command| match command {
        UiMountedPaintCommand::FilledRect {
            identity,
            table_index,
            ..
        } => transcript
            .filled_rects()
            .get(usize::from(*table_index))
            .is_some_and(|row| row.command_identity() == *identity),
        UiMountedPaintCommand::SemanticText {
            identity,
            table_index,
            ..
        } => transcript
            .semantic_text()
            .get(usize::from(*table_index))
            .is_some_and(|row| row.command_identity() == *identity),
    });
    if !represented {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(())
}

fn refresh_table_index(
    commands: &mut HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand>,
    identity: UiMountedPaintCommandIdentity,
    index: usize,
) -> Result<(), UiHostSurfacePresentationDenial> {
    let index =
        u16::try_from(index).map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?;
    match commands
        .get_mut(&identity)
        .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?
    {
        UiMountedPaintCommand::FilledRect { table_index, .. }
        | UiMountedPaintCommand::SemanticText { table_index, .. } => *table_index = index,
    }
    Ok(())
}

fn apply_order_edits(
    order: &mut Vec<UiMountedPaintOrderIdentity>,
    edits: &[UiMountedPaintOrderEdit],
) -> Result<(), UiHostSurfacePresentationDenial> {
    for edit in edits {
        let identity = edit.identity();
        if edit.is_removal() {
            let index = order
                .iter()
                .position(|current| *current == identity)
                .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?;
            order.remove(index);
            continue;
        }
        if let Some(index) = order.iter().position(|current| *current == identity) {
            order.remove(index);
        }
        let index = match edit.predecessor() {
            None => 0,
            Some(predecessor) => order
                .iter()
                .position(|current| *current == predecessor)
                .and_then(|index| index.checked_add(1))
                .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?,
        };
        order.insert(index, identity);
    }
    Ok(())
}

fn validate_order(
    commands: &HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand>,
    order: &[UiMountedPaintOrderIdentity],
    integrity: worth_ui_host_contract::UiMountedPaintOrderIntegrity,
) -> Result<(), UiHostSurfacePresentationDenial> {
    let ordered = order
        .iter()
        .map(|identity| identity.command())
        .collect::<HashSet<_>>();
    let commanded = commands.keys().copied().collect::<HashSet<_>>();
    if ordered.len() != order.len() || ordered != commanded || !integrity.admits(order) {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(())
}

fn apply_command_change(
    commands: &mut HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand>,
    change: &UiMountedPaintCommandChange,
) -> Result<(), UiHostSurfacePresentationDenial> {
    match change {
        UiMountedPaintCommandChange::Insert(command) => {
            if commands
                .insert(command.identity(), command.clone())
                .is_some()
            {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
        }
        UiMountedPaintCommandChange::Replace(command) => {
            if !commands.contains_key(&command.identity()) {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
            commands.insert(command.identity(), command.clone());
        }
        UiMountedPaintCommandChange::Remove(identity) => {
            if commands.remove(identity).is_none() {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
        }
    }
    Ok(())
}

pub(super) fn work_cost(
    work: UiMountedPresentationWorkView<'_>,
) -> Result<UiHostPresentationCostReport, UiHostSurfacePresentationDenial> {
    let (presented_surfaces, rows, bytes, delta_rows, draw_mutations, order_mutations, damage) =
        match work {
            UiMountedPresentationWorkView::Initial(initial) => (
                1,
                initial.commands().len(),
                std::mem::size_of_val(initial.commands()),
                0,
                0,
                0,
                initial.damage().len(),
            ),
            UiMountedPresentationWorkView::Delta(delta) => (
                1,
                delta.changes().len(),
                std::mem::size_of_val(delta.changes()),
                delta.changes().len() + delta.order().len() + delta.damage().len(),
                delta.changes().len(),
                delta.order().len(),
                delta.damage().len(),
            ),
            UiMountedPresentationWorkView::Unchanged(_) => (0, 0, 0, 0, 0, 0, 0),
        };
    Ok(UiHostPresentationCostReport::from_adapter(
        UiHostPresentationCostInput {
            presented_surfaces,
            translated_rows: u64::try_from(rows)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            translated_bytes: u64::try_from(bytes)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            native_resource_cache_hits: 0,
            native_resource_cache_misses: 0,
            asynchronous_handoffs: 0,
            delta_rows_carried: u64::try_from(delta_rows)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            draw_list_mutations: u64::try_from(draw_mutations)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            order_mutations: u64::try_from(order_mutations)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            logical_damage_regions: u64::try_from(damage)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            ..Default::default()
        },
    ))
}

#[cfg(test)]
#[path = "presentation_tests.rs"]
mod tests;
