use worth_ui_host_contract::{
    UiHostProtocolDenial, UiHostProtocolSchemaFamily, UiHostSurfacePresentationDenial,
    UiMountedAllocationProjection, UiMountedCoordinateSpace, UiMountedGeometryPosture,
    UiMountedPaintProjection, UiMountedProjectionView, UiMountedStaticPaintSchemaVersion,
    UiMountedTextSchemaVersion,
};

#[derive(Clone)]
pub(super) struct UiEguiPreparedNativePaint {
    layer: egui::LayerId,
    commands: Vec<UiEguiPreparedNativePaintCommand>,
}

#[derive(Clone)]
struct UiEguiPreparedFilledRect {
    rect: egui::Rect,
    clip_rect: egui::Rect,
    color: egui::Color32,
    layer_semantic_order: u32,
}

#[derive(Clone)]
enum UiEguiPreparedNativePaintCommand {
    FilledRect(UiEguiPreparedFilledRect),
    SemanticText(super::semantic_text::UiEguiPreparedSemanticText),
}

impl UiEguiPreparedNativePaint {
    pub(super) fn prepare(
        view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    ) -> Result<Self, UiHostSurfacePresentationDenial> {
        validate_protocol(view)?;
        let projection = view.projection();
        validate_table_schema(projection)?;
        let mut filled_rects = projection
            .filled_rects()
            .rows()
            .iter()
            .copied()
            .map(|row| translate_row(projection, row))
            .collect::<Result<Vec<_>, _>>()?;
        let referenced_rows = projection
            .nodes()
            .iter()
            .filter(|node| matches!(node.paint(), UiMountedPaintProjection::FilledRect(_)))
            .count();
        if referenced_rows != filled_rects.len() {
            return Err(UiHostSurfacePresentationDenial::MalformedProjection);
        }
        let semantic_text = super::semantic_text::prepare(view)?;
        let mut commands = filled_rects
            .drain(..)
            .map(UiEguiPreparedNativePaintCommand::FilledRect)
            .chain(
                semantic_text
                    .into_iter()
                    .map(UiEguiPreparedNativePaintCommand::SemanticText),
            )
            .collect::<Vec<_>>();
        commands.sort_by_key(UiEguiPreparedNativePaintCommand::layer_semantic_order);
        Ok(Self {
            layer: surface_layer(projection.binding()),
            commands,
        })
    }

    pub(super) fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub(super) fn paint(&self, context: &egui::Context) {
        let painter = context.layer_painter(self.layer);
        for command in &self.commands {
            command.paint(&painter);
        }
    }
}

impl UiEguiPreparedNativePaintCommand {
    fn layer_semantic_order(&self) -> u32 {
        match self {
            Self::FilledRect(row) => row.layer_semantic_order,
            Self::SemanticText(row) => row.layer_semantic_order,
        }
    }

    fn paint(&self, painter: &egui::Painter) {
        match self {
            Self::FilledRect(row) => {
                painter
                    .clone()
                    .with_clip_rect(row.clip_rect)
                    .rect_filled(row.rect, 0.0, row.color);
            }
            Self::SemanticText(row) => {
                painter.clone().with_clip_rect(row.clip_rect).text(
                    row.origin,
                    egui::Align2::LEFT_TOP,
                    row.text.as_ref(),
                    row.font.clone(),
                    row.color,
                );
            }
        }
    }
}

fn surface_layer(binding: worth_ui_host_contract::UiSurfaceBindingGeneration) -> egui::LayerId {
    egui::LayerId::new(
        egui::Order::Middle,
        egui::Id::new(("worth-ui-mounted-surface", binding.diagnostic_value())),
    )
}

fn validate_protocol(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
) -> Result<(), UiHostSurfacePresentationDenial> {
    if !view.projection().filled_rects().rows().is_empty()
        && view.protocol().contract().mounted_frame().revision()
            < UiMountedStaticPaintSchemaVersion::REQUIRED_MOUNTED_FRAME_REVISION
    {
        return Err(UiHostSurfacePresentationDenial::Protocol(
            UiHostProtocolDenial::SchemaTooOld(UiHostProtocolSchemaFamily::MountedFrame),
        ));
    }
    if !view.projection().semantic_text().rows().is_empty()
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
    if !projection.filled_rects().rows().is_empty()
        && projection.filled_rects().schema() != UiMountedStaticPaintSchemaVersion::current()
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    if !projection.semantic_text().rows().is_empty()
        && projection.semantic_text().schema() != UiMountedTextSchemaVersion::current()
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(())
}

fn translate_row(
    projection: &UiMountedProjectionView,
    row: worth_ui_host_contract::UiMountedFilledRectMechanic,
) -> Result<UiEguiPreparedFilledRect, UiHostSurfacePresentationDenial> {
    validate_row_basis(projection, row)?;
    let bounds = row.bounds();
    let clip = row.clip_bounds();
    let channels = row.color().channels();
    Ok(UiEguiPreparedFilledRect {
        rect: egui_rect(bounds),
        clip_rect: egui_rect(clip),
        color: egui::Color32::from_rgba_unmultiplied(
            channels[0],
            channels[1],
            channels[2],
            channels[3],
        ),
        layer_semantic_order: row.layer_semantic_order(),
    })
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
