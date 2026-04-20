use serde::Serialize;
use sha2::{Digest, Sha256};

use super::lowered_declarations::{
    LoweredCompactionDeclaration, LoweredRebuildDeclaration, LoweredReclaimDeclaration,
    LoweredRetentionMaintenanceBatch,
};
use crate::retention::plans::{CompactionPlan, RebuildDebtSummary};

pub(super) fn lower_compaction_declaration(plan: &CompactionPlan) -> LoweredCompactionDeclaration {
    LoweredCompactionDeclaration::new(
        plan.retained_basis_label().to_string(),
        plan.closure_witness().retained_heads().branch_ids().to_vec(),
        plan.closure_witness().stable_bases().basis_labels().to_vec(),
        plan.closure_witness().closure_commit_ids().to_vec(),
        plan.closure_witness().frontier_commit_ids().to_vec(),
        plan.family_labels().to_vec(),
        plan.superseded_families()
            .iter()
            .map(|family| {
                (
                    family.family_label().to_string(),
                    family.artifact_id().to_string(),
                    family.basis_commit_id(),
                )
            })
            .collect(),
        plan.rewritten_range_count(),
    )
}

pub(super) fn lower_reclaim_declaration(
    witness: &crate::ReclaimEligibilityWitness,
) -> LoweredReclaimDeclaration {
    LoweredReclaimDeclaration::Derived {
        retained_basis_label: witness.retained_basis_label().to_string(),
        artifact_family: witness.artifact_family().to_string(),
        artifact_id: witness.artifact_id().to_string(),
    }
}

pub(super) fn lower_authoritative_reclaim(
    range: &crate::PolicyExpiredAuthorityRange,
) -> LoweredReclaimDeclaration {
    LoweredReclaimDeclaration::Authoritative {
        branch_id: range.branch_id().clone(),
        oldest_retained_commit_id: range.oldest_retained_commit_id(),
        expired_commit_ids: range.expired_commit_ids().to_vec(),
    }
}

pub(super) fn lower_rebuild_declaration(debt: &RebuildDebtSummary) -> LoweredRebuildDeclaration {
    LoweredRebuildDeclaration::new(
        debt.retained_basis_label(),
        debt.family_label(),
        debt.rebuild_target_id(),
        debt.debt_reason(),
    )
}

pub(super) fn build_lowered_batch(
    closure_witness: &crate::RetentionClosureWitness,
    closure_summary: crate::retention::plans::RetentionClosureSummary,
    compaction_declarations: Vec<LoweredCompactionDeclaration>,
    reclaim_declarations: Vec<LoweredReclaimDeclaration>,
    rebuild_declarations: Vec<LoweredRebuildDeclaration>,
) -> LoweredRetentionMaintenanceBatch {
    let batch_label = maintenance_batch_digest((
        closure_witness,
        &compaction_declarations,
        &reclaim_declarations,
        &rebuild_declarations,
    ));
    LoweredRetentionMaintenanceBatch::new(
        format!("retention-maintenance:{batch_label}"),
        closure_summary,
        compaction_declarations,
        reclaim_declarations,
        rebuild_declarations,
    )
}

fn maintenance_batch_digest(value: impl Serialize) -> String {
    let json = serde_json::to_vec(&value).expect("retention maintenance batch digest");
    let mut hasher = Sha256::new();
    hasher.update(json);
    format!("{:x}", hasher.finalize())
}
