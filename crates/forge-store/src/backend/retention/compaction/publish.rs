use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        integrity::{
            compaction_product_artifact_id, retention_closure_artifact_id, stable_structural_digest,
        },
        records::{CompactionProductRecord, RetentionClosureRecord},
    },
    failure::StoreError,
    retention::{
        CompactionPlan, CompactionPublicationReport, PublishedCompactionProduct,
        RetainedReadCostSurface, RetainedReadPath, RetentionClosureSummary,
    },
};

use super::{super::basis::retention_basis_records_for_plan, helpers::rebuild_superseded_families};

pub(crate) fn publish_compaction_product<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    plan: CompactionPlan,
) -> Result<CompactionPublicationReport, StoreError> {
    let product_id =
        compaction_product_artifact_id(plan.retained_basis_label(), plan.family_labels());
    if let Some(existing) = backend
        .state()
        .compaction_product_records
        .get(&product_id)
        .cloned()
    {
        let product = PublishedCompactionProduct::new(
            existing.artifact_id.clone(),
            existing.retained_basis_label.clone(),
            existing.compacted_family_labels.clone(),
        );
        return Ok(CompactionPublicationReport::new(
            product,
            rebuild_superseded_families(&existing),
            RetainedReadCostSurface::new(
                RetainedReadPath::CompactionDerived,
                RetentionClosureSummary::from_witness(plan.closure_witness()),
                existing.compacted_family_labels.len() as u64,
                existing.rewritten_range_count,
                0,
                0,
                0,
            ),
        ));
    }

    let closure_record = RetentionClosureRecord {
        artifact_id: retention_closure_artifact_id(plan.retained_basis_label()),
        retained_basis_label: plan.retained_basis_label().to_string(),
        retained_head_branch_ids: plan
            .closure_witness()
            .retained_heads()
            .branch_ids()
            .to_vec(),
        stable_basis_labels: plan
            .closure_witness()
            .stable_bases()
            .basis_labels()
            .to_vec(),
        closure_commit_ids: plan.closure_witness().closure_commit_ids().to_vec(),
        frontier_commit_ids: plan.closure_witness().frontier_commit_ids().to_vec(),
        family_version: crate::RETENTION_FAMILY_VERSION,
    };

    let basis_records =
        retention_basis_records_for_plan(backend, plan.retained_basis_label(), &plan);
    let product = PublishedCompactionProduct::new(
        product_id.clone(),
        plan.retained_basis_label().to_string(),
        plan.family_labels().to_vec(),
    );
    let record = CompactionProductRecord {
        artifact_id: product_id.clone(),
        family_version: crate::COMPACTION_PRODUCT_FAMILY_VERSION,
        retained_basis_label: plan.retained_basis_label().to_string(),
        compacted_family_labels: plan.family_labels().to_vec(),
        product_digest: stable_structural_digest(&product)?,
        closure_record_artifact_id: closure_record.artifact_id.clone(),
        basis_record_artifact_ids: basis_records
            .iter()
            .map(|record| record.artifact_id.clone())
            .collect(),
        rewritten_range_count: plan.rewritten_range_count(),
        superseded_families: plan
            .superseded_families()
            .iter()
            .map(|family| family.family_label().to_string())
            .collect(),
        superseded_artifact_ids: plan
            .superseded_families()
            .iter()
            .map(|family| family.artifact_id().to_string())
            .collect(),
        parity_verified: false,
        cutover_committed: false,
    };

    let mut next = backend.state().clone();
    next.retention_closure_records
        .insert(closure_record.artifact_id.clone(), closure_record);
    for basis_record in basis_records {
        next.retention_basis_records
            .insert(basis_record.artifact_id.clone(), basis_record);
    }
    next.compaction_product_records
        .insert(product_id.clone(), record);
    backend.commit_replacement_state(next)?;
    if plan
        .family_labels()
        .iter()
        .any(|label| label == "snapshot_family")
    {
        backend.counters().record_compacted_snapshot_families(1);
    }
    if plan
        .family_labels()
        .iter()
        .any(|label| label == "branch_delta_layer")
    {
        backend.counters().record_compacted_delta_layers(1);
    }
    if plan
        .family_labels()
        .iter()
        .any(|label| label.starts_with("milestone_6_"))
    {
        backend.counters().record_compacted_layout_families(1);
    }

    Ok(CompactionPublicationReport::new(
        product,
        plan.superseded_families().to_vec(),
        RetainedReadCostSurface::new(
            RetainedReadPath::CompactionDerived,
            RetentionClosureSummary::from_witness(plan.closure_witness()),
            plan.family_labels().len() as u64,
            plan.rewritten_range_count(),
            0,
            0,
            0,
        ),
    ))
}
