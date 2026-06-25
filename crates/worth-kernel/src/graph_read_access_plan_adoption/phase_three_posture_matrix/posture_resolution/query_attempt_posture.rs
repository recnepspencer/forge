use crate::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionPostureRow;

use super::resolved_posture::{
    WorthGraphReadAccessResolvedPosture, WorthGraphReadAccessResolvedPostureInput,
};

pub(crate) fn resolve_query_attempt_posture(
    row: &WorthGraphReadAccessPlanAdoptionPostureRow,
) -> WorthGraphReadAccessResolvedPosture {
    WorthGraphReadAccessResolvedPosture::from_input(WorthGraphReadAccessResolvedPostureInput {
        requirement_identity: row.requirement_row_digest().to_string(),
        posture_family: row.posture_kind().as_str().to_string(),
        source_attempt_digest: Some(row.source_attempt_digest().to_string()),
        source_carried_gap_digest: None,
        source_pairing_digest: Some(row.source_pairing_digest().to_string()),
        source_requirement_record_digest: row.source_requirement_record_digest().to_string(),
        read_family_identity_digest: Some(row.read_family_identity_digest().to_string()),
        requirement_row_digest: Some(row.requirement_row_digest().to_string()),
        query_family_name: Some(row.query_family_name().to_string()),
        query_family_digest_seed: row.query_family_digest_seed().to_string(),
        read_family_target: Some(row.read_family_target().to_string()),
        query_posture: row.query_posture().to_string(),
        denial_kind: row.denial_kind().map(str::to_string),
        owner: None,
        expected_denial: row.denial_kind().map(str::to_string),
        suggested_posture: Some(row.query_posture().to_string()),
        blocker: row.blocker().map(str::to_string),
        removal_trigger: row.removal_trigger().map(str::to_string),
    })
}
