use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::row::{PlanarBooleanLoopContainmentEvidencePosture, PlanarBooleanLoopRoleOutcome};

pub(crate) fn role_outcome_identity(
    request_identity: &str,
    loop_identity: &str,
    source_loop_identities: &[String],
    island_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-role-outcome".to_string(),
        format!("request:{request_identity}"),
        format!("loop:{loop_identity}"),
    ];
    parts.extend(
        source_loop_identities
            .iter()
            .map(|identity| format!("source-loop:{identity}")),
    );
    parts.extend(
        island_identities
            .iter()
            .map(|identity| format!("island:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn containment_posture_identity(
    request_identity: &str,
    loop_identity: &str,
    source_loop_identities: &[String],
    island_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-containment-evidence-posture".to_string(),
        format!("request:{request_identity}"),
        format!("loop:{loop_identity}"),
    ];
    parts.extend(
        source_loop_identities
            .iter()
            .map(|identity| format!("source-loop:{identity}")),
    );
    parts.extend(
        island_identities
            .iter()
            .map(|identity| format!("island:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn role_outcome_set_identity(
    request_identity: &str,
    rows: &[PlanarBooleanLoopRoleOutcome],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-role-outcome-set".to_string(),
        format!("request:{request_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("role-outcome:{}", row.role_outcome_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn containment_posture_set_identity(
    request_identity: &str,
    rows: &[PlanarBooleanLoopContainmentEvidencePosture],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-containment-evidence-posture-set".to_string(),
        format!("request:{request_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("containment-posture:{}", row.containment_posture_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
