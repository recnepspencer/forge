#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MeasurementValue {
    LogicalPixels(u32),
    RatioPermille(u32),
    Milliseconds(u64),
    LayerIndex(i32),
    BreakpointLogicalPixels(u32),
    ViewportWidthPermille(u32),
    ViewportHeightPermille(u32),
    UnitlessForDiagnostics(i64),
}

impl MeasurementValue {
    pub fn logical_pixels(value: u32) -> Self {
        Self::LogicalPixels(value)
    }

    pub fn ratio_permille(value: u32) -> Self {
        Self::RatioPermille(value)
    }

    pub fn milliseconds(value: u64) -> Self {
        Self::Milliseconds(value)
    }

    pub fn layer_index(value: i32) -> Self {
        Self::LayerIndex(value)
    }

    pub fn breakpoint_logical_pixels(value: u32) -> Self {
        Self::BreakpointLogicalPixels(value)
    }

    pub fn viewport_width_permille(value: u32) -> Self {
        Self::ViewportWidthPermille(value)
    }

    pub fn viewport_height_permille(value: u32) -> Self {
        Self::ViewportHeightPermille(value)
    }

    pub fn unitless_for_diagnostics(value: i64) -> Self {
        Self::UnitlessForDiagnostics(value)
    }

    pub(crate) fn is_unitless(&self) -> bool {
        matches!(self, Self::UnitlessForDiagnostics(_))
    }

    pub(crate) fn comparable_order_key(&self) -> Option<ComparableMeasurementOrderKey> {
        match self {
            Self::LogicalPixels(value) => Some(ComparableMeasurementOrderKey::new(
                "logical_pixels",
                i128::from(*value),
            )),
            Self::RatioPermille(value) => Some(ComparableMeasurementOrderKey::new(
                "ratio_permille",
                i128::from(*value),
            )),
            Self::Milliseconds(value) => Some(ComparableMeasurementOrderKey::new(
                "milliseconds",
                i128::from(*value),
            )),
            Self::LayerIndex(value) => Some(ComparableMeasurementOrderKey::new(
                "layer_index",
                i128::from(*value),
            )),
            Self::BreakpointLogicalPixels(value) => Some(ComparableMeasurementOrderKey::new(
                "breakpoint_logical_pixels",
                i128::from(*value),
            )),
            Self::ViewportWidthPermille(value) => Some(ComparableMeasurementOrderKey::new(
                "viewport_width_permille",
                i128::from(*value),
            )),
            Self::ViewportHeightPermille(value) => Some(ComparableMeasurementOrderKey::new(
                "viewport_height_permille",
                i128::from(*value),
            )),
            Self::UnitlessForDiagnostics(_) => None,
        }
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::LogicalPixels(value) => format!("logical_pixels:{value}"),
            Self::RatioPermille(value) => format!("ratio_permille:{value}"),
            Self::Milliseconds(value) => format!("milliseconds:{value}"),
            Self::LayerIndex(value) => format!("layer_index:{value}"),
            Self::BreakpointLogicalPixels(value) => {
                format!("breakpoint_logical_pixels:{value}")
            }
            Self::ViewportWidthPermille(value) => format!("viewport_width_permille:{value}"),
            Self::ViewportHeightPermille(value) => format!("viewport_height_permille:{value}"),
            Self::UnitlessForDiagnostics(value) => format!("unitless:{value}"),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ComparableMeasurementOrderKey {
    unit: &'static str,
    value: i128,
}

impl ComparableMeasurementOrderKey {
    fn new(unit: &'static str, value: i128) -> Self {
        Self { unit, value }
    }

    pub(crate) fn is_not_ordered_before_or_equal_to(self, maximum: Self) -> bool {
        self.unit != maximum.unit || self.value > maximum.value
    }
}
