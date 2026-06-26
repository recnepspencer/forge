use super::{
    certification::WorthUiCounterCaptureRichness, counter_family::WorthUiRuntimeCounterFamily,
    frame_cost_counter::WorthUiFrameCostCounter, measurement_boundary::WorthUiMeasurementBoundary,
    query_evidence::WorthUiMeasurementQueryEvidence,
};

pub fn packet_digest(
    family: WorthUiRuntimeCounterFamily,
    boundary: WorthUiMeasurementBoundary,
    capture_richness: WorthUiCounterCaptureRichness,
    active_plan_digest: u64,
    rows: &[WorthUiFrameCostCounter],
    query_evidence: &[WorthUiMeasurementQueryEvidence],
) -> u64 {
    let mut state = 0xcbf2_9ce4_8422_2325_u64;
    fold_text(&mut state, family.token());
    fold_text(&mut state, boundary.token());
    fold_text(&mut state, capture_richness_token(capture_richness));
    fold_u64(&mut state, active_plan_digest);
    for row in rows {
        fold_text(&mut state, row.name());
        fold_u64(&mut state, row.value());
        fold_text(&mut state, value_kind_token(row.value_kind()));
        fold_text(&mut state, &format!("{:?}", row.work_class()));
    }
    for evidence in query_evidence {
        fold_text(&mut state, &format!("{:?}", evidence.kind()));
        fold_u64(&mut state, evidence.evidence_digest());
    }
    state
}

fn capture_richness_token(richness: WorthUiCounterCaptureRichness) -> &'static str {
    match richness {
        WorthUiCounterCaptureRichness::Minimal => "minimal",
        WorthUiCounterCaptureRichness::Standard => "standard",
        WorthUiCounterCaptureRichness::Full => "full",
        WorthUiCounterCaptureRichness::Support => "support",
    }
}

fn value_kind_token(kind: super::frame_cost_counter::WorthUiCounterValueKind) -> &'static str {
    match kind {
        super::frame_cost_counter::WorthUiCounterValueKind::CountedWork => "counted-work",
        super::frame_cost_counter::WorthUiCounterValueKind::ElapsedTimeAuxiliary => {
            "elapsed-time-auxiliary"
        }
        super::frame_cost_counter::WorthUiCounterValueKind::UnattributedWorkBucket => {
            "unattributed-work-bucket"
        }
    }
}

fn fold_text(state: &mut u64, text: &str) {
    for byte in text.as_bytes() {
        *state ^= u64::from(*byte);
        *state = state.wrapping_mul(0x100_0000_01b3);
    }
}

fn fold_u64(state: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *state ^= u64::from(byte);
        *state = state.wrapping_mul(0x100_0000_01b3);
    }
}
