use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::boundary_position::PlanarBooleanSplitBoundaryPosition;
use super::decision_record::PlanarBooleanEndpointContactDecision;
use super::normalized_schedule::PlanarBooleanEndpointBoundaryNormalizedSplitSchedule;

pub(super) struct EndpointContactDecisionIdentityBasis<'a> {
    pub(super) normalized_cut_identity: &'a str,
    pub(super) duplicate_report_identity: &'a str,
    pub(super) boundary_position: PlanarBooleanSplitBoundaryPosition,
    pub(super) source_endpoint_identity: &'a str,
    pub(super) projected_endpoint_fact_identity: &'a str,
    pub(super) provenance_entry_identities: &'a [String],
    pub(super) event_group_identities: &'a [String],
}

pub(super) fn endpoint_contact_decision_identity(
    basis: EndpointContactDecisionIdentityBasis<'_>,
) -> String {
    let mut parts = vec![
        "planar-boolean-endpoint-contact-decision".to_string(),
        format!("normalized-cut:{}", basis.normalized_cut_identity),
        format!("duplicate-report:{}", basis.duplicate_report_identity),
        format!("boundary-position:{}", basis.boundary_position.as_str()),
        format!("source-endpoint:{}", basis.source_endpoint_identity),
        format!(
            "projected-endpoint:{}",
            basis.projected_endpoint_fact_identity
        ),
    ];
    parts.extend(
        basis
            .provenance_entry_identities
            .iter()
            .map(|identity| format!("provenance:{identity}")),
    );
    parts.extend(
        basis
            .event_group_identities
            .iter()
            .map(|identity| format!("event-group:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn endpoint_boundary_schedule_identity(
    normalized_schedule_identity: &str,
    fragment_cut_identities: &[String],
    decisions: &[PlanarBooleanEndpointContactDecision],
    retained_interval_entry_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-endpoint-boundary-normalized-schedule".to_string(),
        format!("normalized-schedule:{normalized_schedule_identity}"),
    ];
    parts.extend(
        fragment_cut_identities
            .iter()
            .map(|identity| format!("fragment-cut:{identity}")),
    );
    parts.extend(
        decisions
            .iter()
            .map(|decision| format!("decision:{}", decision.decision_identity())),
    );
    parts.extend(
        retained_interval_entry_identities
            .iter()
            .map(|identity| format!("retained-interval:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn endpoint_boundary_schedule_set_identity(
    normalized_schedule_set_identity: &str,
    schedules: &[PlanarBooleanEndpointBoundaryNormalizedSplitSchedule],
) -> String {
    let mut parts = vec![
        "planar-boolean-endpoint-boundary-normalized-schedule-set".to_string(),
        format!("normalized-schedule-set:{normalized_schedule_set_identity}"),
    ];
    parts.extend(
        schedules
            .iter()
            .map(|schedule| format!("schedule:{}", schedule.schedule_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
