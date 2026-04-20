use serde::Serialize;

use super::{
    contracts::{
        Milestone6AccessStructureContract, Milestone6AccessStructureVerification,
        Milestone6AccessStructureVerificationPath, Milestone6CounterContract,
    },
    reports::{Milestone6LayoutReadReport, Milestone6PhysicalLayoutReport},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6ComplexitySurface {
    pub aspect_layout_read: Milestone6ComplexityPathStatus,
    pub structural_block_reuse: Milestone6ComplexityPathStatus,
    pub chunk_model_freeze: Milestone6ComplexityPathStatus,
    pub milestone_7_layout_reference: Milestone6ComplexityPathStatus,
    pub milestone_9_physical_chunk_reference: Milestone6ComplexityPathStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6ComplexityPathStatus {
    pub status: crate::ComplexityStatus,
    pub proof_basis: Option<String>,
    pub debt_reason: Option<String>,
}

impl Milestone6ComplexityPathStatus {
    pub(crate) fn verified(proof_basis: impl Into<String>) -> Self {
        Self { status: crate::ComplexityStatus::Verified, proof_basis: Some(proof_basis.into()), debt_reason: None }
    }
    pub(crate) fn debt(debt_reason: impl Into<String>) -> Self {
        Self { status: crate::ComplexityStatus::Debt, proof_basis: None, debt_reason: Some(debt_reason.into()) }
    }
}

impl Milestone6ComplexitySurface {
    pub(crate) fn derive(
        resolved_layout_support_lane: crate::Milestone6ResolvedLayoutSupportLane,
        layout_read_report: &Milestone6LayoutReadReport,
        physical_layout_report: &Milestone6PhysicalLayoutReport,
        access_structure_contract: &Milestone6AccessStructureContract,
        access_structure_verification: &Milestone6AccessStructureVerification,
        counter_contract: &Milestone6CounterContract,
        layout_materialization_report: Option<&crate::evidence::milestone_6::Milestone6LayoutMaterializationReport>,
    ) -> Self {
        let has_published_materialization = layout_materialization_report.is_some();
        let proof_only_lane =
            resolved_layout_support_lane == crate::Milestone6ResolvedLayoutSupportLane::ProofOnly;
        let aspect_layout_read = if proof_only_lane {
            Milestone6ComplexityPathStatus::debt(
                "proof-only lane intentionally bypassed durable aspect-layout publication",
            )
        } else if counter_contract.aspect_layout_admitted_count
            == counter_contract.aspect_layout_plan_count
            && counter_contract.aspect_layout_fallback_count == 0
            && counter_contract.aspect_layout_rejected_count == 0
            && counter_contract.aspect_layout_slice_read_count
                >= layout_read_report.layout_slices_read as u64
            && counter_contract.aspect_layout_block_decode_count
                >= layout_read_report.blocks_decoded as u64
            && access_structure_verification.aspect_layout_read.verified_at_open
        {
            Milestone6ComplexityPathStatus::verified(format!(
                "{}; {}; admitted aspect layout regime {:?}; scope {}; slices={}; blocks={}; {}",
                access_structure_contract.aspect_layout_read.access_structure,
                access_structure_contract.aspect_layout_read.guarantee,
                layout_read_report.strategy,
                layout_read_report.scope_class,
                layout_read_report.layout_slices_read,
                layout_read_report.blocks_decoded,
                access_structure_verification.aspect_layout_read.verification_basis.as_deref().unwrap_or_default()
            ))
        } else {
            Milestone6ComplexityPathStatus::debt(path_debt_reason(
                "aspect layout read",
                counter_contract.aspect_layout_admitted_count == counter_contract.aspect_layout_plan_count
                    && counter_contract.aspect_layout_fallback_count == 0
                    && counter_contract.aspect_layout_rejected_count == 0
                    && counter_contract.aspect_layout_slice_read_count >= layout_read_report.layout_slices_read as u64
                    && counter_contract.aspect_layout_block_decode_count >= layout_read_report.blocks_decoded as u64,
                &access_structure_verification.aspect_layout_read,
                "aspect layout admission counters do not prove a fallback-free admitted path",
            ))
        };

        let structural_block_reuse = if proof_only_lane {
            Milestone6ComplexityPathStatus::debt("proof-only lane intentionally bypassed durable structural-block publication")
        } else if (counter_contract.structural_block_reuse_admission_count >= counter_contract.aspect_layout_admitted_count || has_published_materialization)
            && !physical_layout_report.structural_block_id.is_empty()
            && access_structure_verification.structural_block_reuse.verified_at_open
        {
            Milestone6ComplexityPathStatus::verified(format!(
                "{}; {}; semantic block id {}; supporting hits={}; supporting misses={}; {}",
                access_structure_contract.structural_block_reuse.access_structure,
                access_structure_contract.structural_block_reuse.guarantee,
                physical_layout_report.structural_block_id,
                counter_contract.structural_block_reuse_hit_count,
                counter_contract.structural_block_reuse_miss_count,
                access_structure_verification.structural_block_reuse.verification_basis.as_deref().unwrap_or_default()
            ))
        } else {
            Milestone6ComplexityPathStatus::debt(path_debt_reason(
                "structural block reuse",
                (counter_contract.structural_block_reuse_admission_count >= counter_contract.aspect_layout_admitted_count || has_published_materialization)
                    && !physical_layout_report.structural_block_id.is_empty(),
                &access_structure_verification.structural_block_reuse,
                "structural block reuse admissions do not cover admitted layout plans",
            ))
        };

        let chunk_model_freeze = if proof_only_lane {
            Milestone6ComplexityPathStatus::debt("proof-only lane intentionally bypassed durable chunk-membership publication")
        } else if (counter_contract.chunk_model_freeze_count >= counter_contract.aspect_layout_admitted_count || has_published_materialization)
            && physical_layout_report.chunk_width > 0
            && !physical_layout_report.determinism_digest.is_empty()
            && access_structure_verification.chunk_model_freeze.verified_at_open
        {
            Milestone6ComplexityPathStatus::verified(format!(
                "{}; {}; width {}; determinism digest {}; {}",
                access_structure_contract.chunk_model_freeze.access_structure,
                access_structure_contract.chunk_model_freeze.guarantee,
                physical_layout_report.chunk_width,
                physical_layout_report.determinism_digest,
                access_structure_verification.chunk_model_freeze.verification_basis.as_deref().unwrap_or_default()
            ))
        } else {
            Milestone6ComplexityPathStatus::debt(path_debt_reason(
                "chunk model freeze",
                (counter_contract.chunk_model_freeze_count >= counter_contract.aspect_layout_admitted_count || has_published_materialization)
                    && physical_layout_report.chunk_width > 0
                    && !physical_layout_report.determinism_digest.is_empty(),
                &access_structure_verification.chunk_model_freeze,
                "chunk model freeze counters do not cover admitted layout plans",
            ))
        };

        let milestone_7_counter_basis =
            (counter_contract.milestone_7_layout_reference_admission_count
                >= counter_contract.aspect_layout_admitted_count
                || has_published_materialization)
                || proof_only_lane;
        let milestone_7_layout_reference = if milestone_7_counter_basis
            && !physical_layout_report.projection_digest.is_empty()
            && access_structure_verification.milestone_7_layout_reference.verified_at_open
        {
            Milestone6ComplexityPathStatus::verified(format!(
                "{}; {}; branch {} frontier {} projection {}; {}",
                access_structure_contract.milestone_7_layout_reference.access_structure,
                access_structure_contract.milestone_7_layout_reference.guarantee,
                physical_layout_report.branch_id.0,
                physical_layout_report.frontier_commit_id.0,
                physical_layout_report.projection_digest,
                access_structure_verification.milestone_7_layout_reference.verification_basis.as_deref().unwrap_or_default()
            ))
        } else {
            Milestone6ComplexityPathStatus::debt(path_debt_reason(
                "milestone 7 layout reference",
                milestone_7_counter_basis && !physical_layout_report.projection_digest.is_empty(),
                &access_structure_verification.milestone_7_layout_reference,
                "milestone 7 layout reference admissions do not cover admitted layout plans",
            ))
        };

        let milestone_9_counter_basis =
            (counter_contract.milestone_9_physical_chunk_reference_admission_count
                >= counter_contract.aspect_layout_admitted_count
                || has_published_materialization)
                || proof_only_lane;
        let milestone_9_physical_chunk_reference = if milestone_9_counter_basis
            && physical_layout_report.milestone_9_chunk_member_count > 0
            && !physical_layout_report.physical_chunk_id.is_empty()
            && access_structure_verification.milestone_9_physical_chunk_reference.verified_at_open
        {
            Milestone6ComplexityPathStatus::verified(format!(
                "{}; {}; physical chunk {} members {}; {}",
                access_structure_contract.milestone_9_physical_chunk_reference.access_structure,
                access_structure_contract.milestone_9_physical_chunk_reference.guarantee,
                physical_layout_report.physical_chunk_id,
                physical_layout_report.milestone_9_chunk_member_count,
                access_structure_verification.milestone_9_physical_chunk_reference.verification_basis.as_deref().unwrap_or_default()
            ))
        } else {
            Milestone6ComplexityPathStatus::debt(path_debt_reason(
                "milestone 9 physical chunk reference",
                milestone_9_counter_basis
                    && physical_layout_report.milestone_9_chunk_member_count > 0
                    && !physical_layout_report.physical_chunk_id.is_empty(),
                &access_structure_verification.milestone_9_physical_chunk_reference,
                "milestone 9 physical chunk references do not cover admitted layout plans",
            ))
        };

        Self {
            aspect_layout_read,
            structural_block_reuse,
            chunk_model_freeze,
            milestone_7_layout_reference,
            milestone_9_physical_chunk_reference,
        }
    }
}

fn path_debt_reason(
    path_name: &str,
    counters_support_claim: bool,
    verification: &Milestone6AccessStructureVerificationPath,
    fallback_reason: &str,
) -> String {
    if !counters_support_claim {
        return fallback_reason.to_string();
    }
    if let Some(gap) = verification.verification_gap.as_deref() {
        return format!("{path_name} access structure verification gap: {gap}");
    }
    fallback_reason.to_string()
}
