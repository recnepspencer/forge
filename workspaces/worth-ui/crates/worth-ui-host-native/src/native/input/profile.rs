use super::super::UiNativeInputObservationStop;

#[derive(Clone, Copy)]
pub(crate) struct UiNativeEventProfile {
    pub(super) scale_factor: f64,
    pub(super) scale_micros: u32,
    pub(super) physical_size: [u32; 2],
}

pub(super) fn event_profile(
    scale_factor: f64,
    physical_size: [u32; 2],
) -> Result<UiNativeEventProfile, UiNativeInputObservationStop> {
    if !scale_factor.is_finite() || scale_factor <= 0.0 {
        return Err(UiNativeInputObservationStop::InvalidScale);
    }
    let micros = (scale_factor * 1_000_000.0).round();
    if !micros.is_finite() || !(1.0..=f64::from(u32::MAX)).contains(&micros) {
        return Err(UiNativeInputObservationStop::InvalidScale);
    }
    Ok(UiNativeEventProfile {
        scale_factor,
        scale_micros: micros as u32,
        physical_size,
    })
}

pub(super) fn logical_subpixels(value: u32, scale_factor: f64) -> i64 {
    (f64::from(value) / scale_factor * 1_000.0).round() as i64
}
