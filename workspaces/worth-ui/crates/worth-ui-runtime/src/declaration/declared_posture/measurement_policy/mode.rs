#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiDeclaredMeasurementMode {
    HugHeight,
    FillViewport,
    ViewportInset {
        horizontal_logical_points: u16,
        vertical_logical_points: u16,
    },
    ViewportRegion {
        horizontal: crate::capability::ComponentViewportAxisPlacement,
        vertical: crate::capability::ComponentViewportAxisPlacement,
    },
    FixedLogicalSize {
        width: u16,
        height: u16,
    },
}

pub(crate) fn measurement_mode_claim(claim: &str) -> Option<UiDeclaredMeasurementMode> {
    match claim {
        "measurement:hug-height" | "measurement:mode:hug-height" => {
            Some(UiDeclaredMeasurementMode::HugHeight)
        }
        _ => None,
    }
}
