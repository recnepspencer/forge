use worth_ui_host_contract::{
    UiHostProtocolDenial, UiHostProtocolSchemaFamily, UiHostSurfacePresentationDenial,
    UiMountedAllocationProjection, UiMountedCoordinateSpace, UiMountedGeometryPosture,
    UiMountedPaintProjection, UiMountedProjectionView, UiMountedStaticPaintSchemaVersion,
};

#[derive(Clone)]
pub(super) struct UiEguiPreparedNativePaint {
    filled_rects: Vec<UiEguiPreparedFilledRect>,
}

#[derive(Clone)]
struct UiEguiPreparedFilledRect {
    rect: egui::Rect,
    clip_rect: egui::Rect,
    color: egui::Color32,
    layer_semantic_order: u32,
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
        filled_rects.sort_by_key(|row| row.layer_semantic_order);
        Ok(Self { filled_rects })
    }

    pub(super) fn is_empty(&self) -> bool {
        self.filled_rects.is_empty()
    }

    pub(super) fn paint(&self, context: &egui::Context) {
        for row in &self.filled_rects {
            let layer = egui::LayerId::new(
                egui::Order::Middle,
                egui::Id::new(("worth-ui-mounted", row.layer_semantic_order)),
            );
            context
                .layer_painter(layer)
                .with_clip_rect(row.clip_rect)
                .rect_filled(row.rect, 0.0, row.color);
        }
    }
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

fn egui_rect(bounds: worth_ui_host_contract::UiMountedCanonicalBox) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(bounds.x(), bounds.y()),
        egui::vec2(bounds.width(), bounds.height()),
    )
}
