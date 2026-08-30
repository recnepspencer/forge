use worth_ui::facade::inspection::{
    UiClientPhysicalPixel, UiPixelsRequired, UiVisualPointAdjudication, UiVisualSnapshotReceipt,
};

pub struct PlatformPulseVisualPointObservation<'a> {
    pub(super) point: UiClientPhysicalPixel,
    pub(super) adjudication: &'a UiVisualPointAdjudication,
}

impl<'a> PlatformPulseVisualPointObservation<'a> {
    pub fn new(point: UiClientPhysicalPixel, adjudication: &'a UiVisualPointAdjudication) -> Self {
        Self {
            point,
            adjudication,
        }
    }
}

pub struct PlatformPulseVisualPointTraceInput<'a> {
    pub(super) receipt: &'a UiVisualSnapshotReceipt<UiPixelsRequired>,
    pub(super) target: PlatformPulseVisualPointObservation<'a>,
    pub(super) background: PlatformPulseVisualPointObservation<'a>,
}

impl<'a> PlatformPulseVisualPointTraceInput<'a> {
    pub fn new(
        receipt: &'a UiVisualSnapshotReceipt<UiPixelsRequired>,
        target: PlatformPulseVisualPointObservation<'a>,
        background: PlatformPulseVisualPointObservation<'a>,
    ) -> Self {
        Self {
            receipt,
            target,
            background,
        }
    }
}
