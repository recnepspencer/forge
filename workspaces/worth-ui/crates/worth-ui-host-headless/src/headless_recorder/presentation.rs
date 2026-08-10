use std::collections::{HashMap, HashSet};

use worth_ui_host_contract::{
    UiHostPresentationCostInput, UiHostPresentationCostReport, UiHostSurfacePresentationDenial,
    UiMountedFrameConsumptionView, UiMountedPaintCommand, UiMountedPaintCommandIdentity,
    UiMountedPaintOrderIdentity, UiMountedPresentationWorkView,
};

use super::recorded_frame::UiHeadlessRecordedFrame;
use super::retained_order::UiHeadlessRetainedOrder;
use super::{UiHeadlessRecorderCapacity, UiHeadlessRetainedPresentation};
use crate::headless_translation::translate_headless_frame;

mod delta;

pub(super) fn apply_work(
    view: &UiMountedFrameConsumptionView<'_>,
    capacity: UiHeadlessRecorderCapacity,
    current: &mut Option<UiHeadlessRetainedPresentation>,
) -> Result<Option<UiHeadlessRecordedFrame>, UiHostSurfacePresentationDenial> {
    match view.presentation_work() {
        UiMountedPresentationWorkView::Initial(initial) => {
            if current.is_some() {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
            let commands = validated_initial_commands(initial)?;
            let transcript = translate_headless_frame(
                view,
                initial.projection(),
                capacity,
                initial.order(),
                initial.damage(),
            )?;
            let recorded = UiHeadlessRecordedFrame::complete(transcript);
            *current = Some(UiHeadlessRetainedPresentation {
                frame: view.frame(),
                surface: view.surface(),
                binding: view.binding(),
                baseline: initial.affinity().baseline(),
                commands,
                order: UiHeadlessRetainedOrder::initial(initial.order(), initial.order_integrity())
                    .map_err(|_| malformed())?,
                auxiliary: initial.auxiliary().clone(),
                reconstruction: vec![recorded.clone()],
            });
            Ok(Some(recorded))
        }
        UiMountedPresentationWorkView::Delta(work) => {
            let Some(current) = current.as_mut() else {
                return Err(malformed());
            };
            delta::apply(view, capacity, current, work).map(Some)
        }
        UiMountedPresentationWorkView::Unchanged(unchanged) => {
            let current = current.as_mut().ok_or_else(malformed)?;
            if !affinity_matches(current, unchanged.affinity()) {
                return Err(malformed());
            }
            current.frame = view.frame();
            Ok(None)
        }
    }
}

fn affinity_matches(
    current: &UiHeadlessRetainedPresentation,
    affinity: worth_ui_host_contract::UiMountedPresentationAffinity,
) -> bool {
    affinity.predecessor() == Some(current.frame)
        && affinity.surface() == current.surface
        && affinity.binding() == current.binding
        && affinity.baseline() == current.baseline
}

fn malformed() -> UiHostSurfacePresentationDenial {
    UiHostSurfacePresentationDenial::MalformedProjection
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
            mechanic,
        } => {
            *identity == UiMountedPaintCommandIdentity::filled_rect(mechanic)
                && projection.filled_rects().rows().contains(mechanic)
        }
        UiMountedPaintCommand::SemanticText {
            identity,
            mechanic,
        } => {
            *identity == UiMountedPaintCommandIdentity::semantic_text(mechanic)
                && projection.semantic_text().rows().contains(mechanic)
        }
    });
    if !aligned {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
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
