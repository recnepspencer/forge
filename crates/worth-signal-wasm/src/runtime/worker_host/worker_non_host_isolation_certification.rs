use crate::boundary::errors::WorthSignalJsError;

use super::{
    canonical_worker_certification_digest, WorkerCommittedTransactionEnvelope,
    WorkerGraphPublicationSummary, WorkerRuntimeNonHostIsolationReport,
};

impl WorkerRuntimeNonHostIsolationReport {
    pub(crate) fn from_certified_worker_run(
        independent_region_recipe_ids: &[String],
        published_recipe_ids: &[String],
        transaction_op_count: u64,
        worker_envelope: &WorkerCommittedTransactionEnvelope,
        publication_summary: &WorkerGraphPublicationSummary,
    ) -> Result<Self, WorthSignalJsError> {
        let declared_independent_region_count = independent_region_recipe_ids.len() as u64;
        let all_declared_regions_were_published =
            independent_region_recipe_ids.iter().all(|region_id| {
                published_recipe_ids
                    .iter()
                    .any(|recipe_id| recipe_id == region_id)
            });
        let all_regions_remain_worker_owned = all_declared_regions_were_published
            && publication_summary.published_recipe_count >= declared_independent_region_count
            && publication_summary.denied_callback_count == 0;
        let placement_frontier_digest =
            canonical_worker_certification_digest(&independent_region_recipe_ids)?;
        let worker_breadth_digest = canonical_worker_certification_digest(&(
            publication_summary.published_source_count,
            publication_summary.published_recipe_count,
            worker_envelope.run_summary.touched_nodes,
            worker_envelope.run_summary.nodes_evaluated,
            worker_envelope.run_summary.nodes_recomputed,
        ))?;
        let main_thread_hosted_digest =
            canonical_worker_certification_digest(&("mainThreadHostedBoundaryAbsent", 0_u64))?;

        Ok(Self {
            declared_independent_region_count,
            declared_independent_region_recipe_ids: independent_region_recipe_ids.to_vec(),
            worker_admitted_source_count: publication_summary.published_source_count,
            worker_admitted_recipe_count: publication_summary.published_recipe_count,
            transaction_op_count,
            worker_touched_node_count: worker_envelope.run_summary.touched_nodes,
            worker_evaluated_node_count: worker_envelope.run_summary.nodes_evaluated,
            worker_recomputed_node_count: worker_envelope.run_summary.nodes_recomputed,
            all_regions_remain_worker_owned,
            broad_placement_collapse_detected: !all_regions_remain_worker_owned,
            placement_frontier_digest,
            worker_breadth_digest,
            main_thread_hosted_digest,
            broadening_denial_artifact: if all_regions_remain_worker_owned {
                "noBroadeningDetected".to_owned()
            } else {
                "workerRegionPublicationMismatch".to_owned()
            },
        })
    }
}
