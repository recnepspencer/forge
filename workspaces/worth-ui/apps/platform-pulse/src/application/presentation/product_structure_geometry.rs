use worth_ui::facade::declaration::{
    ComponentAllocationMeasurementContract, ComponentViewportAxisPlacement, ComponentViewportRegion,
};

pub(super) fn viewport_rect(
    horizontal: ComponentViewportAxisPlacement,
    vertical: ComponentViewportAxisPlacement,
) -> ComponentAllocationMeasurementContract {
    ComponentAllocationMeasurementContract::viewport_region(ComponentViewportRegion::new(
        horizontal, vertical,
    ))
}

pub(super) fn fixed_start(start: u16, extent: u16) -> ComponentViewportAxisPlacement {
    ComponentViewportAxisPlacement::fixed_from_start(start, extent)
        .expect("Pulse Mosaic extent is nonzero")
}

pub(super) fn fixed_end(end: u16, extent: u16) -> ComponentViewportAxisPlacement {
    ComponentViewportAxisPlacement::fixed_from_end(end, extent)
        .expect("Pulse Mosaic extent is nonzero")
}

pub(super) fn stretch(start: u16, end: u16) -> ComponentViewportAxisPlacement {
    ComponentViewportAxisPlacement::stretch_between(start, end)
}
