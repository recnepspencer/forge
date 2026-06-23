use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::naming_row::{
    PlanarBooleanSplitPersistentNameRow, PlanarBooleanSplitSelectorResolutionRow,
    PlanarBooleanSplitSubshapeSignatureRow,
};
use super::query_evolution::PlanarBooleanSplitIdentityEvolutionRow;

pub(super) fn naming_row_identity(
    source_edge_identity: &str,
    artifact_kind: &str,
    artifact_identity: &str,
    query_result_digest: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-split-persistent-name-row".to_string(),
            format!("source-edge:{source_edge_identity}"),
            format!("artifact-kind:{artifact_kind}"),
            format!("artifact:{artifact_identity}"),
            format!("query-result:{query_result_digest}"),
        ],
    )
}

pub(super) fn persistent_name_identity(
    source_edge_identity: &str,
    artifact_kind: &str,
    artifact_identity: &str,
    query_lineage_digest: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-split-persistent-name".to_string(),
            format!("source-edge:{source_edge_identity}"),
            format!("artifact-kind:{artifact_kind}"),
            format!("artifact:{artifact_identity}"),
            format!("query-lineage:{query_lineage_digest}"),
        ],
    )
}

pub(super) fn selector_resolution_row_identity(
    persistent_name_identity: &str,
    artifact_identity: &str,
    selector_basis_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-split-selector-resolution".to_string(),
            format!("persistent-name:{persistent_name_identity}"),
            format!("artifact:{artifact_identity}"),
            format!("selector-basis:{selector_basis_identity}"),
        ],
    )
}

pub(super) fn subshape_signature_row_identity(
    artifact_identity: &str,
    signature_basis_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-split-subshape-signature".to_string(),
            format!("artifact:{artifact_identity}"),
            format!("signature-basis:{signature_basis_identity}"),
        ],
    )
}

pub(super) fn receipt_identity(
    validation_receipt_identity: &str,
    evolution_rows: &[PlanarBooleanSplitIdentityEvolutionRow],
    naming_rows: &[PlanarBooleanSplitPersistentNameRow],
    selector_rows: &[PlanarBooleanSplitSelectorResolutionRow],
    signature_rows: &[PlanarBooleanSplitSubshapeSignatureRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-split-persistent-naming-receipt".to_string(),
        format!("validation:{validation_receipt_identity}"),
    ];
    parts.extend(
        evolution_rows
            .iter()
            .map(|row| format!("evolution:{}", row.result_digest())),
    );
    parts.extend(
        naming_rows
            .iter()
            .map(|row| format!("naming:{}", row.row_identity())),
    );
    parts.extend(
        selector_rows
            .iter()
            .map(|row| format!("selector:{}", row.row_identity())),
    );
    parts.extend(
        signature_rows
            .iter()
            .map(|row| format!("signature:{}", row.row_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
