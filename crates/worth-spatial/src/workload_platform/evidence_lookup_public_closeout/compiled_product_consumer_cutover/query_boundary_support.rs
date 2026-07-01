use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_lookup_public_closeout::EvidenceLookupPublicCloseoutFamilyStageRow;
use crate::workload_platform::evidence_lookup_query_consumer_kit::EvidenceLookupQueryConsumerKitCloseout;
use crate::workload_platform::evidence_lookup_query_surface_matrix::EvidenceLookupQuerySurfaceMatrixCloseout;

pub(super) fn compose_query_boundary_support_digest(
    family_stage_rows: &[EvidenceLookupPublicCloseoutFamilyStageRow],
    query_surface_matrix: &EvidenceLookupQuerySurfaceMatrixCloseout,
    query_consumer_kit: &EvidenceLookupQueryConsumerKitCloseout,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &family_stage_rows
            .iter()
            .filter_map(|row| {
                row.query_import_evidence_digest()
                    .map(|digest| format!("query-import:{digest}"))
            })
            .chain(family_stage_rows.iter().filter_map(|row| {
                row.topology_query_backed_cutover_digest()
                    .map(|digest| format!("topology-query-backed-cutover:{digest}"))
            }))
            .chain(family_stage_rows.iter().filter_map(|row| {
                row.topology_read_family_row_digest()
                    .map(|digest| format!("topology-read-family-row:{digest}"))
            }))
            .chain(std::iter::once(format!(
                "query-matrix:{}",
                query_surface_matrix.matrix_digest()
            )))
            .chain(std::iter::once(format!(
                "query-consumer-kit:{}",
                query_consumer_kit.closeout_digest()
            )))
            .chain(std::iter::once(format!(
                "query-support-rows:{}",
                query_consumer_kit.support_rows().len()
            )))
            .chain(std::iter::once(
                "worth-spatial:evidence-lookup-public-closeout-query-boundary-support:v1"
                    .to_string(),
            ))
            .collect::<Vec<_>>(),
    )
}
