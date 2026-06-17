use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::decision_record::PlanarBooleanSplitVertexCoalescenceDecision;
use super::input_rows::{SplitVertexInputKind, SplitVertexInputRow};
use super::vertex_set::{
    PlanarBooleanSplitVertexIdentityRow, PlanarBooleanSplitVertexIdentitySchedule,
};

pub(super) fn split_vertex_identity(
    source_edge_identity: &str,
    carrier_identity: &str,
    parameter_bits: u64,
    local_frame_identity: &str,
    precision_basis_identity: &str,
    inputs: &[&SplitVertexInputRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-split-vertex".to_string(),
        format!("source-edge:{source_edge_identity}"),
        format!("carrier:{carrier_identity}"),
        format!("parameter-bits:{parameter_bits}"),
        format!("frame:{local_frame_identity}"),
        format!("precision:{precision_basis_identity}"),
    ];
    for input in inputs {
        parts.push(format!("input-kind:{}", input_kind_name(input.input_kind)));
        parts.push(format!("input:{}", input.input_identity));
        if let Some(point_cut_identity) = &input.point_cut_identity {
            parts.push(format!("point-cut:{point_cut_identity}"));
        }
        parts.extend(
            input
                .parameter_fact_identities
                .iter()
                .map(|identity| format!("parameter-fact:{identity}")),
        );
        if let Some(interval_subdivision_identity) = &input.interval_subdivision_identity {
            parts.push(format!(
                "interval-subdivision:{interval_subdivision_identity}"
            ));
        }
        if let Some(normalized_interval_identity) = &input.normalized_interval_identity {
            parts.push(format!(
                "normalized-interval:{normalized_interval_identity}"
            ));
        }
        if let Some(coordinate_fact_identity) = &input.coordinate_fact_identity {
            parts.push(format!("coordinate-fact:{coordinate_fact_identity}"));
        }
        parts.extend(
            input
                .provenance_identities
                .iter()
                .map(|identity| format!("provenance:{identity}")),
        );
        parts.extend(
            input
                .event_group_identities
                .iter()
                .map(|identity| format!("event-group:{identity}")),
        );
    }
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn coalescence_decision_identity(
    split_vertex_identity: &str,
    input_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-split-vertex-coalescence-decision".to_string(),
        format!("split-vertex:{split_vertex_identity}"),
    ];
    parts.extend(
        input_identities
            .iter()
            .map(|identity| format!("input:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn split_vertex_schedule_identity(
    interval_subdivision_schedule_identity: &str,
    vertices: &[PlanarBooleanSplitVertexIdentityRow],
    decisions: &[PlanarBooleanSplitVertexCoalescenceDecision],
) -> String {
    let mut parts = vec![
        "planar-boolean-split-vertex-identity-schedule".to_string(),
        format!("interval-subdivision-schedule:{interval_subdivision_schedule_identity}"),
    ];
    parts.extend(
        vertices
            .iter()
            .map(|vertex| format!("vertex:{}", vertex.split_vertex_identity())),
    );
    parts.extend(
        decisions
            .iter()
            .map(|decision| format!("decision:{}", decision.decision_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn split_vertex_schedule_set_identity(
    interval_subdivision_schedule_set_identity: &str,
    schedules: &[PlanarBooleanSplitVertexIdentitySchedule],
) -> String {
    let mut parts = vec![
        "planar-boolean-split-vertex-identity-schedule-set".to_string(),
        format!("interval-subdivision-schedule-set:{interval_subdivision_schedule_set_identity}"),
    ];
    parts.extend(
        schedules
            .iter()
            .map(|schedule| format!("schedule:{}", schedule.schedule_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn input_kind_name(kind: SplitVertexInputKind) -> &'static str {
    match kind {
        SplitVertexInputKind::PointCut => "point-cut",
        SplitVertexInputKind::IntervalStart => "interval-start",
        SplitVertexInputKind::IntervalEnd => "interval-end",
    }
}
