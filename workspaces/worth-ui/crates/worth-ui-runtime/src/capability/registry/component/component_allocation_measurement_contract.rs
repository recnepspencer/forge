#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentAllocationMeasurementContract {
    FillViewport,
    ViewportInset(super::ComponentViewportInset),
    ViewportRegion(super::ComponentViewportRegion),
    FixedLogicalSize { width: u16, height: u16 },
}

impl ComponentAllocationMeasurementContract {
    pub const fn fill_viewport() -> Self {
        Self::FillViewport
    }

    pub const fn viewport_inset(inset: super::ComponentViewportInset) -> Self {
        Self::ViewportInset(inset)
    }

    pub const fn viewport_region(region: super::ComponentViewportRegion) -> Self {
        Self::ViewportRegion(region)
    }

    pub fn fixed_logical_size(width: u16, height: u16) -> Option<Self> {
        (width != 0 && height != 0).then_some(Self::FixedLogicalSize { width, height })
    }

    pub(crate) fn digest_basis(self) -> String {
        match self {
            Self::FillViewport => "fill-viewport".to_owned(),
            Self::ViewportInset(inset) => inset.digest_basis(),
            Self::ViewportRegion(region) => region.digest_basis(),
            Self::FixedLogicalSize { width, height } => {
                format!("fixed-logical-size:{width}:{height}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentAllocationMeasurementContract;
    use crate::capability::{
        ComponentViewportAxisPlacement, ComponentViewportInset, ComponentViewportRegion,
    };

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
        let region =
            ComponentAllocationMeasurementContract::viewport_region(ComponentViewportRegion::new(
                ComponentViewportAxisPlacement::fixed_from_start(24, 216).unwrap(),
                ComponentViewportAxisPlacement::stretch_between(104, 72),
            ));
        assert_eq!(
            region.digest_basis(),
            "viewport-region:fixed-from-start:24:216:stretch-between:104:72"
        );
        assert!(ComponentAllocationMeasurementContract::fixed_logical_size(0, 24).is_none());
        assert_eq!(
            ComponentAllocationMeasurementContract::fixed_logical_size(160, 24)
                .unwrap()
                .digest_basis(),
            "fixed-logical-size:160:24"
        );
    }
}
