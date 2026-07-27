#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentAllocationMeasurementContract {
    FillViewport,
}

impl ComponentAllocationMeasurementContract {
    pub const fn fill_viewport() -> Self {
        Self::FillViewport
    }

    pub(crate) const fn digest_basis(self) -> &'static str {
        match self {
            Self::FillViewport => "fill-viewport",
        }
    }
}
