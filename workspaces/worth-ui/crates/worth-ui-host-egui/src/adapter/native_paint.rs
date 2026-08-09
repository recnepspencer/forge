use std::collections::{HashMap, HashSet};

use worth_ui_host_contract::{
    UiHostProtocolDenial, UiHostProtocolSchemaFamily, UiHostSurfacePresentationDenial,
    UiMountedAllocationProjection, UiMountedCoordinateSpace, UiMountedGeometryPosture,
    UiMountedPaintCommand, UiMountedPaintCommandChange, UiMountedPaintCommandIdentity,
    UiMountedPaintOrderIdentity, UiMountedPaintProjection, UiMountedPresentationDelta,
    UiMountedPresentationInitial, UiMountedProjectionView, UiMountedStaticPaintSchemaVersion,
    UiMountedTextSchemaVersion,
};

#[derive(Clone)]
pub(super) struct UiEguiPreparedNativePaint {
    layer: egui::LayerId,
    commands: HashMap<UiMountedPaintCommandIdentity, UiEguiPreparedNativePaintCommand>,
    order: Vec<UiMountedPaintOrderIdentity>,
}

#[derive(Clone)]
struct UiEguiPreparedFilledRect {
    rect: egui::Rect,
    clip_rect: egui::Rect,
    color: egui::Color32,
}

#[derive(Clone)]
enum UiEguiPreparedNativePaintCommand {
    FilledRect(UiEguiPreparedFilledRect),
    SemanticText(super::semantic_text::UiEguiPreparedSemanticText),
}

