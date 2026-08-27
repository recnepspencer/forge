#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseCompositionExtent {
    width: u32,
    height: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseLogicalRect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseCompositionLayout {
    extent: PlatformPulseCompositionExtent,
    masthead: PlatformPulseLogicalRect,
    evidence_rail: PlatformPulseLogicalRect,
    service_stage: PlatformPulseLogicalRect,
    status_band: PlatformPulseLogicalRect,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformPulseCompositionLayoutDenial {
    WidthOutsideProductCourtroom,
    HeightOutsideProductCourtroom,
}

impl PlatformPulseCompositionExtent {
    pub const DEFAULT: Self = Self::new(960, 600);
    pub const RESIZED: Self = Self::new(1_120, 700);

    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub const fn width(self) -> u32 {
        self.width
    }

    pub const fn height(self) -> u32 {
        self.height
    }
}

impl PlatformPulseLogicalRect {
    pub const fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub const fn origin(self) -> [u32; 2] {
        [self.x, self.y]
    }

    pub const fn extent(self) -> [u32; 2] {
        [self.width, self.height]
    }

    pub const fn right(self) -> u32 {
        self.x + self.width
    }

    pub const fn bottom(self) -> u32 {
        self.y + self.height
    }

    pub const fn contains(self, point: [u32; 2]) -> bool {
        point[0] >= self.x
            && point[0] < self.right()
            && point[1] >= self.y
            && point[1] < self.bottom()
    }

    pub fn allocation(self) -> ComponentAllocationMeasurementContract {
        let x = u16::try_from(self.x).expect("Pulse authored x fits viewport contract");
        let y = u16::try_from(self.y).expect("Pulse authored y fits viewport contract");
        let width = u16::try_from(self.width).expect("Pulse authored width fits viewport contract");
        let height =
            u16::try_from(self.height).expect("Pulse authored height fits viewport contract");
        viewport_region(
            ComponentViewportAxisPlacement::fixed_from_start(x, width)
                .expect("Pulse authored region width is nonzero"),
            ComponentViewportAxisPlacement::fixed_from_start(y, height)
                .expect("Pulse authored region height is nonzero"),
        )
    }
}

impl PlatformPulseCompositionLayout {
    pub const OUTER_GUTTER: u32 = 24;
    pub const SPACING_RHYTHM: u32 = 8;
    pub const EVIDENCE_RAIL_WIDTH: u32 = 216;

    pub const fn for_extent(
        extent: PlatformPulseCompositionExtent,
    ) -> Result<Self, PlatformPulseCompositionLayoutDenial> {
        if extent.width < PlatformPulseCompositionExtent::DEFAULT.width
            || extent.width > PlatformPulseCompositionExtent::RESIZED.width
        {
            return Err(PlatformPulseCompositionLayoutDenial::WidthOutsideProductCourtroom);
        }
        if extent.height < PlatformPulseCompositionExtent::DEFAULT.height
            || extent.height > PlatformPulseCompositionExtent::RESIZED.height
        {
            return Err(PlatformPulseCompositionLayoutDenial::HeightOutsideProductCourtroom);
        }
        let full_width = extent.width - 2 * Self::OUTER_GUTTER;
        let working_height = extent.height - 176;
        Ok(Self {
            extent,
            masthead: PlatformPulseLogicalRect::new(24, 24, full_width, 56),
            evidence_rail: PlatformPulseLogicalRect::new(
                24,
                104,
                Self::EVIDENCE_RAIL_WIDTH,
                working_height,
            ),
            service_stage: PlatformPulseLogicalRect::new(
                264,
                104,
                extent.width - 288,
                working_height,
            ),
            status_band: PlatformPulseLogicalRect::new(24, extent.height - 48, full_width, 24),
        })
    }

    pub const fn extent(self) -> PlatformPulseCompositionExtent {
        self.extent
    }

    pub const fn masthead(self) -> PlatformPulseLogicalRect {
        self.masthead
    }

    pub const fn evidence_rail(self) -> PlatformPulseLogicalRect {
        self.evidence_rail
    }

    pub const fn service_stage(self) -> PlatformPulseLogicalRect {
        self.service_stage
    }

    pub const fn status_band(self) -> PlatformPulseLogicalRect {
        self.status_band
    }

    pub fn masthead_allocation(self) -> ComponentAllocationMeasurementContract {
        let _ = self;
        viewport_region(
            ComponentViewportAxisPlacement::stretch_between(24, 24),
            ComponentViewportAxisPlacement::fixed_from_start(24, 56)
                .expect("the authored masthead height is nonzero"),
        )
    }

    pub fn masthead_border_allocation(self) -> ComponentAllocationMeasurementContract {
        let _ = self;
        viewport_region(
            ComponentViewportAxisPlacement::stretch_between(23, 23),
            ComponentViewportAxisPlacement::fixed_from_start(23, 58)
                .expect("the authored masthead border height is nonzero"),
        )
    }

    pub fn evidence_rail_allocation(self) -> ComponentAllocationMeasurementContract {
        let _ = self;
        viewport_region(
            ComponentViewportAxisPlacement::fixed_from_start(24, 216)
                .expect("the authored rail width is nonzero"),
            ComponentViewportAxisPlacement::stretch_between(104, 72),
        )
    }

    pub fn evidence_border_allocation(self) -> ComponentAllocationMeasurementContract {
        let _ = self;
        viewport_region(
            ComponentViewportAxisPlacement::fixed_from_start(23, 218)
                .expect("the authored rail border width is nonzero"),
            ComponentViewportAxisPlacement::stretch_between(103, 71),
        )
    }

    pub fn service_stage_allocation(self) -> ComponentAllocationMeasurementContract {
        let _ = self;
        viewport_region(
            ComponentViewportAxisPlacement::stretch_between(264, 24),
            ComponentViewportAxisPlacement::stretch_between(104, 72),
        )
    }

    pub fn service_border_allocation(self) -> ComponentAllocationMeasurementContract {
        let _ = self;
        viewport_region(
            ComponentViewportAxisPlacement::stretch_between(263, 23),
            ComponentViewportAxisPlacement::stretch_between(103, 71),
        )
    }

    pub fn status_band_allocation(self) -> ComponentAllocationMeasurementContract {
        let _ = self;
        viewport_region(
            ComponentViewportAxisPlacement::stretch_between(24, 24),
            ComponentViewportAxisPlacement::fixed_from_end(24, 24)
                .expect("the authored status height is nonzero"),
        )
    }
}

fn viewport_region(
    horizontal: ComponentViewportAxisPlacement,
    vertical: ComponentViewportAxisPlacement,
) -> ComponentAllocationMeasurementContract {
    ComponentAllocationMeasurementContract::viewport_region(ComponentViewportRegion::new(
        horizontal, vertical,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_and_resized_compositions_keep_fixed_gutters_and_absorb_stage_extent() {
        let default =
            PlatformPulseCompositionLayout::for_extent(PlatformPulseCompositionExtent::DEFAULT)
                .unwrap();
        let resized =
            PlatformPulseCompositionLayout::for_extent(PlatformPulseCompositionExtent::RESIZED)
                .unwrap();

        assert_eq!(
            default.masthead(),
            PlatformPulseLogicalRect::new(24, 24, 912, 56)
        );
        assert_eq!(
            default.evidence_rail(),
            PlatformPulseLogicalRect::new(24, 104, 216, 424)
        );
        assert_eq!(
            default.service_stage(),
            PlatformPulseLogicalRect::new(264, 104, 672, 424)
        );
        assert_eq!(
            default.status_band(),
            PlatformPulseLogicalRect::new(24, 552, 912, 24)
        );
        assert_eq!(
            resized.masthead(),
            PlatformPulseLogicalRect::new(24, 24, 1_072, 56)
        );
        assert_eq!(
            resized.evidence_rail(),
            PlatformPulseLogicalRect::new(24, 104, 216, 524)
        );
        assert_eq!(
            resized.service_stage(),
            PlatformPulseLogicalRect::new(264, 104, 832, 524)
        );
        assert_eq!(
            resized.status_band(),
            PlatformPulseLogicalRect::new(24, 652, 1_072, 24)
        );
    }

    #[test]
    fn courtroom_extent_is_bounded_instead_of_silently_scaling_to_any_window() {
        assert_eq!(
            PlatformPulseCompositionLayout::for_extent(PlatformPulseCompositionExtent::new(
                959, 600,
            )),
            Err(PlatformPulseCompositionLayoutDenial::WidthOutsideProductCourtroom),
        );
        assert_eq!(
            PlatformPulseCompositionLayout::for_extent(PlatformPulseCompositionExtent::new(
                960, 701,
            )),
            Err(PlatformPulseCompositionLayoutDenial::HeightOutsideProductCourtroom),
        );
    }

    #[test]
    fn product_regions_lower_to_real_viewport_allocation_contracts() {
        let layout =
            PlatformPulseCompositionLayout::for_extent(PlatformPulseCompositionExtent::DEFAULT)
                .unwrap();
        assert!(matches!(
            layout.evidence_rail_allocation(),
            ComponentAllocationMeasurementContract::ViewportRegion(region)
                if matches!(
                    region.horizontal(),
                    ComponentViewportAxisPlacement::FixedFromStart {
                        start_logical_points: 24,
                        extent_logical_points: 216,
                    }
                ) && matches!(
                    region.vertical(),
                    ComponentViewportAxisPlacement::StretchBetween {
                        start_logical_points: 104,
                        end_logical_points: 72,
                    }
                )
        ));
        assert!(matches!(
            layout.service_stage_allocation(),
            ComponentAllocationMeasurementContract::ViewportRegion(region)
                if matches!(
                    region.horizontal(),
                    ComponentViewportAxisPlacement::StretchBetween {
                        start_logical_points: 264,
                        end_logical_points: 24,
                    }
                )
        ));
    }
}
use worth_ui::facade::declaration::{
    ComponentAllocationMeasurementContract, ComponentViewportAxisPlacement, ComponentViewportRegion,
};
