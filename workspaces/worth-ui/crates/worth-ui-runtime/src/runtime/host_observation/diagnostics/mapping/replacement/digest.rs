use crate::runtime::host_observation::diagnostics::{
    WorthUiDiagnosticSource, WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode,
    WorthUiRuntimeDiagnosticFamily,
};

pub(super) fn phase_denial_diagnostic(
    family: WorthUiRuntimeDiagnosticFamily,
    code: WorthUiRuntimeDiagnosticCode,
    evidence_digest: u64,
) -> WorthUiRuntimeDiagnostic {
    WorthUiRuntimeDiagnostic::new(
        family,
        code,
        WorthUiDiagnosticSource::PhaseDenial { evidence_digest },
        Some(evidence_digest),
    )
}

pub(super) fn runtime_posture_digest(
    posture: crate::runtime::WorthUiRuntimeReplacementPosture,
) -> u64 {
    match posture {
        crate::runtime::WorthUiRuntimeReplacementPosture::Supported => 1,
        crate::runtime::WorthUiRuntimeReplacementPosture::Deferred => 2,
        crate::runtime::WorthUiRuntimeReplacementPosture::Unsupported => 3,
    }
}

pub(super) fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}

pub(super) fn fold(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x100000001b3)
}
