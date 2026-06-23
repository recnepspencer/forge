use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;

use super::action::PlanarBooleanMicroIntervalAction;
use super::span_grouping::IntervalSubdivisionGroupKey;
use super::subdivision_row::{
    PlanarBooleanIntervalSubdivisionNormalizedSchedule,
    PlanarBooleanNormalizedIntervalSubdivisionRow,
};

pub(super) fn interval_subdivision_identity(
    endpoint_schedule_identity: &str,
    key: &IntervalSubdivisionGroupKey,
    provenance_entry_identities: &[String],
    event_group_identities: &[String],
    action: PlanarBooleanMicroIntervalAction,
) -> String {
    let mut parts = vec![
        "planar-boolean-normalized-interval-subdivision".to_string(),
        format!("endpoint-schedule:{endpoint_schedule_identity}"),
        format!("interval-event:{}", key.interval_event_identity()),
        format!("source-edge:{}", key.source_edge_identity()),
        format!("carrier:{}", key.carrier_identity()),
        format!("range-start:{}", canonical_parameter_bits(key.range()[0])),
        format!("range-end:{}", canonical_parameter_bits(key.range()[1])),
        format!("source-interval:{}", key.source_interval_identity()),
        format!(
            "source-range-start:{}",
            canonical_parameter_bits(key.source_range()[0])
        ),
        format!(
            "source-range-end:{}",
            canonical_parameter_bits(key.source_range()[1])
        ),
        format!("source-sense:{}", source_sense_name(key.source_sense())),
        format!("frame:{}", key.local_frame_identity()),
        format!("precision:{}", key.precision_basis_identity()),
        format!("normalized-interval:{}", key.normalized_interval_identity()),
        format!(
            "normalized-range-start:{}",
            canonical_parameter_bits(key.normalized_range()[0])
        ),
        format!(
            "normalized-range-end:{}",
            canonical_parameter_bits(key.normalized_range()[1])
        ),
        format!("action:{}", micro_interval_action_name(action)),
    ];
    parts.extend(
        provenance_entry_identities
            .iter()
            .map(|identity| format!("provenance:{identity}")),
    );
    parts.extend(
        event_group_identities
            .iter()
            .map(|identity| format!("event-group:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn interval_subdivision_schedule_identity(
    endpoint_schedule_identity: &str,
    subdivisions: &[PlanarBooleanNormalizedIntervalSubdivisionRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-interval-subdivision-normalized-schedule".to_string(),
        format!("endpoint-schedule:{endpoint_schedule_identity}"),
    ];
    parts.extend(
        subdivisions
            .iter()
            .map(|row| format!("subdivision:{}", row.subdivision_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn interval_subdivision_schedule_set_identity(
    endpoint_schedule_set_identity: &str,
    schedules: &[PlanarBooleanIntervalSubdivisionNormalizedSchedule],
) -> String {
    let mut parts = vec![
        "planar-boolean-interval-subdivision-normalized-schedule-set".to_string(),
        format!("endpoint-schedule-set:{endpoint_schedule_set_identity}"),
    ];
    parts.extend(
        schedules
            .iter()
            .map(|schedule| format!("schedule:{}", schedule.schedule_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn source_sense_name(sense: PlanarBooleanSourceIntervalSense) -> &'static str {
    match sense {
        PlanarBooleanSourceIntervalSense::Forward => "forward",
        PlanarBooleanSourceIntervalSense::Reversed => "reversed",
    }
}

pub(super) fn micro_interval_action_name(action: PlanarBooleanMicroIntervalAction) -> &'static str {
    match action {
        PlanarBooleanMicroIntervalAction::Retain => "retain",
        PlanarBooleanMicroIntervalAction::AdmittedCollapse => "admitted-collapse",
        PlanarBooleanMicroIntervalAction::PolicyRequired => "policy-required",
    }
}
