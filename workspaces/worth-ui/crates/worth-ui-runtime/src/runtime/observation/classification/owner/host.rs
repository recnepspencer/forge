use crate::fact_contract::{
    UiHostDeviceScaleChangedFact, UiHostViewportChangedFact, UiProducedFact,
};

pub(in crate::runtime::observation::classification) fn classify(
    observation: super::super::super::admission::UiHostObservation,
) -> Result<UiProducedFact, super::super::UiChangeClassificationDenial> {
    let report = observation.report().ok_or(
        super::super::UiChangeClassificationDenial::MissingHostReport {
            family: observation.family(),
        },
    )?;
    match report.report().payload() {
        worth_ui_host_contract::UiHostObservationPayload::Viewport {
            width_subpixels,
            height_subpixels,
        } => Ok(UiProducedFact::HostViewport(
            UiHostViewportChangedFact::new(*width_subpixels, *height_subpixels),
        )),
        worth_ui_host_contract::UiHostObservationPayload::DeviceScale { micros } => Ok(
            UiProducedFact::HostDeviceScale(UiHostDeviceScaleChangedFact::new(*micros)),
        ),
        _ => unreachable!("host admission seals only supported semantic families"),
    }
}