impl UiEguiPreparedNativePaint {
    pub(super) fn prepare_initial(
        view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
        initial: &UiMountedPresentationInitial,
    ) -> Result<Self, UiHostSurfacePresentationDenial> {
        let projection = initial.projection();
        validate_protocol(view, projection)?;
        validate_table_schema(projection)?;
        validate_projection_rows(view, projection)?;
        let commands = initial
            .commands()
            .iter()
            .map(|command| {
                prepare_command(view, command).map(|prepared| (command.identity(), prepared))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        validate_order(&commands, initial.order())?;
        if !initial.order_integrity().admits(initial.order()) {
            return Err(UiHostSurfacePresentationDenial::MalformedProjection);
        }
        Ok(Self {
            layer: surface_layer(projection.binding()),
            commands,
            order: initial.order().to_vec(),
        })
    }

    pub(super) fn apply_delta(
        &self,
        view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
        delta: &UiMountedPresentationDelta,
    ) -> Result<Self, UiHostSurfacePresentationDenial> {
        let mut commands = self.commands.clone();
        let mut order = self.order.clone();
        for change in delta.changes() {
            apply_command_change(view, &mut commands, &mut order, change)?;
        }
        for edit in delta.order() {
            let identity = edit.identity();
            if !commands.contains_key(&identity.command()) {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
            if let Some(index) = order.iter().position(|candidate| *candidate == identity) {
                order.remove(index);
            }
            let index = match edit.predecessor() {
                Some(predecessor) => order
                    .iter()
                    .position(|candidate| *candidate == predecessor)
                    .and_then(|index| index.checked_add(1))
                    .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?,
                None => 0,
            };
            order.insert(index, identity);
        }
        validate_order(&commands, &order)?;
        if !delta.order_integrity().admits(&order) {
            return Err(UiHostSurfacePresentationDenial::MalformedProjection);
        }
        Ok(Self {
            layer: self.layer,
            commands,
            order,
        })
    }

    pub(super) fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub(super) fn order(&self) -> &[UiMountedPaintOrderIdentity] {
        &self.order
    }

    pub(super) fn paint(&self, context: &egui::Context) {
        let painter = context.layer_painter(self.layer);
        for identity in &self.order {
            self.commands
                .get(&identity.command())
                .expect("validated paint order identity must resolve")
                .paint(&painter);
        }
    }
}

impl UiEguiPreparedNativePaintCommand {
    fn paint(&self, painter: &egui::Painter) {
        match self {
            Self::FilledRect(row) => {
                painter
                    .clone()
                    .with_clip_rect(row.clip_rect)
                    .rect_filled(row.rect, 0.0, row.color);
            }
            Self::SemanticText(row) => row.paint(painter),
        }
    }
}

fn apply_command_change(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    commands: &mut HashMap<UiMountedPaintCommandIdentity, UiEguiPreparedNativePaintCommand>,
    order: &mut Vec<UiMountedPaintOrderIdentity>,
    change: &UiMountedPaintCommandChange,
) -> Result<(), UiHostSurfacePresentationDenial> {
    match change {
        UiMountedPaintCommandChange::Insert(command) => {
            let identity = command.identity();
            if commands.contains_key(&identity) {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
            commands.insert(identity, prepare_command(view, command)?);
        }
        UiMountedPaintCommandChange::Replace(command) => {
            let identity = command.identity();
            if !commands.contains_key(&identity) {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
            commands.insert(identity, prepare_command(view, command)?);
        }
        UiMountedPaintCommandChange::Remove(identity) => {
            if commands.remove(identity).is_none() {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
            order.retain(|candidate| candidate.command() != *identity);
        }
    }
    Ok(())
}

fn prepare_command(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    command: &UiMountedPaintCommand,
) -> Result<UiEguiPreparedNativePaintCommand, UiHostSurfacePresentationDenial> {
    match command {
        UiMountedPaintCommand::FilledRect { mechanic, .. } => {
            validate_command_basis(
                view,
                mechanic.frame(),
                mechanic.surface(),
                mechanic.binding(),
            )?;
            Ok(UiEguiPreparedNativePaintCommand::FilledRect(translate_row(
                *mechanic,
            )))
        }
        UiMountedPaintCommand::SemanticText { mechanic, .. } => {
            validate_command_basis(
                view,
                mechanic.frame(),
                mechanic.surface(),
                mechanic.binding(),
            )?;
            if mechanic.content_generation() != view.content_generation()
                || mechanic.capability_generation() != view.capability_generation()
                || mechanic.capability_profile_digest() != view.capability_profile_digest()
            {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
            Ok(UiEguiPreparedNativePaintCommand::SemanticText(
                super::semantic_text::translate(mechanic),
            ))
        }
    }
}

fn validate_command_basis(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
) -> Result<(), UiHostSurfacePresentationDenial> {
    if frame != view.frame() || surface != view.surface() || binding != view.binding() {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(())
}

fn validate_order(
    commands: &HashMap<UiMountedPaintCommandIdentity, UiEguiPreparedNativePaintCommand>,
    order: &[UiMountedPaintOrderIdentity],
) -> Result<(), UiHostSurfacePresentationDenial> {
    let identities = order
        .iter()
        .map(|identity| identity.command())
        .collect::<HashSet<_>>();
    if identities.len() != order.len() || identities != commands.keys().copied().collect() {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(())
}

fn validate_projection_rows(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    projection: &UiMountedProjectionView,
) -> Result<(), UiHostSurfacePresentationDenial> {
    for row in projection.filled_rects().rows().iter().copied() {
        validate_row_basis(projection, row)?;
    }
    let referenced_rows = projection
        .nodes()
        .iter()
        .filter(|node| matches!(node.paint(), UiMountedPaintProjection::FilledRect(_)))
        .count();
    if referenced_rows != projection.filled_rects().rows().len() {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    super::semantic_text::validate_projection(view, projection)
}

fn surface_layer(binding: worth_ui_host_contract::UiSurfaceBindingGeneration) -> egui::LayerId {
    egui::LayerId::new(
        egui::Order::Middle,
        egui::Id::new(("worth-ui-mounted-surface", binding.diagnostic_value())),
    )
}

fn validate_protocol(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    projection: &UiMountedProjectionView,
) -> Result<(), UiHostSurfacePresentationDenial> {
    if !projection.filled_rects().rows().is_empty()
        && view.protocol().contract().mounted_frame().revision()
            < UiMountedStaticPaintSchemaVersion::REQUIRED_MOUNTED_FRAME_REVISION
    {
        return Err(UiHostSurfacePresentationDenial::Protocol(
            UiHostProtocolDenial::SchemaTooOld(UiHostProtocolSchemaFamily::MountedFrame),
        ));
    }
    if !projection.semantic_text().rows().is_empty()
        && view.protocol().contract().mounted_frame().revision()
            < UiMountedTextSchemaVersion::REQUIRED_MOUNTED_FRAME_REVISION
    {
        return Err(UiHostSurfacePresentationDenial::Protocol(
            UiHostProtocolDenial::SchemaTooOld(UiHostProtocolSchemaFamily::MountedFrame),
        ));
    }
    Ok(())
}

fn validate_table_schema(
    projection: &UiMountedProjectionView,
) -> Result<(), UiHostSurfacePresentationDenial> {
    if (!projection.filled_rects().rows().is_empty()
        && projection.filled_rects().schema() != UiMountedStaticPaintSchemaVersion::current())
        || (!projection.semantic_text().rows().is_empty()
            && projection.semantic_text().schema() != UiMountedTextSchemaVersion::current())
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(())
}

fn translate_row(
    row: worth_ui_host_contract::UiMountedFilledRectMechanic,
) -> UiEguiPreparedFilledRect {
    let bounds = row.bounds();
    let channels = row.color().channels();
    UiEguiPreparedFilledRect {
        rect: egui_rect(bounds),
        clip_rect: egui_rect(row.clip_bounds()),
        color: egui::Color32::from_rgba_unmultiplied(
            channels[0],
            channels[1],
            channels[2],
            channels[3],
        ),
    }
}

fn validate_row_basis(
    projection: &UiMountedProjectionView,
    row: worth_ui_host_contract::UiMountedFilledRectMechanic,
) -> Result<(), UiHostSurfacePresentationDenial> {
    if row.schema() != UiMountedStaticPaintSchemaVersion::current()
        || row.frame() != projection.frame()
        || row.surface() != projection.surface()
        || row.binding() != projection.binding()
        || row.bounds() != row.clip_bounds()
        || row.bounds().posture() != UiMountedGeometryPosture::Area
        || row.bounds().coordinate_space() != UiMountedCoordinateSpace::Viewport
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    let node = projection
        .nodes()
        .iter()
        .find(|node| node.mounted_instance() == row.mounted_instance())
        .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?;
    let matching_reference = matches!(
        node.paint(),
        UiMountedPaintProjection::FilledRect(reference)
            if projection.filled_rects().resolve(reference) == Some(&row)
    );
    let matching_allocation = matches!(
        node.allocation(),
        UiMountedAllocationProjection::Known { bounds, basis }
            if bounds == row.bounds() && basis == row.allocation_basis()
    );
    if node.node_receipt() != row.node_receipt() || !matching_reference || !matching_allocation {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(())
}

pub(super) fn egui_rect(bounds: worth_ui_host_contract::UiMountedCanonicalBox) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(bounds.x(), bounds.y()),
        egui::vec2(bounds.width(), bounds.height()),
    )
}

#[cfg(test)]
#[path = "native_paint_tests.rs"]
mod tests;
