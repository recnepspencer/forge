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
}

fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}
