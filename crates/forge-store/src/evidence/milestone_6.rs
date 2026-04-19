use crate::{
    layout::{
        chunk_membership_artifact_id, layout_scope_membership_artifact_id, stable_layout_digest,
        structural_block_artifact_id,
    },
    AdmittedAspectLayoutReadPlan, ChunkModelFrozenPhysicalLayout, DedupAdmittedBlockReuse,
    Milestone6LayoutMaterialization, Milestone6LayoutSupportLane,
    Milestone6LayoutSupportPublicationDisposition, Milestone6ResolvedLayoutSupportLane,
    Milestone7IndependentLayoutReference, Milestone9PhysicalChunkReference,
};
use forge_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;

use super::StoreCounterSnapshot;
use crate::media::DurableBackendFamily;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6LayoutReadReport {
    pub strategy: crate::AspectReadRegime,
    pub scope_class: String,
    pub complexity_status: crate::ComplexityStatus,
    pub fallback_class: crate::AspectLayoutFallbackClass,
    pub layout_slices_read: usize,
    pub blocks_decoded: usize,
    pub control_replay_breadth: usize,
    pub chunk_count: usize,
}

impl From<&AdmittedAspectLayoutReadPlan> for Milestone6LayoutReadReport {
    fn from(plan: &AdmittedAspectLayoutReadPlan) -> Self {
        Self {
            strategy: plan.performance().strategy,
            scope_class: plan.performance().scope_class.clone(),
            complexity_status: plan.performance().complexity_status,
            fallback_class: plan.performance().fallback_class,
            layout_slices_read: plan.performance().layout_slices_read,
            blocks_decoded: plan.performance().blocks_decoded,
            control_replay_breadth: plan.performance().control_replay_breadth,
            chunk_count: plan.performance().chunk_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6PhysicalLayoutReport {
    pub branch_id: BranchId,
    pub frontier_commit_id: CommitId,
    pub scope_class: String,
    pub projection_digest: String,
    pub slice_ids: Vec<String>,
    pub structural_block_id: String,
    pub equivalence_contract_version: u32,
    pub physical_chunk_id: String,
    pub chunk_shape_version: u32,
    pub chunk_width: u64,
    pub determinism_digest: String,
    pub milestone_9_chunk_member_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Milestone6CertificationOrigin {
    ReconstructedWitness,
    PersistedMaterialization,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6LayoutMaterializationReport {
    pub artifact_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6CounterContract {
    pub aspect_layout_plan_count: u64,
    pub aspect_layout_admitted_count: u64,
    pub aspect_layout_fallback_count: u64,
    pub aspect_layout_rejected_count: u64,
    pub aspect_layout_slice_read_count: u64,
    pub aspect_layout_block_decode_count: u64,
    pub aspect_layout_control_replay_breadth: u64,
    pub aspect_layout_whole_state_fallback_count: u64,
    pub structural_block_lookup_count: u64,
    pub structural_block_reuse_admission_count: u64,
    pub structural_block_reuse_hit_count: u64,
    pub structural_block_reuse_miss_count: u64,
    pub chunk_model_freeze_count: u64,
    pub physical_chunk_export_count: u64,
    pub physical_chunk_width_count: u64,
    pub physical_chunk_determinism_violation_count: u64,
    pub milestone_6_proof_only_prepare_count: u64,
    pub milestone_6_on_demand_materialize_count: u64,
    pub milestone_6_policy_eager_resolution_count: u64,
    pub milestone_6_policy_eager_publish_count: u64,
    pub milestone_6_policy_eager_reuse_existing_count: u64,
    pub milestone_7_layout_reference_admission_count: u64,
    pub milestone_9_physical_chunk_reference_admission_count: u64,
}

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
    fn verified(proof_basis: impl Into<String>) -> Self {
        Self {
            status: crate::ComplexityStatus::Verified,
            proof_basis: Some(proof_basis.into()),
            debt_reason: None,
        }
    }

    fn debt(debt_reason: impl Into<String>) -> Self {
        Self {
            status: crate::ComplexityStatus::Debt,
            proof_basis: None,
            debt_reason: Some(debt_reason.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6AccessStructureContract {
    pub backend_family: DurableBackendFamily,
    pub aspect_layout_read: Milestone6AccessStructureClaim,
    pub structural_block_reuse: Milestone6AccessStructureClaim,
    pub chunk_model_freeze: Milestone6AccessStructureClaim,
    pub milestone_7_layout_reference: Milestone6AccessStructureClaim,
    pub milestone_9_physical_chunk_reference: Milestone6AccessStructureClaim,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6AccessStructureClaim {
    pub access_structure: String,
    pub guarantee: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6AccessStructureVerification {
    pub backend_family: DurableBackendFamily,
    pub aspect_layout_read: Milestone6AccessStructureVerificationPath,
    pub structural_block_reuse: Milestone6AccessStructureVerificationPath,
    pub chunk_model_freeze: Milestone6AccessStructureVerificationPath,
    pub milestone_7_layout_reference: Milestone6AccessStructureVerificationPath,
    pub milestone_9_physical_chunk_reference: Milestone6AccessStructureVerificationPath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6AccessStructureVerificationPath {
    pub verified_at_open: bool,
    pub verification_basis: Option<String>,
    pub verification_gap: Option<String>,
}

impl Milestone6AccessStructureVerificationPath {
    pub(crate) fn verified(verification_basis: impl Into<String>) -> Self {
        Self {
            verified_at_open: true,
            verification_basis: Some(verification_basis.into()),
            verification_gap: None,
        }
    }

    pub(crate) fn debt(verification_gap: impl Into<String>) -> Self {
        Self {
            verified_at_open: false,
            verification_basis: None,
            verification_gap: Some(verification_gap.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6CertificationSummary {
    pub verified_path_count: u64,
    pub debt_path_count: u64,
    pub fallback_free_admission: bool,
    pub deterministic_chunk_freeze: bool,
    pub milestone_7_boundary_isolated: bool,
    pub milestone_9_boundary_isolated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6CertificationBundle {
    pub truth_digest: String,
    pub artifact_digest: String,
    pub diagnostics_digest: String,
    pub requested_layout_support_lane: Milestone6LayoutSupportLane,
    pub resolved_layout_support_lane: Milestone6ResolvedLayoutSupportLane,
    pub layout_support_publication_disposition: Milestone6LayoutSupportPublicationDisposition,
    pub certification_origin: Milestone6CertificationOrigin,
    pub layout_materialization_report: Option<Milestone6LayoutMaterializationReport>,
    pub certification_summary: Milestone6CertificationSummary,
    pub access_structure_contract: Milestone6AccessStructureContract,
    pub access_structure_verification: Milestone6AccessStructureVerification,
    pub layout_read_report: Milestone6LayoutReadReport,
    pub physical_layout_report: Milestone6PhysicalLayoutReport,
    pub complexity_status: Milestone6ComplexitySurface,
    pub counter_contract: Milestone6CounterContract,
    pub counter_snapshot: StoreCounterSnapshot,
}

impl Milestone6CertificationBundle {
    pub fn new(
        plan: &AdmittedAspectLayoutReadPlan,
        reuse: &DedupAdmittedBlockReuse,
        frozen: &ChunkModelFrozenPhysicalLayout,
        milestone_7: &Milestone7IndependentLayoutReference,
        milestone_9: &Milestone9PhysicalChunkReference,
        access_structure_verification: Milestone6AccessStructureVerification,
        counter_snapshot: StoreCounterSnapshot,
    ) -> Self {
        Self::build(
            plan,
            reuse,
            frozen,
            milestone_7,
            milestone_9,
            access_structure_verification,
            counter_snapshot,
            Milestone6LayoutSupportLane::ProofOnly,
            Milestone6ResolvedLayoutSupportLane::ProofOnly,
            Milestone6LayoutSupportPublicationDisposition::None,
            Milestone6CertificationOrigin::ReconstructedWitness,
            None,
        )
    }

    pub fn for_lane(
        plan: &AdmittedAspectLayoutReadPlan,
        reuse: &DedupAdmittedBlockReuse,
        frozen: &ChunkModelFrozenPhysicalLayout,
        milestone_7: &Milestone7IndependentLayoutReference,
        milestone_9: &Milestone9PhysicalChunkReference,
        access_structure_verification: Milestone6AccessStructureVerification,
        counter_snapshot: StoreCounterSnapshot,
        requested_layout_support_lane: Milestone6LayoutSupportLane,
        resolved_layout_support_lane: Milestone6ResolvedLayoutSupportLane,
        layout_support_publication_disposition: Milestone6LayoutSupportPublicationDisposition,
    ) -> Self {
        Self::build(
            plan,
            reuse,
            frozen,
            milestone_7,
            milestone_9,
            access_structure_verification,
            counter_snapshot,
            requested_layout_support_lane,
            resolved_layout_support_lane,
            layout_support_publication_disposition,
            Milestone6CertificationOrigin::ReconstructedWitness,
            None,
        )
    }

    fn build(
        plan: &AdmittedAspectLayoutReadPlan,
        reuse: &DedupAdmittedBlockReuse,
        frozen: &ChunkModelFrozenPhysicalLayout,
        milestone_7: &Milestone7IndependentLayoutReference,
        milestone_9: &Milestone9PhysicalChunkReference,
        access_structure_verification: Milestone6AccessStructureVerification,
        counter_snapshot: StoreCounterSnapshot,
        requested_layout_support_lane: Milestone6LayoutSupportLane,
        resolved_layout_support_lane: Milestone6ResolvedLayoutSupportLane,
        layout_support_publication_disposition: Milestone6LayoutSupportPublicationDisposition,
        certification_origin: Milestone6CertificationOrigin,
        layout_materialization_report: Option<Milestone6LayoutMaterializationReport>,
    ) -> Self {
        let layout_read_report = Milestone6LayoutReadReport::from(plan);
        let physical_layout_report = Milestone6PhysicalLayoutReport {
            branch_id: milestone_7.branch_id().clone(),
            frontier_commit_id: milestone_7.frontier_commit_id(),
            scope_class: milestone_7.scope_class().to_string(),
            projection_digest: milestone_7.projection_digest().to_string(),
            slice_ids: reuse
                .slice_ids()
                .iter()
                .map(|slice_id| slice_id.as_str().to_string())
                .collect(),
            structural_block_id: reuse.structural_block_id().as_str().to_string(),
            equivalence_contract_version: reuse.equivalence_contract_version().value(),
            physical_chunk_id: milestone_9.physical_chunk_id().as_str().to_string(),
            chunk_shape_version: milestone_9.chunk_shape_version().value(),
            chunk_width: frozen.chunk_width(),
            determinism_digest: milestone_9.determinism_digest().to_string(),
            milestone_9_chunk_member_count: milestone_9.chunk_member_count(),
        };
        let truth_digest = stable_layout_digest(&Milestone6TruthDigestBasis {
            layout_read_report: &layout_read_report,
            physical_layout_report: &physical_layout_report,
        });
        let scope_membership_artifact_id = layout_scope_membership_artifact_id(plan.request())
            .expect("admitted milestone 6 plan should always yield a scope membership artifact id");
        let structural_block_artifact_id =
            structural_block_artifact_id(reuse.structural_block_id()).to_string();
        let chunk_membership_artifact_id = chunk_membership_artifact_id(frozen).to_string();
        let artifact_digest = stable_layout_digest(&Milestone6ArtifactDigestBasis {
            layout_materialization_artifact_id: layout_materialization_report
                .as_ref()
                .map(|report| report.artifact_id.as_str()),
            scope_membership_artifact_id: &scope_membership_artifact_id,
            structural_block_artifact_id: &structural_block_artifact_id,
            chunk_membership_artifact_id: &chunk_membership_artifact_id,
        });
        let counter_contract = Milestone6CounterContract {
            aspect_layout_plan_count: counter_snapshot.aspect_layout_plan_count,
            aspect_layout_admitted_count: counter_snapshot.aspect_layout_admitted_count,
            aspect_layout_fallback_count: counter_snapshot.aspect_layout_fallback_count,
            aspect_layout_rejected_count: counter_snapshot.aspect_layout_rejected_count,
            aspect_layout_slice_read_count: counter_snapshot.aspect_layout_slice_read_count,
            aspect_layout_block_decode_count: counter_snapshot.aspect_layout_block_decode_count,
            aspect_layout_control_replay_breadth: counter_snapshot
                .aspect_layout_control_replay_breadth,
            aspect_layout_whole_state_fallback_count: counter_snapshot
                .aspect_layout_whole_state_fallback_count,
            structural_block_lookup_count: counter_snapshot.structural_block_lookup_count,
            structural_block_reuse_admission_count: counter_snapshot
                .structural_block_reuse_admission_count,
            structural_block_reuse_hit_count: counter_snapshot.structural_block_reuse_hit_count,
            structural_block_reuse_miss_count: counter_snapshot.structural_block_reuse_miss_count,
            chunk_model_freeze_count: counter_snapshot.chunk_model_freeze_count,
            physical_chunk_export_count: counter_snapshot.physical_chunk_export_count,
            physical_chunk_width_count: counter_snapshot.physical_chunk_width_count,
            physical_chunk_determinism_violation_count: counter_snapshot
                .physical_chunk_determinism_violation_count,
            milestone_6_proof_only_prepare_count: counter_snapshot
                .milestone_6_proof_only_prepare_count,
            milestone_6_on_demand_materialize_count: counter_snapshot
                .milestone_6_on_demand_materialize_count,
            milestone_6_policy_eager_resolution_count: counter_snapshot
                .milestone_6_policy_eager_resolution_count,
            milestone_6_policy_eager_publish_count: counter_snapshot
                .milestone_6_policy_eager_publish_count,
            milestone_6_policy_eager_reuse_existing_count: counter_snapshot
                .milestone_6_policy_eager_reuse_existing_count,
            milestone_7_layout_reference_admission_count: counter_snapshot
                .milestone_7_layout_reference_admission_count,
            milestone_9_physical_chunk_reference_admission_count: counter_snapshot
                .milestone_9_physical_chunk_reference_admission_count,
        };
        let access_structure_contract = Milestone6AccessStructureContract::for_backend_family(
            access_structure_verification.backend_family,
        );
        let complexity_status = Milestone6ComplexitySurface::derive(
            resolved_layout_support_lane,
            &layout_read_report,
            &physical_layout_report,
            &access_structure_contract,
            &access_structure_verification,
            &counter_contract,
            layout_materialization_report.as_ref(),
        );
        let certification_summary = Milestone6CertificationSummary::from_surface(
            &layout_read_report,
            &physical_layout_report,
            &complexity_status,
        );
        let diagnostics_digest = stable_layout_digest(&Milestone6DiagnosticsDigestBasis {
            requested_layout_support_lane: &requested_layout_support_lane,
            resolved_layout_support_lane: &resolved_layout_support_lane,
            layout_support_publication_disposition: &layout_support_publication_disposition,
            certification_origin: &certification_origin,
            layout_materialization_report: &layout_materialization_report,
            access_structure_contract: &access_structure_contract,
            access_structure_verification: &access_structure_verification,
            complexity_status: &complexity_status,
            counter_contract: &counter_contract,
            certification_summary: &certification_summary,
        });
        Self {
            truth_digest,
            artifact_digest,
            diagnostics_digest,
            requested_layout_support_lane,
            resolved_layout_support_lane,
            layout_support_publication_disposition,
            certification_origin,
            layout_materialization_report,
            certification_summary,
            access_structure_contract,
            access_structure_verification,
            layout_read_report,
            physical_layout_report,
            complexity_status,
            counter_contract,
            counter_snapshot,
        }
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("milestone 6 certification serialization")
    }

    pub fn from_materialization(
        materialization: &Milestone6LayoutMaterialization,
        access_structure_verification: Milestone6AccessStructureVerification,
        counter_snapshot: StoreCounterSnapshot,
    ) -> Self {
        Self::from_materialization_in_lane(
            materialization,
            access_structure_verification,
            counter_snapshot,
            Milestone6LayoutSupportLane::OnDemandMaterialized,
            Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized,
            Milestone6LayoutSupportPublicationDisposition::ReusedExisting,
        )
    }

    pub fn from_materialization_in_lane(
        materialization: &Milestone6LayoutMaterialization,
        access_structure_verification: Milestone6AccessStructureVerification,
        counter_snapshot: StoreCounterSnapshot,
        requested_layout_support_lane: Milestone6LayoutSupportLane,
        resolved_layout_support_lane: Milestone6ResolvedLayoutSupportLane,
        layout_support_publication_disposition: Milestone6LayoutSupportPublicationDisposition,
    ) -> Self {
        Self::build(
            materialization.admitted_plan(),
            materialization.block_reuse(),
            materialization.frozen_layout(),
            materialization.milestone_7_reference(),
            materialization.milestone_9_reference(),
            access_structure_verification,
            counter_snapshot,
            requested_layout_support_lane,
            resolved_layout_support_lane,
            layout_support_publication_disposition,
            Milestone6CertificationOrigin::PersistedMaterialization,
            Some(Milestone6LayoutMaterializationReport {
                artifact_id: materialization.artifact_id().to_string(),
            }),
        )
    }
}

impl Milestone6ComplexitySurface {
    fn derive(
        resolved_layout_support_lane: Milestone6ResolvedLayoutSupportLane,
        layout_read_report: &Milestone6LayoutReadReport,
        physical_layout_report: &Milestone6PhysicalLayoutReport,
        access_structure_contract: &Milestone6AccessStructureContract,
        access_structure_verification: &Milestone6AccessStructureVerification,
        counter_contract: &Milestone6CounterContract,
        layout_materialization_report: Option<&Milestone6LayoutMaterializationReport>,
    ) -> Self {
        let has_published_materialization = layout_materialization_report.is_some();
        let proof_only_lane =
            resolved_layout_support_lane == Milestone6ResolvedLayoutSupportLane::ProofOnly;
        let aspect_layout_read =
            if proof_only_lane {
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
                && access_structure_verification
                    .aspect_layout_read
                    .verified_at_open
            {
                Milestone6ComplexityPathStatus::verified(format!(
                "{}; {}; admitted aspect layout regime {:?}; scope {}; slices={}; blocks={}; {}",
                access_structure_contract.aspect_layout_read.access_structure,
                access_structure_contract.aspect_layout_read.guarantee,
                layout_read_report.strategy,
                layout_read_report.scope_class,
                layout_read_report.layout_slices_read,
                layout_read_report.blocks_decoded,
                access_structure_verification
                    .aspect_layout_read
                    .verification_basis
                    .as_deref()
                    .unwrap_or_default()
            ))
            } else {
                Milestone6ComplexityPathStatus::debt(path_debt_reason(
                    "aspect layout read",
                    counter_contract.aspect_layout_admitted_count
                        == counter_contract.aspect_layout_plan_count
                        && counter_contract.aspect_layout_fallback_count == 0
                        && counter_contract.aspect_layout_rejected_count == 0
                        && counter_contract.aspect_layout_slice_read_count
                            >= layout_read_report.layout_slices_read as u64
                        && counter_contract.aspect_layout_block_decode_count
                            >= layout_read_report.blocks_decoded as u64,
                    &access_structure_verification.aspect_layout_read,
                    "aspect layout admission counters do not prove a fallback-free admitted path",
                ))
            };

        let structural_block_reuse = if proof_only_lane {
            Milestone6ComplexityPathStatus::debt(
                "proof-only lane intentionally bypassed durable structural-block publication",
            )
        } else if (counter_contract.structural_block_reuse_admission_count
            >= counter_contract.aspect_layout_admitted_count
            || has_published_materialization)
            && !physical_layout_report.structural_block_id.is_empty()
            && access_structure_verification
                .structural_block_reuse
                .verified_at_open
        {
            Milestone6ComplexityPathStatus::verified(format!(
                "{}; {}; semantic block id {}; supporting hits={}; supporting misses={}; {}",
                access_structure_contract
                    .structural_block_reuse
                    .access_structure,
                access_structure_contract.structural_block_reuse.guarantee,
                physical_layout_report.structural_block_id,
                counter_contract.structural_block_reuse_hit_count,
                counter_contract.structural_block_reuse_miss_count,
                access_structure_verification
                    .structural_block_reuse
                    .verification_basis
                    .as_deref()
                    .unwrap_or_default()
            ))
        } else {
            Milestone6ComplexityPathStatus::debt(path_debt_reason(
                "structural block reuse",
                (counter_contract.structural_block_reuse_admission_count
                    >= counter_contract.aspect_layout_admitted_count
                    || has_published_materialization)
                    && !physical_layout_report.structural_block_id.is_empty(),
                &access_structure_verification.structural_block_reuse,
                "structural block reuse admissions do not cover admitted layout plans",
            ))
        };

        let chunk_model_freeze = if proof_only_lane {
            Milestone6ComplexityPathStatus::debt(
                "proof-only lane intentionally bypassed durable chunk-membership publication",
            )
        } else if (counter_contract.chunk_model_freeze_count
            >= counter_contract.aspect_layout_admitted_count
            || has_published_materialization)
            && physical_layout_report.chunk_width > 0
            && !physical_layout_report.determinism_digest.is_empty()
            && access_structure_verification
                .chunk_model_freeze
                .verified_at_open
        {
            Milestone6ComplexityPathStatus::verified(format!(
                "{}; {}; width {}; determinism digest {}; {}",
                access_structure_contract
                    .chunk_model_freeze
                    .access_structure,
                access_structure_contract.chunk_model_freeze.guarantee,
                physical_layout_report.chunk_width,
                physical_layout_report.determinism_digest,
                access_structure_verification
                    .chunk_model_freeze
                    .verification_basis
                    .as_deref()
                    .unwrap_or_default()
            ))
        } else {
            Milestone6ComplexityPathStatus::debt(path_debt_reason(
                "chunk model freeze",
                (counter_contract.chunk_model_freeze_count
                    >= counter_contract.aspect_layout_admitted_count
                    || has_published_materialization)
                    && physical_layout_report.chunk_width > 0
                    && !physical_layout_report.determinism_digest.is_empty(),
                &access_structure_verification.chunk_model_freeze,
                "chunk model freeze counters do not cover admitted layout plans",
            ))
        };

        let milestone_7_counter_basis = (counter_contract
            .milestone_7_layout_reference_admission_count
            >= counter_contract.aspect_layout_admitted_count
            || has_published_materialization)
            || proof_only_lane;
        let milestone_7_layout_reference = if milestone_7_counter_basis
            && !physical_layout_report.projection_digest.is_empty()
            && access_structure_verification
                .milestone_7_layout_reference
                .verified_at_open
        {
            Milestone6ComplexityPathStatus::verified(format!(
                "{}; {}; branch {} frontier {} projection {}; {}",
                access_structure_contract
                    .milestone_7_layout_reference
                    .access_structure,
                access_structure_contract
                    .milestone_7_layout_reference
                    .guarantee,
                physical_layout_report.branch_id.0,
                physical_layout_report.frontier_commit_id.0,
                physical_layout_report.projection_digest,
                access_structure_verification
                    .milestone_7_layout_reference
                    .verification_basis
                    .as_deref()
                    .unwrap_or_default()
            ))
        } else {
            Milestone6ComplexityPathStatus::debt(path_debt_reason(
                "milestone 7 layout reference",
                milestone_7_counter_basis && !physical_layout_report.projection_digest.is_empty(),
                &access_structure_verification.milestone_7_layout_reference,
                "milestone 7 layout reference admissions do not cover admitted layout plans",
            ))
        };

        let milestone_9_counter_basis = (counter_contract
            .milestone_9_physical_chunk_reference_admission_count
            >= counter_contract.aspect_layout_admitted_count
            || has_published_materialization)
            || proof_only_lane;
        let milestone_9_physical_chunk_reference = if milestone_9_counter_basis
            && physical_layout_report.milestone_9_chunk_member_count > 0
            && !physical_layout_report.physical_chunk_id.is_empty()
            && access_structure_verification
                .milestone_9_physical_chunk_reference
                .verified_at_open
        {
            Milestone6ComplexityPathStatus::verified(format!(
                "{}; {}; physical chunk {} members {}; {}",
                access_structure_contract
                    .milestone_9_physical_chunk_reference
                    .access_structure,
                access_structure_contract
                    .milestone_9_physical_chunk_reference
                    .guarantee,
                physical_layout_report.physical_chunk_id,
                physical_layout_report.milestone_9_chunk_member_count,
                access_structure_verification
                    .milestone_9_physical_chunk_reference
                    .verification_basis
                    .as_deref()
                    .unwrap_or_default()
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

impl Milestone6AccessStructureContract {
    fn for_backend_family(backend_family: DurableBackendFamily) -> Self {
        let backend_label = match backend_family {
            DurableBackendFamily::InMemory => "in-memory Milestone 6 derived layout registry",
            DurableBackendFamily::LocalFileAtomicRewrite => {
                "local-file Milestone 6 derived layout registry rebuilt atomically per write"
            }
            DurableBackendFamily::SqliteTransactional => {
                "sqlite Milestone 6 derived layout rows keyed by artifact id"
            }
        };
        Self {
            backend_family,
            aspect_layout_read: Milestone6AccessStructureClaim {
                access_structure: format!(
                    "{backend_label}: scope-to-slice membership records keyed by canonical target/scope/projection identity"
                ),
                guarantee: "admitted aspect layout reads are durably certified through explicit Milestone 6 scope membership records rather than only through the materialization blob".to_string(),
            },
            structural_block_reuse: Milestone6AccessStructureClaim {
                access_structure: format!(
                    "{backend_label}: semantic structural-block records keyed by cross-branch structural block identity"
                ),
                guarantee: "structural block reuse is durably certified through explicit semantic structural-block records carrying cross-branch block identity, equivalence version, canonical slice membership, and supporting layout publication references".to_string(),
            },
            chunk_model_freeze: Milestone6AccessStructureClaim {
                access_structure: format!(
                    "{backend_label}: chunk-membership records keyed by physical chunk identity"
                ),
                guarantee: "frozen chunk layout is durably certified through explicit Milestone 6 chunk-membership records rather than only through the materialization blob".to_string(),
            },
            milestone_7_layout_reference: Milestone6AccessStructureClaim {
                access_structure: format!(
                    "{backend_label}: compile-time milestone 7 reference wrappers derived from admitted layout proofs"
                ),
                guarantee:
                    "milestone 7 layout references never expose slice, block, or placement internals"
                        .to_string(),
            },
            milestone_9_physical_chunk_reference: Milestone6AccessStructureClaim {
                access_structure: format!(
                    "{backend_label}: compile-time milestone 9 chunk wrappers derived from deterministic chunk witnesses"
                ),
                guarantee:
                    "milestone 9 references expose only physical chunk identity plus determinism metadata, never authority or mutation rights"
                        .to_string(),
            },
        }
    }
}

impl Milestone6CertificationSummary {
    fn from_surface(
        layout_read_report: &Milestone6LayoutReadReport,
        physical_layout_report: &Milestone6PhysicalLayoutReport,
        complexity_status: &Milestone6ComplexitySurface,
    ) -> Self {
        let statuses = [
            &complexity_status.aspect_layout_read,
            &complexity_status.structural_block_reuse,
            &complexity_status.chunk_model_freeze,
            &complexity_status.milestone_7_layout_reference,
            &complexity_status.milestone_9_physical_chunk_reference,
        ];
        let verified_path_count = statuses
            .iter()
            .filter(|status| status.status == crate::ComplexityStatus::Verified)
            .count() as u64;
        let debt_path_count = statuses.len() as u64 - verified_path_count;
        Self {
            verified_path_count,
            debt_path_count,
            fallback_free_admission: layout_read_report.fallback_class
                == crate::AspectLayoutFallbackClass::None,
            deterministic_chunk_freeze: physical_layout_report.chunk_width > 0
                && !physical_layout_report.determinism_digest.is_empty(),
            milestone_7_boundary_isolated: complexity_status.milestone_7_layout_reference.status
                == crate::ComplexityStatus::Verified,
            milestone_9_boundary_isolated: complexity_status
                .milestone_9_physical_chunk_reference
                .status
                == crate::ComplexityStatus::Verified,
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

#[derive(Serialize)]
struct Milestone6TruthDigestBasis<'a> {
    layout_read_report: &'a Milestone6LayoutReadReport,
    physical_layout_report: &'a Milestone6PhysicalLayoutReport,
}

#[derive(Serialize)]
struct Milestone6ArtifactDigestBasis<'a> {
    layout_materialization_artifact_id: Option<&'a str>,
    scope_membership_artifact_id: &'a str,
    structural_block_artifact_id: &'a str,
    chunk_membership_artifact_id: &'a str,
}

#[derive(Serialize)]
struct Milestone6DiagnosticsDigestBasis<'a> {
    requested_layout_support_lane: &'a Milestone6LayoutSupportLane,
    resolved_layout_support_lane: &'a Milestone6ResolvedLayoutSupportLane,
    layout_support_publication_disposition: &'a Milestone6LayoutSupportPublicationDisposition,
    certification_origin: &'a Milestone6CertificationOrigin,
    layout_materialization_report: &'a Option<Milestone6LayoutMaterializationReport>,
    access_structure_contract: &'a Milestone6AccessStructureContract,
    access_structure_verification: &'a Milestone6AccessStructureVerification,
    complexity_status: &'a Milestone6ComplexitySurface,
    counter_contract: &'a Milestone6CounterContract,
    certification_summary: &'a Milestone6CertificationSummary,
}
