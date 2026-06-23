use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::normalized_cut::{
    PlanarBooleanNormalizedEdgeSplitSchedule, PlanarBooleanNormalizedSplitCut,
};

pub(crate) fn normalized_cut_identity(
    source_edge_identity: &str,
    carrier_identity: &str,
    parameter_bits: u64,
    kind_rank: u8,
    local_frame_identity: &str,
    precision_basis_identity: &str,
    provenance_identities: &[String],
    parameter_fact_identities: &[String],
    event_group_identities: &[String],
    exact_endpoint_source_identity: Option<&str>,
    exact_projected_endpoint_fact_identity: Option<&str>,
    shared_endpoint_source_identities: &[String],
    shared_endpoint_projection_fact_digests: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-normalized-split-cut".to_string(),
        format!("source-edge:{source_edge_identity}"),
        format!("carrier:{carrier_identity}"),
        format!("parameter-bits:{parameter_bits}"),
        format!("kind-rank:{kind_rank}"),
        format!("local-frame:{local_frame_identity}"),
        format!("precision-basis:{precision_basis_identity}"),
    ];
    parts.extend(
        provenance_identities
            .iter()
            .map(|identity| format!("provenance:{identity}")),
    );
    parts.extend(
        parameter_fact_identities
            .iter()
            .map(|identity| format!("parameter-fact:{identity}")),
    );
    parts.extend(
        event_group_identities
            .iter()
            .map(|identity| format!("event-group:{identity}")),
    );
    if let Some(identity) = exact_endpoint_source_identity {
        parts.push(format!("exact-source-endpoint:{identity}"));
    }
    if let Some(identity) = exact_projected_endpoint_fact_identity {
        parts.push(format!("exact-projected-endpoint:{identity}"));
    }
    parts.extend(
        shared_endpoint_source_identities
            .iter()
            .map(|identity| format!("shared-source-endpoint:{identity}")),
    );
    parts.extend(
        shared_endpoint_projection_fact_digests
            .iter()
            .map(|identity| format!("shared-projected-endpoint:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn duplicate_report_identity(
    normalized_cut_identity: &str,
    provenance_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-duplicate-split-report".to_string(),
        format!("normalized-cut:{normalized_cut_identity}"),
    ];
    parts.extend(
        provenance_identities
            .iter()
            .map(|identity| format!("provenance:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn normalized_schedule_identity(
    ordered_schedule_identity: &str,
    cuts: &[PlanarBooleanNormalizedSplitCut],
    retained_interval_entry_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-normalized-edge-split-schedule".to_string(),
        format!("ordered-schedule:{ordered_schedule_identity}"),
    ];
    parts.extend(cuts.iter().map(|cut| format!("cut:{}", cut.cut_identity())));
    parts.extend(cuts.iter().flat_map(|cut| {
        cut.event_group_identities()
            .iter()
            .map(|identity| format!("cut-event-group:{identity}"))
    }));
    parts.extend(
        retained_interval_entry_identities
            .iter()
            .map(|identity| format!("retained-interval:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn normalized_schedule_set_identity(
    ordered_schedule_set_identity: &str,
    schedules: &[PlanarBooleanNormalizedEdgeSplitSchedule],
) -> String {
    let mut parts = vec![
        "planar-boolean-normalized-edge-split-schedule-set".to_string(),
        format!("ordered-schedule-set:{ordered_schedule_set_identity}"),
    ];
    parts.extend(
        schedules
            .iter()
            .map(|schedule| format!("schedule:{}", schedule.schedule_identity())),
    );
    parts.extend(schedules.iter().flat_map(|schedule| {
        schedule
            .retained_interval_entry_identities()
            .iter()
            .map(|identity| format!("retained-interval:{identity}"))
    }));
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
