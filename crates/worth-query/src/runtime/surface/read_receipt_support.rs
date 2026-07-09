use crate::identity::ResultDigest;
use crate::memory_workspace::WorthQueryEntity;
use crate::relationship_proof::{RelationshipProofSupportProfile, RelationshipProofSupportStatus};

pub(super) fn materialized_result_digest(
    query_digest: &str,
    basis_digest: &str,
    rows: &[WorthQueryEntity],
) -> ResultDigest {
    ResultDigest::from_parts(
        &rows
            .iter()
            .flat_map(|row| {
                std::iter::once(format!(
                    "row_identity:{}",
                    row.identity().terminal_projection_for_reporting()
                ))
                .chain(row.terminal_result_digest_parts())
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
