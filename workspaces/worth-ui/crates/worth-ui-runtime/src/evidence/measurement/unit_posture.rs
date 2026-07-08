#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UiMeasurementUnitPosture {
    LogicalPx,
    PhysicalPx,
    UnitlessScale,
}

impl UiMeasurementUnitPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogicalPx => "logical_px",
            Self::PhysicalPx => "physical_px",
            Self::UnitlessScale => "unitless_scale",
        }
    }
}
