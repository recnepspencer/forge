use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::row::{PlanarBooleanAdmittedReconstructedLoop, PlanarBooleanBornLoop};

pub(crate) fn admitted_reconstructed_loop_identity(
    request_identity: &str,
    loop_candidate_identity: &str,
    source_loop_identity: &str,
    fragment_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-admitted-reconstructed-loop".to_string(),
        format!("request:{request_identity}"),
        format!("loop-candidate:{loop_candidate_identity}"),
        format!("source-loop:{source_loop_identity}"),
    ];
    parts.extend(
        fragment_identities
            .iter()
            .map(|identity| format!("fragment:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn born_loop_identity(
    request_identity: &str,
    loop_candidate_identity: &str,
    source_loop_identities: &[String],
    chain_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-born-loop".to_string(),
        format!("request:{request_identity}"),
        format!("loop-candidate:{loop_candidate_identity}"),
    ];
    parts.extend(
        source_loop_identities
            .iter()
            .map(|identity| format!("source-loop:{identity}")),
    );
    parts.extend(
        chain_identities
            .iter()
            .map(|identity| format!("overlap-chain:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn admitted_reconstructed_loop_set_identity(
    request_identity: &str,
    rows: &[PlanarBooleanAdmittedReconstructedLoop],
) -> String {
    let mut parts = vec![
        "planar-boolean-admitted-reconstructed-loop-set".to_string(),
        format!("request:{request_identity}"),
    ];
    parts.extend(rows.iter().map(|row| {
        format!(
            "admitted-reconstructed-loop:{}",
            row.reconstructed_loop_identity()
        )
    }));
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn born_loop_set_identity(
    request_identity: &str,
    rows: &[PlanarBooleanBornLoop],
) -> String {
    let mut parts = vec![
        "planar-boolean-born-loop-set".to_string(),
        format!("request:{request_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("born-loop:{}", row.born_loop_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
