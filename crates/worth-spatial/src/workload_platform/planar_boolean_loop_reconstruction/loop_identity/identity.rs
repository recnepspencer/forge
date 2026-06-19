use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::row::{
    PlanarBooleanLoopIdentityRow, PlanarBooleanLoopPersistentNamePropagationRow,
    PlanarBooleanLoopSubshapeSignatureRow,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopClassifiedProductKind;

pub(crate) fn naming_support_identity(
    request_identity: &str,
    split_ledger_receipt_identity: &str,
    split_persistent_naming_receipt_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-loop-naming-authority-support".to_string(),
            format!("request:{request_identity}"),
            format!("split-ledger:{split_ledger_receipt_identity}"),
            format!("split-persistent-naming:{split_persistent_naming_receipt_identity}"),
        ],
    )
}

pub(crate) fn canonical_loop_identity(
    request_identity: &str,
    tracked_loop_identity: &str,
    loop_kind: PlanarBooleanLoopClassifiedProductKind,
    role_outcome_identity: &str,
    degenerate_outcome_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-canonical-loop-identity".to_string(),
            format!("request:{request_identity}"),
            format!("tracked-loop:{tracked_loop_identity}"),
            format!("loop-kind:{:?}", loop_kind),
            format!("role-outcome:{role_outcome_identity}"),
            format!("degenerate-outcome:{degenerate_outcome_identity}"),
        ],
    )
}

pub(crate) fn loop_identity_row_identity(
    canonical_loop_identity: &str,
    tracked_loop_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-loop-identity-row".to_string(),
            format!("canonical:{canonical_loop_identity}"),
            format!("tracked:{tracked_loop_identity}"),
        ],
    )
}

pub(crate) fn propagated_persistent_name_identity(
    canonical_loop_identity: &str,
    upstream_persistent_name_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-loop-persistent-name".to_string(),
            format!("canonical-loop:{canonical_loop_identity}"),
            format!("upstream-name:{upstream_persistent_name_identity}"),
        ],
    )
}

pub(crate) fn propagated_name_row_identity(
    canonical_loop_identity: &str,
    propagated_persistent_name_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-loop-persistent-name-row".to_string(),
            format!("canonical-loop:{canonical_loop_identity}"),
            format!("propagated-name:{propagated_persistent_name_identity}"),
        ],
    )
}

pub(crate) fn propagated_subshape_signature_identity(
    canonical_loop_identity: &str,
    upstream_artifact_identity: &str,
    upstream_signature_basis_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-loop-subshape-signature".to_string(),
            format!("canonical-loop:{canonical_loop_identity}"),
            format!("artifact:{upstream_artifact_identity}"),
            format!("upstream-basis:{upstream_signature_basis_identity}"),
        ],
    )
}

pub(crate) fn propagated_subshape_signature_row_identity(
    canonical_loop_identity: &str,
    propagated_signature_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-loop-subshape-signature-row".to_string(),
            format!("canonical-loop:{canonical_loop_identity}"),
            format!("signature:{propagated_signature_identity}"),
        ],
    )
}

pub(crate) fn loop_identity_map_identity(
    request_identity: &str,
    rows: &[PlanarBooleanLoopIdentityRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-identity-map".to_string(),
        format!("request:{request_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("identity-row:{}", row.row_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn loop_name_map_identity(
    request_identity: &str,
    rows: &[PlanarBooleanLoopPersistentNamePropagationRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-persistent-name-propagation-map".to_string(),
        format!("request:{request_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("name-row:{}", row.row_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn loop_signature_map_identity(
    request_identity: &str,
    rows: &[PlanarBooleanLoopSubshapeSignatureRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-subshape-signature-map".to_string(),
        format!("request:{request_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("signature-row:{}", row.row_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
