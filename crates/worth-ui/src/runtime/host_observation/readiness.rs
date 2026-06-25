#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiHostMeasurementReadinessPosture {
    Ready,
    MissingAvailableBounds,
    StaleMountedProductView,
    Denied,
}

impl WorthUiHostMeasurementReadinessPosture {
    pub fn token(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::MissingAvailableBounds => "missing_available_bounds",
            Self::StaleMountedProductView => "stale_mounted_product_view",
            Self::Denied => "denied",
        }
    }
}
