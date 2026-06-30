#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RawLayoutMeasurementKind {
    Width,
    Height,
    Gap,
    Padding,
    ZOrder,
    Timing,
    Breakpoint,
}

impl RawLayoutMeasurementKind {
    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::Width => "width",
            Self::Height => "height",
            Self::Gap => "gap",
            Self::Padding => "padding",
            Self::ZOrder => "z_order",
            Self::Timing => "timing",
            Self::Breakpoint => "breakpoint",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawLayoutMeasurementForDiagnostics {
    kind: RawLayoutMeasurementKind,
    value: i64,
}

impl RawLayoutMeasurementForDiagnostics {
    pub fn width(value: i64) -> Self {
        Self::new(RawLayoutMeasurementKind::Width, value)
    }

    pub fn height(value: i64) -> Self {
        Self::new(RawLayoutMeasurementKind::Height, value)
    }

    pub fn gap(value: i64) -> Self {
        Self::new(RawLayoutMeasurementKind::Gap, value)
    }

    pub fn padding(value: i64) -> Self {
        Self::new(RawLayoutMeasurementKind::Padding, value)
    }

    pub fn z_order(value: i64) -> Self {
        Self::new(RawLayoutMeasurementKind::ZOrder, value)
    }

    pub fn timing(value: i64) -> Self {
        Self::new(RawLayoutMeasurementKind::Timing, value)
    }

    pub fn breakpoint(value: i64) -> Self {
        Self::new(RawLayoutMeasurementKind::Breakpoint, value)
    }

    fn new(kind: RawLayoutMeasurementKind, value: i64) -> Self {
        Self { kind, value }
    }

    pub fn kind(&self) -> &RawLayoutMeasurementKind {
        &self.kind
    }

    pub fn value(&self) -> i64 {
        self.value
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!("{}:{}", self.kind.digest_basis(), self.value)
    }
}
