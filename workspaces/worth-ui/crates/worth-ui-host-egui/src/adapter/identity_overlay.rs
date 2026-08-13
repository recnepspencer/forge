use worth_ui_host_contract::{
    UiHostSurfacePresentationDenial, UiMountedDiagnosticProjection,
    UiMountedIdentityOverlayMechanic, UiMountedProjectionView,
};

#[derive(Clone)]
pub(super) struct UiEguiPreparedIdentityOverlay {
    target: Option<worth_ui_host_contract::UiMountedInstanceIdentity>,
    layer: egui::LayerId,
    clip_rect: egui::Rect,
    strips: Vec<egui::Rect>,
    color: egui::Color32,
}

impl UiEguiPreparedIdentityOverlay {
    pub(super) fn prepare(
        context: &egui::Context,
        projection: &UiMountedProjectionView,
    ) -> Result<Self, UiHostSurfacePresentationDenial> {
        let mechanics = projection
            .nodes()
            .iter()
            .filter_map(|node| match node.diagnostic() {
                UiMountedDiagnosticProjection::IdentityOverlay(mechanic) => Some((node, mechanic)),
                _ => None,
            })
            .collect::<Vec<_>>();
        if mechanics.len() > 1 {
            return Err(UiHostSurfacePresentationDenial::MalformedProjection);
        }
        let Some((node, mechanic)) = mechanics.first() else {
            return Ok(Self::empty(projection.binding()));
        };
        validate_mechanic(projection, node, *mechanic)?;
        let geometry = UiEguiClientGeometry::observe(context)?;
        if !geometry.matches(mechanic.coordinate_basis()) {
            return Err(UiHostSurfacePresentationDenial::MalformedProjection);
        }
        let strips = geometry.overlay_strips(*mechanic)?.into();
        let channels = mechanic.color().channels();
        let color = egui::Color32::from_rgba_unmultiplied(
            channels[0],
            channels[1],
            channels[2],
            channels[3],
        );
        Ok(Self {
            target: Some(node.mounted_instance()),
            layer: overlay_layer(projection.binding()),
            clip_rect: geometry.logical_client,
            strips,
            color,
        })
    }

