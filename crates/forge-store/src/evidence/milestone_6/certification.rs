use serde::Serialize;

use super::{
    complexity::Milestone6ComplexitySurface,
    contracts::{
        Milestone6AccessStructureContract, Milestone6AccessStructureVerification,
        Milestone6CounterContract,
    },
    reports::{Milestone6LayoutReadReport, Milestone6PhysicalLayoutReport},
};
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

use super::super::StoreCounterSnapshot;

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
            plan, reuse, frozen, milestone_7, milestone_9, access_structure_verification,
            counter_snapshot, requested_layout_support_lane, resolved_layout_support_lane,
            layout_support_publication_disposition,
            Milestone6CertificationOrigin::ReconstructedWitness, None,
        )
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

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("milestone 6 certification serialization")
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
        let physical_layout_report =
            Milestone6PhysicalLayoutReport::from_references(reuse, frozen, milestone_7, milestone_9);
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
        let counter_contract = Milestone6CounterContract::from_snapshot(&counter_snapshot);
        let access_structure_contract =
            Milestone6AccessStructureContract::for_backend_family(access_structure_verification.backend_family);
        let complexity_status = Milestone6ComplexitySurface::derive(
            resolved_layout_support_lane,
            &layout_read_report,
            &physical_layout_report,
            &access_structure_contract,
            &access_structure_verification,
            &counter_contract,
            layout_materialization_report.as_ref(),
        );
        let certification_summary =
            Milestone6CertificationSummary::from_surface(&layout_read_report, &physical_layout_report, &complexity_status);
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
            fallback_free_admission: layout_read_report.fallback_class == crate::AspectLayoutFallbackClass::None,
            deterministic_chunk_freeze: physical_layout_report.chunk_width > 0
                && !physical_layout_report.determinism_digest.is_empty(),
            milestone_7_boundary_isolated: complexity_status.milestone_7_layout_reference.status
                == crate::ComplexityStatus::Verified,
            milestone_9_boundary_isolated: complexity_status.milestone_9_physical_chunk_reference.status
                == crate::ComplexityStatus::Verified,
        }
    }
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
