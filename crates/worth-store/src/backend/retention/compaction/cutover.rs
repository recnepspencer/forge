use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    failure::{StoreError, StoreErrorKind},
    retention::{
        CompactionCutoverReport, PublishedCompactionProduct, RetainedReadCostSurface,
        RetainedReadPath, RetentionClosureSummary,
    },
};

use super::helpers::rebuild_superseded_families;

pub(crate) fn cutover_compaction_product<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    product: PublishedCompactionProduct,
) -> Result<CompactionCutoverReport, StoreError> {
    let mut next = backend.state().clone();
    let (
        retained_basis_label,
        artifact_id,
        closure_record_artifact_id,
        compacted_family_count,
        rewritten_range_count,
        superseded_families,
    ) = {
        let record = next
            .compaction_product_records
            .get_mut(product.product_id())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CompactionCutoverViolation,
                    format!(
                        "compaction product `{}` was not published before cutover",
                        product.product_id()
                    ),
                )
            })?;
        if !record.parity_verified {
            backend.counters().record_compaction_cutover_rejection();
            return Err(StoreError::new(
                StoreErrorKind::CompactionCutoverViolation,
                format!(
                    "compaction product `{}` must be parity-verified before cutover",
                    product.product_id()
                ),
            ));
        }
        record.cutover_committed = true;
        (
            record.retained_basis_label.clone(),
            record.artifact_id.clone(),
            record.closure_record_artifact_id.clone(),
            record.compacted_family_labels.len() as u64,
            record.rewritten_range_count,
            rebuild_superseded_families(record),
        )
    };
    let closure_record = next
        .retention_closure_records
        .get(&closure_record_artifact_id)
        .cloned()
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::CompactionPlanBasisAmbiguous,
                "compaction closure record disappeared during cutover",
            )
        })?;
    backend.commit_replacement_state(next)?;
    backend.counters().record_compaction_cutover();
    Ok(CompactionCutoverReport::new(
        crate::CompactionCutoverWitness::new(retained_basis_label, artifact_id),
        superseded_families,
        RetainedReadCostSurface::new(
            RetainedReadPath::CompactionDerived,
            RetentionClosureSummary::new(
                closure_record.retained_head_branch_ids.len() as u64,
                closure_record.stable_basis_labels.len() as u64,
                closure_record.closure_commit_ids.len() as u64,
                closure_record.frontier_commit_ids.len() as u64,
            ),
            compacted_family_count,
            rewritten_range_count,
            0,
            0,
            0,
        ),
    ))
}