    pub(super) fn prepare_delta(
        context: &egui::Context,
        view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
        current: &Self,
        changes: &[worth_ui_host_contract::UiMountedPresentationNodeChange],
    ) -> Result<Self, UiHostSurfacePresentationDenial> {
        let touched = |target| {
            changes
                .iter()
                .any(|change| change.mounted_instance() == target)
        };
        let retained_target = current.target.filter(|target| !touched(*target));
        let mut replacement = None;
        for change in changes {
            let worth_ui_host_contract::UiMountedPresentationNodeChange::Upsert(state) = change
            else {
                continue;
            };
            let UiMountedDiagnosticProjection::IdentityOverlay(mechanic) = state.diagnostic()
            else {
                continue;
            };
            if replacement.replace((*state, mechanic)).is_some() {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
        }
        if retained_target.is_some() && replacement.is_some() {
            return Err(UiHostSurfacePresentationDenial::MalformedProjection);
        }
        let Some((state, mechanic)) = replacement else {
            return Ok(
                retained_target.map_or_else(|| Self::empty(view.binding()), |_| current.clone())
            );
        };
        validate_state_mechanic(view, state, mechanic)?;
        let geometry = UiEguiClientGeometry::observe(context)?;
        if !geometry.matches(mechanic.coordinate_basis()) {
            return Err(UiHostSurfacePresentationDenial::MalformedProjection);
        }
        let channels = mechanic.color().channels();
        Ok(Self {
            target: Some(state.mounted_instance()),
            layer: overlay_layer(view.binding()),
            clip_rect: geometry.logical_client,
            strips: geometry.overlay_strips(mechanic)?.into(),
            color: egui::Color32::from_rgba_unmultiplied(
                channels[0],
                channels[1],
                channels[2],
                channels[3],
            ),
        })
    }

    fn empty(binding: worth_ui_host_contract::UiSurfaceBindingGeneration) -> Self {
        Self {
            target: None,
            layer: overlay_layer(binding),
            clip_rect: egui::Rect::NOTHING,
            strips: Vec::new(),
            color: egui::Color32::TRANSPARENT,
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.strips.is_empty()
    }

    pub(super) fn paint(&self, context: &egui::Context) {
        let painter = context
            .layer_painter(self.layer)
            .with_clip_rect(self.clip_rect);
        for strip in &self.strips {
            painter.rect_filled(*strip, 0.0, self.color);
        }
    }
}

fn validate_state_mechanic(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    state: worth_ui_host_contract::UiMountedPresentationNodeState,
    mechanic: UiMountedIdentityOverlayMechanic,
) -> Result<(), UiHostSurfacePresentationDenial> {
    if mechanic.successor_frame() != view.frame()
        || mechanic.surface() != view.surface()
        || mechanic.binding() != view.binding()
        || mechanic.target_receipt().mounted_instance() != state.mounted_instance()
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct UiEguiClientGeometry {
    logical_client: egui::Rect,
    physical_dimensions: [u32; 2],
    scale: [f32; 2],
}

impl UiEguiClientGeometry {
    fn observe(context: &egui::Context) -> Result<Self, UiHostSurfacePresentationDenial> {
        context.input(|input| {
            let logical_client = input.viewport_rect();
            let pixels_per_point = input.pixels_per_point();
            let size = logical_client.size();
            if !pixels_per_point.is_finite()
                || pixels_per_point <= 0.0
                || !size.x.is_finite()
                || !size.y.is_finite()
                || size.x <= 0.0
                || size.y <= 0.0
            {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
            let physical_dimensions = [
                physical_dimension(size.x, pixels_per_point)?,
                physical_dimension(size.y, pixels_per_point)?,
            ];
            Ok(Self {
                logical_client,
                physical_dimensions,
                scale: [
                    physical_dimensions[0] as f32 / size.x,
                    physical_dimensions[1] as f32 / size.y,
                ],
            })
        })
    }

    fn matches(self, basis: worth_ui_host_contract::UiMountedClientCoordinateBasis) -> bool {
        basis.client_physical_dimensions() == self.physical_dimensions
            && basis.viewport_logical_dimensions()
                == [self.logical_client.width(), self.logical_client.height()]
            && basis.scale() == self.scale
            && basis.translation() == [self.logical_client.min.x, self.logical_client.min.y]
            && basis.orientation()
                == worth_ui_host_contract::UiHostCoordinateOrientation::TopLeftOrigin
            && basis.rounding()
                == worth_ui_host_contract::UiHostCoordinateRounding::PixelCenterNearest
    }

    fn overlay_strips(
        self,
        mechanic: UiMountedIdentityOverlayMechanic,
    ) -> Result<[egui::Rect; 4], UiHostSurfacePresentationDenial> {
        let target = mechanic.target_region();
        if target.right() > self.physical_dimensions[0]
            || target.bottom() > self.physical_dimensions[1]
        {
            return Err(UiHostSurfacePresentationDenial::MalformedProjection);
        }
        let width = u32::from(mechanic.border_width());
        let left = target.left();
        let top = target.top();
        let right = target.right();
        let bottom = target.bottom();
        Ok([
            self.logical_rect(left, top, right, top.saturating_add(width).min(bottom)),
            self.logical_rect(left, bottom.saturating_sub(width).max(top), right, bottom),
            self.logical_rect(left, top, left.saturating_add(width).min(right), bottom),
            self.logical_rect(right.saturating_sub(width).max(left), top, right, bottom),
        ])
    }

    fn logical_rect(self, left: u32, top: u32, right: u32, bottom: u32) -> egui::Rect {
        let origin = self.logical_client.min;
        egui::Rect::from_min_max(
            egui::pos2(
                origin.x + left as f32 / self.scale[0],
                origin.y + top as f32 / self.scale[1],
            ),
            egui::pos2(
                origin.x + right as f32 / self.scale[0],
                origin.y + bottom as f32 / self.scale[1],
            ),
        )
    }
}

fn validate_mechanic(
    projection: &UiMountedProjectionView,
    node: &worth_ui_host_contract::UiMountedNodeProjectionView,
    mechanic: UiMountedIdentityOverlayMechanic,
) -> Result<(), UiHostSurfacePresentationDenial> {
    if mechanic.successor_frame() != projection.frame()
        || mechanic.surface() != projection.surface()
        || mechanic.binding() != projection.binding()
        || mechanic.target_receipt().mounted_instance() != node.mounted_instance()
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(())
}

fn physical_dimension(
    logical: f32,
    pixels_per_point: f32,
) -> Result<u32, UiHostSurfacePresentationDenial> {
    let physical = logical * pixels_per_point;
    if !physical.is_finite() || physical <= 0.0 || physical > u32::MAX as f32 {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(physical.round() as u32)
}

fn overlay_layer(binding: worth_ui_host_contract::UiSurfaceBindingGeneration) -> egui::LayerId {
    egui::LayerId::new(
        egui::Order::Foreground,
        egui::Id::new(("worth-ui-identity-overlay", binding.diagnostic_value())),
    )
}

#[cfg(test)]
#[path = "identity_overlay_tests.rs"]
mod tests;
