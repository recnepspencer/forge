use crate::identity::ResultDigest;
use crate::memory_workspace::ForgeQueryEntity;
use crate::relationship_proof::{RelationshipProofSupportProfile, RelationshipProofSupportStatus};

pub(super) fn materialized_result_digest(
    query_digest: &str,
    basis_digest: &str,
    rows: &[ForgeQueryEntity],
) -> ResultDigest {
    ResultDigest::from_parts(
        &rows
            .iter()
            .flat_map(|row| {
                let external_row = serde_json::to_string(row.external_row())
                    .expect("read external row should serialize");
                [
                    format!("row_identity:{}", row.identity()),
                    format!("external_row:{external_row}"),
                ]
            })
            .chain(std::iter::once(format!("query:{query_digest}")))
            .chain(std::iter::once(format!("basis:{basis_digest}")))
            .collect::<Vec<_>>(),
    )
}

pub(super) fn relationship_proof_support_surface_count(
    profile: Option<&RelationshipProofSupportProfile>,
    status: RelationshipProofSupportStatus,
) -> usize {
    profile
        .map(|profile| {
            profile
                .surfaces()
                .iter()
                .filter(|(_, surface_status)| *surface_status == status)
                .count()
        })
        .unwrap_or(0)
}
