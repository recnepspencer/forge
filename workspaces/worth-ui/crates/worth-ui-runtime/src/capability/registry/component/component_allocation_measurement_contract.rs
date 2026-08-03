#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentAllocationMeasurementContract {
    FillViewport,
    ViewportInset(super::ComponentViewportInset),
}

impl ComponentAllocationMeasurementContract {
    pub const fn fill_viewport() -> Self {
        Self::FillViewport
    }

    pub const fn viewport_inset(inset: super::ComponentViewportInset) -> Self {
        Self::ViewportInset(inset)
    }

    pub(crate) fn digest_basis(self) -> String {
        match self {
            Self::FillViewport => "fill-viewport".to_owned(),
            Self::ViewportInset(inset) => inset.digest_basis(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentAllocationMeasurementContract;
    use crate::capability::ComponentViewportInset;

    #[test]
    fn viewport_inset_digest_preserves_both_axes_and_differs_from_fill() {
        let fill = ComponentAllocationMeasurementContract::fill_viewport();
        let inset = ComponentAllocationMeasurementContract::viewport_inset(
            ComponentViewportInset::symmetric(48, 24),
        );
        let changed = ComponentAllocationMeasurementContract::viewport_inset(
            ComponentViewportInset::symmetric(48, 23),
        );

        assert_ne!(fill.digest_basis(), inset.digest_basis());
        assert_ne!(inset.digest_basis(), changed.digest_basis());
        assert_eq!(inset.digest_basis(), "viewport-inset:48:24");
    }
}
