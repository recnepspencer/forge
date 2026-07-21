use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiHostObservationValue, UiMeasurementRequest,
    UiMeasurementRequestFamily, UiViewportExtentObservation, WorthUiHostCapability,
    WorthUiHostCapabilityReport, WorthUiHostContract, WorthUiMeasurementHostAdapter,
    WorthUiOperationalHostAdapter,
};

#[derive(Clone, Default)]
pub struct WorthUiHostEgui {
    context: egui::Context,
}

impl WorthUiHostEgui {
    pub fn new(context: egui::Context) -> Self {
        Self { context }
    }
}

impl WorthUiMeasurementHostAdapter for WorthUiHostEgui {
    fn observe_measurement(&self, request: &UiMeasurementRequest) -> UiHostObservationValue {
        match request.family() {
            UiMeasurementRequestFamily::ViewportExtent => {
                let size = self.context.input(|input| input.screen_rect().size());
                UiHostObservationValue::ViewportExtent(UiViewportExtentObservation {
                    width: size.x,
                    height: size.y,
                })
            }
            UiMeasurementRequestFamily::DpiScaleFactor => {
                UiHostObservationValue::DpiScaleFactor(UiDpiScaleFactorObservation {
                    scale_factor: self.context.pixels_per_point(),
                })
            }
            family => unreachable!(
                "egui operational capability report does not admit {family:?} observation"
            ),
        }
    }
}

impl WorthUiOperationalHostAdapter for WorthUiHostEgui {
    fn operational_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::egui()
    }

    fn operational_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::available(vec![
            WorthUiHostCapability::DpiObservation,
            WorthUiHostCapability::ViewportObservation,
            WorthUiHostCapability::CanvasSpatialDraw,
            WorthUiHostCapability::CanvasSpatialHitTest,
            WorthUiHostCapability::CanvasSpatialOverlay,
            WorthUiHostCapability::CanvasSpatialToolState,
            WorthUiHostCapability::CanvasSpatialRenderResource,
            WorthUiHostCapability::RealtimeOverlayDraw,
            WorthUiHostCapability::RealtimeOverlaySurface,
            WorthUiHostCapability::RealtimeOverlayHook,
        ])
    }

    fn consume_output(
        &self,
        output: &worth_ui_host_contract::WorthUiHostOutputEnvelope,
    ) -> worth_ui_host_contract::WorthUiHostOutputDisposition {
        use worth_ui_host_contract::{WorthUiHostOutputDisposition, WorthUiHostOutputPayload};

        let generation = output.generation();
        let receipt = output.receipt_reference();
        let id = egui::Id::new((
            "worth-ui-host-output",
            generation.active_artifact_digest(),
            generation.active_plan_digest(),
            generation.frame_epoch(),
            receipt.digest(),
        ));
        let payload_label = match output.payload() {
            WorthUiHostOutputPayload::Ordinary(value) => format!(
                "ordinary:{:?}:{}",
                value.target(),
                value.touched_row_count()
            ),
            WorthUiHostOutputPayload::VirtualizedData(value) => {
                format!("virtualized:{}x{}", value.row_count(), value.column_count())
            }
            WorthUiHostOutputPayload::CanvasSpatial(value) => format!(
                "canvas:{:?}:{}",
                value.target(),
                value.visible_primitive_count()
            ),
            WorthUiHostOutputPayload::RealtimeOverlay(value) => {
                format!("realtime:{}", value.overlay_row_count())
            }
            _ => return WorthUiHostOutputDisposition::UnsupportedPayload,
        };
        let label = format!(
            "worth-ui:{}:{payload_label}",
            generation.active_plan_digest()
        );
        self.context
            .layer_painter(egui::LayerId::new(egui::Order::Foreground, id))
            .text(
                egui::Pos2::ZERO,
                egui::Align2::LEFT_TOP,
                label,
                egui::FontId::monospace(10.0),
                egui::Color32::GRAY,
            );
        WorthUiHostOutputDisposition::Consumed
    }
}
