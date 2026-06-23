use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::ordered_schedule::{
    PlanarBooleanOrderedEdgeSplitSchedule, PlanarBooleanOrderedEdgeSplitScheduleEntry,
};

pub(crate) fn ordered_entry_identity(raw_entry_identity: &str, order_ordinal: usize) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-ordered-edge-split-schedule-entry".to_string(),
            format!("raw-entry:{raw_entry_identity}"),
            format!("ordinal:{order_ordinal}"),
        ],
    )
}

pub(crate) fn schedule_order_digest(
    raw_schedule_identity: &str,
    entries: &[PlanarBooleanOrderedEdgeSplitScheduleEntry],
) -> String {
    let mut parts = vec![
        "planar-boolean-edge-split-schedule-order-digest".to_string(),
        format!("raw-schedule:{raw_schedule_identity}"),
    ];
    for entry in entries {
        parts.push(format!("ordered-ordinal:{}", entry.order_ordinal()));
        parts.push(format!("raw-entry:{}", entry.raw_entry().entry_identity()));
        entry.order_key().append_digest_parts(&mut parts);
    }
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn ordered_schedule_identity(raw_schedule_identity: &str, order_digest: &str) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-ordered-edge-split-schedule".to_string(),
            format!("raw-schedule:{raw_schedule_identity}"),
            format!("order-digest:{order_digest}"),
        ],
    )
}

pub(crate) fn ordered_schedule_set_identity(
    raw_schedule_set_identity: &str,
    schedules: &[PlanarBooleanOrderedEdgeSplitSchedule],
) -> String {
    let mut parts = vec![
        "planar-boolean-ordered-edge-split-schedule-set".to_string(),
        format!("raw-schedule-set:{raw_schedule_set_identity}"),
    ];
    parts.extend(
        schedules
            .iter()
            .map(|schedule| format!("schedule:{}", schedule.schedule_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
