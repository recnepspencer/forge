#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiDeclaredMeasurementMode {
    HugHeight,
}

pub(crate) fn measurement_mode_claim(claim: &str) -> Option<UiDeclaredMeasurementMode> {
    match claim {
        "measurement:hug-height" | "measurement:mode:hug-height" => {
            Some(UiDeclaredMeasurementMode::HugHeight)
        }
        _ => None,
    }
}
