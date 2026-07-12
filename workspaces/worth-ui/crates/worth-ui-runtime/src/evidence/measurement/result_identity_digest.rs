use super::UiMeasurementResult;

pub(crate) fn measurement_result_identity_digest(result: &UiMeasurementResult) -> u64 {
    stable_text_digest(result.evidence_category().as_str())
        ^ result.request_identity().as_u64().rotate_left(7)
        ^ result.evidence_generation().as_u64().rotate_left(13)
        ^ stable_text_digest(result.unit_posture().as_str()).rotate_left(17)
        ^ stable_text_digest(result.coordinate_space().as_str()).rotate_left(23)
        ^ stable_text_digest(result.rounding_posture().as_str()).rotate_left(29)
        ^ result
            .assumption_profile()
            .profile_identity_digest()
            .rotate_left(31)
        ^ measurement_value_digest(result.value()).rotate_left(37)
}

fn measurement_value_digest(value: &super::UiMeasurementValue) -> u64 {
    use super::UiMeasurementValue;
    match value {
        UiMeasurementValue::TextIntrinsicSize(value) => pair(value.width, value.height),
        UiMeasurementValue::TextBaselineMetrics(value) => {
            triple(value.ascent, value.descent, value.baseline)
        }
        UiMeasurementValue::FontMetrics(value) => {
            triple(value.ascent, value.descent, value.line_gap)
        }
        UiMeasurementValue::NativeControlIntrinsicSize(value) => pair(value.width, value.height),
        UiMeasurementValue::ViewportExtent(value) => pair(value.width, value.height),
        UiMeasurementValue::DpiScaleFactor(value) => u64::from(value.scale_factor.to_bits()),
        UiMeasurementValue::PortalAnchorRect(value) => {
            pair(value.x, value.y) ^ pair(value.width, value.height).rotate_left(29)
        }
        UiMeasurementValue::ScrollContainerViewport(value) => pair(value.width, value.height),
    }
}

fn pair(first: f32, second: f32) -> u64 {
    u64::from(first.to_bits()) ^ u64::from(second.to_bits()).rotate_left(32)
}

fn triple(first: f32, second: f32, third: f32) -> u64 {
    pair(first, second) ^ u64::from(third.to_bits()).rotate_left(17)
}

fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}
