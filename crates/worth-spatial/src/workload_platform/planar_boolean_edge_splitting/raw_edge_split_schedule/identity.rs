use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::schedule::{PlanarBooleanRawEdgeSplitSchedule, PlanarBooleanRawEdgeSplitScheduleEntry};

pub(crate) fn raw_entry_identity(source_edge_identity: &str, candidate_identity: &str) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-raw-edge-split-schedule-entry".to_string(),
            format!("source-edge:{source_edge_identity}"),
            format!("candidate:{candidate_identity}"),
        ],
    )
}

pub(crate) fn raw_schedule_identity(
    source_edge_identity: &str,
    carrier_identity: &str,
    entries: &[PlanarBooleanRawEdgeSplitScheduleEntry],
) -> String {
    let mut parts = vec![
        "planar-boolean-raw-edge-split-schedule".to_string(),
        format!("source-edge:{source_edge_identity}"),
        format!("carrier:{carrier_identity}"),
    ];
    parts.extend(
        entries
            .iter()
            .map(|entry| format!("entry:{}", entry.entry_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn raw_schedule_set_identity(
    point_posture_set_identity: &str,
    interval_candidate_set_identity: &str,
    schedules: &[PlanarBooleanRawEdgeSplitSchedule],
) -> String {
    let mut parts = vec![
        "planar-boolean-raw-edge-split-schedule-set".to_string(),
        format!("point-posture-set:{point_posture_set_identity}"),
        format!("interval-candidate-set:{interval_candidate_set_identity}"),
    ];
    parts.extend(
        schedules
            .iter()
            .map(|schedule| format!("schedule:{}", schedule.schedule_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
