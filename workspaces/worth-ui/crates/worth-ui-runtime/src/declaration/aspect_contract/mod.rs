mod aspect_contract;
mod aspect_name;
mod consumed;
mod coverage_report;
mod denial;
mod published;

pub use aspect_contract::UiAspectContract;
pub use aspect_name::{UiAspectFamily, UiAspectName, UiAspectSemanticSlice};
pub use consumed::UiConsumedAspectContract;
pub use coverage_report::{UiAspectCoverageEntry, UiAspectCoverageReport};
pub(crate) use denial::UiAspectContractAdmission;
pub use denial::UiAspectContractAdmissionDenial;
pub use published::UiPublishedAspectContract;

fn digest_aspect_names(aspects: &[UiAspectName]) -> u64 {
    aspects
        .iter()
        .fold(0x9E37_79B9_7F4A_7C15, |digest, aspect| {
            digest.rotate_left(5) ^ stable_text_digest(aspect.digest_text())
        })
}

fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}
