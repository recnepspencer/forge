use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessPlanAdoptionCarriedGapRow, WorthGraphReadAccessPlanAdoptionPostureKind,
};

use super::resolved_posture::{
    WorthGraphReadAccessResolvedPosture, WorthGraphReadAccessResolvedPostureInput,
};

pub(crate) fn resolve_carried_gap_posture(
    gap: &WorthGraphReadAccessPlanAdoptionCarriedGapRow,
) -> WorthGraphReadAccessResolvedPosture {
    WorthGraphReadAccessResolvedPosture::from_input(WorthGraphReadAccessResolvedPostureInput {
        requirement_identity: format!("carried_gap:{}", gap.source_gap_digest()),
        posture_family: WorthGraphReadAccessPlanAdoptionPostureKind::CarriedCapabilityGap
            .as_str()
            .to_string(),
        source_attempt_digest: None,
        source_carried_gap_digest: Some(gap.source_gap_digest().to_string()),
        source_pairing_digest: None,
        source_requirement_record_digest: gap.source_requirement_record_digest().to_string(),
        read_family_identity_digest: None,
        requirement_row_digest: None,
        query_family_name: None,
        query_family_digest_seed: gap.query_family_anchor_digest().to_string(),
        read_family_target: Some(gap.read_family_target().to_string()),
        query_posture: gap.suggested_posture().to_string(),
        denial_kind: Some(gap.expected_denial().to_string()),
        owner: Some(gap.owner()),
        expected_denial: Some(gap.expected_denial().to_string()),
        suggested_posture: Some(gap.suggested_posture().to_string()),
        blocker: Some(gap.blocker().to_string()),
        removal_trigger: Some(gap.removal_trigger().to_string()),
    })
}
