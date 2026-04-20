use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        integrity::stable_structural_digest,
    },
    failure::{StoreError, StoreErrorKind},
    retention::{PublishedCompactionProduct, RetainedReadCostSurface, RetainedReadPath, RetentionClosureSummary},
};

pub(crate) fn verify_compaction_product<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    product: PublishedCompactionProduct,
) -> Result<RetainedReadCostSurface, StoreError> {
    let mut next = backend.state().clone();
    let (closure_record_artifact_id, compacted_family_count, rewritten_range_count) = {
        let record = next.compaction_product_records.get_mut(product.product_id()).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::CompactionPlanBasisAmbiguous,
                format!(
                    "compaction product `{}` was not published before verification",
                    product.product_id()
                ),
            )
        })?;
        if record.product_digest != stable_structural_digest(&product)? {
            backend.counters().record_retention_truth_parity_failure();
            return Err(StoreError::new(
                StoreErrorKind::CompactionProductShadowAuthorityViolation,
                format!(
                    "compaction product `{}` digest no longer matches its published payload",
                    product.product_id()
                ),
            ));
        }
        let closure_record_artifact_id = record.closure_record_artifact_id.clone();
        if !next.retention_closure_records.contains_key(&closure_record_artifact_id) {
            backend.counters().record_retention_truth_parity_failure();
            return Err(StoreError::new(
                StoreErrorKind::CompactionPlanBasisAmbiguous,
                format!(
                    "compaction product `{}` is missing closure record `{}`",
                    product.product_id(),
                    closure_record_artifact_id
                ),
            ));
        }
        record.parity_verified = true;
        (
            closure_record_artifact_id,
            record.compacted_family_labels.len() as u64,
            record.rewritten_range_count,
        )
    };
    let closure_record = next.retention_closure_records.get(&closure_record_artifact_id).cloned().ok_or_else(|| {
        StoreError::new(
            StoreErrorKind::CompactionPlanBasisAmbiguous,
            "compaction closure record disappeared during verification",
        )
    })?;
    backend.commit_replacement_state(next)?;
    Ok(RetainedReadCostSurface::new(
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
    ))
}
