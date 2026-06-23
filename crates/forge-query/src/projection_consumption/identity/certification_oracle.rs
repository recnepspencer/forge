use super::scope::{certification_scope_encoder, seal};
use crate::ForgeQueryEvidenceTag;

pub(crate) fn compose_oracle_parity_lane_digest(
    declaration_digest: &str,
    contract_digest: &str,
    fact_set_digest: &str,
    receipt_digest: &str,
) -> String {
    seal(
        certification_scope_encoder("projection_consumption_oracle_parity_lane_v1")
            .field_shape(
                ForgeQueryEvidenceTag::new("declaration"),
                declaration_digest,
            )
            .field_shape(ForgeQueryEvidenceTag::new("contract"), contract_digest)
            .field_shape(ForgeQueryEvidenceTag::new("fact_set"), fact_set_digest)
            .field_shape(ForgeQueryEvidenceTag::new("receipt"), receipt_digest),
    )
}

pub(crate) fn compose_oracle_comparison_row_digest(
    lane: &str,
    expected_digest: &str,
    actual_digest: &str,
    matched: bool,
) -> String {
    seal(
        certification_scope_encoder("projection_consumption_oracle_comparison_row_v1")
            .field_shape(ForgeQueryEvidenceTag::new("lane"), lane)
            .field_shape(ForgeQueryEvidenceTag::new("expected"), expected_digest)
            .field_shape(ForgeQueryEvidenceTag::new("actual"), actual_digest)
            .field_bool(ForgeQueryEvidenceTag::new("match"), matched),
    )
}

pub(crate) fn compose_oracle_manifest_row_digest(
    lane: &str,
    lane_name: &str,
    owner: &str,
    sources: &[&str],
    forbidden_helpers: &[&str],
    fields: &[&str],
) -> String {
    seal(
        certification_scope_encoder("projection_consumption_oracle_manifest_row_v1")
            .field_shape(ForgeQueryEvidenceTag::new("lane"), lane)
            .field_shape(ForgeQueryEvidenceTag::new("lane_name"), lane_name)
            .field_shape(ForgeQueryEvidenceTag::new("owner"), owner)
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("source"),
                sources.iter().copied(),
            )
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("forbidden_helper"),
                forbidden_helpers.iter().copied(),
            )
            .field_value_sequence(ForgeQueryEvidenceTag::new("field"), fields.iter().copied()),
    )
}

pub(crate) fn compose_oracle_report_digest(
    comparison_row_digests: impl IntoIterator<Item = impl AsRef<str>>,
    manifest_digest: &str,
) -> String {
    let comparison_rows = comparison_row_digests
        .into_iter()
        .map(|digest| digest.as_ref().to_string())
        .collect::<Vec<_>>();
    seal(
        certification_scope_encoder("projection_consumption_oracle_report_v1")
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("comparison_row"),
                comparison_rows,
            )
            .field_shape(ForgeQueryEvidenceTag::new("manifest"), manifest_digest),
    )
}
