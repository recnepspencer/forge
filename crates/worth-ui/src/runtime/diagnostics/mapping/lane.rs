use crate::runtime::diagnostics::{
    WorthUiDiagnosticSource, WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode,
    WorthUiRuntimeDiagnosticFamily,
};
use crate::runtime::WorthUiLaneAdmissionDenial;

pub(crate) fn diagnostic_for_lane_admission(
    denial: &WorthUiLaneAdmissionDenial,
) -> WorthUiRuntimeDiagnostic {
    let evidence_digest = lane_admission_digest(denial);
    WorthUiRuntimeDiagnostic::new(
        WorthUiRuntimeDiagnosticFamily::LaneAdmission,
        WorthUiRuntimeDiagnosticCode::LaneAdmissionDenied,
        WorthUiDiagnosticSource::LaneAdmission {
            lane: denial.lane(),
            evidence_digest,
        },
        Some(evidence_digest),
    )
}

fn lane_admission_digest(denial: &WorthUiLaneAdmissionDenial) -> u64 {
    fold(
        fold(0xB0_00_00_01, lane_admission_reason_digest(denial.reason())),
        denial.lane().map(|lane| lane.canonical_tag()).unwrap_or(0),
    )
}

fn lane_admission_reason_digest(reason: crate::runtime::WorthUiLaneAdmissionDenialReason) -> u64 {
    match reason {
        crate::runtime::WorthUiLaneAdmissionDenialReason::UnsupportedLaneReference => 1,
        crate::runtime::WorthUiLaneAdmissionDenialReason::PrivateComponentLaneClaim => 2,
        crate::runtime::WorthUiLaneAdmissionDenialReason::MissingQuerySupportLinks => 3,
    }
}

fn fold(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x100000001b3)
}
